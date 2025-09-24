use std::fs;
use std::process;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Config {
    log_file_path: String,
    service_name: String,
    time_interval: u64,
    message: String,
}

fn load_config() -> Config {
    let config_content = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading config.toml: {}", e);
            process::exit(1);
        }
    };

    let mut log_file_path = None;
    let mut service_name = None;
    let mut time_interval = None;
    let mut message = None;

    for line in config_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            
            match key {
                "LOG_FILE_PATH" => log_file_path = Some(value.to_string()),
                "SERVICE_NAME" => service_name = Some(value.to_string()),
                "TIME_INTERVAL" => {
                    time_interval = value.parse().ok();
                }
                "MESSAGE" => message = Some(value.to_string()),
                _ => {}
            }
        }
    }

    let log_file_path = log_file_path.unwrap_or_else(|| {
        eprintln!("LOG_FILE_PATH missing or invalid");
        process::exit(1);
    });

    let service_name = service_name.unwrap_or_else(|| {
        eprintln!("SERVICE_NAME missing or invalid");
        process::exit(1);
    });

    let time_interval = time_interval.unwrap_or_else(|| {
        eprintln!("TIME_INTERVAL missing or invalid");
        process::exit(1);
    });

    let message = message.unwrap_or_else(|| {
        eprintln!("MESSAGE missing or invalid");
        process::exit(1);
    });

    if time_interval == 0 {
        eprintln!("TIME_INTERVAL must be greater than 0");
        process::exit(1);
    }

    Config {
        log_file_path,
        service_name,
        time_interval,
        message,
    }
}

fn log_message(config: &Config, message: &str) {
    let log_dir = Path::new(&config.log_file_path);
    if let Err(e) = fs::create_dir_all(log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return;
    }

    let log_file_path = log_dir.join("service.log");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let log_entry = format!("[{}] {}\n", timestamp, message);
    
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
        }
    }
}

fn check_system_resources(config: &Config) -> Result<(), String> {
    // Check memory usage
    let meminfo = fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("Failed to read memory info: {}", e))?;
    
    let mut total_mem = 0u64;
    let mut available_mem = 0u64;
    
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total_mem = line.split_whitespace().nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available_mem = line.split_whitespace().nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }
    }
    
    if total_mem > 0 {
        let used_mem = total_mem - available_mem;
        let mem_usage = (used_mem * 100) / total_mem;
        if mem_usage >= 80 {
            let error_msg = format!("Memory usage at {}% (>= 80%)", mem_usage);
            log_message(config, &error_msg);
            return Err(error_msg);
        }
    }

    // Check disk usage for current directory
    let statvfs_output = std::process::Command::new("df")
        .arg(".")
        .output()
        .map_err(|e| format!("Failed to check disk usage: {}", e))?;
    
    let output = String::from_utf8_lossy(&statvfs_output.stdout);
    if let Some(line) = output.lines().nth(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            if let Ok(usage) = parts[4].trim_end_matches('%').parse::<u32>() {
                if usage >= 80 {
                    let error_msg = format!("Disk usage at {}% (>= 80%)", usage);
                    log_message(config, &error_msg);
                    return Err(error_msg);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let config = load_config();
    
    if let Err(error) = check_system_resources(&config) {
        eprintln!("System resource error: {}", error);
        process::exit(1);
    }
    
    println!("{} starting...", config.service_name);
    log_message(&config, "Service started");
    
    loop {
        thread::sleep(Duration::from_secs(config.time_interval));
        println!("{}", config.message);
    }
}
