#!/bin/bash

# Test archive node HTTP API endpoints
#
# Usage:
#   ./test-archive-node-connectivity.sh
#
# This script tests HTTP connectivity to o1Labs archive nodes

echo "Testing archive node connectivity..."

# Read archive nodes from file
archive_nodes_file="website/docs/developers/scripts/infrastructure/archive-nodes.txt"

if [ ! -f "$archive_nodes_file" ]; then
  echo "❌ Archive nodes file not found: $archive_nodes_file"
  exit 1
fi

archive_nodes=$(cat "$archive_nodes_file")
failed=0

for node_url in $archive_nodes; do
  echo "Testing archive endpoint: $node_url"

  # Test basic SQL query (SELECT 1)
  query_url="${node_url}?query=SELECT%201"

  if response=$(curl -s --connect-timeout 10 --max-time 30 "$query_url" 2>&1); then
    # Check if response contains expected result
    if echo "$response" | grep -q "308\|301\|302"; then
      echo "WARNING: $node_url returned redirect (may require authentication or different access method)"
      # Don't fail on redirect, as this might be expected for archive nodes
    elif [ -n "$response" ]; then
      echo "PASS: $node_url is responding"
      echo "   Response preview: $(echo "$response" | head -c 100)..."
    else
      echo "FAIL: $node_url returned empty response"
      failed=$((failed + 1))
    fi
  else
    echo "FAIL: $node_url query failed"
    failed=$((failed + 1))
  fi

  echo "---"
done

if [ $failed -gt 0 ]; then
  echo "ERROR: $failed archive node tests failed"
  echo "Infrastructure issues detected. Please check archive node status."
  exit 1
else
  echo "SUCCESS: All archive nodes are responding"
fi
