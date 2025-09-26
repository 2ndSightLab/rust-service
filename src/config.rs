use crate::error::ServiceError;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Config {
    pub LOG_FILE_PATH: String,
    pub SERVICE_NAME: String,
    pub TIME_INTERVAL: u64,
    pub MESSAGE: String,
    pub MEMORY_THRESHOLD: u32,
    pub DISK_THRESHOLD: u32,
    pub MIN_FD_LIMIT: u64,
    pub MAX_SERVICE_NAME_LEN: usize,
    pub MAX_MESSAGE_LEN: usize,
    pub MAX_LOG_PATH_LEN: usize,
    pub MIN_LOG_INTERVAL_MS: u64,
    pub MAX_LOG_FILE_SIZE: u64,
    pub MAX_TIME_INTERVAL: u64,
    pub MAX_THRESHOLD_PERCENT: u32,
    pub MAX_FD_LIMIT: u64,
    pub MAX_CONFIG_FIELD_LEN: usize,
}

fn validate_config_field<T: PartialOrd>(value: &T, min: &T, max: &T, name: &str) -> Result<(), ServiceError> {
    if value < min || value > max {
        return Err(ServiceError::Config(format!("{name} out of range")));
    }
    Ok(())
}

fn sanitize_message(message: &str, max_len: usize) -> Result<String, ServiceError> {
    let SANITIZED: String = message.chars()
        .filter(|&c| c.is_ascii_graphic() || c == ' ')
        .take(max_len)
        .collect::<String>()
        .replace(['[', ']'], "");
    
    if SANITIZED.is_empty() {
        return Err(ServiceError::Config("Message cannot be empty after sanitization".to_string()));
    }
    Ok(SANITIZED)
}

pub fn load_config() -> Result<Config, ServiceError> {
    const ALLOWED_CONFIGS: &[&str] = &[
        "/etc/rust-service/config.toml",
        "/opt/rust-service/config.toml",
        "/usr/local/etc/rust-service/config.toml"
    ];
    
    let CONFIG_PATH = ALLOWED_CONFIGS.iter()
        .find(|&&path| Path::new(path).exists())
        .ok_or_else(|| ServiceError::Config("No valid config file found".to_string()))?;

    let CONTENT = fs::read_to_string(CONFIG_PATH)
        .map_err(|e| ServiceError::Config(format!("Failed to read config file {CONFIG_PATH}: {e}")))?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let METADATA = fs::metadata(CONFIG_PATH)
            .map_err(|e| ServiceError::Config(format!("Cannot read config file metadata for {CONFIG_PATH}: {e}")))?;
        
        if METADATA.mode() & 0o022 != 0 {
            return Err(ServiceError::Config("Config file has insecure permissions".to_string()));
        }
    }
    
    let mut CONFIG: Config = toml::from_str(&CONTENT)
        .map_err(|e| ServiceError::Config(format!("Invalid configuration format in {CONFIG_PATH}: {e}")))?;

    // Validate all fields using configurable limits
    validate_config_field(&CONFIG.SERVICE_NAME.len(), &1, &CONFIG.MAX_SERVICE_NAME_LEN, "service_name")?;
    validate_config_field(&CONFIG.MESSAGE.len(), &1, &CONFIG.MAX_MESSAGE_LEN, "message")?;
    validate_config_field(&CONFIG.LOG_FILE_PATH.len(), &1, &CONFIG.MAX_LOG_PATH_LEN, "log_file_path")?;
    validate_config_field(&CONFIG.TIME_INTERVAL, &1, &CONFIG.MAX_TIME_INTERVAL, "time_interval")?;
    validate_config_field(&CONFIG.MEMORY_THRESHOLD, &1, &CONFIG.MAX_THRESHOLD_PERCENT, "memory_threshold")?;
    validate_config_field(&CONFIG.DISK_THRESHOLD, &1, &CONFIG.MAX_THRESHOLD_PERCENT, "disk_threshold")?;
    validate_config_field(&CONFIG.MIN_FD_LIMIT, &1, &CONFIG.MAX_FD_LIMIT, "min_fd_limit")?;
    validate_config_field(&CONFIG.MAX_SERVICE_NAME_LEN, &1, &CONFIG.MAX_CONFIG_FIELD_LEN, "max_service_name_len")?;
    validate_config_field(&CONFIG.MAX_MESSAGE_LEN, &1, &CONFIG.MAX_CONFIG_FIELD_LEN, "max_message_len")?;
    validate_config_field(&CONFIG.MAX_LOG_PATH_LEN, &1, &CONFIG.MAX_CONFIG_FIELD_LEN, "max_log_path_len")?;

    CONFIG.MESSAGE = sanitize_message(&CONFIG.MESSAGE, CONFIG.MAX_MESSAGE_LEN)?;

    if !CONFIG.SERVICE_NAME.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(ServiceError::Config("Invalid service name characters".to_string()));
    }

    let LOG_PATH = Path::new(&CONFIG.LOG_FILE_PATH);
    if !LOG_PATH.is_absolute() {
        return Err(ServiceError::Config("Log path must be absolute".to_string()));
    }

    Ok(CONFIG)
}
