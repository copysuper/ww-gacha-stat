use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    commands::ApiResponse,
    config::{load_or_create_app_config_state, AppConfig},
    error::{AppError, AppResult},
    gacha_analysis::{analyze_pool_file, build_pool_analysis_summaries},
    gacha_analysis::model::{AnalysisData, PoolAnalysisSummary},
    gacha_storage::load_pool_file_from_path,
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

fn build_base_ssr_id_set(config: &AppConfig) -> HashSet<String> {
    config.base_ssr_ids.iter().cloned().collect()
}
