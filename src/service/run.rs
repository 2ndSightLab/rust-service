use log::info;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use crate::service::config_reader::load_config;
use crate::service::{ServiceError, monitoring};

pub use crate::service::config::Config;

/// Trait for service-specific configuration
pub trait ServiceConfig: Clone + Send + Sync + 'static {
    /// Loads configuration from system directories.
    ///
    /// # Errors
    /// Returns `ServiceError::Config` if configuration loading or validation fails.
    fn load() -> Result<Self, ServiceError>;
    fn service_name(&self) -> &str;
    fn log_file_path(&self) -> &str;
}

/// Action trait for extensible service functionality
pub trait Action<C: ServiceConfig>: Send + Sync {
    /// Executes the action with the given configuration.
    ///
    /// # Errors
    /// Returns `ServiceError` if the action fails to execute.
    fn execute(&self, config: &C) -> Result<(), ServiceError>;
    fn name(&self) -> &str;
}

/// Service runner that manages and executes actions
pub struct ServiceRunner<C: ServiceConfig> {
    actions: Vec<Box<dyn Action<C>>>,
}

impl<C: ServiceConfig> Default for ServiceRunner<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ServiceConfig> ServiceRunner<C> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_action(mut self, ACTION: Box<dyn Action<C>>) -> Self {
        self.actions.push(ACTION);
        self
    }

    /// Runs all registered actions.
    ///
    /// # Errors
    /// Returns an error if any action fails to execute.
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let CONFIG = C::load()?;

        // Check system resources before starting
        if let Ok(config) = crate::service::config_reader::load_config() {
            monitoring::check_resources(&config)?;
        }

        let RUNNING = Arc::new(AtomicBool::new(true));

        ctrlc::set_handler({
            let RUNNING = Arc::clone(&RUNNING);
            move || {
                RUNNING.store(false, Ordering::SeqCst);
            }
        })?;

        info!("Starting {}", CONFIG.service_name());

        // Start all actions - they handle their own execution patterns
        for action in &self.actions {
            if let Err(e) = action.execute(&CONFIG) {
                log::error!("Action failed: {e}");
                return Err(e.into());
            }
        }

        // Wait for shutdown signal
        while RUNNING.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(100));
        }

        info!("Service shutting down gracefully");
        Ok(())
    }
}

// Keep backward compatibility
impl ServiceConfig for Config {
    fn load() -> Result<Self, ServiceError> {
        load_config()
    }

    fn service_name(&self) -> &str {
        &self.SERVICE_NAME
    }

    fn log_file_path(&self) -> &str {
        &self.LOG_FILE_PATH
    }
}
