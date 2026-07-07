mod app_paths;
mod commands;
mod config;
mod error;
mod gacha_analysis;
mod gacha_merge;
mod gacha_storage;

use tracing_subscriber::{fmt, EnvFilter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::gacha::analyze_local_pool,
            commands::gacha::merge_local_pool,
            commands::settings::get_app_config,
            commands::settings::update_app_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_logging() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = fmt().with_env_filter(env_filter).try_init();
}
