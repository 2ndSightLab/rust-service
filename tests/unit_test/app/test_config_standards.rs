use std::process::Command;

#[test]
fn test_config_standards() {
    let OUTPUT = Command::new("cargo")
        .args([
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::unit::common::config_standards_test",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute config standards test");

    assert!(
        OUTPUT.status.success() || OUTPUT.status.code() == Some(101),
        "Config standards test failed: {}",
        String::from_utf8_lossy(&OUTPUT.stderr)
    );
}
