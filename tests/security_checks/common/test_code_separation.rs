#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_no_test_code_in_src() {
        let src_dir = Path::new("src");
        assert!(src_dir.exists(), "src directory not found");
        
        fn check_file_for_test_code(path: &Path) -> Vec<String> {
            let mut violations = Vec::new();
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains("#[cfg(test)]") {
                    violations.push(format!("{}: Contains #[cfg(test)]", path.display()));
                }
                if content.contains("#[test]") {
                    violations.push(format!("{}: Contains #[test]", path.display()));
                }
            }
            violations
        }
        
        fn walk_src_dir(dir: &Path) -> Vec<String> {
            let mut violations = Vec::new();
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        violations.extend(walk_src_dir(&path));
                    } else if path.extension().map_or(false, |ext| ext == "rs") {
                        violations.extend(check_file_for_test_code(&path));
                    }
                }
            }
            violations
        }
        
        let violations = walk_src_dir(src_dir);
        assert!(violations.is_empty(), "Test code found in src: {:?}", violations);
    }
}
