use super::test_prerequisites;
use std::fs;

#[test]
fn test_service_binary_exists() {
    println!("RUNNING: test_service_binary_exists - Testing service binary installation");
    let PATHS = test_prerequisites::get_test_paths().unwrap();

    assert!(
        fs::metadata(&PATHS.binary).is_ok(),
        "Binary should exist at {}",
        PATHS.binary
    );

    println!("Service binary validation completed");
}

#[test]
fn test_service_config_exists() {
    println!("RUNNING: test_service_config_exists - Testing service configuration installation");
    let PATHS = test_prerequisites::get_test_paths().unwrap();

    assert!(
        fs::metadata(&PATHS.config).is_ok(),
        "Config should exist at {}",
        PATHS.config
    );

    println!("Service config validation completed");
}
