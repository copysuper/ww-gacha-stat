use std::fs;

use tracing::info;

use crate::{app_paths, error::AppResult};

use super::model::AppConfig;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigState {
    pub config: AppConfig,
    pub config_file_path: String,
    pub resolved_data_dir: String,
    pub resolved_assets_dir: String,
}

pub fn load_or_create_app_config_state() -> AppResult<AppConfigState> {
    let config_file_path = app_paths::app_config_file_path()?;

    let config = if config_file_path.exists() {
        let content = fs::read_to_string(&config_file_path)?;
        serde_json::from_str::<AppConfig>(&content)?
    } else {
        let default_config = AppConfig::default();
        save_app_config(&default_config)?;
        default_config
    };

    let state = AppConfigState {
        config,
        config_file_path: config_file_path.display().to_string(),
        resolved_data_dir: app_paths::data_dir()?.display().to_string(),
        resolved_assets_dir: app_paths::assets_dir()?.display().to_string(),
    };

    info!(
        config_file_path = %state.config_file_path,
        resolved_data_dir = %state.resolved_data_dir,
        "应用配置已加载"
    );

    Ok(state)
}

pub fn update_app_config(config: AppConfig) -> AppResult<AppConfigState> {
    save_app_config(&config)?;
    info!("应用配置已更新");
    load_or_create_app_config_state()
}

fn save_app_config(config: &AppConfig) -> AppResult<()> {
    let config_dir = app_paths::config_dir()?;
    let config_file_path = app_paths::app_config_file_path()?;

    fs::create_dir_all(config_dir)?;

    let content = serde_json::to_string_pretty(config)?;
    fs::write(config_file_path, content)?;

    Ok(())
}
