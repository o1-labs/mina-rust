#!/bin/bash

# Test infrastructure command scripts
# This corresponds to the sixth step in test-docs-infrastructure.yaml
#
# Usage:
#   ./test-infrastructure-scripts.sh
#
# This script tests infrastructure command scripts against live endpoints

echo "🔍 Testing infrastructure command scripts..."

# Dynamically discover all bash scripts in the infrastructure directory
script_dir="website/docs/developers/scripts/infrastructure"

if [ ! -d "$script_dir" ]; then
  echo "❌ Script directory not found: $script_dir"
  exit 1
fi

# Get all .sh files in the infrastructure directory
script_files=$(ls "$script_dir"/*.sh 2>/dev/null)

if [ -z "$script_files" ]; then
  echo "❌ No bash scripts found in $script_dir"
  exit 1
fi

failed=0

# Use first plain node for testing infrastructure scripts
plain_nodes_file="website/docs/developers/scripts/infrastructure/plain-nodes.txt"
test_endpoint=""
if [ -f "$plain_nodes_file" ]; then
  first_node=$(head -n 1 "$plain_nodes_file")
  test_endpoint="${first_node}graphql"
else
  exit 1
fi

for script_file in $script_files; do
  if [ ! -f "$script_file" ]; then
    echo "❌ Script file not found: $script_file"
    failed=$((failed + 1))
    continue
  fi

  echo "Testing script: $script_file with the endpoint $test_endpoint"

  # Execute the script with test endpoint and capture output
  if output=$(bash "$script_file" "$test_endpoint" 2>&1); then
    echo "✅ Script executed successfully"

    # Try to parse output as JSON using jq
    if json_response=$(echo "$output" | jq . 2>/dev/null); then
      # Valid JSON response - check for GraphQL errors
      if echo "$json_response" | jq -e '.errors' > /dev/null 2>&1; then
        echo "❌ Script returned GraphQL errors:"
        echo "$json_response" | jq '.errors'
        failed=$((failed + 1))
      elif echo "$json_response" | jq -e '.data' > /dev/null 2>&1; then
        echo "✅ Script response contains valid data: $(echo "$json_response" | head -c 100)..."
      else
        echo "⚠️  Unexpected JSON response format: $(echo "$json_response" | head -c 100)..."
      fi
    else
      echo "❌ Script did not return valid JSON"
      failed=$((failed + 1))
    fi
  else
    echo "❌ Script execution failed: $script_file"
    failed=$((failed + 1))
  fi

  echo "---"
done

if [ $failed -gt 0 ]; then
  echo "💥 $failed infrastructure script tests failed"
  echo "Some infrastructure scripts may need updates."
  exit 1
else
  echo "🎉 All infrastructure command scripts are working"
fi