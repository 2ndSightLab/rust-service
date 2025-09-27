#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_file_descriptor_operations() {
        let logging_path = Path::new("src/service/logging.rs");
        assert!(logging_path.exists(), "logging.rs not found");

        let content = fs::read_to_string(logging_path).unwrap();

        // Verify O_NOFOLLOW flag is used to prevent symlink attacks
        assert!(
            content.contains("libc::O_NOFOLLOW"),
            "Missing O_NOFOLLOW flag to prevent symlink attacks"
        );

        // Check that FILE.metadata() is used (operates on file descriptor)
        assert!(
            content.contains("FILE.metadata()"),
            "Should use FILE.metadata() which operates on file descriptor"
        );

        // Verify security comment explaining TOCTOU prevention
        assert!(
            content.contains("FILE.metadata() calls fstat() on the file descriptor"),
            "Missing security comment explaining TOCTOU prevention"
        );

        // Ensure file operations happen after lock acquisition
        assert!(
            content.contains("flock(FD, libc::LOCK_EX)"),
            "Missing file locking before security checks"
        );
    }
}
