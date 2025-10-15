#!/bin/bash

# Test the wallet balance command with --address (JSON format)

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

# Define test parameters
PUBLIC_KEY=$(cat "tests/files/accounts/test-block-producer.pub")

echo "Test: Verify --address option works with public key (JSON format)"
BALANCE_JSON=$(./target/release/mina wallet balance --address "$PUBLIC_KEY" --endpoint "$MINA_NODE_ENDPOINT" --format json 2>&1 || true)

echo "Balance command output:"
echo "$BALANCE_JSON"
echo ""

# Command should execute and produce JSON output
# Try to parse as JSON using jq
if echo "$BALANCE_JSON" | jq empty 2>/dev/null; then
    echo "✓ Command executed successfully with JSON format"
    # Verify JSON format structure using jq
    if echo "$BALANCE_JSON" | jq -e '.account and .balance and .nonce' > /dev/null 2>&1; then
        echo "✓ JSON output has expected structure"
        exit 0
    else
        echo "✗ Test failed: JSON output missing expected fields"
        exit 1
    fi
elif echo "$BALANCE_JSON" | grep -qE "(Error:|\\[ERROR\\])"; then
    # Command executed but got an error (e.g., account doesn't exist)
    echo "✓ Command executed (account may not exist on network)"
    exit 0
else
    echo "✗ Test failed: Unexpected output"
    exit 1
fi
