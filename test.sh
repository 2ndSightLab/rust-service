#!/bin/bash

echo "Running Rust Service Test Suite"
echo "==============================="

# Run unit tests
echo "Running unit tests..."
cargo test unit_tests
UNIT_RESULT=$?

echo ""
echo "Running security checks..."
cargo test security_checks
SECURITY_RESULT=$?

echo ""
echo "Running all tests (complete suite)..."
cargo test
ALL_RESULT=$?

echo ""
echo "Test Results Summary:"
echo "===================="

if [ $UNIT_RESULT -eq 0 ]; then
    echo "✅ Unit Tests: PASSED"
else
    echo "❌ Unit Tests: FAILED"
fi

if [ $SECURITY_RESULT -eq 0 ]; then
    echo "✅ Security Checks: PASSED"
else
    echo "❌ Security Checks: FAILED"
fi

if [ $ALL_RESULT -eq 0 ]; then
    echo "✅ Complete Test Suite: PASSED"
    echo ""
    echo "🎉 All tests completed successfully!"
    exit 0
else
    echo "❌ Complete Test Suite: FAILED"
    echo ""
    echo "💥 Test suite failed!"
    exit 1
fi
