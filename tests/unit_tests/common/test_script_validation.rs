#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_script_format_validation() {
        let script_content = fs::read_to_string("test.sh").expect("test.sh file not found");

        // Verify the script contains the required output format
        assert!(
            script_content.contains("Test Results Summary:"),
            "test.sh missing Test Results Summary section"
        );
        assert!(
            script_content.contains("===================="),
            "test.sh missing summary separator"
        );
        assert!(
            script_content.contains("✅ Security Checks: PASSED"),
            "test.sh missing security checks summary line"
        );
        assert!(
            script_content.contains("✅ Unit Tests: PASSED"),
            "test.sh missing unit tests summary line"
        );
        assert!(
            script_content.contains("✅ All Tests: PASSED"),
            "test.sh missing total tests summary line"
        );
        assert!(
            script_content.contains("🎉 All tests completed successfully!"),
            "test.sh missing success message"
        );

        // Verify the script calculates totals correctly
        assert!(
            script_content.contains("TOTAL_TESTS=$((SECURITY_TESTS + UNIT_TESTS))"),
            "test.sh not calculating total tests correctly"
        );

        // Verify the script extracts counts from cargo output
        assert!(
            script_content.contains("grep -A20 \"Running tests/security_checks.rs\""),
            "test.sh not extracting security test counts"
        );
        assert!(
            script_content.contains("grep -A20 \"Running tests/unit_tests.rs\""),
            "test.sh not extracting unit test counts"
        );
    }
}
