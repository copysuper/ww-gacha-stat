use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("文件读写失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON 序列化或反序列化失败: {0}")]
    Json(#[from] serde_json::Error),

    #[error("路径解析失败: {0}")]
    PathResolve(String),

    #[error("参数校验失败: {0}")]
    Validation(String),

    #[error("网络请求失败: {0}")]
    Network(String),
}
