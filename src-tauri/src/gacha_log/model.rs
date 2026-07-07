use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedGachaUrl {
    pub url: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaLogExtractResult {
    pub log_file_path: String,
    pub total_url_count: usize,
    pub latest: ExtractedGachaUrl,
}
