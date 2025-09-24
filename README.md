# rust-service

__Summary__

A rust service that runs and prints out hello periodically.

__Warning!__

I used this code to learn the basics of rust and write a service with logging, configuration, and unit tests in one day. This is not production ready code. Q did not follow all my directives below. 
There are some missing best practices and security vulnerabilities in this code. You can read the related blog post to learn how I learned and how it all worked out in the end. This is a working program. 
Just not production ready.

Blog:

__Variables__

* LOG_FILE_PATH: The path to a folder where log files are written.
* SERVICE_NAME: The name of the service.
* TIME_INTERVAL: The intervals as which the services writes a message to the screen.
* MESSAGE: The message that gets written to the screen at a particular interval.

__Global Context__

* Always follow all the instructions in this README.md when making code changes.
* Always use the latest stable version or "edition" for rust, any third party libraries, code, or tools.
* Keep the code as simple as possible while meeting all the requirements.
* Follow all rust best practices.
* Make sure no security vulnerabilities exist in the code.
* Avoid third-party libraries if not needed.
* Do not use third-party libraries, components, or code without explicitly asking and getting permission first: Explain why you need the third party library and how it will be used. Explain who created the library and the last time it was updated.
* Check for security vulnerabilities in any code, whether included from an external source, in Rust or in this code and report it to the screen for evaluation.
* Use the cargo add bash command to add crates.

__Functionality__

* Set the LOG_FILE_PATH, SERVICE_NAME, TIME_INTERVAL and MESSAGE from a configuration file.
* Validate that the configuration file values are of correct type and length.
* If any malicious or malformed values are provided write an explanatory message to screen and logs and exit.
* Run a rust application as a service that can be started from the command line so can watch the output in the terminal window.
* On startup print a banner that says SERVICE_NAME starting...
* At the specified time interval in TIME_INTERVAL, print the MESSAGE
* Handle all errors correctly so that unexpected errors do not terminate the program.
* Check system memory and disk space, write an error message to the log file, and terminate if either one is at 80% capacity.

__Components__

* toml crate: Required to parse the config.toml file as specified in step 3 of the implementation. Used to deserialize the TOML configuration file into Rust structs for validation. Created by the toml-rs organization on GitHub. The toml crate is actively maintained with regular updates.

__Implementation Steps__

1. Create `Cargo.toml` with basic project configuration
2. Create configuration file structure (`config.toml`) with required variables
3. Create main source file (`src/main.rs`) with basic structure
4. Implement configuration parser with validation
5. Add system resource monitoring (memory/disk usage)
6. Create logging functionality to write to specified log file path
7. Implement main service loop with timer and message printing
8. Add error handling and graceful shutdown
9. Test configuration validation and resource monitoring

