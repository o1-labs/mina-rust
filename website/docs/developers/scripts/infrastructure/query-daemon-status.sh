#!/bin/bash
# Usage: $0 GRAPHQL_ENDPOINT
# GRAPHQL_ENDPOINT: GraphQL endpoint URL (required)

if [ -z "$1" ]; then
    echo "Error: GRAPHQL_ENDPOINT is required"
    echo "Usage: $0 GRAPHQL_ENDPOINT"
    exit 1
fi

GRAPHQL_ENDPOINT="$1"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
QUERY=$(cat "$SCRIPT_DIR/queries/daemon-status.graphql")
curl -s -X POST "$GRAPHQL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -d "{\"query\": \"$QUERY\"}"