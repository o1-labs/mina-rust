#!/bin/bash

# Test the wallet balance command text format output structure

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

echo "Test: Verify text format output structure"
TEXT_OUTPUT=$(./target/release/mina wallet balance \
    --address "B62qkiqPXFDayJV8JutYvjerERZ35EKrdmdcXh3j1rDUHRs1bJkFFcX" \
    --endpoint "$MINA_NODE_ENDPOINT" \
    --format text 2>&1 || true)

echo "Text format output:"
echo "$TEXT_OUTPUT"
echo ""

if echo "$TEXT_OUTPUT" | grep -q "Account:"; then
    # Check for proper text format structure
    if echo "$TEXT_OUTPUT" | grep -qE "Balance:" && \
       echo "$TEXT_OUTPUT" | grep -qE "Total:.*MINA" && \
       echo "$TEXT_OUTPUT" | grep -qE "Nonce:"; then
        echo "✓ Test passed: Text format has proper structure"
        exit 0
    else
        echo "✗ Test failed: Text output structure is incomplete"
        echo "Expected fields: Balance:, Total: X MINA, Nonce:"
        exit 1
    fi
elif echo "$TEXT_OUTPUT" | grep -qE "(Error:|\\[ERROR\\])"; then
    echo "✗ Test failed: Could not retrieve account balance"
    echo "The test account may not exist on the network"
    exit 1
else
    echo "✗ Test failed: Unexpected output format"
    exit 1
fi
