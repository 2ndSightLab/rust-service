#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_no_path_based_metadata_checks() {
        let config_path = Path::new("src/service/config.rs");
        assert!(config_path.exists(), "config.rs not found");

        let content = fs::read_to_string(config_path).unwrap();

        // Ensure metadata checks use file descriptors, not paths
        assert!(
            !content.contains("fs::metadata(CONFIG_PATH)"),
            "Race condition: using fs::metadata() on path instead of file descriptor"
        );

        // Verify file is opened before metadata check
        assert!(
            content.contains("FILE.metadata()"),
            "Missing file descriptor metadata check"
        );

        // Ensure file is opened before permission validation
        let open_pos = content.find("fs::File::open").unwrap_or(usize::MAX);
        let metadata_pos = content.find("FILE.metadata()").unwrap_or(0);
        assert!(
            open_pos < metadata_pos,
            "File must be opened before metadata check to prevent race conditions"
        );
    }
}
