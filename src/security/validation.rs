use crate::config::Config;
use crate::error::ServiceError;
use super::uid::get_current_uid;
use super::limits::get_file_descriptor_limit;
use users::get_user_by_name;

/// Validates service user name and prevents running as root.
///
/// # Errors
/// Returns `ServiceError::Config` if:
/// - Service name contains invalid characters
/// - Service name exceeds maximum length
/// - Service is running as root user
pub fn validate_service_user(SERVICE_NAME: &str, MAX_LEN: usize) -> Result<(), ServiceError> {
    #[cfg(unix)]
    {
        // Sanitize service name to prevent command injection
        if !SERVICE_NAME.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ServiceError::Config("Invalid service name characters".to_string()));
        }
        
        if SERVICE_NAME.len() > MAX_LEN {
            return Err(ServiceError::Config(format!("Service name too long: {} > {MAX_LEN}", SERVICE_NAME.len())));
        }
        
        let CURRENT_UID = get_current_uid();
        
        // Get expected UID for service user using native system calls
        if let Some(user) = get_user_by_name(SERVICE_NAME) {
            let EXPECTED_UID = user.uid();
            if CURRENT_UID != EXPECTED_UID {
                return Err(ServiceError::Config(format!("Service running as wrong user (expected: {EXPECTED_UID}, actual: {CURRENT_UID})")));
            }
        }
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
        let UID = get_current_uid();
        if UID == 0 {
            return Err(ServiceError::Config("Service should not run as root".to_string()));
        }
        
        // Check file descriptor limits using configurable minimum
        let FD_LIMIT = get_file_descriptor_limit().map_err(ServiceError::Config)?;
        if FD_LIMIT < CONFIG.MIN_FD_LIMIT {
            return Err(ServiceError::Config(format!("Insufficient file descriptor limit: {FD_LIMIT} < {}", CONFIG.MIN_FD_LIMIT)));
        }
    }
    
    Ok(())
}
