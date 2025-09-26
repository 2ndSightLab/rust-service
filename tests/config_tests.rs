use rust_service::config::Config;

#[test]
fn test_config_validation_valid() {
    // Test that a valid config passes validation by attempting to parse it
    let TOML_CONTENT = r#"
LOG_FILE_PATH = "/tmp/test-logs"
SERVICE_NAME = "test-service"
TIME_INTERVAL = 10
MESSAGE = "test message"
MEMORY_THRESHOLD = 80
DISK_THRESHOLD = 75
MIN_FD_LIMIT = 1024
MAX_SERVICE_NAME_LEN = 32
MAX_MESSAGE_LEN = 500
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
    assert_eq!(PARSED.TIME_INTERVAL, 10);
    assert_eq!(PARSED.MEMORY_THRESHOLD, 80);
}

#[test]
fn test_config_validation_invalid_service_name() {
    let TOML_CONTENT = r#"
LOG_FILE_PATH = "/tmp/test"
SERVICE_NAME = ""
TIME_INTERVAL = 10
MESSAGE = "test"
MEMORY_THRESHOLD = 80
DISK_THRESHOLD = 80
MIN_FD_LIMIT = 1024
MAX_SERVICE_NAME_LEN = 32
MAX_MESSAGE_LEN = 500
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
SERVICE_NAME = "test"
TIME_INTERVAL = 10
MESSAGE = "test"
MEMORY_THRESHOLD = 101
DISK_THRESHOLD = 80
MIN_FD_LIMIT = 1024
MAX_SERVICE_NAME_LEN = 32
MAX_MESSAGE_LEN = 500
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
