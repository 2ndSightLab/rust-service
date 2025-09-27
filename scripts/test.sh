#!/bin/bash

# Colors
GREEN='\033[0;32m'
RED='\033[1;31m'
NC='\033[0m' # No Color

echo "Running Rust Service Test Suite"
echo "==============================="

echo "Select build type:"
echo "1) Debug"
echo "2) Release"
read -p "Enter choice (1 or 2): " choice

case $choice in
    1)
        BUILD_TYPE="debug"
        DIR_SUFFIX="-debug"
        ;;
    2)
        BUILD_TYPE="release"
        DIR_SUFFIX=""
        ;;
    *)
        echo "Invalid choice. Please enter 1 or 2."
        exit 1
        ;;
esac

echo "Building $BUILD_TYPE binaries..."
echo "$choice" | ./scripts/build.sh

echo "Installing $BUILD_TYPE binaries..."
echo "$choice" | ./scripts/install.sh

echo "Running all tests..."

# Run unit tests first
echo "Running unit tests..."
cargo test --test unit_tests 2>&1 | while read -r line; do
    if [[ "$line" == *"... ok" ]]; then
        echo -e "${line% ok} ${GREEN}ok${NC}"
    elif [[ "$line" == *"... FAILED" ]]; then
        echo -e "${line% FAILED} ${RED}FAILED${NC}"
    else
        echo "$line"
    fi
done
UNIT_RESULT=${PIPESTATUS[0]}
UNIT_OUTPUT=$(cargo test --test unit_tests 2>&1)
UNIT_PASSED=$(echo "$UNIT_OUTPUT" | grep -o '[0-9]\+ passed' | head -1 | grep -o '[0-9]\+' || echo "0")
UNIT_FAILED=$(echo "$UNIT_OUTPUT" | grep -o '[0-9]\+ failed' | head -1 | grep -o '[0-9]\+' || echo "0")

# Run security tests second
echo "Running security tests..."
cargo test --test security_checks 2>&1 | while read -r line; do
    if [[ "$line" == *"... ok" ]]; then
        echo -e "${line% ok} ${GREEN}ok${NC}"
    elif [[ "$line" == *"... FAILED" ]]; then
        echo -e "${line% FAILED} ${RED}FAILED${NC}"
    else
        echo "$line"
    fi
done
SECURITY_RESULT=${PIPESTATUS[0]}
SECURITY_OUTPUT=$(cargo test --test security_checks 2>&1)
SECURITY_PASSED=$(echo "$SECURITY_OUTPUT" | grep -o '[0-9]\+ passed' | head -1 | grep -o '[0-9]\+' || echo "0")
SECURITY_FAILED=$(echo "$SECURITY_OUTPUT" | grep -o '[0-9]\+ failed' | head -1 | grep -o '[0-9]\+' || echo "0")

# Run integration tests last
echo "Running integration tests..."
timeout 60s cargo test --test integration -- --test-threads=1 2>&1 | while read -r line; do
    if [[ "$line" == *"... ok" ]]; then
        echo -e "${line% ok} ${GREEN}ok${NC}"
    elif [[ "$line" == *"... FAILED" ]]; then
        echo -e "${line% FAILED} ${RED}FAILED${NC}"
    else
        echo "$line"
    fi
done
INTEGRATION_RESULT=${PIPESTATUS[0]}
INTEGRATION_OUTPUT=$(timeout 60s cargo test --test integration -- --test-threads=1 2>&1)
INTEGRATION_PASSED=$(echo "$INTEGRATION_OUTPUT" | grep -o '[0-9]\+ passed' | head -1 | grep -o '[0-9]\+' || echo "0")
INTEGRATION_FAILED=$(echo "$INTEGRATION_OUTPUT" | grep -o '[0-9]\+ failed' | head -1 | grep -o '[0-9]\+' || echo "0")

# Calculate totals
TOTAL_PASSED=$((UNIT_PASSED + SECURITY_PASSED + INTEGRATION_PASSED))
TOTAL_FAILED=$((UNIT_FAILED + SECURITY_FAILED + INTEGRATION_FAILED))

echo ""
echo "Test Results Summary:"
echo "===================="

# Unit Tests
if [ $UNIT_RESULT -eq 0 ]; then
    echo -e "✅ Unit Tests: ${GREEN}PASSED${NC} ($UNIT_PASSED passed, $UNIT_FAILED failed)"
else
    echo -e "❌ Unit Tests: ${RED}FAILED${NC} ($UNIT_PASSED passed, $UNIT_FAILED failed)"
fi

# Security Checks
if [ $SECURITY_RESULT -eq 0 ]; then
    echo -e "✅ Security Checks: ${GREEN}PASSED${NC} ($SECURITY_PASSED passed, $SECURITY_FAILED failed)"
else
    echo -e "❌ Security Checks: ${RED}FAILED${NC} ($SECURITY_PASSED passed, $SECURITY_FAILED failed)"
fi

# Integration Tests
if [ $INTEGRATION_RESULT -eq 0 ]; then
    echo -e "✅ Integration Tests: ${GREEN}PASSED${NC} ($INTEGRATION_PASSED passed, $INTEGRATION_FAILED failed)"
else
    echo -e "❌ Integration Tests: ${RED}FAILED${NC} ($INTEGRATION_PASSED passed, $INTEGRATION_FAILED failed)"
fi

echo ""

# Overall result
if [ $TOTAL_FAILED -eq 0 ]; then
    echo -e "✅ All Tests: ${GREEN}PASSED${NC} ($TOTAL_PASSED passed, $TOTAL_FAILED failed)"
    exit 0
else
    echo -e "❌ All Tests: ${RED}FAILED${NC} ($TOTAL_PASSED passed, $TOTAL_FAILED failed)"
    exit 1
fi
