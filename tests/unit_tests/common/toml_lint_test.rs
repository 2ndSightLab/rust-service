#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_non_snake_case_override_present() {
        let CARGO_CONTENT = fs::read_to_string("Cargo.toml").expect("Cargo.toml not found");

        // Check that non_snake_case is explicitly allowed
        assert!(
            CARGO_CONTENT.contains("non_snake_case = \"allow\""),
            "Cargo.toml must contain 'non_snake_case = \"allow\"' to override Rust naming conventions"
        );

        // Verify it's in the lints section
        assert!(
            CARGO_CONTENT.contains("[lints.rust]"),
            "Cargo.toml must have [lints.rust] section"
        );
    }
}
