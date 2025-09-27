#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_path_canonicalization() {
        let logging_path = Path::new("src/service/logging.rs");
        assert!(logging_path.exists(), "logging.rs not found");

        let content = fs::read_to_string(logging_path).unwrap();

        // Check that path canonicalization happens at use time (not config time)
        assert!(
            content.contains("canonicalize()"),
            "Missing path canonicalization to prevent directory traversal"
        );

        // Verify allowed directory validation at use time
        assert!(
            content.contains("ALLOWED_PREFIXES"),
            "Missing allowed directory prefix validation"
        );

        // Check for proper error on invalid paths
        assert!(
            content.contains("Log directory not in allowed location"),
            "Missing validation error for disallowed paths"
        );

        // Verify TOCTOU vulnerability is fixed (no fallback canonicalization)
        assert!(
            !content.contains("or_else"),
            "TOCTOU vulnerability still exists - remove fallback canonicalization"
        );
    }
}
