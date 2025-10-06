use std::process::Command;

#[test]
fn test_script_validation() {
    let OUTPUT = Command::new("cargo")
        .args([
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::unit::common::test_script_validation",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute script validation test");

    assert!(
        OUTPUT.status.success() || OUTPUT.status.code() == Some(101),
        "Script validation test failed: {}",
        String::from_utf8_lossy(&OUTPUT.stderr)
    );
}
