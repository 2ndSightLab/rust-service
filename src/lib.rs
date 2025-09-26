use log::info;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub mod config;
pub mod error;
pub mod logging;
pub mod monitoring;
pub mod security;

use config::{load_config, Config};
use logging::FileLogger;
use monitoring::check_resources;
use security::{validate_runtime_security, validate_service_user};

pub use error::ServiceError;

pub trait Action: Send + Sync {
    fn execute(&self, config: &Config) -> Result<(), ServiceError>;
    fn name(&self) -> &str;
}

pub struct ServiceRunner {
    actions: Vec<Box<dyn Action>>,
}

impl ServiceRunner {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add_action(mut self, action: Box<dyn Action>) -> Self {
        self.actions.push(action);
        self
    }

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
