#!/bin/bash

echo "Select build type:"
echo "1) Debug"
echo "2) Release"
read -p "Enter choice (1 or 2): " choice

case $choice in
    1)
        echo "Building in debug mode..."
        cargo build
        ;;
    2)
        echo "Building in release mode..."
        cargo build --release
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
