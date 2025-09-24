#!/bin/bash
# Test sync status query
# Usage: ./02-test-sync-status.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing syncStatus query..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

response=$(GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT" ./website/docs/developers/scripts/graphql-api/queries/curl/sync-status.sh)
echo "Response: $response"

status=$(echo "$response" | jq -r '.data.syncStatus // empty')
if [ -n "$status" ]; then
  echo "✓ Sync Status: $status"

  case "$status" in
    "SYNCED"|"CATCHUP"|"BOOTSTRAP"|"CONNECTING"|"LISTENING")
      echo "✓ Valid sync status received"
      ;;
    *)
      echo "? Unknown sync status: $status"
      ;;
  esac
else
  echo "✗ Failed to get sync status"
  exit 1
fi