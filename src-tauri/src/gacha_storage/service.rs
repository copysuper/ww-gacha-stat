use std::{fs, path::Path};

use tracing::info;

use crate::{error::AppResult, gacha_storage::model::PoolFile};

pub fn load_pool_file_from_path(path: &Path) -> AppResult<PoolFile> {
    let content = fs::read_to_string(path)?;
    let pool_file = serde_json::from_str::<PoolFile>(&content)?;

    info!(path = %path.display(), pool_count = pool_file.len(), "本地 pool.json 读取完成");

    Ok(pool_file)
}
