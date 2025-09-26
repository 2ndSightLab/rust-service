#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_no_circular_user_validation() {
        let validation_path = Path::new("src/security/validation.rs");
        assert!(validation_path.exists(), "validation.rs not found");

        let content = fs::read_to_string(validation_path).unwrap();

        // Ensure no user lookup by service name (prevents circular dependency)
        assert!(
            !content.contains("get_user_by_name"),
            "Circular dependency: service validates against user with same name"
        );

        // Ensure no UID comparison with service user
        assert!(
            !content.contains("EXPECTED_UID"),
            "Still comparing UIDs with service user (circular dependency)"
        );

        // Verify documentation explains the security fix
        assert!(
            content.contains("circular dependency"),
            "Missing documentation about circular dependency fix"
        );
    }
}
