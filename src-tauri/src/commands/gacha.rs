use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    commands::ApiResponse,
    config::{load_or_create_app_config_state, AppConfig},
    error::{AppError, AppResult},
    gacha_analysis::model::{AnalysisData, PoolAnalysisSummary, PoolFile},
    gacha_analysis::{analyze_pool_file, build_pool_analysis_summaries},
    gacha_merge::merge_pool_files,
    gacha_merge::model::PoolMergeResult,
    gacha_storage::{load_pool_file_from_path, save_pool_file_to_path},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeLocalPoolRequest {
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeLocalPoolResponse {
    pub file_path: String,
    pub analysis_list: Vec<AnalysisData>,
    pub summary_list: Vec<PoolAnalysisSummary>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeLocalPoolRequest {
    pub old_file_path: Option<String>,
    pub new_file_path: String,
    pub output_file_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeLocalPoolResponse {
    pub old_file_path: Option<String>,
    pub new_file_path: String,
    pub output_file_path: String,
    pub merge_result: PoolMergeResult,
    pub analysis_list: Vec<AnalysisData>,
    pub summary_list: Vec<PoolAnalysisSummary>,
}

#[tauri::command]
pub fn analyze_local_pool(
    request: AnalyzeLocalPoolRequest,
) -> ApiResponse<AnalyzeLocalPoolResponse> {
    match analyze_local_pool_inner(request) {
        Ok(response) => {
            info!("analyze_local_pool 执行成功");
            ApiResponse::ok(response)
        }
        Err(error) => {
            error!(error = %error, "analyze_local_pool 执行失败");
            ApiResponse::err(error.to_string())
        }
    }
}

#[tauri::command]
pub fn merge_local_pool(request: MergeLocalPoolRequest) -> ApiResponse<MergeLocalPoolResponse> {
    match merge_local_pool_inner(request) {
        Ok(response) => {
            info!("merge_local_pool 执行成功");
            ApiResponse::ok(response)
        }
        Err(error) => {
            error!(error = %error, "merge_local_pool 执行失败");
            ApiResponse::err(error.to_string())
        }
    }
}

fn analyze_local_pool_inner(
    request: AnalyzeLocalPoolRequest,
) -> AppResult<AnalyzeLocalPoolResponse> {
    if request.file_path.trim().is_empty() {
        return Err(AppError::Validation(
            "本地 pool.json 路径不能为空".to_string(),
        ));
    }

    let config_state = load_or_create_app_config_state()?;
    let app_config = config_state.config;
    let pool_path = PathBuf::from(&request.file_path);
    let pool_file = load_pool_file_from_path(&pool_path)?;
    let analysis_list = analyze_pool_file(
        &pool_file,
        app_config.skip_first_ssr,
        &build_base_ssr_id_set(&app_config),
    );
    let summary_list = build_pool_analysis_summaries(&analysis_list);

    Ok(AnalyzeLocalPoolResponse {
        file_path: request.file_path,
        analysis_list,
        summary_list,
    })
}

fn merge_local_pool_inner(request: MergeLocalPoolRequest) -> AppResult<MergeLocalPoolResponse> {
    if request.new_file_path.trim().is_empty() {
        return Err(AppError::Validation(
            "新 pool.json 路径不能为空".to_string(),
        ));
    }

    if request.output_file_path.trim().is_empty() {
        return Err(AppError::Validation(
            "输出 pool.json 路径不能为空".to_string(),
        ));
    }

    let config_state = load_or_create_app_config_state()?;
    let app_config = config_state.config;
    let old_pool_file = load_optional_pool_file(request.old_file_path.as_deref())?;
    let new_pool_path = PathBuf::from(&request.new_file_path);
    let output_pool_path = PathBuf::from(&request.output_file_path);
    let new_pool_file = load_pool_file_from_path(&new_pool_path)?;
    let (merged_pool_file, merge_result) = merge_pool_files(&old_pool_file, &new_pool_file);

    save_pool_file_to_path(&output_pool_path, &merged_pool_file)?;

    let analysis_list = analyze_pool_file(
        &merged_pool_file,
        app_config.skip_first_ssr,
        &build_base_ssr_id_set(&app_config),
    );
    let summary_list = build_pool_analysis_summaries(&analysis_list);

    Ok(MergeLocalPoolResponse {
        old_file_path: request.old_file_path,
        new_file_path: request.new_file_path,
        output_file_path: request.output_file_path,
        merge_result,
        analysis_list,
        summary_list,
    })
}

fn load_optional_pool_file(file_path: Option<&str>) -> AppResult<PoolFile> {
    let Some(file_path) = file_path else {
        return Ok(PoolFile::new());
    };

    if file_path.trim().is_empty() {
        return Ok(PoolFile::new());
    }

    let path = PathBuf::from(file_path);
    if !path.exists() {
        info!(path = %path.display(), "旧 pool.json 不存在，将按空数据合并");
        return Ok(PoolFile::new());
    }

    load_pool_file_from_path(&path)
}

fn build_base_ssr_id_set(config: &AppConfig) -> HashSet<String> {
    config.base_ssr_ids.iter().cloned().collect()
}
