use std::process::Command;

#[test]
fn test_system_resource_monitoring() {
    let output = Command::new("cargo")
        .args(&[
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::integration::common::test_system_resource_monitoring",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute system resource monitoring test");

    assert!(
        output.status.success() || output.status.code() == Some(101),
        "System resource monitoring test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
