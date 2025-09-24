#!/bin/bash
# Test network information queries
# Usage: ./test-network-info.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing network information queries..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/network-id.sh)
echo "Response: $response"

network_id=$(echo "$response" | jq -r '.data.networkID // empty')
if [ -n "$network_id" ]; then
  echo "✓ Network ID: $network_id"
else
  echo "✗ Failed to get network ID"
  exit 1
fi

version=$(echo "$response" | jq -r '.data.version // empty')
if [ -n "$version" ]; then
  echo "✓ Node version: $version"
else
  echo "? Version query failed (might not be available)"
fi