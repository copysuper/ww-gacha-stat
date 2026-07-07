use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolMergeSummary {
    pub pool_name: String,
    pub old_count: usize,
    pub new_count: usize,
    pub merged_count: usize,
    pub appended_old_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolMergeResult {
    pub summaries: Vec<PoolMergeSummary>,
    pub total_old_count: usize,
    pub total_new_count: usize,
    pub total_merged_count: usize,
    pub total_appended_old_count: usize,
}

impl PoolMergeResult {
    pub fn from_summaries(summaries: Vec<PoolMergeSummary>) -> Self {
        Self {
            total_old_count: summaries.iter().map(|item| item.old_count).sum(),
            total_new_count: summaries.iter().map(|item| item.new_count).sum(),
            total_merged_count: summaries.iter().map(|item| item.merged_count).sum(),
            total_appended_old_count: summaries.iter().map(|item| item.appended_old_count).sum(),
            summaries,
        }
    }
}
