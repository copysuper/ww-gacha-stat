use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type RequestParams = BTreeMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGachaParams {
    pub player_id: String,
    pub record_id: String,
    pub card_pool_id: String,
    pub card_pool_type: String,
    pub server_id: String,
    pub language_code: String,
    pub params: RequestParams,
}
