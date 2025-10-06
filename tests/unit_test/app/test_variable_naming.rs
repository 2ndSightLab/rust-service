use std::process::Command;

#[test]
fn test_variable_naming() {
    let OUTPUT = Command::new("cargo")
        .args([
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::unit::common::variable_naming_test",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute variable naming test");

    assert!(
        OUTPUT.status.success() || OUTPUT.status.code() == Some(101),
        "Variable naming test failed: {}",
        String::from_utf8_lossy(&OUTPUT.stderr)
    );
}
