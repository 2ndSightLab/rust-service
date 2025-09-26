#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_service_user_existence_required() {
        let validation_path = Path::new("src/security/validation.rs");
        assert!(validation_path.exists(), "validation.rs not found");
        
        let content = fs::read_to_string(validation_path).unwrap();
        
        // Check that service user existence is required
        assert!(content.contains("ok_or_else"), 
                "Missing required service user existence check");
        
        // Verify error message for non-existent user
        assert!(content.contains("Service user") && content.contains("does not exist"), 
                "Missing error message for non-existent service user");
        
        // Ensure no silent continuation for missing users
        assert!(!content.contains("if let Some(user)"), 
                "Still allowing silent continuation for missing service users");
    }
}
