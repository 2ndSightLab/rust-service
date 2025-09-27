use crate::integration::common::test_prerequisites;

use std::fs;

#[test]
fn test_service_binary_exists() {
    println!("RUNNING: test_service_binary_exists - Testing service binary installation");

    let paths = test_prerequisites::get_test_paths().unwrap();
    assert!(
        fs::metadata(&paths.binary_path).is_ok(),
        "Binary should exist at {}",
        paths.binary_path
    );

    println!("Service binary validation completed");
}

#[test]
fn test_service_config_exists() {
    println!("RUNNING: test_service_config_exists - Testing service configuration installation");

    let paths = test_prerequisites::get_test_paths().unwrap();
    assert!(
        fs::metadata(&paths.config_path).is_ok(),
        "Config should exist at {}",
        paths.config_path
    );

    println!("Service config validation completed");
}
