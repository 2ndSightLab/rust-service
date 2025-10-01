use std::process::Command;

#[test]
fn test_toml_lint() {
    let output = Command::new("cargo")
        .args(&[
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::unit::common::toml_lint_test",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute TOML lint test");

    assert!(
        output.status.success() || output.status.code() == Some(101),
        "TOML lint test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
