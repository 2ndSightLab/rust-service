#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_path_canonicalization() {
        let config_path = Path::new("src/config.rs");
        assert!(config_path.exists(), "config.rs not found");
        
        let content = fs::read_to_string(config_path).unwrap();
        
        // Check that path canonicalization is implemented
        assert!(content.contains("canonicalize()"), 
                "Missing path canonicalization to prevent directory traversal");
        
        // Verify allowed directory validation
        assert!(content.contains("ALLOWED_PREFIXES"), 
                "Missing allowed directory prefix validation");
        
        // Check for proper error on invalid paths
        assert!(content.contains("Log path not in allowed directory"), 
                "Missing validation error for disallowed paths");
    }
}
