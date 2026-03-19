use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AppError {
    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("目录扫描错误: {0}")]
    ScanError(String),

    #[error("README 解析错误: {0}")]
    ParseError(String),

    #[error("摘要生成错误: {0}")]
    GenerateError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML 序列化错误: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("TOML 反序列化错误: {0}")]
    TomlDeError(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
