use crate::service::error::ServiceError;

pub fn get_file_descriptor_limit() -> Result<u64, ServiceError> {
    #[cfg(unix)]
    {
        let mut RLIMIT = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };

        // SAFETY: getrlimit() is safe when passed a valid resource type and a valid pointer
        // to a properly initialized rlimit struct. We provide both requirements here.
        let RESULT = unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &raw mut RLIMIT) };

        if RESULT != 0 {
            // Get errno for better error reporting
            let ERRNO = unsafe { *libc::__errno_location() };
            return Err(ServiceError::Config(format!(
                "Failed to get file descriptor limit: errno {ERRNO}"
            )));
        }

        // Validate the returned limit is reasonable
        if RLIMIT.rlim_cur == 0 || RLIMIT.rlim_cur == u64::MAX {
            return Err(ServiceError::Config(
                "Invalid file descriptor limit returned by system".to_string(),
            ));
        }

        Ok(RLIMIT.rlim_cur)
    }
    #[cfg(not(unix))]
    {
        Err(ServiceError::Config(
            "File descriptor limit check not supported on this platform".to_string(),
        ))
    }
}
