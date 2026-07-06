use serde::Deserialize;
use tracing::{error, info};

use crate::{
    commands::ApiResponse,
    config::{load_or_create_app_config_state, update_app_config as save_app_config, AppConfig},
};

use crate::config::service::AppConfigState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppConfigRequest {
    pub config: AppConfig,
}

#[tauri::command]
pub fn get_app_config() -> ApiResponse<AppConfigState> {
    match load_or_create_app_config_state() {
        Ok(state) => {
            info!("get_app_config 执行成功");
            ApiResponse::ok(state)
        }
        Err(error) => {
            error!(error = %error, "get_app_config 执行失败");
            ApiResponse::err(error.to_string())
        }
    }
}

#[tauri::command]
pub fn update_app_config(request: UpdateAppConfigRequest) -> ApiResponse<AppConfigState> {
    match save_app_config(request.config) {
        Ok(state) => {
            info!("update_app_config 执行成功");
            ApiResponse::ok(state)
        }
        Err(error) => {
            error!(error = %error, "update_app_config 执行失败");
            ApiResponse::err(error.to_string())
        }
    }
}
