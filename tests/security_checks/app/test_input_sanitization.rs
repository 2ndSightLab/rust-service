#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_message_sanitization_whitelist() {
        let config_path = Path::new("src/service/config.rs");
        let logging_path = Path::new("src/service/logging.rs");

        assert!(
            config_path.exists() && logging_path.exists(),
            "Source files not found"
        );

        let config_content = fs::read_to_string(config_path).unwrap();
        let logging_content = fs::read_to_string(logging_path).unwrap();

        // Check whitelist approach in config sanitization
        assert!(
            config_content.contains(
                "is_ascii_alphanumeric() || C == ' ' || C == '.' || C == '-' || C == '_'"
            ),
            "Config sanitization not using whitelist approach"
        );

        // Check whitelist approach in logging sanitization
        assert!(
            logging_content.contains(
                "is_ascii_alphanumeric() || c == ' ' || c == '.' || c == '-' || c == '_'"
            ),
            "Logging sanitization not using whitelist approach"
        );

        // Ensure no ascii_graphic filter (too permissive)
        assert!(
            !config_content.contains("is_ascii_graphic()"),
            "Config still using permissive ascii_graphic filter"
        );
        assert!(
            !logging_content.contains("is_ascii_graphic()"),
            "Logging still using permissive ascii_graphic filter"
        );
    }
}
