use crate::config::Config;
use crate::error::ServiceError;
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

static LAST_LOG_TIME: AtomicU64 = AtomicU64::new(0);
static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct FileLogger {
    LOG_FILE_PATH: String,
}

impl FileLogger {
    #[must_use]
    pub const fn new(LOG_FILE_PATH: String) -> Self {
        Self { LOG_FILE_PATH }
    }
    
    pub fn set_config(CONFIG_VALUE: Config) {
        let _ = CONFIG.set(CONFIG_VALUE);
    }
}

fn get_config_value<T, F>(GETTER: F, DEFAULT: T) -> T 
where 
    F: FnOnce(&Config) -> T,
{
    CONFIG.get().map_or(DEFAULT, GETTER)
}

fn map_io_error<T>(RESULT: std::io::Result<T>, CONTEXT: &str) -> Result<T, ServiceError> {
    RESULT.map_err(|E| ServiceError::Config(format!("{CONTEXT}: {E}")))
}

impl log::Log for FileLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let NOW = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(DURATION) => {
                u64::try_from(DURATION.as_millis()).unwrap_or(0)
            },
            Err(_) => return,
        };
        
        // Rate limiting using configurable interval
        let MIN_INTERVAL = get_config_value(|c| c.MIN_LOG_INTERVAL_MS, 100);
        
        let LAST_TIME = LAST_LOG_TIME.load(Ordering::Relaxed);
        if NOW.saturating_sub(LAST_TIME) < MIN_INTERVAL {
            return;
        }
        LAST_LOG_TIME.store(NOW, Ordering::Relaxed);
        
        // Get configurable message length limit
        let MAX_MSG_LEN = get_config_value(|c| c.MAX_MESSAGE_LEN, 500);
        
        // Sanitize message with configurable length using whitelist approach
        let ESCAPED_MSG = record.args().to_string()
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric() || c == ' ' || c == '.' || c == '-' || c == '_')
            .take(MAX_MSG_LEN)
            .collect::<String>();
        
        let MESSAGE = format!("[{}] [{}] {ESCAPED_MSG}", NOW / 1000, record.level());
        println!("{MESSAGE}");
        let _ = write_to_log_file(&self.LOG_FILE_PATH, &MESSAGE);
    }

    fn flush(&self) {}
}




fn write_to_log_file(LOG_FILE_PATH: &str, MESSAGE: &str) -> Result<(), ServiceError> {
    let LOG_DIR = Path::new(LOG_FILE_PATH);
    map_io_error(fs::create_dir_all(LOG_DIR), "Cannot create log directory")?;

    let LOG_FILE = LOG_DIR.join("service.log");
    
    #[cfg(unix)]
    let FILE = {
        use std::os::unix::fs::OpenOptionsExt;
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&LOG_FILE)
    };
    
    #[cfg(not(unix))]
    let FILE = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&LOG_FILE);
    
    let mut FILE = map_io_error(FILE, "Cannot open log file")?;

    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        use std::os::unix::fs::MetadataExt;
        
        let FD = FILE.as_raw_fd();
        // Blocking lock (no race condition)
        if unsafe { libc::flock(FD, libc::LOCK_EX) } != 0 {
            return Err(ServiceError::Config("Cannot acquire file lock".to_string()));
        }
        
        // Ensure unlock on any error after this point
        let RESULT = (|| {
            // SECURITY: FILE.metadata() calls fstat() on the file descriptor, not the path
            // This prevents TOCTOU attacks since we're checking the already-opened file
            let METADATA = map_io_error(FILE.metadata(), "Cannot get file metadata")?;
            
            let CURRENT_UID = crate::security::get_current_uid()
                .map_err(|e| ServiceError::Config(format!("Cannot get current UID: {e}")))?;
            if !METADATA.file_type().is_file() || METADATA.uid() != CURRENT_UID {
                return Err(ServiceError::Config("Log file security check failed".to_string()));
            }
            Ok(())
        })();
        
        if RESULT.is_err() {
            unsafe { libc::flock(FD, libc::LOCK_UN) };
            return RESULT;
        }
    }

    let MAX_SIZE = get_config_value(|c| c.MAX_LOG_FILE_SIZE, 10_485_760);

    let CURRENT_POS = map_io_error(FILE.seek(SeekFrom::End(0)), "Cannot seek file")?;
    
    if CURRENT_POS > MAX_SIZE {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            unsafe { libc::flock(FILE.as_raw_fd(), libc::LOCK_UN) };
        }
        return Err(ServiceError::Config("Log file too large".to_string()));
    }

    map_io_error(writeln!(FILE, "{MESSAGE}"), "Cannot write to log file")?;

    Ok(())
}
