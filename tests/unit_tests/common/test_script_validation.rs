#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_script_format_validation() {
        let script_content =
            fs::read_to_string("./scripts/test.sh").expect("test.sh file not found");

        // Verify the script contains the required output format
        assert!(
            script_content.contains("Test Results Summary:"),
            "test.sh missing Test Results Summary section"
        );
        assert!(
            script_content.contains("===================="),
            "test.sh missing summary separator"
        );

        // Check for both PASSED and FAILED format patterns with counts (allowing for color codes)
        assert!(
            script_content.contains("Unit Tests:")
                && (script_content.contains("PASSED") || script_content.contains("FAILED"))
                && script_content.contains("($UNIT_PASSED passed, $UNIT_FAILED failed)"),
            "test.sh missing unit tests summary with pass/fail counts"
        );
        assert!(
            script_content.contains("Security Checks:")
                && (script_content.contains("PASSED") || script_content.contains("FAILED"))
                && script_content.contains("($SECURITY_PASSED passed, $SECURITY_FAILED failed)"),
            "test.sh missing security checks summary with pass/fail counts"
        );
        assert!(
            script_content.contains("Integration Tests:")
                && (script_content.contains("PASSED") || script_content.contains("FAILED"))
                && script_content
                    .contains("($INTEGRATION_PASSED passed, $INTEGRATION_FAILED failed)"),
            "test.sh missing integration tests summary with pass/fail counts"
        );
        assert!(
            script_content.contains("All Tests:")
                && (script_content.contains("PASSED") || script_content.contains("FAILED"))
                && script_content.contains("($TOTAL_PASSED passed, $TOTAL_FAILED failed)"),
            "test.sh missing total tests summary with pass/fail counts"
        );

        // Verify the script extracts counts from cargo output
        assert!(
            script_content.contains("grep -o '[0-9]\\+ passed'"),
            "test.sh not extracting passed test counts"
        );
        assert!(
            script_content.contains("grep -o '[0-9]\\+ failed'"),
            "test.sh not extracting failed test counts"
        );
    }
}
