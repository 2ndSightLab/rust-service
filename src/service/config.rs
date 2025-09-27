pub use crate::action::config_action::{
    ActionConfig, get_config_file_name as get_action_config_file_name,
    validate_all_action_config_fields,
};
pub use crate::service::config_service::{
    Config, get_config_file_name as get_service_config_file_name, validate_all_config_fields,
};
use crate::service::error::ServiceError;
use std::fs;
use std::path::Path;

/// Generic function to load config from system directories by filename
fn load_config_by_filename(filename: &str) -> Result<String, ServiceError> {
    let ALLOWED_CONFIGS = [
        format!("/etc/rust-service-debug/{filename}"),
        format!("/opt/rust-service-debug/{filename}"),
        format!("/usr/local/etc/rust-service-debug/{filename}"),
        format!("/etc/rust-service/{filename}"),
        format!("/opt/rust-service/{filename}"),
        format!("/usr/local/etc/rust-service/{filename}"),
    ];

    read_config_file(
        &ALLOWED_CONFIGS
            .iter()
            .map(std::string::String::as_str)
            .collect::<Vec<_>>(),
    )
}

/// Validates that a configuration field value is within specified bounds.
///
/// # Errors
/// Returns `ServiceError` if the value is outside the min/max range.
pub fn validate_config_field<T: PartialOrd>(
    value: &T,
    min: &T,
    max: &T,
    name: &str,
) -> Result<(), ServiceError> {
    if value < min || value > max {
        return Err(ServiceError::Config(format!("{name} out of range")));
    }
    Ok(())
}

/// Sanitizes a message by filtering allowed characters and checking length.
///
/// # Errors
/// Returns `ServiceError` if the message is too long after sanitization.
pub fn sanitize_message(MESSAGE: &str, MAX_LEN: usize) -> Result<String, ServiceError> {
    let SANITIZED: String = MESSAGE
        .chars()
        .filter(|&C| C.is_ascii_alphanumeric() || C == ' ' || C == '.' || C == '-' || C == '_')
        .take(MAX_LEN)
        .collect();

    if SANITIZED.is_empty() {
        return Err(ServiceError::Config(
            "Message cannot be empty after sanitization".to_string(),
        ));
    }
    Ok(SANITIZED)
}

/// Reads configuration from the first available file in the provided paths.
///
/// # Errors
/// Returns `ServiceError` if no configuration file is found or readable.
pub fn read_config_file(config_paths: &[&str]) -> Result<String, ServiceError> {
    let CONFIG_PATH = config_paths
        .iter()
        .find(|&&path| Path::new(path).exists())
        .ok_or_else(|| ServiceError::Config("No valid config file found".to_string()))?;

    // Open file and check permissions on file descriptor to prevent race conditions
    let FILE = fs::File::open(CONFIG_PATH).map_err(|E| {
        ServiceError::Config(format!("Failed to open config file {CONFIG_PATH}: {E}"))
    })?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let METADATA = FILE.metadata().map_err(|E| {
            ServiceError::Config(format!(
                "Cannot read config file metadata for {CONFIG_PATH}: {E}"
            ))
        })?;

        if METADATA.mode() & 0o022 != 0 {
            return Err(ServiceError::Config(
                "Config file has insecure permissions".to_string(),
            ));
        }
    }

    let CONTENT = fs::read_to_string(CONFIG_PATH).map_err(|E| {
        ServiceError::Config(format!("Failed to read config file {CONFIG_PATH}: {E}"))
    })?;

    Ok(CONTENT)
}

/// Loads and validates configuration from system directories.
///
/// # Errors
/// Returns `ServiceError::Config` if:
/// - No valid config file is found in system directories
/// - Config file has invalid permissions or format
/// - Configuration values fail validation checks
pub fn load_config() -> Result<Config, ServiceError> {
    let FILENAME = crate::service::config_service::get_config_file_name();
    let CONTENT = load_config_by_filename(FILENAME)?;

    let CONFIG: Config = toml::from_str(&CONTENT)
        .map_err(|e| ServiceError::Config(format!("Invalid configuration format: {e}")))?;

    // Validate all fields using configurable limits
    validate_all_config_fields(&CONFIG)?;

    if !CONFIG
        .SERVICE_NAME
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(ServiceError::Config(
            "Invalid service name characters".to_string(),
        ));
    }

    let LOG_PATH = Path::new(&CONFIG.LOG_FILE_PATH);
    if !LOG_PATH.is_absolute() {
        return Err(ServiceError::Config(
            "Log path must be absolute".to_string(),
        ));
    }

    // Basic path validation - detailed security checks happen at use time
    let ALLOWED_PREFIXES = ["/var/log", "/opt"];
    if !ALLOWED_PREFIXES
        .iter()
        .any(|prefix| LOG_PATH.starts_with(prefix))
    {
        return Err(ServiceError::Config(
            "Log path not in allowed directory".to_string(),
        ));
    }

    // Validate install and config directories
    let INSTALL_PATH = Path::new(&CONFIG.INSTALL_DIR);
    let CONFIG_PATH_DIR = Path::new(&CONFIG.CONFIG_DIR);

    if !INSTALL_PATH.is_absolute() || !CONFIG_PATH_DIR.is_absolute() {
        return Err(ServiceError::Config(
            "Install and config paths must be absolute".to_string(),
        ));
    }

    Ok(CONFIG)
}

/// Loads and validates action configuration from system directories.
///
/// # Errors
/// Returns `ServiceError` if configuration cannot be loaded or is invalid.
pub fn load_action_config() -> Result<ActionConfig, ServiceError> {
    let FILENAME = crate::action::config_action::get_config_file_name();
    let CONTENT = load_config_by_filename(FILENAME)?;

    let CONFIG: ActionConfig = toml::from_str(&CONTENT)
        .map_err(|e| ServiceError::Config(format!("Invalid action configuration format: {e}")))?;

    Ok(CONFIG)
}
