pub fn get_current_uid() -> u32 {
    #[cfg(unix)]
    {
        // SAFETY: getuid() is always safe to call - it simply returns the current user ID
        // and has no side effects or memory safety concerns
        unsafe { libc::getuid() }
    }
    #[cfg(not(unix))]
    {
        0
    }
}
