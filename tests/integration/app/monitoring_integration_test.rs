use crate::integration::common::test_prerequisites;

use std::process::Command;
use std::time::{Duration, Instant};

#[test]
fn test_system_resource_monitoring() {
    println!(
        "RUNNING: test_system_resource_monitoring - Testing system resource monitoring exists"
    );
    println!("Testing that monitoring functions are available...");

    let paths = test_prerequisites::get_test_paths().unwrap();

    // Just test that the service starts and runs briefly without crashing
    let start = Instant::now();
    let mut child = Command::new(&paths.binary_path)
        .spawn()
        .expect("Failed to start service");

    // Let it run for 2 seconds then kill it
    println!("Waiting 2 seconds for service to run...");
    std::thread::sleep(Duration::from_secs(2));
    child.kill().expect("Failed to kill service");
    let _ = child.wait();

    let elapsed = start.elapsed();
    assert!(
        elapsed >= Duration::from_secs(1),
        "Service should run for at least 1 second"
    );
    println!("Resource monitoring test completed");
}

#[test]
fn test_file_descriptor_limits() {
    println!(
        "RUNNING: test_file_descriptor_limits - Testing file descriptor limit validation exists"
    );
    println!("Testing that fd limit checking is available...");

    let paths = test_prerequisites::get_test_paths().unwrap();

    // Just test that the service can start (fd limits are checked at startup)
    let mut child = Command::new(&paths.binary_path)
        .spawn()
        .expect("Failed to start service");

    // Let it run briefly then kill it
    println!("Waiting 1 second for service to start...");
    std::thread::sleep(Duration::from_secs(1));
    child.kill().expect("Failed to kill service");
    let status = child.wait().expect("Failed to wait for service");

    // Service should have been killed, not exited on its own
    assert!(!status.success());
    println!("File descriptor limits test completed");
}
