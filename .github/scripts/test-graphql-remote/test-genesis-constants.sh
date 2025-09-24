#!/bin/bash
# Test genesis constants query
# Usage: ./test-genesis-constants.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing genesisConstants query..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/genesis-constants.sh)
echo "Response: $response"

account_creation_fee=$(echo "$response" | jq -r '.data.genesisConstants.accountCreationFee // empty')
if [ -n "$account_creation_fee" ]; then
  echo "✓ Genesis constants query successful"

  # Extract and display some key constants
  coinbase=$(echo "$response" | jq -r '.data.genesisConstants.coinbase // empty')
  if [ -n "$coinbase" ]; then
    echo "  Coinbase reward: $coinbase"
  fi

  fee=$(echo "$response" | jq -r '.data.genesisConstants.accountCreationFee // empty')
  if [ -n "$fee" ]; then
    echo "  Account creation fee: $fee"
  fi
else
  echo "✗ Failed to get genesis constants"
  exit 1
fi