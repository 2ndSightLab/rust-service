use rust_service::error::ServiceError;

fn parse_memory_value(LINE: &str, PREFIX: &str) -> Result<u64, ServiceError> {
    if let Some(VALUE_STR) = LINE.strip_prefix(PREFIX).and_then(|s| s.split_whitespace().next()) {
        VALUE_STR.parse().map_err(|_| ServiceError::Config(format!("Failed to parse memory value: {VALUE_STR}")))
    } else {
        Err(ServiceError::Config(format!("Invalid memory line format: {LINE}")))
    }
}

#[test]
fn test_parse_memory_value_valid() {
    let LINE = "MemTotal:        8000000 kB";
    let RESULT = parse_memory_value(LINE, "MemTotal:");
    assert!(RESULT.is_ok());
    assert_eq!(RESULT.unwrap(), 8000000);
}

#[test]
fn test_parse_memory_value_invalid() {
    let LINE = "MemTotal:        invalid kB";
    let RESULT = parse_memory_value(LINE, "MemTotal:");
    assert!(RESULT.is_err());
}

#[test]
fn test_parse_memory_value_missing_prefix() {
    let LINE = "SomeOther:       8000000 kB";
    let RESULT = parse_memory_value(LINE, "MemTotal:");
    assert!(RESULT.is_err());
}
