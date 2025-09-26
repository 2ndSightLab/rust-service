#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_file_lock_cleanup_on_error() {
        let src_path = Path::new("src/logging.rs");
        assert!(src_path.exists(), "logging.rs not found");
        
        let content = fs::read_to_string(src_path).unwrap();
        
        // Check that flock unlock is called on error paths
        assert!(content.contains("libc::flock(FD, libc::LOCK_UN)"), 
                "Missing file lock cleanup on error");
        
        // Verify unlock is called in error closure
        assert!(content.contains("unsafe { libc::flock(FD, libc::LOCK_UN) };"), 
                "Missing explicit unlock in error handling");
    }
}
