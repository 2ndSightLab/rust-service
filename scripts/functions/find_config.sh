#!/bin/bash

# Function to find the config file location
# Matches the exact logic from src/service/config.rs ALLOWED_CONFIGS
find_config_file() {
    local config_name="$1"
    
    # Default to config-service.toml if no name provided
    if [[ -z "$config_name" ]]; then
        config_name="config-service.toml"
    fi
    
    # SECURITY: These hardcoded paths follow Unix FHS (Filesystem Hierarchy Standard)
    # and are protected by filesystem permissions. This matches the application logic.
    # Check for debug versions first, then release versions
    local allowed_configs=(
        "/etc/rust-service-debug/${config_name}"
        "/opt/rust-service-debug/${config_name}"
        "/usr/local/etc/rust-service-debug/${config_name}"
        "/etc/rust-service/${config_name}"
        "/opt/rust-service/${config_name}"
        "/usr/local/etc/rust-service/${config_name}"
    )
    
    # Check each path in order
    for path in "${allowed_configs[@]}"; do
        if [[ -f "$path" ]]; then
            echo "$path"
            return 0
        fi
    done
    
    # Config file not found
    echo "Error: Config file $config_name not found in system directories" >&2
    return 1
}

# Export function for use in other scripts
export -f find_config_file
