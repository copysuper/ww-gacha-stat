use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use time::OffsetDateTime;

use tracing::info;

use crate::{
    error::{AppError, AppResult},
    gacha_params::model::RequestParams,
    gacha_storage::model::PoolFile,
};

const DATA_FILE_NAME: &str = "data.json";

pub fn load_pool_file_from_path(path: &Path) -> AppResult<PoolFile> {
    let content = fs::read_to_string(path)?;
    let pool_file = serde_json::from_str::<PoolFile>(&content)?;

    info!(path = %path.display(), pool_count = pool_file.len(), "本地 pool.json 读取完成");

    Ok(pool_file)
}

pub fn save_pool_file_to_path(path: &Path, pool_file: &PoolFile) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    backup_pool_file_if_needed(path)?;

    let content = serde_json::to_string_pretty(pool_file)?;
    fs::write(path, content)?;

    info!(
        path = %path.display(),
        pool_count = pool_file.len(),
        record_count = count_records(pool_file),
        "本地 pool.json 保存完成"
    );

    Ok(())
}

pub fn player_data_dir(data_root: &Path, player_id: &str) -> AppResult<PathBuf> {
    validate_player_id(player_id)?;

    Ok(data_root.join(player_id))
}

pub fn request_params_file_path_for_player(
    data_root: &Path,
    player_id: &str,
) -> AppResult<PathBuf> {
    Ok(player_data_dir(data_root, player_id)?.join(DATA_FILE_NAME))
}

pub fn load_request_params_from_path(path: &Path) -> AppResult<RequestParams> {
    let content = fs::read_to_string(path)?;
    let params = serde_json::from_str::<RequestParams>(&content)?;

    info!(
        path = %path.display(),
        param_count = params.len(),
        "本地 data.json 请求参数读取完成"
    );

    Ok(params)
}

pub fn save_request_params_to_path(path: &Path, params: &RequestParams) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(params)?;
    fs::write(path, content)?;

    info!(
        path = %path.display(),
        param_count = params.len(),
        "本地 data.json 请求参数保存完成"
    );

    Ok(())
}

pub fn load_request_params_for_player(
    data_root: &Path,
    player_id: &str,
) -> AppResult<RequestParams> {
    let path = request_params_file_path_for_player(data_root, player_id)?;

    load_request_params_from_path(&path)
}

pub fn save_request_params_for_player(
    data_root: &Path,
    player_id: &str,
    params: &RequestParams,
) -> AppResult<PathBuf> {
    let path = request_params_file_path_for_player(data_root, player_id)?;
    save_request_params_to_path(&path, params)?;

    Ok(path)
}

fn backup_pool_file_if_needed(path: &Path) -> AppResult<()> {
    if !path.exists() {
        return Ok(());
    }

    let metadata = fs::metadata(path)?;
    let modified = metadata.modified()?;

    // 参考实现要求：如果 pool.json 不是今天写入，则保存前覆盖生成一个 pool.json.bak。
    // 当前后端先按 UTC 自然日判断，后续如引入本地时区配置，可把这里切换为本地 00:00。
    if modified_before_recent_day(modified) {
        let backup_path = path.with_file_name("pool.json.bak");
        fs::rename(path, &backup_path)?;

        info!(
            source = %path.display(),
            backup = %backup_path.display(),
            "旧 pool.json 已备份"
        );
    }

    Ok(())
}

fn modified_before_recent_day(modified: SystemTime) -> bool {
    let modified_at = OffsetDateTime::from(modified);
    let today_start = OffsetDateTime::now_utc().date().midnight().assume_utc();

    modified_at < today_start
}

fn count_records(pool_file: &PoolFile) -> usize {
    pool_file.values().map(Vec::len).sum()
}

fn validate_player_id(player_id: &str) -> AppResult<()> {
    if player_id.trim().is_empty() {
        return Err(AppError::Validation("玩家 ID 不能为空".to_string()));
    }

    if player_id.contains('/') || player_id.contains('\\') || player_id.contains("..") {
        return Err(AppError::Validation(
            "玩家 ID 不能包含路径分隔符或上级目录片段".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, time::SystemTime};

    use super::{
        load_request_params_for_player, request_params_file_path_for_player,
        save_request_params_for_player,
    };
    use crate::gacha_params::model::RequestParams;

    #[test]
    fn request_params_cache_should_round_trip_by_player_id() {
        let data_root = std::env::temp_dir().join(format!(
            "ww-gacha-stat-storage-test-{:?}",
            SystemTime::now()
        ));
        let mut params = RequestParams::new();
        params.insert("playerId".to_string(), "10000001".to_string());
        params.insert("recordId".to_string(), "abc".to_string());

        let saved_path = save_request_params_for_player(&data_root, "10000001", &params)
            .expect("params should be saved");
        let loaded = load_request_params_for_player(&data_root, "10000001")
            .expect("params should be loaded");

        assert_eq!(saved_path, data_root.join("10000001").join("data.json"));
        assert_eq!(loaded["playerId"], "10000001");
        assert_eq!(loaded["recordId"], "abc");

        let _ = fs::remove_dir_all(data_root);
    }

    #[test]
    fn request_params_cache_should_reject_unsafe_player_id() {
        let data_root = std::env::temp_dir();
        let error = request_params_file_path_for_player(&data_root, "../10000001")
            .expect_err("unsafe player id should fail");

        assert!(error.to_string().contains("玩家 ID"));
    }
}
