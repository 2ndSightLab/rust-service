use crate::service::config::validate_config_field;
use crate::service::error::ServiceError;
use serde::Deserialize;

#[must_use]
pub const fn get_config_file_name() -> &'static str {
    "config-action.toml"
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct ActionConfig {
    pub MESSAGE: String,
    pub MAX_MESSAGE_LEN: usize,
    pub TIME_INTERVAL: u64,
    pub MAX_TIME_INTERVAL: u64,
    pub DEFAULT_MESSAGE_LEN: usize,
}

impl Default for ActionConfig {
    fn default() -> Self {
        Self {
            MESSAGE: "Default message".to_string(),
            MAX_MESSAGE_LEN: 500,
            TIME_INTERVAL: 5,
            MAX_TIME_INTERVAL: 86400,
            DEFAULT_MESSAGE_LEN: 100,
        }
    }
}

/// Validates all action configuration fields.
///
/// # Errors
/// Returns `ServiceError` if any configuration field is invalid.
pub fn validate_all_action_config_fields(config: &ActionConfig) -> Result<(), ServiceError> {
    validate_config_field(
        &config.MESSAGE.len(),
        &1,
        &config.MAX_MESSAGE_LEN,
        "message",
    )?;
    validate_config_field(
        &config.TIME_INTERVAL,
        &1,
        &config.MAX_TIME_INTERVAL,
        "time_interval",
    )?;
    Ok(())
}
