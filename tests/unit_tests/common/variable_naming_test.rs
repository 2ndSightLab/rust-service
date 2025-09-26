#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_screaming_snake_case_variables() {
        fn check_file(path: &Path, regex: &Regex, violations: &mut Vec<String>) {
            if let Ok(content) = fs::read_to_string(path) {
                for (line_num, line) in content.lines().enumerate() {
                    if let Some(captures) = regex.captures(line) {
                        let var_name = &captures[1];
                        // Skip if it's already SCREAMING_SNAKE_CASE
                        if !var_name
                            .chars()
                            .all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
                        {
                            violations.push(format!(
                                "{}:{}: Variable '{}' not in SCREAMING_SNAKE_CASE",
                                path.display(),
                                line_num + 1,
                                var_name
                            ));
                        }
                    }
                }
            }
        }

        fn walk_dir(dir: &Path, regex: &Regex, violations: &mut Vec<String>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        walk_dir(&path, regex, violations);
                    } else if path.extension().is_some_and(|ext| ext == "rs") {
                        check_file(&path, regex, violations);
                    }
                }
            }
        }

        let src_dir = Path::new("src");
        assert!(src_dir.exists(), "Source directory not found");

        // Match variable declarations that are NOT in SCREAMING_SNAKE_CASE
        let non_screaming_regex =
            Regex::new(r"let\s+([a-z][a-zA-Z0-9_]*|[A-Z][a-z][a-zA-Z0-9_]*)\s*[=:]").unwrap();
        let mut violations = Vec::new();

        walk_dir(src_dir, &non_screaming_regex, &mut violations);

        assert!(
            violations.is_empty(),
            "Found variables not in SCREAMING_SNAKE_CASE:\n{}",
            violations.join("\n")
        );
    }
}
