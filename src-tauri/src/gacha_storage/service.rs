use std::{fs, path::Path, time::SystemTime};

use time::OffsetDateTime;

use tracing::info;

use crate::{error::AppResult, gacha_storage::model::PoolFile};

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
