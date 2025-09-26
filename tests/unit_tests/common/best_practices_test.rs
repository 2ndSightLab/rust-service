#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_best_practices_compliance() {
        let output = Command::new("./best-practices.sh")
            .output()
            .expect("Failed to execute best-practices.sh");

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            output.status.success(),
            "best-practices.sh failed with exit code: {}. Output: {}",
            output.status.code().unwrap_or(-1),
            stderr
        );

        // Only fail on actual clippy warnings (lines that start with "warning:" and contain file paths)
        let clippy_warnings: Vec<&str> = stderr
            .lines()
            .filter(|line| line.trim().starts_with("warning:") && line.contains(" --> "))
            .collect();

        assert!(
            clippy_warnings.is_empty(),
            "Best practices check found clippy warnings:\n{}",
            clippy_warnings.join("\n")
        );
    }
}
