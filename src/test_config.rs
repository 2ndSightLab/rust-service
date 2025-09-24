use std::fs;

#[derive(Debug)]
struct Config {
    log_file_path: String,
    service_name: String,
    time_interval: u64,
    message: String,
}

fn load_config(file_path: &str) -> Result<Config, String> {
    let config_content = fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading {}: {}", file_path, e))?;

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

    let log_file_path = log_file_path.ok_or("LOG_FILE_PATH missing or invalid")?;
    let service_name = service_name.ok_or("SERVICE_NAME missing or invalid")?;
    let time_interval = time_interval.ok_or("TIME_INTERVAL missing or invalid")?;
    let message = message.ok_or("MESSAGE missing or invalid")?;

    if time_interval == 0 {
        return Err("TIME_INTERVAL must be greater than 0".to_string());
    }

    Ok(Config {
        log_file_path,
        service_name,
        time_interval,
        message,
    })
}

fn main() {
    let test_files = [
        ("config.toml", "Valid configuration"),
        ("test_configs/invalid_time.toml", "Invalid TIME_INTERVAL (0)"),
        ("test_configs/missing_field.toml", "Missing TIME_INTERVAL field"),
        ("test_configs/malformed.toml", "Malformed TOML syntax"),
    ];

    println!("=== Configuration Validation Test Results ===\n");

    for (file, description) in test_files {
        print!("Testing {}: ", description);
        match load_config(file) {
            Ok(config) => println!("✓ PASSED - Config loaded: {:?}", config),
            Err(e) => println!("✗ FAILED (expected) - {}", e),
        }
    }

    println!("\n=== Resource Monitoring Test ===");
    
    // Test memory check
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
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
            println!("Memory usage: {}% (threshold: 80%)", mem_usage);
            if mem_usage >= 80 {
                println!("✗ Memory usage exceeds 80% threshold");
            } else {
                println!("✓ Memory usage within acceptable limits");
            }
        }
    }

    println!("\n=== All Tests Complete ===");
}
