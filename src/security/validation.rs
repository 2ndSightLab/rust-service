use crate::config::Config;
use crate::error::ServiceError;
use super::uid::get_current_uid;
use super::limits::get_file_descriptor_limit;

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
        
        // Get expected UID for service user
        if let Ok(OUTPUT) = std::process::Command::new("id").arg("-u").arg(SERVICE_NAME).output()
            && OUTPUT.status.success()
            && let Ok(EXPECTED_UID) = String::from_utf8_lossy(&OUTPUT.stdout).trim().parse::<u32>()
            && CURRENT_UID != EXPECTED_UID {
                return Err(ServiceError::Config(format!("Service running as wrong user (expected: {EXPECTED_UID}, actual: {CURRENT_UID})")));
            }
    }
    Ok(())
}

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
