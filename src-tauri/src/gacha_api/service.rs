use serde::Deserialize;
use tracing::{info, warn};

use crate::{
    config::model::CardPoolConfig,
    error::{AppError, AppResult},
    gacha_analysis::model::GachaRecord,
    gacha_params::model::RequestParams,
};

use super::model::{GachaApiFetchResult, GachaApiPoolResult};

const CN_GACHA_QUERY_URL: &str = "https://gmserver-api.aki-game2.com/gacha/record/query";
const GLOBAL_GACHA_QUERY_URL: &str = "https://gmserver-api.aki-game2.net/gacha/record/query";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GachaApiResponse {
    code: i64,
    data: Option<Vec<GachaRecord>>,
    msg: Option<String>,
    message: Option<String>,
}

pub fn request_url_for_player(player_id: &str) -> &'static str {
    if player_id.starts_with('1') {
        CN_GACHA_QUERY_URL
    } else {
        GLOBAL_GACHA_QUERY_URL
    }
}

pub fn build_request_body(base_params: &RequestParams, card_pool_type: &str) -> RequestParams {
    let mut body = base_params.clone();
    body.insert("cardPoolType".to_string(), card_pool_type.to_string());
    body
}

pub async fn fetch_gacha_pool_records(
    client: &reqwest::Client,
    base_params: &RequestParams,
    pool: &CardPoolConfig,
) -> AppResult<Vec<GachaRecord>> {
    let player_id = required_param(base_params, "playerId")?;
    let url = request_url_for_player(player_id);
    let body = build_request_body(base_params, &pool.pool_type);

    info!(
        pool_name = %pool.name,
        card_pool_type = %pool.pool_type,
        server_url = %url,
        "开始请求单个卡池抽卡记录"
    );

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|error| AppError::Network(format!("抽卡接口请求失败: {error}")))?;

    let status = response.status();
    if !status.is_success() {
        return Err(AppError::Network(format!(
            "抽卡接口 HTTP 状态异常: {status}"
        )));
    }

    let body = response
        .json::<GachaApiResponse>()
        .await
        .map_err(|error| AppError::Network(format!("抽卡接口响应解析失败: {error}")))?;

    if body.code != 0 {
        let message = body
            .msg
            .or(body.message)
            .unwrap_or_else(|| "未知业务错误".to_string());
        return Err(AppError::Validation(format!(
            "抽卡接口返回业务错误 code={}, message={}",
            body.code, message
        )));
    }

    let records = body.data.unwrap_or_default();
    info!(
        pool_name = %pool.name,
        card_pool_type = %pool.pool_type,
        record_count = records.len(),
        "单个卡池抽卡记录请求完成"
    );

    Ok(records)
}

pub async fn fetch_gacha_pool_file(
    base_params: &RequestParams,
    card_pools: &[CardPoolConfig],
) -> AppResult<GachaApiFetchResult> {
    validate_base_params(base_params)?;

    if card_pools.is_empty() {
        warn!("卡池配置为空，将返回空 pool.json 数据");
    }

    let client = reqwest::Client::new();
    let mut pool_file = crate::gacha_analysis::model::PoolFile::new();
    let mut pool_results = Vec::new();

    for pool in card_pools {
        let records = fetch_gacha_pool_records(&client, base_params, pool).await?;
        let record_count = records.len();
        pool_file.insert(pool.name.clone(), records);
        pool_results.push(GachaApiPoolResult {
            pool_name: pool.name.clone(),
            card_pool_type: pool.pool_type.clone(),
            record_count,
        });
    }

    Ok(GachaApiFetchResult {
        pool_file,
        pool_results,
    })
}

fn validate_base_params(params: &RequestParams) -> AppResult<()> {
    for key in [
        "playerId",
        "recordId",
        "cardPoolId",
        "cardPoolType",
        "serverId",
        "languageCode",
    ] {
        required_param(params, key)?;
    }

    Ok(())
}

fn required_param<'a>(params: &'a RequestParams, key: &str) -> AppResult<&'a str> {
    params
        .get(key)
        .filter(|value| !value.trim().is_empty())
        .map(String::as_str)
        .ok_or_else(|| AppError::Validation(format!("抽卡请求参数缺少 {key}")))
}

#[cfg(test)]
mod tests {
    use super::{build_request_body, request_url_for_player};
    use crate::gacha_params::model::RequestParams;

    #[test]
    fn request_url_for_player_should_follow_player_id_prefix() {
        assert!(request_url_for_player("10000001").contains("aki-game2.com"));
        assert!(request_url_for_player("90000001").contains("aki-game2.net"));
    }

    #[test]
    fn build_request_body_should_override_card_pool_type() {
        let mut params = RequestParams::new();
        params.insert("playerId".to_string(), "10000001".to_string());
        params.insert("cardPoolType".to_string(), "1".to_string());
        params.insert("unknown".to_string(), "keep".to_string());

        let body = build_request_body(&params, "3");

        assert_eq!(body["cardPoolType"], "3");
        assert_eq!(body["unknown"], "keep");
    }
}
