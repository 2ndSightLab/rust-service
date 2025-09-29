use chrono::Utc;
use log::info;
use rust_service::service::{Action, ServiceError};
use rust_service::Config;

struct TimeAction;

impl Action<Config> for TimeAction {
    fn execute(&self, _config: &Config) -> Result<(), ServiceError> {
        let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        info!("Current time: {current_time}");
        Ok(())
    }

    fn name(&self) -> &str {
        "time"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_action_name() {
        let ACTION = TimeAction;
        assert_eq!(ACTION.name(), "time");
    }

    #[test]
    fn test_time_action_execute_returns_ok() {
        let ACTION = TimeAction;
        let CONFIG = Config {
            LOG_FILE_PATH: "/tmp/test.log".to_string(),
            INSTALL_DIR: "/opt/test".to_string(),
            CONFIG_DIR: "/etc/test".to_string(),
            SERVICE_NAME: "test-service".to_string(),
            MEMORY_THRESHOLD: 80,
            DISK_THRESHOLD: 80,
            MIN_FD_LIMIT: 1024,
            MAX_SERVICE_NAME_LEN: 50,
            MAX_LOG_PATH_LEN: 255,
            MIN_LOG_INTERVAL_MS: 100,
            MAX_LOG_FILE_SIZE: 10_485_760,
            MAX_TIME_INTERVAL: 3600,
            MAX_THRESHOLD_PERCENT: 95,
            MAX_FD_LIMIT: 65536,
            MAX_CONFIG_FIELD_LEN: 1000,
        };

        let RESULT = ACTION.execute(&CONFIG);
        assert!(RESULT.is_ok());
    }

    #[test]
    fn test_time_format_validation() {
        let NOW = Utc::now();
        let FORMATTED = NOW.format("%Y-%m-%d %H:%M:%S UTC").to_string();

        // Check format matches expected pattern
        assert!(FORMATTED.contains("UTC"));
        assert!(FORMATTED.len() >= 20); // YYYY-MM-DD HH:MM:SS UTC

        // Check it contains valid date components
        let PARTS: Vec<&str> = FORMATTED.split(' ').collect();
        assert_eq!(PARTS.len(), 3);
        assert_eq!(PARTS[2], "UTC");
    }
}
