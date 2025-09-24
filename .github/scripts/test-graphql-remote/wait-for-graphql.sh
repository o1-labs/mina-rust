#!/bin/bash
# Wait for GraphQL endpoint to become available
# Usage: ./wait-for-graphql.sh [GRAPHQL_ENDPOINT] [TIMEOUT_SECONDS] [CHECK_INTERVAL]

set -e

GRAPHQL_ENDPOINT="${1:-http://localhost:3085/graphql}"
TIMEOUT_SECONDS="${2:-60}"
CHECK_INTERVAL="${3:-2}"

echo "Waiting for GraphQL endpoint to become available..."
echo "Endpoint: $GRAPHQL_ENDPOINT"
echo "Timeout: ${TIMEOUT_SECONDS}s, Check interval: ${CHECK_INTERVAL}s"

MAX_ATTEMPTS=$(( TIMEOUT_SECONDS / CHECK_INTERVAL ))

for i in $(seq 1 $MAX_ATTEMPTS); do
    if curl -s --max-time 5 -X POST "$GRAPHQL_ENDPOINT" \
       -H "Content-Type: application/json" \
       -d '{"query":"{ networkID }"}' > /dev/null 2>&1; then
        echo "✓ GraphQL endpoint is ready after $((i * CHECK_INTERVAL)) seconds"
        exit 0
    fi
    echo "Waiting... ($i/$MAX_ATTEMPTS)"
    sleep "${CHECK_INTERVAL}"
done

echo "✗ GraphQL endpoint not available after ${TIMEOUT_SECONDS} seconds"
exit 1
