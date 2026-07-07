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

pub fn default_card_pools() -> Vec<CardPoolConfig> {
    vec![
        CardPoolConfig {
            name: "角色活动唤取".to_string(),
            pool_type: "1".to_string(),
        },
        CardPoolConfig {
            name: "武器活动唤取".to_string(),
            pool_type: "2".to_string(),
        },
        CardPoolConfig {
            name: "角色常驻唤取".to_string(),
            pool_type: "3".to_string(),
        },
        CardPoolConfig {
            name: "武器常驻唤取".to_string(),
            pool_type: "4".to_string(),
        },
        CardPoolConfig {
            name: "新手唤取".to_string(),
            pool_type: "5".to_string(),
        },
        CardPoolConfig {
            name: "新手自选唤取".to_string(),
            pool_type: "6".to_string(),
        },
        CardPoolConfig {
            name: "新手自选唤取（感恩定向唤取）".to_string(),
            pool_type: "7".to_string(),
        },
        CardPoolConfig {
            name: "角色新旅唤取".to_string(),
            pool_type: "8".to_string(),
        },
        CardPoolConfig {
            name: "武器新旅唤取".to_string(),
            pool_type: "9".to_string(),
        },
        CardPoolConfig {
            name: "角色联动唤取".to_string(),
            pool_type: "10".to_string(),
        },
        CardPoolConfig {
            name: "武器联动唤取".to_string(),
            pool_type: "11".to_string(),
        },
    ]
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
            card_pools: default_card_pools(),
        }
    }
}
