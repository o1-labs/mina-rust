#!/bin/bash

# Test plain node API capabilities
# This corresponds to the fifth step in test-docs-infrastructure.yaml
#
# Usage:
#   ./test-plain-node-capabilities.sh
#
# This script tests API capabilities of o1Labs plain nodes

echo "🔍 Testing plain node API capabilities..."

# Read plain nodes from file
plain_nodes_file="website/docs/developers/scripts/infrastructure/plain-nodes.txt"
plain_nodes=$(cat "$plain_nodes_file")

# Test with first available node
for node_url in $plain_nodes; do
  graphql_url="${node_url}graphql"

  echo "Testing API capabilities on: $graphql_url"

  # Test network ID query using website script
  network_success=false
  if network_response=$(bash website/docs/developers/scripts/graphql-api/queries/curl/network-id.sh "$graphql_url" 2>&1); then
    if echo "$network_response" | jq -e '.data.networkID' > /dev/null 2>&1; then
      network_id=$(echo "$network_response" | jq -r '.data.networkID')
      echo "✅ Network ID query successful: $network_id"
      network_success=true
    else
      echo "⚠️  Network ID query failed or unexpected response"
    fi
  else
    echo "⚠️  Network ID query script failed"
  fi

  # Test best chain query using website script
  chain_success=false
  if chain_response=$(bash website/docs/developers/scripts/graphql-api/queries/curl/best-chain.sh "$graphql_url" 2>&1); then
    if echo "$chain_response" | jq -e '.data.bestChain[0].stateHash' > /dev/null 2>&1; then
      state_hash=$(echo "$chain_response" | jq -r '.data.bestChain[0].stateHash')
      echo "✅ Best chain query successful: ${state_hash:0:16}..."
      chain_success=true
    else
      echo "⚠️  Best chain query failed or unexpected response"
    fi
  else
    echo "⚠️  Best chain query script failed"
  fi

  # We only need to test one working node
  if [ "$network_success" = true ] && [ "$chain_success" = true ]; then
    echo "🎉 Plain node API capabilities verified"
    break
  fi
done