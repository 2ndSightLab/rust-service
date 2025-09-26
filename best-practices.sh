#!/bin/bash

echo "Running Rust Best Practices Check"
echo "================================="

# Exit on any error
set -e

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
ls -lh target/release/rust-service

echo "7. Cross-compilation check..."
if [[ $(uname -m) == "x86_64" ]]; then
    cargo check --target x86_64-unknown-linux-gnu
else
    echo "Skipping cross-compilation check (not on x86_64 architecture)"
fi

echo "All essential best practices checks completed successfully!"
