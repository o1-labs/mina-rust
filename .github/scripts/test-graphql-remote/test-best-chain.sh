#!/bin/bash
# Test best chain query
# Usage: ./03-test-best-chain.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing bestChain query..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/best-chain.sh)
echo "Response: $response"

state_hash=$(echo "$response" | jq -r '.data.bestChain[0].stateHash // empty')
if [ -n "$state_hash" ]; then
  echo "✓ Best chain query successful"

  # Extract block height if available
  height=$(echo "$response" | jq -r '.data.bestChain[0].protocolState.consensusState.blockHeight // empty')
  if [ -n "$height" ]; then
    echo "  Latest block height: $height"
  fi
else
  echo "✗ Failed to get best chain"
  exit 1
fi