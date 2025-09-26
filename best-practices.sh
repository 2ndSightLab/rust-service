#!/bin/bash

echo "Running Rust Best Practices Check"
echo "================================="

cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery
