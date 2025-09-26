pub fn get_file_descriptor_limit() -> Result<u64, String> {
    #[cfg(unix)]
    {
        let mut RLIMIT = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
        // SAFETY: getrlimit() is safe when passed a valid resource type and a valid pointer
        // to a properly initialized rlimit struct. We provide both requirements here.
        if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &raw mut RLIMIT) } == 0 {
            Ok(RLIMIT.rlim_cur)
        } else {
            Err("Failed to get file descriptor limit".to_string())
        }
    }
    #[cfg(not(unix))]
    {
        Err("File descriptor limit check not supported on this platform".to_string())
    }
}
