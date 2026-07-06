use std::path::PathBuf;

use directories::ProjectDirs;

use crate::error::{AppError, AppResult};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "copysuper";
const APPLICATION: &str = "ww-gacha-stat";
const CONFIG_FILE_NAME: &str = "app-config.json";

fn project_dirs() -> AppResult<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).ok_or_else(|| {
        AppError::PathResolve(
            "无法解析应用目录，请检查当前系统环境是否支持标准用户目录".to_string(),
        )
    })
}

pub fn config_dir() -> AppResult<PathBuf> {
    Ok(project_dirs()?.config_dir().to_path_buf())
}

pub fn data_dir() -> AppResult<PathBuf> {
    Ok(project_dirs()?.data_dir().to_path_buf())
}

pub fn assets_dir() -> AppResult<PathBuf> {
    Ok(data_dir()?.join("assets"))
}

pub fn app_config_file_path() -> AppResult<PathBuf> {
    Ok(config_dir()?.join(CONFIG_FILE_NAME))
}
