#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_service_name_validation_only() {
        let validation_path = Path::new("src/security/validation.rs");
        assert!(validation_path.exists(), "validation.rs not found");

        let content = fs::read_to_string(validation_path).unwrap();

        // Check that service name format is validated
        assert!(
            content.contains("is_alphanumeric"),
            "Missing service name character validation"
        );

        // Verify length validation exists
        assert!(
            content.contains("Service name too long"),
            "Missing service name length validation"
        );

        // Ensure no user existence checks (security fix)
        assert!(
            !content.contains("does not exist"),
            "Still checking user existence (creates circular dependency)"
        );
    }
}
