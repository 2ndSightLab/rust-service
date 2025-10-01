#!/bin/bash
set -e

# Set standard directory variables and source all functions
SCRIPTS_DIR="$(dirname "$(readlink -f "$0")")"
for func in "$SCRIPTS_DIR/functions"/*.sh; do source "$func"; done
PROJECT_DIR=$(get_project_dir)
SCRIPT_DIR_RELATIVE=$(dirname "$0")

echo "Select binary type to run:"
echo "1) Debug"
echo "2) Release"
read -p "Enter choice (1 or 2): " choice

case $choice in
    1)
        DEBUG_SUFFIX="-debug"
        BUILD_TYPE="debug"
        ;;
    2)
        DEBUG_SUFFIX=""
        BUILD_TYPE="release"
        ;;
    *)
        echo "Invalid choice. Please enter 1 or 2."
        exit 1
        ;;
esac

# Find config file
PROJECT_NAME=$(get_project_name)
CONFIG_FILE=$(get_config_file "$PROJECT_NAME" "$BUILD_TYPE")

# Get project name
PROJECT_NAME=$(get_project_name)

# Get current architecture
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -d' ' -f2)

# Check if binary exists, if not build it
BINARY_PATH="target/$CURRENT_ARCH/$BUILD_TYPE/$PROJECT_NAME"
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "Binary not found at $BINARY_PATH, building..."
    echo "$choice" | "$SCRIPTS_DIR"/build.sh
fi

# Set service user based on debug mode
if [[ "$choice" == "1" ]]; then
    SERVICE_USER="$PROJECT_NAME-debug"
else
    SERVICE_USER="$PROJECT_NAME"
fi

# First run the install.sh script
echo "$choice" | "$SCRIPTS_DIR_RELATIVE"/install.sh

# Read directories from config file
INSTALL_DIR=$(get_install_directory "$BUILD_TYPE")
CONFIG_DIR="$INSTALL_DIR"

# Validate target user exists and has correct setup
if ! id "$SERVICE_USER" &>/dev/null; then
    echo "Error: $SERVICE_USER user does not exist"
    exit 1
fi

# Verify binary exists and is executable
if [[ ! -x "$INSTALL_DIR/$PROJECT_NAME" ]]; then
    echo "Error: Service binary not found or not executable at $INSTALL_DIR/$PROJECT_NAME"
    exit 1
fi

# Verify config file exists and has correct permissions
if [[ ! -f "$CONFIG_DIR/service.toml" ]]; then
    echo "Error: Config file not found at $CONFIG_DIR/service.toml"
    exit 1
fi

# Check config file permissions (should not be world-writable)
if [[ $(stat -c "%a" "$CONFIG_DIR/service.toml") -gt 644 ]]; then
    echo "Error: Config file has insecure permissions"
    exit 1
fi

# Run the program with validated user
echo "Starting service..."
sudo -u "$SERVICE_USER" "$INSTALL_DIR/$PROJECT_NAME"
