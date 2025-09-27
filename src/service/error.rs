use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Config error: {0}")]
    Config(String),
}
