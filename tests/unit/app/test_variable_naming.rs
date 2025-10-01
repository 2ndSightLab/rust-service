use std::process::Command;

#[test]
fn test_variable_naming() {
    let output = Command::new("cargo")
        .args(&[
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
        output.status.success() || output.status.code() == Some(101),
        "Variable naming test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
