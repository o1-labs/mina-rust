#!/bin/bash
# Check GraphQL endpoint availability
# Usage: ./01-check-endpoint-availability.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing GraphQL endpoint availability..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

if curl -s --max-time 10 -X POST "$GRAPHQL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ networkID }"}' > /dev/null; then
  echo "✓ GraphQL endpoint is accessible"
  exit 0
else
  echo "✗ GraphQL endpoint is not accessible"
  exit 1
fi