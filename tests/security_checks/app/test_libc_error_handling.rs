#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_libc_functions_return_results() {
        let uid_path = Path::new("src/security/uid.rs");
        let limits_path = Path::new("src/security/limits.rs");

        assert!(
            uid_path.exists() && limits_path.exists(),
            "Security module files not found"
        );

        let uid_content = fs::read_to_string(uid_path).unwrap();
        let limits_content = fs::read_to_string(limits_path).unwrap();

        // Check that get_current_uid returns Result for error handling
        assert!(
            uid_content.contains("Result<u32, ServiceError>"),
            "get_current_uid should return Result for proper error handling"
        );

        // Check that get_file_descriptor_limit returns Result with ServiceError
        assert!(
            limits_content.contains("Result<u64, ServiceError>"),
            "get_file_descriptor_limit should return ServiceError type"
        );

        // Verify proper error handling for getrlimit failure (updated implementation)
        assert!(
            limits_content.contains("let RESULT = unsafe { libc::getrlimit")
                && limits_content.contains("if RESULT != 0"),
            "Missing proper error checking for getrlimit system call"
        );
    }
}
