use crate::action::ActionConfig;
use crate::service::config_reader::load_action_config;
use crate::service::{Action, Config, ServiceError};
use log::info;

struct ExecAction {
    ACTION_CONFIG: ActionConfig,
}

impl ExecAction {
    /// Creates a new `ExecAction` instance.
    ///
    /// # Errors
    /// Returns `ServiceError` if action configuration cannot be loaded.
    fn new() -> Result<Self, ServiceError> {
        let ACTION_CONFIG = load_action_config()?;
        Ok(Self { ACTION_CONFIG })
    }
}

impl Action<Config> for ExecAction {
    fn execute(&self, _config: &Config) -> Result<(), ServiceError> {
        println!("{}", self.ACTION_CONFIG.MESSAGE);
        info!("{}", self.ACTION_CONFIG.MESSAGE);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "message"
    }
}

/// # Errors
/// Returns `ServiceError` if action configuration cannot be loaded.
pub fn new() -> Result<Box<dyn Action<Config>>, ServiceError> {
    Ok(Box::new(ExecAction::new()?))
}
