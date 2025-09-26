use log::info;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

pub mod config;
pub mod error;
pub mod logging;
pub mod monitoring;
pub mod security;

use config::{Config, load_config};
use logging::FileLogger;
use monitoring::check_resources;
use security::{validate_runtime_security, validate_service_user};

pub use error::ServiceError;

/// Action trait for extensible service functionality
pub trait Action: Send + Sync {
    /// Executes the action with the given configuration.
    ///
    /// # Errors
    /// Returns `ServiceError` if the action fails to execute.
    fn execute(&self, config: &Config) -> Result<(), ServiceError>;
    fn name(&self) -> &str;
}

/// Service runner that manages and executes actions
pub struct ServiceRunner {
    actions: Vec<Box<dyn Action>>,
}

impl Default for ServiceRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRunner {
    #[must_use]
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_action(mut self, action: Box<dyn Action>) -> Self {
        self.actions.push(action);
        self
    }

    /// Runs all registered actions.
    ///
    /// # Errors
    /// Returns an error if any action fails to execute.
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let CONFIG = load_config()?;

        validate_runtime_security(&CONFIG)?;
        validate_service_user(&CONFIG.SERVICE_NAME, CONFIG.MAX_SERVICE_NAME_LEN)?;

        let LOGGER = FileLogger::new(CONFIG.LOG_FILE_PATH.clone());
        FileLogger::set_config(CONFIG.clone());
        log::set_boxed_logger(Box::new(LOGGER))?;
        log::set_max_level(log::LevelFilter::Info);

        let RUNNING = Arc::new(AtomicBool::new(true));

        ctrlc::set_handler({
            let RUNNING = Arc::clone(&RUNNING);
            move || {
                RUNNING.store(false, Ordering::SeqCst);
            }
        })?;

        check_resources(&CONFIG)?;

        info!("{} starting...", CONFIG.SERVICE_NAME);
        info!("Log file path: {}", CONFIG.LOG_FILE_PATH);

        while RUNNING.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(CONFIG.TIME_INTERVAL));
            if RUNNING.load(Ordering::SeqCst) {
                for action in &self.actions {
                    if let Err(e) = action.execute(&CONFIG) {
                        log::error!("Action '{}' failed: {}", action.name(), e);
                    }
                }
            }
        }

        info!("Service shutting down gracefully");
        Ok(())
    }
}
