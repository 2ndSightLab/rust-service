# rust-service

__Summary__

A Rust service that runs and prints messages periodically with logging, configuration, security validation, and system monitoring.

__Blog Posts:__

Written in one day having never used rust before:\
https://medium.com/cloud-security/how-i-learned-rust-in-one-day-with-amazon-q-1398b75270c5

Fixed on the second day to make it more production-ready by fixing security vulnerabilities and following rust best practices:\
https://medium.com/cloud-security/getting-amazon-q-to-help-write-production-ready-rust-code-1b3146338bad

Checking security vulnerabilities with Amazon Q:\
https://medium.com/cloud-security/using-ai-to-check-for-security-vulnerabilities-across-your-code-base-fcd48e246d04

Turning AI security findings into repeatable, deterministic security checks:\
https://medium.com/cloud-security/turn-your-security-findings-into-automated-checks-0a08efe57358

Preventing Q from making the same mistakes over and over again:\
https://medium.com/cloud-security/preventing-amazon-q-from-making-the-same-mistakes-over-and-over-4220c4c1a356

Turning the service into an extensible service library anyone can use that runs their own actions:\
https://medium.com/cloud-security/an-extensible-library-anyone-can-use-to-build-a-rust-service-f88eddf9d14f

__Sample Service:__

This repository shows how to use this code to build your own service. This sample service prints the current time periodically:\
https://github.com/2ndSightLab/test-rust-service

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
* MAX_FD_LIMIT: Maximum file descriptor limit
* MAX_CONFIG_FIELD_LEN: Maximum configuration field length

__Architecture__

The code is organized into modular components:

* `src/main.rs` - Application entry point and orchestration
* `src/service/service.rs` - Library interface and module exports
* `src/config/config.rs` - Configuration parsing and validation
* `src/service/error.rs` - Custom error types using thiserror
* `src/service/logging.rs` - File logging with security checks
* `src/service/monitoring.rs` - System resource monitoring
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
* `users` - User information queries
* `nix` - Unix system calls
* `regex` - Pattern matching (dev dependency)

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
# Build
./scripts/build.sh

# Run tests
./scripts/test.sh

# Check best practices
./scripts/best-practices.sh

# Install the program
./scripts/install.sh

# Run service
./scripts/run.sh
```

__Configuration File Locations__

The service looks for configuration files in these locations (in order):
1. `/etc/rust-service/config-service.toml` (service configuration)
2. `/etc/rust-service/config-action.toml` (action configuration)
3. `/opt/rust-service/config-service.toml` (service configuration)
4. `/opt/rust-service/config-action.toml` (action configuration)

