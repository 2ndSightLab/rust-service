#!/bin/bash

# Get current architecture
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -d' ' -f2)

echo "Building for architecture: $CURRENT_ARCH"
echo "Select build type:"
echo "1) Debug (all binaries including tests, examples, and benchmarks)"
echo "2) Release"
read -p "Enter choice (1 or 2): " choice

case $choice in
    1)
        echo "Building in debug mode..."
        echo "Building everything (main binary, tests, examples, benchmarks)..."
        cargo build --all-targets --target $CURRENT_ARCH
        ;;
    2)
        echo "Building in release mode..."
        cargo build --release --target $CURRENT_ARCH
        ;;
    *)
        echo "Invalid choice. Please enter 1 or 2."
        exit 1
        ;;
esac

if [ $? -eq 0 ]; then
    echo "Build completed successfully!"
else
    echo "Build failed!"
    exit 1
fi
