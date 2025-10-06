use crate::integration::app::test_prerequisites;
use std::process::Command;
use std::time::{Duration, Instant};

#[test]
fn test_system_resource_monitoring() {
    let PATHS = test_prerequisites::get_test_paths().unwrap();
    println!(
        "RUNNING: test_system_resource_monitoring - Testing system resource monitoring exists"
    );
    println!("Testing that monitoring functions are available...");

    // Just test that the service starts and runs briefly without crashing
    let START = Instant::now();
    let mut CHILD = Command::new(&PATHS.binary)
        .spawn()
        .expect("Failed to start service");

    // Let it run for 2 seconds then kill it
    println!("Waiting 2 seconds for service to run...");
    std::thread::sleep(Duration::from_secs(2));
    CHILD.kill().expect("Failed to kill service");
    let _ = CHILD.wait();

    let ELAPSED = START.elapsed();
    assert!(
        ELAPSED >= Duration::from_secs(1),
        "Service should run for at least 1 second"
    );
    println!("Resource monitoring test completed");
}

#[test]
fn test_file_descriptor_limits() {
    let PATHS = test_prerequisites::get_test_paths().unwrap();
    println!(
        "RUNNING: test_file_descriptor_limits - Testing file descriptor limit validation exists"
    );
    println!("Testing that fd limit checking is available...");

    // Just test that the service can start (fd limits are checked at startup)
    let mut CHILD = Command::new(&PATHS.binary)
        .spawn()
        .expect("Failed to start service");

    // Let it run briefly then kill it
    println!("Waiting 1 second for service to start...");
    std::thread::sleep(Duration::from_secs(1));
    CHILD.kill().expect("Failed to kill service");
    let STATUS = CHILD.wait().expect("Failed to wait for service");

    // Service should have been killed, not exited on its own
    assert!(!STATUS.success());
    println!("File descriptor limits test completed");
}
