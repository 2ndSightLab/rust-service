use std::process::Command;

#[test]
fn test_prerequisites_check() {
    let OUTPUT = Command::new("cargo")
        .args([
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
        OUTPUT.status.success() || OUTPUT.status.code() == Some(101),
        "Prerequisites check test failed: {}",
        String::from_utf8_lossy(&OUTPUT.stderr)
    );
}
