use std::process::Command;

#[test]
fn test_prerequisites_check() {
    let output = Command::new("cargo")
        .args(&[
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::integration::common::test_prerequisites_check",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute prerequisites check test");

    assert!(
        output.status.success() || output.status.code() == Some(101),
        "Prerequisites check test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
