pub mod model;
pub mod service;

pub use model::AppConfig;
pub use service::{load_or_create_app_config_state, update_app_config};
