#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_all_dependencies_declared() {
        let cargo_path = Path::new("Cargo.toml");
        assert!(cargo_path.exists(), "Cargo.toml not found");

        let cargo_content = fs::read_to_string(cargo_path).unwrap();

        // Check that essential dependencies are declared
        assert!(
            cargo_content.contains("users ="),
            "Missing users dependency"
        );
        assert!(cargo_content.contains("libc ="), "Missing libc dependency");
        assert!(
            cargo_content.contains("thiserror ="),
            "Missing thiserror dependency"
        );
        assert!(cargo_content.contains("log ="), "Missing log dependency");
        assert!(
            cargo_content.contains("serde ="),
            "Missing serde dependency"
        );
    }
}
