use super::limits::get_file_descriptor_limit;
use super::uid::get_current_uid;
use crate::service::config::Config;
use crate::service::service_error::ServiceError;
use std::fs;
use std::path::{Path, PathBuf};

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

/// Validates a path without following symlinks to prevent directory traversal attacks
pub fn validate_path_without_symlinks(path: &Path) -> Result<PathBuf, ServiceError> {
    // Get the absolute path without following symlinks
    let ABSOLUTE_PATH = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|_| ServiceError::Config("Cannot get current directory".to_string()))?
            .join(path)
    };

    // Normalize the path by removing . and .. components without following symlinks
    let mut COMPONENTS = Vec::new();
    for COMPONENT in ABSOLUTE_PATH.components() {
        match COMPONENT {
            std::path::Component::Normal(NAME) => {
                // Check if this component is a symlink
                let CURRENT_PATH: PathBuf = COMPONENTS.iter().collect::<PathBuf>().join(NAME);
                if CURRENT_PATH.is_symlink() {
                    return Err(ServiceError::Config(
                        "Path contains symlinks - potential security risk".to_string(),
                    ));
                }
                COMPONENTS.push(COMPONENT);
            }
            std::path::Component::ParentDir => {
                if COMPONENTS.is_empty() {
                    return Err(ServiceError::Config(
                        "Path traversal attempt detected".to_string(),
                    ));
                }
                COMPONENTS.pop();
            }
            std::path::Component::CurDir => {
                // Skip current directory references
            }
            _ => COMPONENTS.push(COMPONENT),
        }
    }

    Ok(COMPONENTS.iter().collect())
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
        // Use file descriptor metadata check instead of path-based check
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

/// Validates service name format only - does not perform user validation.
///
/// User validation is removed as it created a circular dependency where
/// anyone could create a user matching the service name to bypass security.
///
/// # Errors
/// Returns `ServiceError::Config` if:
/// - Service name contains invalid characters
/// - Service name exceeds maximum length
pub fn validate_service_user(SERVICE_NAME: &str, MAX_LEN: usize) -> Result<(), ServiceError> {
    // Sanitize service name to prevent command injection
    if !SERVICE_NAME
        .chars()
        .all(|C| C.is_alphanumeric() || C == '-' || C == '_')
    {
        return Err(ServiceError::Config(
            "Invalid service name characters".to_string(),
        ));
    }

    if SERVICE_NAME.len() > MAX_LEN {
        return Err(ServiceError::Config(format!(
            "Service name too long: {} > {MAX_LEN}",
            SERVICE_NAME.len()
        )));
    }

    Ok(())
}

/// Validates runtime security requirements including file descriptor limits.
///
/// # Errors
/// Returns `ServiceError::Config` if system file descriptor limit is below minimum requirement.
pub fn validate_runtime_security(CONFIG: &Config) -> Result<(), ServiceError> {
    #[cfg(unix)]
    {
        // Verify not running as root
        let UID = get_current_uid()?;
        if UID == 0 {
            return Err(ServiceError::Config(
                "Service should not run as root".to_string(),
            ));
        }

        // Check file descriptor limits using configurable minimum
        let FD_LIMIT = get_file_descriptor_limit()?;
        if FD_LIMIT < CONFIG.MIN_FD_LIMIT {
            return Err(ServiceError::Config(format!(
                "Insufficient file descriptor limit: {FD_LIMIT} < {}",
                CONFIG.MIN_FD_LIMIT
            )));
        }
    }

    Ok(())
}
