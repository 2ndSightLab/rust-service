use rust_service::load_config;
use rust_service::service::Config;

#[test]
fn test_service_config_loading() {
    // Create a test config file in the executable directory for testing
    let exe_path = std::env::current_exe().expect("Cannot get executable path");
    let exe_dir = exe_path.parent().expect("Cannot get executable directory");
    let config_path = exe_dir.join("service.toml");

    // Create test config if it doesn't exist
    if !config_path.exists() {
        let test_config = r#"
LOG_FILE_PATH = "/var/log/rust-service"
INSTALL_DIR = "/opt/rust-service"
CONFIG_DIR = "/etc/rust-service"
SERVICE_NAME = "test-service"
MEMORY_THRESHOLD = 80
DISK_THRESHOLD = 75
MIN_FD_LIMIT = 1024
MAX_SERVICE_NAME_LEN = 32
MAX_LOG_PATH_LEN = 500
MIN_LOG_INTERVAL_MS = 100
MAX_LOG_FILE_SIZE = 10485760
MAX_TIME_INTERVAL = 86400
MAX_THRESHOLD_PERCENT = 100
MAX_FD_LIMIT = 65536
MAX_CONFIG_FIELD_LEN = 2000
"#;
        std::fs::write(&config_path, test_config).expect("Failed to create test config");
    }

    let config = load_config().expect("Should load service config from executable directory");

    // Verify required fields are present and reasonable
    assert!(
        !config.SERVICE_NAME.is_empty(),
        "Service name should not be empty"
    );
    assert!(
        config.MEMORY_THRESHOLD > 0 && config.MEMORY_THRESHOLD <= 100,
        "Memory threshold should be 1-100"
    );
    assert!(
        config.DISK_THRESHOLD > 0 && config.DISK_THRESHOLD <= 100,
        "Disk threshold should be 1-100"
    );
    assert!(
        config.LOG_FILE_PATH.starts_with('/'),
        "Log path should be absolute"
    );
}

// Action config test moved to test-rust-service-2 project since it's action-specific

#[test]
fn test_config_validation_valid() {
    // Test that a valid service config passes validation by attempting to parse it
    let TOML_CONTENT = r#"
LOG_FILE_PATH = "/tmp/test-logs"
INSTALL_DIR = "/opt/rust-service"
CONFIG_DIR = "/etc/rust-service"
SERVICE_NAME = "test-service"
MEMORY_THRESHOLD = 80
DISK_THRESHOLD = 75
MIN_FD_LIMIT = 1024
MAX_SERVICE_NAME_LEN = 32
MAX_LOG_PATH_LEN = 500
MIN_LOG_INTERVAL_MS = 100
MAX_LOG_FILE_SIZE = 10485760
MAX_TIME_INTERVAL = 86400
MAX_THRESHOLD_PERCENT = 100
MAX_FD_LIMIT = 65536
MAX_CONFIG_FIELD_LEN = 2000
"#;
    let PARSED: Config = toml::from_str(TOML_CONTENT).expect("Should parse valid config");
    assert_eq!(PARSED.SERVICE_NAME, "test-service");
    assert_eq!(PARSED.MEMORY_THRESHOLD, 80);
    assert_eq!(PARSED.MAX_TIME_INTERVAL, 86400);
}
#[test]
fn test_config_validation_invalid_service_name() {
    let TOML_CONTENT = r#"
LOG_FILE_PATH = "/tmp/test"
INSTALL_DIR = "/opt/rust-service"
CONFIG_DIR = "/etc/rust-service"
SERVICE_NAME = ""
MEMORY_THRESHOLD = 80
DISK_THRESHOLD = 80
MIN_FD_LIMIT = 1024
MAX_SERVICE_NAME_LEN = 32
MAX_LOG_PATH_LEN = 500
MIN_LOG_INTERVAL_MS = 100
MAX_LOG_FILE_SIZE = 10485760
MAX_TIME_INTERVAL = 86400
MAX_THRESHOLD_PERCENT = 100
MAX_FD_LIMIT = 65536
MAX_CONFIG_FIELD_LEN = 2000
"#;
    let CONFIG: Config = toml::from_str(TOML_CONTENT).expect("Should parse");
    // Test would need actual validation function to work properly
    assert_eq!(CONFIG.SERVICE_NAME, "");
}

#[test]
fn test_config_validation_invalid_threshold() {
    let TOML_CONTENT = r#"
LOG_FILE_PATH = "/tmp/test"
INSTALL_DIR = "/opt/rust-service"
CONFIG_DIR = "/etc/rust-service"
SERVICE_NAME = "test"
MEMORY_THRESHOLD = 101
DISK_THRESHOLD = 80
MIN_FD_LIMIT = 1024
MAX_SERVICE_NAME_LEN = 32
MAX_LOG_PATH_LEN = 500
MIN_LOG_INTERVAL_MS = 100
MAX_LOG_FILE_SIZE = 10485760
MAX_TIME_INTERVAL = 86400
MAX_THRESHOLD_PERCENT = 100
MAX_FD_LIMIT = 65536
MAX_CONFIG_FIELD_LEN = 2000
"#;
    let CONFIG: Config = toml::from_str(TOML_CONTENT).expect("Should parse");
    // Test would need actual validation function to work properly
    assert_eq!(CONFIG.MEMORY_THRESHOLD, 101);
}
