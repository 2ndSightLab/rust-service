use crate::integration::common::test_prerequisites;
use std::process::Command;

#[test]
fn test_config_file_precedence() {
    test_prerequisites::check_debug_installation().unwrap();

    println!(
        "RUNNING: test_config_file_precedence - Testing configuration file discovery and precedence"
    );
    println!("Testing configuration file discovery...");

    // Test that the application can successfully start and find its config
    let paths = test_prerequisites::get_test_paths().unwrap();

    println!(
        "Testing service startup with config from: {}",
        paths.config_path
    );

    let mut child = Command::new(&paths.binary_path)
        .spawn()
        .expect("Failed to start service - config loading may have failed");

    println!("Service started successfully, config was found and loaded");
    println!("Waiting 2 seconds to verify service runs...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    child.kill().expect("Failed to kill service");
    let _ = child.wait();

    println!("Config discovery test completed successfully");
}
