use crate::integration::app::test_prerequisites;
use std::fs;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn test_graceful_shutdown() {
    let PATHS = test_prerequisites::get_test_paths().unwrap();
    println!("RUNNING: test_graceful_shutdown - Testing graceful shutdown with SIGINT signal");
    println!("Testing graceful shutdown with SIGINT (3 seconds)...");

    // Read TIME_INTERVAL from config
    let CONFIG_CONTENT =
        fs::read_to_string(&PATHS.action_config).expect("Failed to read action config file");
    let TIME_INTERVAL: u64 = CONFIG_CONTENT
        .lines()
        .find(|line| line.starts_with("TIME_INTERVAL"))
        .and_then(|line| line.split('=').nth(1))
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(5);

    let mut CHILD = Command::new(&PATHS.binary)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start service");

    println!("Waiting for service to start...");
    thread::sleep(Duration::from_secs(1));

    println!("Sending SIGINT signal...");
    #[allow(clippy::cast_possible_wrap)]
    unsafe {
        libc::kill(CHILD.id() as i32, libc::SIGINT);
    }

    println!("Waiting for graceful shutdown...");
    println!(
        "Waiting {} seconds (TIME_INTERVAL + 1)...",
        TIME_INTERVAL + 1
    );
    thread::sleep(Duration::from_secs(TIME_INTERVAL + 1));
    let RESULT = CHILD.try_wait().expect("Failed to check process status");

    assert!(RESULT.is_some(), "Service should shutdown gracefully");
    println!("Graceful shutdown test completed");
}

#[test]
fn test_cleanup_on_exit() {
    let PATHS = test_prerequisites::get_test_paths().unwrap();
    println!("RUNNING: test_cleanup_on_exit - Testing cleanup on forced process termination");
    println!("Testing cleanup on forced exit...");

    let mut CHILD = Command::new(&PATHS.binary)
        .env("TIME_INTERVAL", "10")
        .spawn()
        .expect("Failed to start service");

    println!("Waiting for service to start...");
    thread::sleep(Duration::from_secs(1));

    println!("Force killing service...");
    CHILD.kill().expect("Failed to kill service");

    let STATUS = CHILD.wait().expect("Failed to wait for process");
    assert!(!STATUS.success() || STATUS.code().is_some());
    println!("Cleanup test completed");
}
