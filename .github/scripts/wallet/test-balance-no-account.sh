#!/bin/bash

# Test the wallet balance command error when no account specified

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

echo "Test: Verify error when no account specified"
ERROR_OUTPUT=$(./target/release/mina wallet balance --endpoint "$MINA_NODE_ENDPOINT" 2>&1 || true)

echo "Command output:"
echo "$ERROR_OUTPUT"
echo ""

if echo "$ERROR_OUTPUT" | grep -qE "(\\[ERROR\\].*Either --address or --from must be provided|Either --address or --from must be provided)"; then
    echo "✓ Test passed: Proper error when no account specified"
    exit 0
else
    echo "✗ Test failed: Expected error message not found"
    exit 1
fi
