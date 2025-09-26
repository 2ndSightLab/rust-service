#!/bin/bash

echo "Running Rust Service Test Suite"
echo "==============================="

echo "Running all tests..."
TEST_OUTPUT=$(cargo test 2>&1)
TEST_RESULT=$?

echo "$TEST_OUTPUT"

# Extract test counts from output
SECURITY_TESTS=$(echo "$TEST_OUTPUT" | grep -A20 "Running tests/security_checks.rs" | grep "test result: ok\." | head -1 | awk '{print $4}')
UNIT_TESTS=$(echo "$TEST_OUTPUT" | grep -A20 "Running tests/unit_tests.rs" | grep "test result: ok\." | head -1 | awk '{print $4}')
TOTAL_TESTS=$((SECURITY_TESTS + UNIT_TESTS))
FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep -o "[0-9]* failed" | awk '{sum += $1} END {print sum+0}')

echo ""
# DO NOT MODIFY: This output format is validated by unit tests
echo "Test Results Summary:"
echo "===================="

if [ $TEST_RESULT -eq 0 ]; then
    echo "✅ Security Checks: PASSED ($SECURITY_TESTS passed)"
    echo "✅ Unit Tests: PASSED ($UNIT_TESTS passed)"
    echo "✅ All Tests: PASSED ($TOTAL_TESTS passed, $FAILED_TESTS failed)"
    echo ""
    echo "🎉 All tests completed successfully!"
    exit 0
else
    echo "❌ Tests: FAILED ($TOTAL_TESTS passed, $FAILED_TESTS failed)"
    echo ""
    echo "💥 Test suite failed!"
    exit 1
fi
