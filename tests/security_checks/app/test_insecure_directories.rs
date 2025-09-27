#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_no_insecure_directories() {
        let config_path = Path::new("src/service/config.rs");
        let logging_path = Path::new("src/service/logging.rs");

        assert!(config_path.exists(), "config.rs not found");
        assert!(logging_path.exists(), "logging.rs not found");

        let config_content = fs::read_to_string(config_path).unwrap();
        let logging_content = fs::read_to_string(logging_path).unwrap();

        // Check that /tmp is not in allowed directories
        assert!(
            !config_content.contains("\"/tmp\""),
            "/tmp found in config allowed directories"
        );
        assert!(
            !logging_content.contains("\"/tmp\""),
            "/tmp found in logging allowed directories"
        );

        // Verify secure directories are present
        assert!(
            config_content.contains("\"/var/log\""),
            "Missing /var/log in config"
        );
        assert!(
            logging_content.contains("\"/var/log\""),
            "Missing /var/log in logging"
        );
    }
}
