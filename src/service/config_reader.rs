use crate::action::config::ActionConfig;
use crate::security::validation::read_config_file;
pub use crate::service::config::{
    Config, get_config_file_name as get_service_config_file_name, validate_all_config_fields,
};
use crate::service::service_error::ServiceError;
use std::path::Path;

/// Generic function to load config from executable directory by filename
fn load_config_by_filename(filename: &str) -> Result<String, ServiceError> {
    let EXE_PATH = std::env::current_exe()
        .map_err(|e| ServiceError::Config(format!("Cannot determine executable path: {e}")))?;

    let EXE_DIR = EXE_PATH
        .parent()
        .ok_or_else(|| ServiceError::Config("Cannot determine executable directory".to_string()))?;

    let CONFIG_PATH = EXE_DIR.join(filename);

    // Only canonicalize if file exists to prevent directory traversal
    let FINAL_PATH = if CONFIG_PATH.exists() {
        CONFIG_PATH
            .canonicalize()
            .map_err(|e| ServiceError::Config(format!("Cannot canonicalize config path: {e}")))?
    } else {
        CONFIG_PATH
    };

    read_config_file(&[FINAL_PATH
        .to_str()
        .ok_or_else(|| ServiceError::Config("Invalid config path".to_string()))?])
}

/// Loads and validates configuration from system directories.
///
/// # Errors
/// Returns `ServiceError::Config` if:
/// - No valid config file is found in system directories
/// - Config file has invalid permissions or format
/// - Configuration values fail validation checks
pub fn load_config() -> Result<Config, ServiceError> {
    let FILENAME = crate::service::config::get_config_file_name();
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

    // Validate install directory
    let INSTALL_PATH = Path::new(&CONFIG.INSTALL_DIR);

    if !INSTALL_PATH.is_absolute() {
        return Err(ServiceError::Config(
            "Install path must be absolute".to_string(),
        ));
    }

    Ok(CONFIG)
}

/// Loads and validates action configuration from executable directory.
///
/// # Errors
/// Returns `ServiceError` if configuration cannot be loaded or is invalid.
pub fn load_action_config() -> Result<ActionConfig, ServiceError> {
    let FILENAME = crate::action::config::get_config_file_name();
    let CONTENT = load_config_by_filename(FILENAME)?;

    let CONFIG: ActionConfig = toml::from_str(&CONTENT)
        .map_err(|e| ServiceError::Config(format!("Invalid action configuration format: {e}")))?;

    Ok(CONFIG)
}
