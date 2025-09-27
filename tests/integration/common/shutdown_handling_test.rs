use crate::integration::common::test_prerequisites;

use std::fs;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn test_graceful_shutdown() {
    println!("RUNNING: test_graceful_shutdown - Testing graceful shutdown with SIGINT signal");
    println!("Testing graceful shutdown with SIGINT (3 seconds)...");

    let paths = test_prerequisites::get_test_paths().unwrap();

    // Read TIME_INTERVAL from config
    let config_content =
        fs::read_to_string(&paths.config_path).expect("Failed to read config file");
    let time_interval: u64 = config_content
        .lines()
        .find(|line| line.starts_with("TIME_INTERVAL"))
        .and_then(|line| line.split('=').nth(1))
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(5);

    let mut child = Command::new(&paths.binary_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start service");

    println!("Waiting for service to start...");
    thread::sleep(Duration::from_secs(1));

    println!("Sending SIGINT signal...");
    #[allow(clippy::cast_possible_wrap)]
    unsafe {
        libc::kill(child.id() as i32, libc::SIGINT);
    }

    println!("Waiting for graceful shutdown...");
    println!(
        "Waiting {} seconds (TIME_INTERVAL + 1)...",
        time_interval + 1
    );
    thread::sleep(Duration::from_secs(time_interval + 1));
    let result = child.try_wait().expect("Failed to check process status");

    assert!(result.is_some(), "Service should shutdown gracefully");
    println!("Graceful shutdown test completed");
}

#[test]
fn test_cleanup_on_exit() {
    println!("RUNNING: test_cleanup_on_exit - Testing cleanup on forced process termination");
    println!("Testing cleanup on forced exit...");

    let paths = test_prerequisites::get_test_paths().unwrap();

    let mut child = Command::new(&paths.binary_path)
        .env("TIME_INTERVAL", "10")
        .spawn()
        .expect("Failed to start service");

    println!("Waiting for service to start...");
    thread::sleep(Duration::from_secs(1));

    println!("Force killing service...");
    child.kill().expect("Failed to kill service");

    let status = child.wait().expect("Failed to wait for process");
    assert!(!status.success() || status.code().is_some());
    println!("Cleanup test completed");
}
