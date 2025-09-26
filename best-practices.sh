#!/bin/bash

echo "Running Rust Best Practices Check"
echo "================================="

# Exit on any error
set -e

# Get binary name from Cargo.toml
BINARY_NAME=$(grep -A 10 "^\[\[bin\]\]" Cargo.toml | grep "^name" | head -1 | sed 's/name = "\(.*\)"/\1/' | tr -d '"')
if [ -z "$BINARY_NAME" ]; then
    # Fallback to package name if no explicit binary name
    BINARY_NAME=$(grep "^name" Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/' | tr -d '"')
fi

echo "1. Code formatting check..."
cargo fmt --check

echo "2. Clippy linting (all levels)..."
cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery

echo "3. Documentation generation (without strict requirements)..."
cargo doc --no-deps --document-private-items

echo "4. Dead code detection..."
RUSTFLAGS="-W dead_code -W unused_imports -W unused_variables" cargo check

echo "5. Dependency tree analysis..."
cargo tree --duplicates

echo "6. Binary size analysis..."
cargo build --release
ls -lh target/release/$BINARY_NAME

echo "7. Cross-compilation check..."
if [[ $(uname -m) == "x86_64" ]]; then
    cargo check --target x86_64-unknown-linux-gnu
else
    echo "Skipping cross-compilation check (not on x86_64 architecture)"
fi

echo "All essential best practices checks completed successfully!"
