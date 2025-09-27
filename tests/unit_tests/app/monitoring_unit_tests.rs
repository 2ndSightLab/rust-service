use rust_service::service::error::ServiceError;

fn parse_memory_value(LINE: &str, PREFIX: &str) -> Result<u64, ServiceError> {
    LINE.strip_prefix(PREFIX)
        .and_then(|s| s.split_whitespace().next())
        .map_or_else(
            || {
                Err(ServiceError::Config(format!(
                    "Invalid memory line format: {LINE}"
                )))
            },
            |VALUE_STR| {
                VALUE_STR.parse().map_err(|_| {
                    ServiceError::Config(format!("Failed to parse memory value: {VALUE_STR}"))
                })
            },
        )
}

#[test]
fn test_parse_memory_value_valid() {
    let LINE = "MemTotal:        8000000 kB";
    let RESULT = parse_memory_value(LINE, "MemTotal:");
    assert!(RESULT.is_ok());
    assert_eq!(RESULT.unwrap(), 8_000_000);
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
