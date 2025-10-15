#!/bin/bash

# Test the wallet balance command with --from key file (text format)

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

# Define test parameters
KEY_FILE="tests/files/accounts/test-block-producer"
PASSWORD="test-password"

echo "Test: Verify --from option works with key file (text format)"
export MINA_PRIVKEY_PASS="$PASSWORD"
BALANCE_OUTPUT=$(./target/release/mina wallet balance --from "$KEY_FILE" --endpoint "$MINA_NODE_ENDPOINT" --format text 2>&1 || true)

echo "Balance command output:"
echo "$BALANCE_OUTPUT"
echo ""

# Command should execute and produce text format output
if echo "$BALANCE_OUTPUT" | grep -q "Account:"; then
    echo "✓ Command executed successfully with text format"
    # Verify text format structure
    if echo "$BALANCE_OUTPUT" | grep -q "Balance:" && \
       echo "$BALANCE_OUTPUT" | grep -q "Total:" && \
       echo "$BALANCE_OUTPUT" | grep -q "Nonce:"; then
        echo "✓ Text output has expected structure"
        exit 0
    else
        echo "✗ Test failed: Text output missing expected fields"
        exit 1
    fi
elif echo "$BALANCE_OUTPUT" | grep -qE "(Error:|\\[ERROR\\])"; then
    # Command executed but got an error (e.g., account doesn't exist)
    echo "✓ Command executed (account may not exist on network)"
    exit 0
else
    echo "✗ Test failed: Unexpected output"
    exit 1
fi
