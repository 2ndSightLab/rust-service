use crate::service::config::Config;
use crate::service::error::ServiceError;
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
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
                // SECURITY: Overflow protection - try_from with safe fallback prevents panics
                u64::try_from(DURATION.as_millis()).unwrap_or(0)
            }
            Err(_) => return,
        };

        // Rate limiting using configurable interval
        let MIN_INTERVAL = get_config_value(|C| C.MIN_LOG_INTERVAL_MS, 100);

        let LAST_TIME = LAST_LOG_TIME.load(Ordering::Relaxed);
        // SECURITY: saturating_sub prevents integer underflow/overflow
        if NOW.saturating_sub(LAST_TIME) < MIN_INTERVAL {
            return;
        }
        LAST_LOG_TIME.store(NOW, Ordering::Relaxed);

        // Get configurable message length limit from action config
        let ACTION_CONFIG = crate::service::config::load_action_config().unwrap_or_default();
        let MAX_MSG_LEN = ACTION_CONFIG.MAX_MESSAGE_LEN;

        // Sanitize message with configurable length using whitelist approach
        let ESCAPED_MSG = record
            .args()
            .to_string()
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

    // Try to open the directory first to check existence via file descriptor
    let DIR_RESULT = fs::File::open(LOG_DIR);

    let LOG_FILE = if let Ok(dir_fd) = DIR_RESULT {
        // Directory exists, get canonical path via file descriptor
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let FD = dir_fd.as_raw_fd();

            // Use readlink on /proc/self/fd/{fd} to get canonical path
            let PROC_PATH = format!("/proc/self/fd/{FD}");
            let CANONICAL_PATH = fs::read_link(&PROC_PATH)
                .map_err(|_| ServiceError::Config("Cannot resolve directory path".to_string()))?;

            let ALLOWED_PREFIXES = ["/var/log", "/opt"];
            if !ALLOWED_PREFIXES
                .iter()
                .any(|prefix| CANONICAL_PATH.starts_with(prefix))
            {
                return Err(ServiceError::Config(
                    "Log directory not in allowed location".to_string(),
                ));
            }

            CANONICAL_PATH.join("service.log")
        }
        #[cfg(not(unix))]
        {
            return Err(ServiceError::Config("Platform not supported".to_string()));
        }
    } else {
        // Directory doesn't exist, create it and then validate
        fs::create_dir_all(LOG_DIR)
            .map_err(|_| ServiceError::Config("Cannot create log directory".to_string()))?;

        // Use canonicalize for path validation after creation
        let CANONICAL_DIR = LOG_DIR
            .canonicalize()
            .map_err(|_| ServiceError::Config("Invalid log directory path".to_string()))?;

        let ALLOWED_PREFIXES = ["/var/log", "/opt"];
        if !ALLOWED_PREFIXES
            .iter()
            .any(|prefix| CANONICAL_DIR.starts_with(prefix))
        {
            return Err(ServiceError::Config(
                "Log directory not in allowed location".to_string(),
            ));
        }

        CANONICAL_DIR.join("service.log")
    };

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
        use std::os::unix::fs::MetadataExt;
        use std::os::unix::io::AsRawFd;

        // RAII guard ensures file lock is automatically released on scope exit
        struct FileLockGuard(i32);
        impl Drop for FileLockGuard {
            fn drop(&mut self) {
                // SAFETY: flock() with LOCK_UN is safe on a valid file descriptor
                // Ignore errors on unlock as we're in destructor
                let _ = unsafe { libc::flock(self.0, libc::LOCK_UN) };
            }
        }

        let FD = FILE.as_raw_fd();

        // SAFETY: flock() is safe when called on a valid file descriptor with valid flags
        let RESULT = unsafe { libc::flock(FD, libc::LOCK_EX) };
        if RESULT != 0 {
            // Get errno for better error reporting
            let ERRNO = unsafe { *libc::__errno_location() };
            return Err(ServiceError::Config(format!(
                "Cannot acquire file lock: errno {ERRNO}"
            )));
        }
        let _LOCK_GUARD = FileLockGuard(FD);

        // SECURITY: FILE.metadata() calls fstat() on the file descriptor, not the path
        let METADATA = map_io_error(FILE.metadata(), "Cannot get file metadata")?;

        let CURRENT_UID = crate::security::get_current_uid()
            .map_err(|e| ServiceError::Config(format!("Cannot get current UID: {e}")))?;
        if !METADATA.file_type().is_file() || METADATA.uid() != CURRENT_UID {
            return Err(ServiceError::Config(
                "Log file security check failed".to_string(),
            ));
        }
    }

    let MAX_SIZE = get_config_value(|C| C.MAX_LOG_FILE_SIZE, 10_485_760);

    let CURRENT_POS = map_io_error(FILE.seek(SeekFrom::End(0)), "Cannot seek file")?;

    if CURRENT_POS > MAX_SIZE {
        return Err(ServiceError::Config("Log file too large".to_string()));
    }

    map_io_error(writeln!(FILE, "{MESSAGE}"), "Cannot write to log file")?;

    Ok(())
}
