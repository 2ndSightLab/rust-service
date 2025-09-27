#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_checked_arithmetic_operations() {
        let monitoring_path = Path::new("src/service/monitoring.rs");
        assert!(monitoring_path.exists(), "monitoring.rs not found");

        let content = fs::read_to_string(monitoring_path).unwrap();

        // Ensure disk calculations use checked arithmetic
        assert!(
            content.contains("checked_mul"),
            "Missing checked multiplication for disk calculations"
        );

        // Verify no unsafe multiplication in disk operations
        assert!(
            !content.contains("STATS.blocks() * STATS.fragment_size()"),
            "Unsafe multiplication found - use checked_mul() to prevent overflow"
        );

        // Ensure existing checked operations are preserved
        assert!(
            content.contains("checked_sub"),
            "Missing checked subtraction operations"
        );
        assert!(
            content.contains("checked_div"),
            "Missing checked division operations"
        );

        // Verify overflow error handling
        assert!(
            content.contains("calculation overflow"),
            "Missing overflow error messages"
        );
    }
}
