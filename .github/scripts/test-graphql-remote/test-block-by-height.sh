#!/bin/bash
# Test block query by height
# Usage: ./04-test-block-by-height.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing block query by height..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

# Get current height from best chain first
best_chain_response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/best-chain.sh)

current_height=$(echo "$best_chain_response" | jq -r '.data.bestChain[0].protocolState.consensusState.blockHeight // empty')
if [ -n "$current_height" ]; then
  echo "Current height: $current_height"

  # Test with current height - use website script if available, otherwise create query
  if [ -f "./website/docs/developers/scripts/graphql-api/queries/curl/block.sh" ]; then
    response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/block.sh)
  else
    # Fallback: create direct query
    response=$(curl -s --max-time 15 -X POST "$GRAPHQL_ENDPOINT" \
      -H "Content-Type: application/json" \
      -d "{\"query\":\"{ block(height: $current_height) { stateHash protocolState { consensusState { blockHeight } } } }\"}")
  fi

  echo "Block query response: $response"

  block_state_hash=$(echo "$response" | jq -r '.data.block.stateHash // empty')
  if [ -n "$block_state_hash" ]; then
    echo "✓ Block query by height successful"
  else
    echo "✗ Failed to get block by height"
    exit 1
  fi
else
  echo "? Could not determine current height, skipping block query test"
fi