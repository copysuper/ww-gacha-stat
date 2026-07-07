use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use tracing::info;

use crate::{
    config::AppConfig,
    error::{AppError, AppResult},
};

use super::model::{ExtractedGachaUrl, GachaLogExtractResult};

const GACHA_RECORD_MARKER: &str = "/aki/gacha/index.html#/record?";
const GACHA_RECORD_MARKER_BYTES: &[u8] = GACHA_RECORD_MARKER.as_bytes();

pub fn resolve_game_log_file_path(config: &AppConfig) -> AppResult<PathBuf> {
    let game_root_dir = config
        .game_root_dir
        .as_ref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::Validation("未配置游戏根目录，无法查找日志文件".to_string()))?;

    let log_relative_path = config.game_log_file_relative_path.trim();
    if log_relative_path.is_empty() {
        return Err(AppError::Validation("游戏日志相对路径不能为空".to_string()));
    }

    Ok(PathBuf::from(game_root_dir).join(log_relative_path))
}

pub fn extract_latest_gacha_url_from_file(
    log_file_path: &Path,
) -> AppResult<GachaLogExtractResult> {
    if !log_file_path.exists() {
        return Err(AppError::Validation(format!(
            "游戏日志文件不存在: {}",
            log_file_path.display()
        )));
    }

    let mut file = File::open(log_file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    let mut total_url_count = 0usize;
    let mut latest: Option<ExtractedGachaUrl> = None;

    // 游戏日志在 Windows 上可能混入非 UTF-8 字节，不能用 BufRead::lines()。
    // 这里按字节逐行扫描，只提取 ASCII URL 片段，避免编码问题影响抽卡 URL 提取。
    for (line_index, line) in content.split(|byte| *byte == b'\n').enumerate() {
        for url in extract_gacha_urls_from_line_bytes(line) {
            total_url_count += 1;
            latest = Some(ExtractedGachaUrl {
                url,
                line_number: line_index + 1,
            });
        }
    }

    let latest = latest.ok_or_else(|| {
        AppError::Validation("日志中未找到抽卡记录 URL，请先在游戏内打开抽卡记录页面".to_string())
    })?;

    info!(
        log_file_path = %log_file_path.display(),
        total_url_count,
        latest_line_number = latest.line_number,
        "抽卡日志 URL 提取完成"
    );

    Ok(GachaLogExtractResult {
        log_file_path: log_file_path.display().to_string(),
        total_url_count,
        latest,
    })
}

#[cfg(test)]
fn extract_gacha_urls_from_line(line: &str) -> Vec<String> {
    extract_gacha_urls_from_line_bytes(line.as_bytes())
}

fn extract_gacha_urls_from_line_bytes(line: &[u8]) -> Vec<String> {
    let mut urls = Vec::new();
    let mut search_start = 0usize;

    while let Some(relative_index) = find_bytes(&line[search_start..], GACHA_RECORD_MARKER_BYTES) {
        let marker_index = search_start + relative_index;
        let url_start = find_url_start(line, marker_index);
        let url_end = find_url_end(line, marker_index);
        let raw_url = &line[url_start..url_end];
        let sanitized_url = sanitize_url(raw_url);

        if !sanitized_url.is_empty() {
            urls.push(sanitized_url);
        }

        search_start = url_end;
    }

    urls
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }

    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn find_url_start(line: &[u8], marker_index: usize) -> usize {
    line[..marker_index]
        .windows(b"http".len())
        .rposition(|window| window == b"http")
        .unwrap_or(marker_index)
}

fn find_url_end(line: &[u8], marker_index: usize) -> usize {
    let tail = &line[marker_index..];
    let end_offset = tail
        .iter()
        .position(|byte| {
            byte.is_ascii_whitespace()
                || matches!(byte, b'"' | b'\'' | b'`' | b'<' | b'>' | b')' | b']' | b'}')
        })
        .unwrap_or(tail.len());

    marker_index + end_offset
}

fn sanitize_url(raw_url: &[u8]) -> String {
    let trimmed = trim_url_bytes(raw_url);
    String::from_utf8_lossy(trimmed).into_owned()
}

fn trim_url_bytes(raw_url: &[u8]) -> &[u8] {
    let mut start = 0usize;
    let mut end = raw_url.len();

    while start < end && is_url_trim_byte(raw_url[start]) {
        start += 1;
    }

    while end > start && is_url_trim_byte(raw_url[end - 1]) {
        end -= 1;
    }

    &raw_url[start..end]
}

fn is_url_trim_byte(byte: u8) -> bool {
    byte.is_ascii_whitespace()
        || matches!(
            byte,
            b'"' | b'\'' | b'`' | b'<' | b'>' | b')' | b']' | b'}' | b',' | b'\r'
        )
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::{
        config::AppConfig,
        gacha_log::service::{extract_gacha_urls_from_line, resolve_game_log_file_path},
    };

    use super::extract_latest_gacha_url_from_file;

    fn sample_url(player_id: &str) -> String {
        format!(
            "https://example.com/aki/gacha/index.html#/record?player_id={player_id}&record_id=abc&resources_id=1101&gacha_type=1&svr_id=prod-cn&lang=zh-Hans"
        )
    }

    #[test]
    fn extract_gacha_urls_from_line_should_find_url() {
        let line = format!("log prefix {} log suffix", sample_url("10000001"));
        let urls = extract_gacha_urls_from_line(&line);

        assert_eq!(urls, vec![sample_url("10000001")]);
    }

    #[test]
    fn extract_latest_gacha_url_from_file_should_use_last_match() {
        let path = temp_log_file_path("latest");
        fs::write(
            &path,
            format!(
                "first {}\nno url here\nsecond {}\n",
                sample_url("10000001"),
                sample_url("10000002")
            ),
        )
        .expect("write temp log");

        let result = extract_latest_gacha_url_from_file(&path).expect("extract should succeed");
        assert_eq!(result.total_url_count, 2);
        assert_eq!(result.latest.line_number, 3);
        assert_eq!(result.latest.url, sample_url("10000002"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn extract_latest_gacha_url_from_file_should_report_missing_url() {
        let path = temp_log_file_path("missing");
        fs::write(&path, "no gacha record url").expect("write temp log");

        let error = extract_latest_gacha_url_from_file(&path).expect_err("missing url should fail");
        assert!(error.to_string().contains("未找到抽卡记录 URL"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn extract_latest_gacha_url_from_file_should_ignore_invalid_utf8_bytes() {
        let path = temp_log_file_path("invalid-utf8");
        let mut bytes = vec![0xff, 0xfe, b'\n'];
        bytes.extend_from_slice(format!("prefix {} suffix", sample_url("10000003")).as_bytes());
        bytes.extend_from_slice(&[b'\n', 0x80, 0x81]);
        fs::write(&path, bytes).expect("write temp log");

        let result = extract_latest_gacha_url_from_file(&path).expect("extract should succeed");
        assert_eq!(result.total_url_count, 1);
        assert_eq!(result.latest.line_number, 2);
        assert_eq!(result.latest.url, sample_url("10000003"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn resolve_game_log_file_path_should_join_config_paths() {
        let config = AppConfig {
            game_root_dir: Some("/game/root".to_string()),
            game_log_file_relative_path: "Client/Saved/Logs/Client.log".to_string(),
            ..Default::default()
        };

        let path = resolve_game_log_file_path(&config).expect("path should resolve");
        assert_eq!(
            path,
            PathBuf::from("/game/root").join("Client/Saved/Logs/Client.log")
        );
    }

    fn temp_log_file_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "ww-gacha-stat-{name}-{}-{}.log",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ))
    }
}
