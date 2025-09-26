#!/bin/bash

echo "Running Rust Service Test Suite"
echo "==============================="

# Run all tests
echo "Running tests..."
cargo test

# Check test result
if [ $? -eq 0 ]; then
    echo ""
    echo "✅ All tests PASSED"
    echo "Test suite completed successfully!"
    exit 0
else
    echo ""
    echo "❌ Some tests FAILED"
    echo "Test suite failed!"
    exit 1
fi
