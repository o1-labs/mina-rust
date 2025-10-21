#!/bin/bash

# Test the wallet balance command JSON format output structure

# Check for required environment variables before enabling strict mode
if [ -z "${MINA_NODE_ENDPOINT:-}" ]; then
    echo "Error: MINA_NODE_ENDPOINT environment variable is not set"
    echo "Please set it to a GraphQL endpoint URL, e.g.:"
    echo "  export MINA_NODE_ENDPOINT=http://mina-rust-plain-1.gcp.o1test.net/graphql"
    exit 1
fi

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

# Check for jq
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed"
    echo "Install with: apt-get install jq (Ubuntu/Debian) or brew install jq (macOS)"
    exit 1
fi

echo "Test: Verify JSON format output structure"
JSON_OUTPUT=$(./target/release/mina wallet balance \
    --address "B62qkiqPXFDayJV8JutYvjerERZ35EKrdmdcXh3j1rDUHRs1bJkFFcX" \
    --endpoint "$MINA_NODE_ENDPOINT" \
    --format json 2>&1 || true)

echo "JSON format output:"
echo "$JSON_OUTPUT"
echo ""

# Use jq to validate JSON structure
if echo "$JSON_OUTPUT" | jq empty 2>/dev/null; then
    # Check for proper JSON structure using jq
    if echo "$JSON_OUTPUT" | jq -e '.account and .balance.total and .balance.total_mina and .nonce' > /dev/null 2>&1; then
        echo "✓ Test passed: JSON format has proper structure"
        exit 0
    else
        echo "✗ Test failed: JSON structure is incomplete"
        echo "Expected fields: account, balance.total, balance.total_mina, nonce"
        exit 1
    fi
elif echo "$JSON_OUTPUT" | grep -qE "(Error:|\\[ERROR\\])"; then
    echo "✗ Test failed: Could not retrieve account balance"
    echo "The test account may not exist on the network"
    exit 1
else
    echo "✗ Test failed: Output is not valid JSON"
    exit 1
fi
