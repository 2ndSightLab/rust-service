#!/bin/bash

# Set standard directory variables and source all functions
SCRIPTS_DIR="$(dirname "$(readlink -f "$0")")"
for func in "$SCRIPTS_DIR/functions"/*.sh; do source "$func"; done
PROJECT_DIR=$(get_project_dir)

echo "Running Rust Best Practices Check"
echo "================================="

# Ask user for build type
echo "Select build type:"
echo "1) Debug"
echo "2) Release"
read -p "Enter choice (1 or 2): " choice

case $choice in
    1)
        BUILD_TYPE="debug"
        BUILD_FLAG=""
        ;;
    2)
        BUILD_TYPE="release"
        BUILD_FLAG="--release"
        ;;
    *)
        echo "Invalid choice. Defaulting to Debug."
        BUILD_TYPE="debug"
        BUILD_FLAG=""
        ;;
esac

echo "Using $BUILD_TYPE build..."

# Read directory paths from config file
# Find config file
PROJECT_NAME=$(get_project_name)
CONFIG_FILE=$(get_config_file "$PROJECT_NAME" "$BUILD_TYPE")

# Check if we're using lib.toml and adjust accordingly
if [[ "$CONFIG_FILE" == *"lib.toml" ]]; then
    # Using lib config, get install directory from function
    INSTALL_DIR=$(get_install_directory "$BUILD_TYPE")
    CONFIG_DIR="/etc/rust-service"
    LOG_FILE_PATH="/var/log/rust-service"
    # Get binary name from lib config or Cargo.toml
    LIB_NAME=$(read_config_value "LIB_NAME" "$CONFIG_FILE")
    BINARY_NAME="$LIB_NAME"
else
    # Using service config, get install directory from function
    INSTALL_DIR=$(get_install_directory "$BUILD_TYPE")
    CONFIG_DIR=$(read_config_value "CONFIG_DIR" "$CONFIG_FILE")
    LOG_FILE_PATH=$(read_config_value "LOG_FILE_PATH" "$CONFIG_FILE")
fi

# Exit on any error
set -e

# Get binary name from config file (same as install.sh)
if [[ -z "$BINARY_NAME" ]]; then
# Get project name
PROJECT_NAME=$(get_project_name)
    # Set binary name based on project type
    if [[ -f "$PROJECT_DIR/config/service.toml" ]] || [[ "$CONFIG_FILE" == *"service.toml" ]]; then
        BINARY_NAME="$PROJECT_NAME"
    # Finally fall back to lib config
    else
        if [[ -f "$PROJECT_DIR/config/lib.toml" ]]; then
            LIB_NAME=$(read_config_value "LIB_NAME" "$PROJECT_DIR/config/lib.toml")
        fi
        BINARY_NAME="$LIB_NAME"
    fi
fi

echo "1. Code formatting check..."
cargo fmt --check

echo "2. Clippy linting (all levels) - DENY ALL WARNINGS except naming..."
cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::pedantic -W clippy::nursery -A non_snake_case -A clippy::upper_case_acronyms

echo "3. Documentation generation..."
RUSTDOCFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo doc --no-deps --document-private-items

echo "4. Dead code detection - DENY ALL WARNINGS except naming..."
RUSTFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo check

echo "5. Dependency tree analysis..."
cargo tree --duplicates

echo "6. Binary size analysis..."
# Check if we're being called from a service project (look for service config in calling directory)
CALLING_DIR=$(pwd)
if [[ -f "$CALLING_DIR/config/service.toml" ]]; then
    # We're being called from a service project, build and check its binary
    RUSTFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo build $BUILD_FLAG
    SERVICE_BINARY="$PROJECT_NAME"
    ls -lh target/$BUILD_TYPE/$SERVICE_BINARY
elif [[ -f "$PROJECT_DIR/config/lib.toml" ]] && [[ ! -f "$PROJECT_DIR/config/service.toml" ]] && [[ ! -f "$PROJECT_DIR/src/main.rs" ]]; then
    # We're in a library project that doesn't produce binaries
    echo "Skipping binary size analysis for library project"
else
    # Default behavior for other cases
    RUSTFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo build $BUILD_FLAG
    ls -lh target/$BUILD_TYPE/$BINARY_NAME
fi

echo "7. Architecture-specific check..."
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -d' ' -f2)
echo "Running checks for current architecture: $CURRENT_ARCH"
RUSTFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo check --target $CURRENT_ARCH

echo "8. License compliance check..."
cargo license || echo "Warning: cargo license not installed"

echo "9. Cargo.toml validation..."
cargo verify-project

echo "10. Test coverage analysis..."
cargo tarpaulin --out Stdout || echo "Warning: cargo tarpaulin not installed"

echo "11. Memory safety checks..."
echo -e "${RED}⚠️  WARNING: miri checks SKIPPED to avoid downloading components${NC}"
echo -e "${RED}   Memory safety checks will be SKIPPED${NC}"
echo -e "${RED}   Run manually with: cargo +nightly miri test${NC}"

echo "All essential best practices checks completed successfully!"
