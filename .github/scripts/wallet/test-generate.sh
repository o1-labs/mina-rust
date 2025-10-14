#!/bin/bash

set -euo pipefail

# Test the wallet generate command

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

# Define test parameters
TEST_KEY="/tmp/mina-test-generated-key-$$"
TEST_PUBKEY="${TEST_KEY}.pub"
PASSWORD="test-password-e2e"

# Cleanup function
cleanup() {
    # shellcheck disable=SC2317
    rm -f "$TEST_KEY" "$TEST_PUBKEY"
}

# Set trap to cleanup on exit
trap cleanup EXIT

echo "Testing: mina wallet generate"
echo "Output file: $TEST_KEY"
echo ""

# Generate a new key
export MINA_PRIVKEY_PASS="$PASSWORD"
GENERATE_OUTPUT=$(./target/release/mina wallet generate --output "$TEST_KEY")

echo "Generate command output:"
echo "$GENERATE_OUTPUT"
echo ""

# Verify the private key file was created
if [ ! -f "$TEST_KEY" ]; then
    echo "✗ Test failed: Private key file was not created"
    exit 1
fi
echo "✓ Private key file created"

# Verify the public key file was created
if [ ! -f "$TEST_PUBKEY" ]; then
    echo "✗ Test failed: Public key file was not created"
    exit 1
fi
echo "✓ Public key file created"

# Extract the public key from the generate output
EXPECTED_PUBKEY=$(cat "$TEST_PUBKEY")
echo "Expected public key: $EXPECTED_PUBKEY"

# Verify the key can be read back with wallet address command
ACTUAL_PUBKEY=$(./target/release/mina wallet address --from "$TEST_KEY")
echo "Actual public key:   $ACTUAL_PUBKEY"
echo ""

# Compare the public keys
if [ "$ACTUAL_PUBKEY" = "$EXPECTED_PUBKEY" ]; then
    echo "✓ Test passed: Generated key can be read back successfully"
    exit 0
else
    echo "✗ Test failed: Public key mismatch"
    echo "  Expected: $EXPECTED_PUBKEY"
    echo "  Got:      $ACTUAL_PUBKEY"
    exit 1
fi
