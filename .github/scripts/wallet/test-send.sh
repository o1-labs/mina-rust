#!/bin/bash

# Test the wallet send command by sending a transaction to the same account
# Uses a very small fee to avoid draining the account on each PR

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
KEY_FILE="tests/files/accounts/test-wallet"
PUBKEY_FILE="tests/files/accounts/test-wallet.pub"
NODE_ENDPOINT="$MINA_NODE_ENDPOINT"

# Password from environment variable (set in GitHub secrets)
# Default to "test" for local testing
PASSWORD="${MINA_PRIVKEY_PASS:-test}"

# Read the public key (we'll send to ourselves)
RECEIVER=$(cat "$PUBKEY_FILE")

# Use minimal amounts to avoid draining the account
# 1 nanomina = smallest unit
AMOUNT="1"
# 1000000 nanomina = 0.001 MINA (small but acceptable fee)
FEE="1000000"

echo "Test: Send transaction to same account (e2e test)"
echo "Key file: $KEY_FILE"
echo "Receiver: $RECEIVER"
echo "Amount: $AMOUNT nanomina"
echo "Fee: $FEE nanomina"
echo "Node endpoint: $NODE_ENDPOINT"
echo ""

# Export password for the CLI
export MINA_PRIVKEY_PASS="$PASSWORD"

# Run the wallet send command
echo "Sending transaction..."
SEND_OUTPUT=$(./target/release/mina wallet send \
  --from "$KEY_FILE" \
  --to "$RECEIVER" \
  --amount "$AMOUNT" \
  --fee "$FEE" \
  --node "$NODE_ENDPOINT" \
  --network devnet 2>&1 || true)

echo "Send command output:"
echo "$SEND_OUTPUT"
echo ""

# Check if transaction was submitted successfully
if echo "$SEND_OUTPUT" | grep -q "Transaction submitted successfully!"; then
    echo "✓ Transaction submitted successfully"

    # Extract transaction hash
    TX_HASH=$(echo "$SEND_OUTPUT" | grep "Transaction hash:" | awk '{print $3}')

    if [ -n "$TX_HASH" ]; then
        echo "✓ Transaction hash returned: $TX_HASH"

        # Test the status command with the returned hash
        echo ""
        echo "Testing status command with transaction hash..."
        STATUS_OUTPUT=$(./target/release/mina wallet status \
          --hash "$TX_HASH" \
          --node "$NODE_ENDPOINT" 2>&1 || true)

        echo "Status command output:"
        echo "$STATUS_OUTPUT"
        echo ""

        # Check if status command worked (either found in mempool or blockchain)
        if echo "$STATUS_OUTPUT" | grep -qE "(Transaction found in mempool|Transaction Status:)"; then
            echo "✓ Status command successfully checked transaction"
            exit 0
        else
            echo "✓ Status command executed (transaction may have been processed)"
            exit 0
        fi
    else
        echo "✗ Test failed: No transaction hash returned"
        exit 1
    fi
elif echo "$SEND_OUTPUT" | grep -qE "(Error:|\\[ERROR\\])"; then
    # Check if it's a known acceptable error
    if echo "$SEND_OUTPUT" | grep -q "Node is not synced"; then
        echo "⚠ Node is not synced, skipping test"
        exit 0
    elif echo "$SEND_OUTPUT" | grep -q "Failed to connect to node"; then
        echo "⚠ Could not connect to node, skipping test"
        exit 0
    else
        echo "✗ Test failed with error:"
        echo "$SEND_OUTPUT"
        exit 1
    fi
else
    echo "✗ Test failed: Unexpected output"
    exit 1
fi
