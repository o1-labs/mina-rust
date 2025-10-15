#!/bin/bash

set -euo pipefail

# Test the wallet address command with encrypted key file

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

# Define test parameters
KEY_FILE="tests/files/accounts/test-block-producer"
PUBKEY_FILE="tests/files/accounts/test-block-producer.pub"
PASSWORD="test-password"

# Read expected public key
EXPECTED_PUBKEY=$(cat "$PUBKEY_FILE")

echo "Testing: mina wallet address"
echo "Key file: $KEY_FILE"
echo "Expected public key: $EXPECTED_PUBKEY"
echo ""

# Run the wallet address command with password from environment variable
export MINA_PRIVKEY_PASS="$PASSWORD"
ACTUAL_PUBKEY=$(./target/release/mina wallet address --from "$KEY_FILE")

echo "Actual public key: $ACTUAL_PUBKEY"
echo ""

# Compare the public keys
if [ "$ACTUAL_PUBKEY" = "$EXPECTED_PUBKEY" ]; then
    echo "✓ Test passed: Public key matches expected value"
    exit 0
else
    echo "✗ Test failed: Public key mismatch"
    echo "  Expected: $EXPECTED_PUBKEY"
    echo "  Got:      $ACTUAL_PUBKEY"
    exit 1
fi
