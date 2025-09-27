use super::limits::get_file_descriptor_limit;
use super::uid::get_current_uid;
use crate::service::config::Config;
use crate::service::error::ServiceError;

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
