use std::process::Command;

#[test]
fn test_security_workflow_integration() {
    let output = Command::new("cargo")
        .args(&[
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::integration::common::test_security_workflow_integration",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute security workflow integration test");

    assert!(
        output.status.success() || output.status.code() == Some(101),
        "Security workflow integration test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
