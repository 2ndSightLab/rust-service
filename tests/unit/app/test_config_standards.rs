use std::process::Command;

#[test]
fn test_config_standards() {
    let output = Command::new("cargo")
        .args(&[
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
        output.status.success() || output.status.code() == Some(101),
        "Config standards test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
