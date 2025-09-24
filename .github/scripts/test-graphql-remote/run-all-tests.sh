#!/bin/bash
# Master test runner for GraphQL remote tests
# Usage: ./run-all-tests.sh [GRAPHQL_ENDPOINT]
#
# This script runs all individual GraphQL test scripts against a remote endpoint.
# It's useful for testing compatibility with different node implementations.

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo -e "${BLUE}GraphQL Remote Test Suite${NC}"
echo -e "${BLUE}========================${NC}"
echo "Endpoint: $GRAPHQL_ENDPOINT"
echo "Script directory: $SCRIPT_DIR"
echo "Repository root: $REPO_ROOT"
echo ""

# Change to repository root for access to website scripts
cd "$REPO_ROOT"

# Function to run a test script
run_test() {
    local test_name="$1"
    local script_path="$2"

    ((TOTAL_TESTS++))
    echo -e "${YELLOW}[$TOTAL_TESTS]${NC} Running $test_name..."

    if [ ! -f "$script_path" ]; then
        echo -e "${RED}   ✗ Script not found: $script_path${NC}"
        ((FAILED_TESTS++))
        return 1
    fi

    # Make script executable
    chmod +x "$script_path"

    # Run the test with the endpoint
    if "$script_path" "$GRAPHQL_ENDPOINT"; then
        echo -e "${GREEN}   ✓ PASSED${NC}"
        ((PASSED_TESTS++))
        return 0
    else
        echo -e "${RED}   ✗ FAILED${NC}"
        ((FAILED_TESTS++))
        return 1
    fi
}

# List of GraphQL endpoints implemented in Rust (based on our analysis)
echo -e "${BLUE}Testing GraphQL endpoints that are implemented in Rust node:${NC}"
echo ""

# Core network and status queries
run_test "Check endpoint availability" "$SCRIPT_DIR/check-endpoint-availability.sh"
run_test "Sync status query" "$SCRIPT_DIR/test-sync-status.sh"
run_test "Network ID query" "$SCRIPT_DIR/test-network-info.sh"

# Daemon and blockchain info
run_test "Daemon status query" "$SCRIPT_DIR/test-daemon-status.sh"
run_test "Best chain query" "$SCRIPT_DIR/test-best-chain.sh"
run_test "Block query by height" "$SCRIPT_DIR/test-block-by-height.sh"
run_test "Genesis block query" "$SCRIPT_DIR/test-genesis-block.sh"
run_test "Genesis constants query" "$SCRIPT_DIR/test-genesis-constants.sh"

# Advanced queries
run_test "Complex nested query" "$SCRIPT_DIR/test-complex-nested-query.sh"
run_test "Error handling" "$SCRIPT_DIR/test-error-handling.sh"

echo ""
echo -e "${BLUE}===============================================${NC}"
echo -e "${BLUE}Test Results Summary${NC}"
echo -e "${BLUE}===============================================${NC}"
echo "Total tests: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    echo ""
    echo "The endpoint appears to be compatible with the expected GraphQL schema."
    echo "All implemented Rust node endpoints work correctly."
else
    echo -e "${RED}❌ Some tests failed${NC}"
    echo ""
    echo "Some GraphQL endpoints may not be working correctly or may have"
    echo "different implementations between the Rust and OCaml nodes."

    success_rate=$(( PASSED_TESTS * 100 / TOTAL_TESTS ))
    echo "Success rate: $success_rate%"

    if [ $success_rate -ge 80 ]; then
        echo -e "${YELLOW}Most endpoints are working correctly.${NC}"
    elif [ $success_rate -ge 50 ]; then
        echo -e "${YELLOW}Moderate compatibility - some issues detected.${NC}"
    else
        echo -e "${RED}Low compatibility - significant issues detected.${NC}"
    fi

    exit 1
fi