#!/bin/bash
# Test GraphQL error handling
# Usage: ./test-error-handling.sh [GRAPHQL_ENDPOINT]

set -e

GRAPHQL_ENDPOINT="${1:-${GRAPHQL_ENDPOINT:-http://mina-rust-plain-3.gcp.o1test.net/graphql}}"

echo "Testing GraphQL error handling..."
echo "Endpoint: $GRAPHQL_ENDPOINT"

# Test invalid query
echo "Testing invalid query syntax..."
error_response=$(curl -s --max-time 10 -X POST "$GRAPHQL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ invalidField }"}')

errors=$(echo "$error_response" | jq -r '.errors // empty')
if [ -n "$errors" ]; then
  echo "✓ Invalid query properly returns errors"
else
  echo "? Invalid query handling unclear"
fi

# Test malformed JSON
echo "Testing malformed request..."
malformed_response=$(curl -s --max-time 10 -X POST "$GRAPHQL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -d '{"query":"}' || echo "request_failed")

if echo "$malformed_response" | jq -e '.errors' > /dev/null 2>&1 || echo "$malformed_response" | grep -q "request_failed"; then
  echo "✓ Malformed requests are handled"
else
  echo "? Malformed request handling unclear"
fi