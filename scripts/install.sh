#!/bin/bash

set -e

# Set standard directory variables and source all functions
SCRIPTS_DIR="$(dirname "$(readlink -f "$0")")"
for func in "$SCRIPTS_DIR/functions"/*.sh; do source "$func"; done
PROJECT_DIR=$(get_project_dir)

# Validate running as appropriate user (not root for safety)
if [[ $EUID -eq 0 ]]; then
    echo "Warning: Running as root. This script will use sudo for privileged operations."
fi

# Check if this is a library project first
PROJECT_TYPE=$(get_project_type)
if [[ "$PROJECT_TYPE" == "lib" ]]; then
    echo "No installation required for library projects."
    echo "Libraries are used as dependencies and don't need to be installed."
    exit 0
fi

# Check for command line arguments or non-interactive mode
if [[ "$1" == "--debug" ]] || [[ -n "$CI" ]] || [[ ! -t 0 ]]; then
    choice=1
elif [[ "$1" == "--release" ]]; then
    choice=2
else
    echo "Select binary type to install:"
    echo "1) Debug (all binaries including tests, examples, and benchmarks)"
    echo "2) Release"
    read -p "Enter choice (1 or 2): " choice
fi

case $choice in
    1)
        BINARY_TYPE="debug"
        DEBUG_SUFFIX="-debug"
        ;;
    2)
        BINARY_TYPE="release"
        DEBUG_SUFFIX=""
        ;;
    *)
        echo "Invalid choice. Please enter 1 or 2."
        exit 1
        ;;
esac

# Get project name
PROJECT_NAME=$(get_project_name)

# Get current architecture
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -d' ' -f2)

# Get project type and check if installation is needed
PROJECT_TYPE=$(get_project_type)

# Get binary path for service projects
BINARY_PATH=$(get_build_artifact "$PROJECT_TYPE" "$choice" "$PROJECT_DIR" "$CURRENT_ARCH" "$PROJECT_NAME")

# Read directories from local config file
LOCAL_CONFIG=$(get_local_config_file)
INSTALL_DIR=$(get_install_directory "$BINARY_TYPE")
LOG_DIR=$(read_config_value "LOG_FILE_PATH" "$LOCAL_CONFIG")
CONFIG_DIR="$INSTALL_DIR"

# Set service user based on debug mode
if [[ "$choice" == "1" ]]; then
    SERVICE_USER="$PROJECT_NAME-debug"
else
    SERVICE_USER="$PROJECT_NAME"
fi

echo "Installing $PROJECT_NAME ($BINARY_TYPE)..."
echo "Install dir: $INSTALL_DIR"
echo "Config dir: $CONFIG_DIR"
echo "Log dir: $LOG_DIR"

# Validate binary exists before installation
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "Binary not found at $BINARY_PATH"
    read -p "Would you like to build the project now? (y/n): " build_choice
    if [[ "$build_choice" =~ ^[Yy]$ ]]; then
        echo "Building project..."
        "$SCRIPTS_DIR/build.sh" "--$BINARY_TYPE"
        if [[ ! -f "$BINARY_PATH" ]]; then
            echo "Error: Build failed or binary still not found."
            exit 1
        fi
    else
        echo "Installation cancelled. Run './scripts/build.sh' first."
        exit 1
    fi
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

# Copy binary
echo "Installing binary..."
sudo cp "$BINARY_PATH" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/$PROJECT_NAME"

echo "Installing configuration..."
# Get the local config file path based on project type
LOCAL_CONFIG=$(get_local_config_file)

if [[ -f "$LOCAL_CONFIG" ]]; then
    sudo cp "$LOCAL_CONFIG" "$CONFIG_DIR/"
else
    echo "Error: Local config file not found at $LOCAL_CONFIG"
    exit 1
fi

# Also install action config from local directory
LOCAL_ACTION_CONFIG="$PROJECT_DIR/config/action.toml"
if [[ -f "$LOCAL_ACTION_CONFIG" ]]; then
    sudo cp "$LOCAL_ACTION_CONFIG" "$CONFIG_DIR/"
fi

# Set ownership
echo "Setting permissions..."
sudo chown -R "$SERVICE_USER:$SERVICE_USER" "$INSTALL_DIR"
sudo chown -R "$SERVICE_USER:$SERVICE_USER" "$LOG_DIR"
sudo chown root:root "$CONFIG_DIR/service.toml"
sudo chmod 644 "$CONFIG_DIR/service.toml"

# Set permissions for action config if it exists
if [[ -f "$CONFIG_DIR/action.toml" ]]; then
    sudo chown root:root "$CONFIG_DIR/action.toml"
    sudo chmod 644 "$CONFIG_DIR/action.toml"
fi
sudo chown root:root "$CONFIG_DIR/service.toml"
sudo chmod 644 "$CONFIG_DIR/service.toml"

# Set permissions for action config if it exists
if [[ -f "$CONFIG_DIR/action.toml" ]]; then
    sudo chown root:root "$CONFIG_DIR/action.toml"
    sudo chmod 644 "$CONFIG_DIR/action.toml"
fi

echo "Installation complete!"
echo "Binary: $INSTALL_DIR/$PROJECT_NAME"
echo "Config: $CONFIG_DIR/service.toml"
echo "Logs: $LOG_DIR/"
