use tracing::info;
use url::{form_urlencoded, Url};

use crate::error::{AppError, AppResult};

use super::model::{ParsedGachaParams, RequestParams};

const PARAM_MAPPINGS: [(&str, &str); 6] = [
    ("player_id", "playerId"),
    ("record_id", "recordId"),
    ("resources_id", "cardPoolId"),
    ("gacha_type", "cardPoolType"),
    ("svr_id", "serverId"),
    ("lang", "languageCode"),
];

pub fn parse_gacha_url_params(raw_url: &str) -> AppResult<ParsedGachaParams> {
    let raw_url = raw_url.trim();
    if raw_url.is_empty() {
        return Err(AppError::Validation("抽卡 URL 不能为空".to_string()));
    }

    let url = Url::parse(raw_url)
        .map_err(|error| AppError::Validation(format!("抽卡 URL 格式无效: {error}")))?;

    validate_gacha_record_url(raw_url, &url)?;
    let query = extract_query(raw_url)?;

    let mut params = RequestParams::new();
    for (key, value) in form_urlencoded::parse(query.as_bytes()) {
        let mapped_key = map_query_key(key.as_ref());
        params.insert(mapped_key.to_string(), value.to_string());
    }

    let parsed = ParsedGachaParams {
        player_id: required_param(&params, "playerId")?,
        record_id: required_param(&params, "recordId")?,
        card_pool_id: required_param(&params, "cardPoolId")?,
        card_pool_type: required_param(&params, "cardPoolType")?,
        server_id: required_param(&params, "serverId")?,
        language_code: required_param(&params, "languageCode")?,
        params,
    };

    info!(
        player_id = %parsed.player_id,
        server_id = %parsed.server_id,
        card_pool_type = %parsed.card_pool_type,
        language_code = %parsed.language_code,
        param_count = parsed.params.len(),
        "抽卡 URL 参数解析完成"
    );

    Ok(parsed)
}

fn validate_gacha_record_url(raw_url: &str, url: &Url) -> AppResult<()> {
    if !url.path().ends_with("/aki/gacha/index.html") || !raw_url.contains("#/record") {
        return Err(AppError::Validation(
            "不是有效的鸣潮抽卡记录 URL".to_string(),
        ));
    }

    Ok(())
}

fn extract_query(raw_url: &str) -> AppResult<&str> {
    let query_start = raw_url
        .find('?')
        .ok_or_else(|| AppError::Validation("抽卡 URL 缺少 query 参数".to_string()))?;
    let query = &raw_url[query_start + 1..];

    if query.trim().is_empty() {
        return Err(AppError::Validation("抽卡 URL 缺少 query 参数".to_string()));
    }

    Ok(query)
}

fn map_query_key(key: &str) -> &str {
    PARAM_MAPPINGS
        .iter()
        .find_map(|(source_key, target_key)| (*source_key == key).then_some(*target_key))
        .unwrap_or(key)
}

fn required_param(params: &RequestParams, key: &str) -> AppResult<String> {
    params
        .get(key)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| AppError::Validation(format!("抽卡 URL 缺少必要参数: {key}")))
}

#[cfg(test)]
mod tests {
    use super::parse_gacha_url_params;

    fn sample_url() -> &'static str {
        "https://example.com/aki/gacha/index.html#/record?player_id=10000001&record_id=abc-123&resources_id=1101&gacha_type=1&svr_id=prod-cn&lang=zh-Hans&extra_token=xyz"
    }

    #[test]
    fn parse_gacha_url_params_should_map_known_query_keys() {
        let parsed = parse_gacha_url_params(sample_url()).expect("sample url should be valid");

        assert_eq!(parsed.player_id, "10000001");
        assert_eq!(parsed.record_id, "abc-123");
        assert_eq!(parsed.card_pool_id, "1101");
        assert_eq!(parsed.card_pool_type, "1");
        assert_eq!(parsed.server_id, "prod-cn");
        assert_eq!(parsed.language_code, "zh-Hans");
        assert_eq!(parsed.params["playerId"], "10000001");
        assert_eq!(parsed.params["recordId"], "abc-123");
        assert_eq!(parsed.params["cardPoolId"], "1101");
        assert_eq!(parsed.params["cardPoolType"], "1");
        assert_eq!(parsed.params["serverId"], "prod-cn");
        assert_eq!(parsed.params["languageCode"], "zh-Hans");
        assert_eq!(parsed.params["extra_token"], "xyz");
    }

    #[test]
    fn parse_gacha_url_params_should_decode_percent_encoded_query() {
        let parsed = parse_gacha_url_params(
            "https://example.com/aki/gacha/index.html#/record?player_id=10000001&record_id=abc&resources_id=1101&gacha_type=1&svr_id=prod-cn&lang=zh-Hans&note=%E9%B8%A3%E6%BD%AE",
        )
        .expect("encoded url should be valid");

        assert_eq!(parsed.params["note"], "鸣潮");
    }

    #[test]
    fn parse_gacha_url_params_should_reject_missing_required_param() {
        let error = parse_gacha_url_params(
            "https://example.com/aki/gacha/index.html#/record?player_id=10000001&record_id=abc&resources_id=1101&gacha_type=1&lang=zh-Hans",
        )
        .expect_err("missing server id should fail");

        assert!(error.to_string().contains("serverId"));
    }

    #[test]
    fn parse_gacha_url_params_should_reject_non_record_url() {
        let error = parse_gacha_url_params(
            "https://example.com/aki/gacha/index.html#/home?player_id=10000001&record_id=abc&resources_id=1101&gacha_type=1&svr_id=prod-cn&lang=zh-Hans",
        )
        .expect_err("non record url should fail");

        assert!(error.to_string().contains("抽卡记录 URL"));
    }
}
