#!/bin/bash
# Test complex nested query
# Usage: ./test-complex-nested-query.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing complex nested query..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

query='{ bestChain(maxLength: 2) { stateHash protocolState { consensusState { blockHeight epoch slot } blockchainState { snarkedLedgerHash } } } syncStatus networkID }'

response=$(curl -s --max-time 15 -X POST "$GRAPHQL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -d "{\"query\":\"$query\"}")

echo "Complex query response: $response"

# Check if all expected fields are present
state_hash_complex=$(echo "$response" | jq -r '.data.bestChain[0].stateHash // empty')
sync_status_complex=$(echo "$response" | jq -r '.data.syncStatus // empty')
network_id_complex=$(echo "$response" | jq -r '.data.networkID // empty')

if [ -n "$state_hash_complex" ] && [ -n "$sync_status_complex" ] && [ -n "$network_id_complex" ]; then
  echo "✓ Complex nested query successful"
else
  echo "✗ Complex nested query failed"
  exit 1
fi