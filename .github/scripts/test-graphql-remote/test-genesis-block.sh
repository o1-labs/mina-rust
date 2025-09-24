#!/bin/bash
# Test genesis block query
# Usage: ./test-genesis-block.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing genesisBlock query..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/genesis-block.sh)
echo "Response: $response"

genesis_state_hash=$(echo "$response" | jq -r '.data.genesisBlock.stateHash // empty')
if [ -n "$genesis_state_hash" ]; then
  echo "✓ Genesis block query successful"

  # Extract genesis block height
  genesis_height=$(echo "$response" | jq -r '.data.genesisBlock.protocolState.consensusState.blockHeight // empty')
  if [ -n "$genesis_height" ]; then
    echo "Genesis block height: $genesis_height"
  fi
else
  echo "✗ Failed to get genesis block"
  exit 1
fi