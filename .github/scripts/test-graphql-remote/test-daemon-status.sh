#!/bin/bash
# Test daemon status query
# Usage: ./test-daemon-status.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing daemonStatus query..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/daemon-status.sh)
echo "Response: $response"

chain_id_check=$(echo "$response" | jq -r '.data.daemonStatus.chainId // empty')
if [ -n "$chain_id_check" ]; then
  echo "✓ Daemon status query successful"

  # Extract chain ID
  chain_id=$(echo "$response" | jq -r '.data.daemonStatus.chainId // empty')
  if [ -n "$chain_id" ]; then
    echo "  Chain ID: $chain_id"
  fi

  # Extract commit ID if available
  commit_id=$(echo "$response" | jq -r '.data.daemonStatus.commitId // empty')
  if [ -n "$commit_id" ]; then
    echo "  Commit ID: $commit_id"
  fi
else
  echo "✗ Failed to get daemon status"
  exit 1
fi