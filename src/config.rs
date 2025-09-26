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
    let sanitized: String = message.chars()
        .filter(|&c| c.is_ascii_graphic() || c == ' ')
        .take(max_len)
        .collect::<String>()
        .replace(['[', ']'], "");
    
    if sanitized.is_empty() {
        return Err(ServiceError::Config("Message cannot be empty after sanitization".to_string()));
    }
    Ok(sanitized)
}

pub fn load_config() -> Result<Config, ServiceError> {
    const ALLOWED_CONFIGS: &[&str] = &[
        "/etc/rust-service/config.toml",
        "/opt/rust-service/config.toml",
        "/usr/local/etc/rust-service/config.toml"
    ];
    
    let config_path = ALLOWED_CONFIGS.iter()
        .find(|&&path| Path::new(path).exists())
        .ok_or_else(|| ServiceError::Config("No valid config file found".to_string()))?;

    let content = fs::read_to_string(config_path)
        .map_err(|e| ServiceError::Config(format!("Failed to read config file {config_path}: {e}")))?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::metadata(config_path)
            .map_err(|e| ServiceError::Config(format!("Cannot read config file metadata for {config_path}: {e}")))?;
        
        if metadata.mode() & 0o022 != 0 {
            return Err(ServiceError::Config("Config file has insecure permissions".to_string()));
        }
    }
    
    let mut config: Config = toml::from_str(&content)
        .map_err(|e| ServiceError::Config(format!("Invalid configuration format in {config_path}: {e}")))?;

    // Validate all fields using configurable limits
    validate_config_field(&config.SERVICE_NAME.len(), &1, &config.MAX_SERVICE_NAME_LEN, "service_name")?;
    validate_config_field(&config.MESSAGE.len(), &1, &config.MAX_MESSAGE_LEN, "message")?;
    validate_config_field(&config.LOG_FILE_PATH.len(), &1, &config.MAX_LOG_PATH_LEN, "log_file_path")?;
    validate_config_field(&config.TIME_INTERVAL, &1, &config.MAX_TIME_INTERVAL, "time_interval")?;
    validate_config_field(&config.MEMORY_THRESHOLD, &1, &config.MAX_THRESHOLD_PERCENT, "memory_threshold")?;
    validate_config_field(&config.DISK_THRESHOLD, &1, &config.MAX_THRESHOLD_PERCENT, "disk_threshold")?;
    validate_config_field(&config.MIN_FD_LIMIT, &1, &config.MAX_FD_LIMIT, "min_fd_limit")?;
    validate_config_field(&config.MAX_SERVICE_NAME_LEN, &1, &config.MAX_CONFIG_FIELD_LEN, "max_service_name_len")?;
    validate_config_field(&config.MAX_MESSAGE_LEN, &1, &config.MAX_CONFIG_FIELD_LEN, "max_message_len")?;
    validate_config_field(&config.MAX_LOG_PATH_LEN, &1, &config.MAX_CONFIG_FIELD_LEN, "max_log_path_len")?;

    config.MESSAGE = sanitize_message(&config.MESSAGE, config.MAX_MESSAGE_LEN)?;

    if !config.SERVICE_NAME.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(ServiceError::Config("Invalid service name characters".to_string()));
    }

    let log_path = Path::new(&config.LOG_FILE_PATH);
    if !log_path.is_absolute() {
        return Err(ServiceError::Config("Log path must be absolute".to_string()));
    }

    Ok(config)
}
