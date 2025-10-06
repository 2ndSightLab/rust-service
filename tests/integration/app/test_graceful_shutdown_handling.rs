use std::process::Command;

#[test]
fn test_graceful_shutdown_handling() {
    let OUTPUT = Command::new("cargo")
        .args([
            "test",
            "--manifest-path",
            "../rust-common-tests/Cargo.toml",
            "tests::integration::common::test_graceful_shutdown_handling",
            "--",
            "--nocapture",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute graceful shutdown test");

    assert!(
        OUTPUT.status.success() || OUTPUT.status.code() == Some(101),
        "Graceful shutdown test failed: {}",
        String::from_utf8_lossy(&OUTPUT.stderr)
    );
}
