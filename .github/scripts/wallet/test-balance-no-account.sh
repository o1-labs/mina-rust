#!/bin/bash

set -euo pipefail

# Test the wallet balance command error when no account specified

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

echo "Test: Verify error when no account specified"
ERROR_OUTPUT=$(./target/release/mina wallet balance --endpoint "http://mina-rust-plain-1.gcp.o1test.net/graphql" 2>&1 || true)

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
