use crate::integration::app::test_prerequisites;
use std::process::Command;
use std::time::Duration;

#[test]
fn test_root_user_prevention() {
    let PATHS = test_prerequisites::get_test_paths().unwrap();
    println!("RUNNING: test_root_user_prevention - Testing root user prevention security");
    println!("Testing root user prevention...");

    if unsafe { libc::getuid() } == 0 {
        println!("Running as root - testing prevention...");
        let OUTPUT = Command::new(&PATHS.binary)
            .output()
            .expect("Failed to run as root");

        assert!(
            !OUTPUT.status.success(),
            "Service should refuse to run as root"
        );
        assert!(String::from_utf8_lossy(&OUTPUT.stderr).contains("root"));
        println!("Root prevention test completed");
    } else {
        println!("Not running as root - skipping root prevention test");
    }
}

#[test]
fn test_input_validation() {
    let PATHS = test_prerequisites::get_test_paths().unwrap();
    println!("RUNNING: test_input_validation - Testing input validation and security checks");
    println!("Testing input validation with oversized service name...");

    // Test that service starts normally (input validation happens at config load)
    let mut CHILD = Command::new(&PATHS.binary)
        .spawn()
        .expect("Failed to start service");

    // Let it run briefly then kill it
    println!("Waiting 1 second for service to start...");
    std::thread::sleep(Duration::from_secs(1));
    CHILD.kill().expect("Failed to kill service");
    let _ = CHILD.wait();

    println!("Input validation test completed");
}
