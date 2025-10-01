use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Cannot determine executable path: {0}")]
    ExecutablePath(#[from] std::io::Error),

    #[error("Cannot determine executable directory")]
    ExecutableDirectory,

    #[error("Cannot canonicalize config path: {0}")]
    CanonicalizePath(std::io::Error),

    #[error("Invalid config path")]
    InvalidPath,

    #[error("Invalid configuration format: {0}")]
    InvalidFormat(#[from] toml::de::Error),

    #[error("Invalid service name characters")]
    InvalidServiceName,

    #[error("Log path must be absolute")]
    LogPathNotAbsolute,

    #[error("Log path not in allowed directory")]
    LogPathNotAllowed,

    #[error("Install and config paths must be absolute")]
    PathsNotAbsolute,

    #[error("Config error: {0}")]
    Generic(String),
}
