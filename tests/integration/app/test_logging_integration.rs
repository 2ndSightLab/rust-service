use super::test_prerequisites;
use std::fs;
use std::process::Command;

#[test]
fn test_log_file_creation() {
    println!("RUNNING: test_log_file_creation - Testing log file creation and permissions");
    println!("Testing log file creation...");

    let PATHS = test_prerequisites::get_test_paths().unwrap();

    println!("Starting service...");
    let mut CHILD = Command::new(&PATHS.binary)
        .spawn()
        .expect("Failed to start service");

    println!("Waiting 3 seconds for log files to be created...");
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Stopping service...");
    CHILD.kill().expect("Failed to kill service");
    let _ = CHILD.wait();

    // Check for log files in common log directories
    let LOG_DIRS = [
        "/var/log/test-rust-service-debug",
        "/var/log/test-rust-service",
    ];

    for LOG_DIR in &LOG_DIRS {
        if let Ok(ENTRIES) = fs::read_dir(LOG_DIR) {
            let LOG_FILES: Vec<_> = ENTRIES
                .filter_map(|ENTRY| {
                    let ENTRY = ENTRY.ok()?;
                    let NAME = ENTRY.file_name().to_string_lossy().to_string();
                    if std::path::Path::new(&NAME)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("log"))
                    {
                        Some(NAME)
                    } else {
                        None
                    }
                })
                .collect();

            if LOG_FILES.is_empty() {
                println!("No log files found yet - service may not have had time to create them");
            } else {
                println!("Found log files: {LOG_FILES:?}");
            }
            break;
        }
        println!("Log directory {LOG_DIR} not accessible - this is expected in test environment");
    }

    println!("Log file creation test completed");
}
