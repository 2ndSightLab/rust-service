use crate::service::error::ServiceError;

/// Gets the current user ID.
///
/// # Errors
/// Returns `ServiceError::Config` if unable to get the current user ID.
pub fn get_current_uid() -> Result<u32, ServiceError> {
    #[cfg(unix)]
    {
        // SAFETY: getuid() is always safe - it cannot fail and has no side effects
        let UID = unsafe { libc::getuid() };

        // Validate UID is reasonable (not overflow value)
        if UID == u32::MAX {
            return Err(ServiceError::Config(
                "Invalid UID returned by system".to_string(),
            ));
        }

        Ok(UID)
    }
    #[cfg(not(unix))]
    {
        Err(ServiceError::Config(
            "UID operations not supported on this platform".to_string(),
        ))
    }
}
