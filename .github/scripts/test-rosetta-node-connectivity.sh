#!/bin/bash

# Test Rosetta node API endpoints
#
# Usage:
#   ./test-rosetta-node-connectivity.sh
#
# This script tests Rosetta API connectivity to o1Labs Rosetta nodes

echo "Testing Rosetta node connectivity..."

# Read Rosetta nodes from file
rosetta_nodes_file="website/docs/developers/scripts/infrastructure/rosetta-nodes.txt"

if [ ! -f "$rosetta_nodes_file" ]; then
  echo "FAIL: Rosetta nodes file not found: $rosetta_nodes_file"
  exit 1
fi

rosetta_nodes=$(cat "$rosetta_nodes_file")
failed=0

for node_url in $rosetta_nodes; do
  echo "Testing Rosetta endpoint: $node_url"

  # Determine network parameters based on URL
  if echo "$node_url" | grep -q "mainnet"; then
    network="mainnet"
    blockchain="mina"
  else
    network="devnet"
    blockchain="mina"
  fi

  # Test /network/list endpoint
  if response=$(bash website/docs/developers/scripts/rosetta/network-list.sh "$node_url" 2>&1); then
    # Check if it's valid JSON
    if echo "$response" | jq . > /dev/null 2>&1; then
      # Check for network_identifiers in response
      if echo "$response" | jq -e '.network_identifiers' > /dev/null 2>&1; then
        echo "PASS: $node_url /network/list successful"
        network_count=$(echo "$response" | jq '.network_identifiers | length')
        echo "   Networks available: $network_count"
      else
        echo "WARNING: $node_url /network/list unexpected response format"
      fi
    else
      echo "WARNING: $node_url /network/list did not return valid JSON"
    fi
  else
    echo "FAIL: $node_url /network/list query failed"
    failed=$((failed + 1))
  fi

  # Test /network/status endpoint
  if response=$(bash website/docs/developers/scripts/rosetta/network-status.sh "$node_url" "$network" "$blockchain" 2>&1); then
    if echo "$response" | jq . > /dev/null 2>&1; then
      if echo "$response" | jq -e '.current_block_identifier' > /dev/null 2>&1; then
        echo "PASS: $node_url /network/status successful"
        block_index=$(echo "$response" | jq -r '.current_block_identifier.index // "unknown"')
        echo "   Current block: $block_index"
      else
        echo "WARNING: $node_url /network/status unexpected response format"
      fi
    else
      echo "WARNING: $node_url /network/status did not return valid JSON"
    fi
  else
    echo "FAIL: $node_url /network/status query failed"
    failed=$((failed + 1))
  fi

  echo "---"
done

if [ $failed -gt 0 ]; then
  echo "ERROR: $failed Rosetta node tests failed"
  echo "Infrastructure issues detected. Please check Rosetta node status."
  exit 1
else
  echo "SUCCESS: All Rosetta nodes are responding"
fi
