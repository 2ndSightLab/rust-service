# rust-service

__Summary__

A Rust service that runs and prints messages periodically with logging, configuration, security validation, and system monitoring.

__Status__

This code follows Rust best practices and has been refactored for security and maintainability. All tests pass and the code compiles without warnings.

__Blog Post:__

https://medium.com/cloud-security/how-i-learned-rust-in-one-day-with-amazon-q-1398b75270c5

__Configuration Variables__

* LOG_FILE_PATH: The directory path where log files are written
* SERVICE_NAME: The name of the service (used for display and validation)
* TIME_INTERVAL: The interval in seconds between message outputs
* MESSAGE: The message that gets written to screen and logs
* MEMORY_THRESHOLD: Memory usage threshold percentage (default: 80)
* DISK_THRESHOLD: Disk usage threshold percentage (default: 80)
* MIN_FD_LIMIT: Minimum file descriptor limit required
* MAX_SERVICE_NAME_LEN: Maximum allowed service name length
* MAX_MESSAGE_LEN: Maximum allowed message length
* MAX_LOG_PATH_LEN: Maximum allowed log path length
* MIN_LOG_INTERVAL_MS: Minimum logging interval in milliseconds
* MAX_LOG_FILE_SIZE: Maximum log file size in bytes
* MAX_TIME_INTERVAL: Maximum time interval in seconds
* MAX_THRESHOLD_PERCENT: Maximum threshold percentage

__Architecture__

The code is organized into modular components:

* `src/main.rs` - Application entry point and orchestration
* `src/config.rs` - Configuration parsing and validation
* `src/error.rs` - Custom error types using thiserror
* `src/logging.rs` - File logging with security checks
* `src/monitoring.rs` - System resource monitoring
* `src/security/` - Security validation modules:
  * `uid.rs` - User ID operations
  * `limits.rs` - System limits checking
  * `validation.rs` - Security validation functions

__Dependencies__

* `log` - Standard logging framework
* `serde` - Serialization/deserialization
* `toml` - TOML configuration parsing
* `ctrlc` - Graceful shutdown handling
* `libc` - System calls for security checks
* `thiserror` - Error handling

__Functionality__

* Loads configuration from protected system directories
* Validates all configuration values for type, length, and security
* Performs security checks (prevents running as root, validates user)
* Monitors system resources (memory and disk usage)
* Implements secure file logging with proper locking
* Provides graceful shutdown on Ctrl+C
* Returns proper error codes instead of using process::exit()

__Security Features__

* Input validation prevents command injection
* File operations use secure permissions and locking
* Prevents running as root user
* Validates service user identity
* Checks system resource limits
* Uses protected configuration file locations

__Building and Testing__

```bash
# Build (interactive script)
./build.sh

# Run tests
./test.sh

# Install the program
./install.sh

# Run service
./run.sh
```

__Configuration File Locations__

The service looks for configuration files in these locations (in order):
1. `/etc/rust-service/config.toml`
2. `/opt/rust-service/config.toml`
3. `/usr/local/etc/rust-service/config.toml`

