use std::process::Command;

pub fn run_common_tests(test_type: &str) {
    let output = Command::new("cargo")
        .args(&["test", "--manifest-path", "../rust-common-tests/Cargo.toml", "--test", test_type])
        .current_dir(".")
        .output()
        .expect(&format!("Failed to execute {} tests", test_type));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print the output so we can see individual test results
    println!("{}", stdout);
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }
    
    // Check if any tests failed
    let failed_count = stdout.lines()
        .filter(|line| line.contains("test result:"))
        .filter_map(|line| {
            if line.contains("failed") {
                line.split_whitespace()
                    .find(|word| word.parse::<u32>().is_ok() && line.split(word).nth(1).unwrap_or("").contains("failed"))
                    .and_then(|s| s.parse::<u32>().ok())
            } else {
                None
            }
        })
        .sum::<u32>();
    
    assert_eq!(failed_count, 0, "Some {} tests failed", test_type);
}
