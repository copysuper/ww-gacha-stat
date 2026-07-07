use serde::Serialize;

use crate::gacha_analysis::model::PoolFile;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaApiPoolResult {
    pub pool_name: String,
    pub card_pool_type: String,
    pub record_count: usize,
}

#[derive(Debug, Clone)]
pub struct GachaApiFetchResult {
    pub pool_file: PoolFile,
    pub pool_results: Vec<GachaApiPoolResult>,
}
