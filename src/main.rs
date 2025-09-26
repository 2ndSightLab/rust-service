use log::info;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

mod config;
mod error;
mod logging;
mod monitoring;
mod security;

use config::load_config;
use logging::FileLogger;
use monitoring::check_resources;
use security::{validate_runtime_security, validate_service_user};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let CONFIG = load_config()?;

    // Validate runtime security with config
    validate_runtime_security(&CONFIG)?;

    // Validate service user after config is loaded
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
            info!("{}", CONFIG.MESSAGE);
        }
    }

    info!("Service shutting down gracefully");
    Ok(())
}


