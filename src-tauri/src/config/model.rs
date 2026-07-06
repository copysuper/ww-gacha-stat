use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceSource {
    Github,
    Gitee,
}

impl Default for ResourceSource {
    fn default() -> Self {
        Self::Github
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPoolConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub pool_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub game_root_dir: Option<String>,
    pub game_log_file_relative_path: String,
    pub resource_source: ResourceSource,
    pub data_dir: Option<String>,
    pub assets_dir: Option<String>,
    pub log_level: LogLevel,
    pub skip_first_ssr: bool,
    pub base_ssr_ids: Vec<String>,
    pub card_pools: Vec<CardPoolConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            game_root_dir: None,
            game_log_file_relative_path: "Client/Saved/Logs/Client.log".to_string(),
            resource_source: ResourceSource::Github,
            data_dir: None,
            assets_dir: None,
            log_level: LogLevel::Info,
            skip_first_ssr: false,
            base_ssr_ids: vec![
                "1104".to_string(),
                "1203".to_string(),
                "1301".to_string(),
                "1503".to_string(),
                "1405".to_string(),
                "21010015".to_string(),
                "21020015".to_string(),
                "21030015".to_string(),
                "21040015".to_string(),
                "21050015".to_string(),
            ],
            card_pools: vec![],
        }
    }
}
