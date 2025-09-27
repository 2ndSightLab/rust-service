#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_file_lock_cleanup_on_error() {
        let src_path = Path::new("src/service/logging.rs");
        assert!(src_path.exists(), "logging.rs not found");

        let content = fs::read_to_string(src_path).unwrap();

        // Check for RAII guard pattern for automatic cleanup
        assert!(
            content.contains("FileLockGuard"),
            "Missing RAII guard for file lock cleanup"
        );

        // Verify Drop implementation for automatic unlock
        assert!(
            content.contains("impl Drop for FileLockGuard"),
            "Missing Drop implementation for automatic unlock"
        );

        // Check that guard is used
        assert!(
            content.contains("let _LOCK_GUARD = FileLockGuard(FD)"),
            "Missing RAII guard usage"
        );
    }
}
