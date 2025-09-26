#!/bin/bash

set -e

# Validate running as appropriate user (not root for safety)
if [[ $EUID -eq 0 ]]; then
    echo "Warning: Running as root. This script will use sudo for privileged operations."
fi

SERVICE_NAME="rust-service"
SERVICE_USER="rust-service"
INSTALL_DIR="/opt/rust-service"
LOG_DIR="/var/log/rust-service"
CONFIG_DIR="/etc/rust-service"

echo "Installing $SERVICE_NAME..."

# Validate binary exists before installation
if [[ ! -f "target/release/rust-service" ]]; then
    echo "Error: Binary not found. Run 'cargo build --release' first."
    exit 1
fi

# Create service user with restricted shell
if ! id "$SERVICE_USER" &>/dev/null; then
    echo "Creating service user: $SERVICE_USER"
    sudo useradd --system --no-create-home --shell /bin/false --home-dir "$INSTALL_DIR" "$SERVICE_USER"
fi

# Create directories
echo "Creating directories..."
sudo mkdir -p "$INSTALL_DIR"
sudo mkdir -p "$LOG_DIR"
sudo mkdir -p "$CONFIG_DIR"

# Copy binary
echo "Installing binary..."
sudo cp target/release/rust-service "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/rust-service"

# Copy config
echo "Installing configuration..."
sudo cp config.toml "$CONFIG_DIR/"

# Set ownership
echo "Setting permissions..."
sudo chown -R "$SERVICE_USER:$SERVICE_USER" "$INSTALL_DIR"
sudo chown -R "$SERVICE_USER:$SERVICE_USER" "$LOG_DIR"
sudo chown root:root "$CONFIG_DIR/config.toml"
sudo chmod 644 "$CONFIG_DIR/config.toml"

echo "Installation complete!"
echo "Binary: $INSTALL_DIR/rust-service"
echo "Config: $CONFIG_DIR/config.toml"
echo "Logs: $LOG_DIR/"
echo ""
echo "To run: sudo -u $SERVICE_USER $INSTALL_DIR/rust-service"
