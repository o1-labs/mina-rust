#!/bin/bash

# Test seed node response headers
# This corresponds to the second step in test-docs-infrastructure.yaml
#
# Usage:
#   ./test-seed-node-headers.sh
#
# This script tests HTTP response headers from o1Labs seed nodes

echo "🔍 Testing seed node HTTP response headers..."

# Read seed nodes from file
seed_file="website/docs/developers/scripts/infrastructure/seed-nodes.txt"

if [ ! -f "$seed_file" ]; then
  echo "❌ Seed nodes file not found: $seed_file"
  exit 1
fi

# Extract hostnames from multiaddress format and test headers
while IFS= read -r seed_address; do
  # Extract hostname from multiaddress: /peer-id/https/hostname/port
  hostname=$(echo "$seed_address" | cut -d'/' -f4)

  if [ -z "$hostname" ]; then
    echo "❌ Could not extract hostname from: $seed_address"
    continue
  fi

  echo "Checking headers for $hostname (from $seed_address)..."

  # Get response headers (ignore cert issues for now)
  if headers=$(curl -s -I --connect-timeout 10 --max-time 30 "https://$hostname" 2>/dev/null); then
    echo "✅ $hostname returned headers:"
    echo "$headers" | head -5 | sed 's/^/  /'
  else
    echo "⚠️  $hostname did not return headers (may be expected for WebRTC endpoints)"
  fi

  echo "---"
done < "$seed_file"