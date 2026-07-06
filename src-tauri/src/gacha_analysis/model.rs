use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaRecord {
    pub card_pool_type: String,
    pub resource_id: i64,
    pub quality_level: u8,
    pub resource_type: Option<String>,
    pub name: String,
    pub count: Option<i64>,
    pub time: String,
}

pub type PoolFile = std::collections::BTreeMap<String, Vec<GachaRecord>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitData {
    pub id: i64,
    pub name: String,
    pub count: usize,
    pub event: bool,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolRankSummary {
    pub count: usize,
    pub rate: f64,
    pub avg: f64,
    pub min: usize,
    pub max: usize,
    pub current_pity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolAnalysisSummary {
    pub pool_name: String,
    pub is_empty: bool,
    pub total_count: usize,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub ssr: PoolRankSummary,
    pub sr: PoolRankSummary,
    pub r: PoolRankSummary,
    pub ssr_event_count: usize,
    pub ssr_permanent_count: usize,
    pub latest_ssr: Option<HitData>,
    pub latest_sr: Option<HitData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisData {
    pub is_empty: bool,
    pub pool_name: String,
    pub total_count: usize,
    pub no_up_ssr_count: usize,
    pub no_up_sr_count: usize,
    pub no_up_r_count: usize,
    pub ssr_count: usize,
    pub sr_count: usize,
    pub r_count: usize,
    pub ssr_avg: f64,
    pub ssr_min: usize,
    pub ssr_max: usize,
    pub sr_avg: f64,
    pub sr_min: usize,
    pub sr_max: usize,
    pub r_avg: f64,
    pub r_min: usize,
    pub r_max: usize,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub ssr_data_list: Vec<HitData>,
    pub sr_data_list: Vec<HitData>,
    pub r_data_list: Vec<HitData>,
}

impl AnalysisData {
    pub fn empty(pool_name: impl Into<String>) -> Self {
        Self {
            is_empty: true,
            pool_name: pool_name.into(),
            total_count: 0,
            no_up_ssr_count: 0,
            no_up_sr_count: 0,
            no_up_r_count: 0,
            ssr_count: 0,
            sr_count: 0,
            r_count: 0,
            ssr_avg: 0.0,
            ssr_min: 0,
            ssr_max: 0,
            sr_avg: 0.0,
            sr_min: 0,
            sr_max: 0,
            r_avg: 0.0,
            r_min: 0,
            r_max: 0,
            start_date: None,
            end_date: None,
            ssr_data_list: Vec::new(),
            sr_data_list: Vec::new(),
            r_data_list: Vec::new(),
        }
    }
}

impl PoolRankSummary {
    pub fn empty() -> Self {
        Self {
            count: 0,
            rate: 0.0,
            avg: 0.0,
            min: 0,
            max: 0,
            current_pity: 0,
        }
    }
}
