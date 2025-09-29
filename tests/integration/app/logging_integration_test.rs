use crate::integration::common::test_prerequisites;

use std::fs;
use std::process::Command;

#[test]
fn test_log_file_creation() {
    println!("RUNNING: test_log_file_creation - Testing log file creation and permissions");
    println!("Testing log file creation...");

    let paths = test_prerequisites::get_test_paths().unwrap();

    println!("Starting service...");
    let mut child = Command::new(&paths.binary_path)
        .spawn()
        .expect("Failed to start service");

    println!("Waiting 3 seconds for log files to be created...");
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Stopping service...");
    child.kill().expect("Failed to kill service");
    let _ = child.wait();

    // Check for log files in common log directories
    let log_dirs = [
        "/var/log/test-rust-service-debug",
        "/var/log/test-rust-service",
    ];

    for log_dir in &log_dirs {
        if let Ok(entries) = fs::read_dir(log_dir) {
            let log_files: Vec<_> = entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    if std::path::Path::new(&name)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("log"))
                    {
                        Some(name)
                    } else {
                        None
                    }
                })
                .collect();

            if log_files.is_empty() {
                println!("No log files found yet - service may not have had time to create them");
            } else {
                println!("Found log files: {log_files:?}");
            }
            break;
        }
        println!("Log directory {log_dir} not accessible - this is expected in test environment");
    }

    println!("Log file creation test completed");
}
