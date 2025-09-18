#!/bin/bash
# Usage: $0 GRAPHQL_ENDPOINT
# GRAPHQL_ENDPOINT: GraphQL endpoint URL (required)

if [ -z "$1" ]; then
    echo "Error: GRAPHQL_ENDPOINT is required"
    echo "Usage: $0 GRAPHQL_ENDPOINT"
    exit 1
fi

GRAPHQL_ENDPOINT="$1"

# Extract base URL from GraphQL endpoint (remove /graphql suffix)
BASE_URL=${GRAPHQL_ENDPOINT%/graphql}

curl -s -I "$BASE_URL/"