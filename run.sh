#!/bin/bash
set -e

# First run the install.sh script
./install.sh

# Validate target user exists and has correct setup
if ! id "rust-service" &>/dev/null; then
    echo "Error: rust-service user does not exist"
    exit 1
fi

# Verify binary exists and is executable
if [[ ! -x "/opt/rust-service/rust-service" ]]; then
    echo "Error: Service binary not found or not executable"
    exit 1
fi

# Verify config file exists and has correct permissions
if [[ ! -f "/etc/rust-service/config.toml" ]]; then
    echo "Error: Config file not found"
    exit 1
fi

# Check config file permissions (should not be world-writable)
if [[ $(stat -c "%a" "/etc/rust-service/config.toml") -gt 644 ]]; then
    echo "Error: Config file has insecure permissions"
    exit 1
fi

# Run the program with validated user
sudo -u rust-service /opt/rust-service/rust-service
