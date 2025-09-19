#!/bin/bash

# Test plain node GraphQL endpoints
# This corresponds to the fourth step in test-docs-infrastructure.yaml
#
# Usage:
#   ./test-plain-node-connectivity.sh
#
# This script tests GraphQL connectivity to o1Labs plain nodes

echo "🔍 Testing plain node GraphQL connectivity..."

# Read plain nodes from file
plain_nodes_file="website/docs/developers/scripts/infrastructure/plain-nodes.txt"

if [ ! -f "$plain_nodes_file" ]; then
  echo "❌ Plain nodes file not found: $plain_nodes_file"
  exit 1
fi

plain_nodes=$(cat "$plain_nodes_file")
failed=0

for node_url in $plain_nodes; do
  echo "Testing GraphQL endpoint: $node_url"

  # Test basic HTTP connectivity
  if curl -s --connect-timeout 10 --max-time 30 "$node_url" > /dev/null 2>&1; then
    echo "✅ $node_url is reachable via HTTP"
  else
    echo "❌ $node_url is not reachable via HTTP"
    failed=$((failed + 1))
    continue
  fi

  # Test GraphQL endpoint using website scripts
  graphql_url="${node_url}graphql"

  # Test daemon status query using the website script
  if response=$(bash website/docs/developers/scripts/graphql-api/queries/curl/daemon-status.sh "$graphql_url" 2>/dev/null); then
    # Extract JSON response (skip curl progress output)
    json_response=$(echo "$response" | grep "^{")

    # Check if it's valid JSON
    if echo "$json_response" | jq . > /dev/null 2>&1; then
      # Check for GraphQL errors
      if echo "$json_response" | jq -e '.errors' > /dev/null 2>&1; then
        echo "⚠️  $graphql_url returned GraphQL error:"
        echo "$json_response" | jq '.errors'
      # Check for valid data
      elif echo "$json_response" | jq -e '.data.daemonStatus' > /dev/null 2>&1; then
        echo "✅ $graphql_url GraphQL query successful"
        sync_status=$(echo "$json_response" | jq -r '.data.daemonStatus.syncStatus // "unknown"')
        chain_id=$(echo "$json_response" | jq -r '.data.daemonStatus.chainId // "unknown"')
        echo "   Sync Status: $sync_status, Chain ID: ${chain_id:0:16}..."
      else
        echo "⚠️  $graphql_url unexpected response format"
      fi
    else
      echo "⚠️  $graphql_url did not return valid JSON: $(echo "$response" | head -c 100)..."
    fi
  else
    echo "❌ $graphql_url GraphQL query failed"
    failed=$((failed + 1))
  fi

  echo "---"
done

if [ $failed -gt 0 ]; then
  echo "💥 $failed plain node tests failed"
  echo "Infrastructure issues detected. Please check plain node status."
  exit 1
else
  echo "🎉 All plain nodes are healthy and responding"
fi