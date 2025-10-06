use std::process::Command;

#[test]
fn test_best_practices() {
    let OUTPUT = Command::new("cargo")
        .args([
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::unit::common::best_practices_test",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute best practices test");

    assert!(
        OUTPUT.status.success() || OUTPUT.status.code() == Some(101),
        "Best practices test failed: {}",
        String::from_utf8_lossy(&OUTPUT.stderr)
    );
}
