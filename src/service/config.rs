use crate::security::validation::{sanitize_message, validate_config_field};
use crate::service::service_error::ServiceError;
use serde::Deserialize;
use std::fs;

#[must_use]
pub const fn get_config_file_name() -> &'static str {
    "service.toml"
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Config {
    pub LOG_FILE_PATH: String,
    pub INSTALL_DIR: String,
    pub CONFIG_DIR: String,
    pub SERVICE_NAME: String,
    pub MEMORY_THRESHOLD: u32,
    pub DISK_THRESHOLD: u32,
    pub MIN_FD_LIMIT: u64,
    pub MAX_SERVICE_NAME_LEN: usize,
    pub MAX_LOG_PATH_LEN: usize,
    pub MIN_LOG_INTERVAL_MS: u64,
    pub MAX_LOG_FILE_SIZE: u64,
    pub MAX_TIME_INTERVAL: u64,
    pub MAX_THRESHOLD_PERCENT: u32,
    pub MAX_FD_LIMIT: u64,
    pub MAX_CONFIG_FIELD_LEN: usize,
}

/// Validates all service configuration fields.
///
/// # Errors
/// Returns `ServiceError` if any configuration field is invalid.
pub fn validate_all_config_fields(config: &Config) -> Result<(), ServiceError> {
    let _sanitized_name = sanitize_message(&config.SERVICE_NAME, config.MAX_SERVICE_NAME_LEN)?;

    validate_config_field(
        &config.SERVICE_NAME.len(),
        &1,
        &config.MAX_SERVICE_NAME_LEN,
        "service_name",
    )?;
    validate_config_field(
        &config.LOG_FILE_PATH.len(),
        &1,
        &config.MAX_LOG_PATH_LEN,
        "log_file_path",
    )?;
    validate_config_field(
        &config.INSTALL_DIR.len(),
        &1,
        &config.MAX_LOG_PATH_LEN,
        "install_dir",
    )?;
    validate_config_field(
        &config.CONFIG_DIR.len(),
        &1,
        &config.MAX_LOG_PATH_LEN,
        "config_dir",
    )?;
    validate_config_field(
        &config.MEMORY_THRESHOLD,
        &1,
        &config.MAX_THRESHOLD_PERCENT,
        "memory_threshold",
    )?;
    validate_config_field(
        &config.DISK_THRESHOLD,
        &1,
        &config.MAX_THRESHOLD_PERCENT,
        "disk_threshold",
    )?;
    validate_config_field(
        &config.MIN_FD_LIMIT,
        &1,
        &config.MAX_FD_LIMIT,
        "min_fd_limit",
    )?;
    validate_config_field(
        &config.MAX_SERVICE_NAME_LEN,
        &1,
        &config.MAX_CONFIG_FIELD_LEN,
        "max_service_name_len",
    )?;
    validate_config_field(
        &config.MAX_LOG_PATH_LEN,
        &1,
        &config.MAX_CONFIG_FIELD_LEN,
        "max_log_path_len",
    )?;
    Ok(())
}

/// Loads configuration from executable directory with validation against secure paths
pub fn load_secure_config() -> Result<Config, ServiceError> {
    // Validate paths against secure directories: "/var/log", "/opt"
    let exe_path = std::env::current_exe()
        .map_err(|e| ServiceError::Config(format!("Cannot determine executable path: {e}")))?;

    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| ServiceError::Config("Cannot determine executable directory".to_string()))?;

    let config_path = exe_dir.join("service.toml");

    if !config_path.exists() {
        return Err(ServiceError::Config(
            "No valid config file found".to_string(),
        ));
    }

    // Open file first to get file descriptor
    let FILE = fs::File::open(&config_path).map_err(|e| {
        ServiceError::Config(format!(
            "Failed to open config file {}: {e}",
            config_path.display()
        ))
    })?;

    // Use file descriptor for metadata check to prevent race conditions
    let _metadata = FILE
        .metadata()
        .map_err(|e| ServiceError::Config(format!("Cannot read config file metadata: {e}")))?;

    let content = fs::read_to_string(&config_path)
        .map_err(|e| ServiceError::Config(format!("Failed to read config file: {e}")))?;

    let config: Config =
        toml::from_str(&content).map_err(|e| ServiceError::Config(format!("Invalid TOML: {e}")))?;

    validate_all_config_fields(&config)?;
    Ok(config)
}
