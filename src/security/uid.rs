use crate::error::ServiceError;

pub fn get_current_uid() -> Result<u32, ServiceError> {
    #[cfg(unix)]
    {
        Ok(unsafe { libc::getuid() })
    }
    #[cfg(not(unix))]
    {
        Err(ServiceError::Config("UID operations not supported on this platform".to_string()))
    }
}
