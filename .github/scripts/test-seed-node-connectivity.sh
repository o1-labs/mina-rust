#!/bin/bash

# Test seed node connectivity
# This corresponds to the first step in test-docs-infrastructure.yaml
#
# Usage:
#   ./test-seed-node-connectivity.sh
#
# This script tests connectivity to o1Labs seed nodes

echo "🔍 Testing o1Labs seed node connectivity..."

# Read seed nodes from file
seed_file="website/docs/developers/scripts/infrastructure/seed-nodes.txt"

if [ ! -f "$seed_file" ]; then
  echo "❌ Seed nodes file not found: $seed_file"
  exit 1
fi

failed=0

# Extract hostnames from multiaddress format and test connectivity
while IFS= read -r seed_address; do
  # Extract hostname from multiaddress: /peer-id/https/hostname/port
  hostname=$(echo "$seed_address" | cut -d'/' -f4)

  if [ -z "$hostname" ]; then
    echo "❌ Could not extract hostname from: $seed_address"
    failed=$((failed + 1))
    continue
  fi

  echo "Testing connectivity to $hostname (from $seed_address)..."

  # Test HTTPS connectivity (port 443)
  if curl -s --connect-timeout 10 --max-time 30 "https://$hostname" > /dev/null 2>&1; then
    echo "✅ $hostname is reachable via HTTPS"
  else
    echo "❌ $hostname is not reachable via HTTPS"
    failed=$((failed + 1))
  fi

  # Test basic DNS resolution (non-fatal for now)
  if nslookup "$hostname" > /dev/null 2>&1; then
    echo "✅ $hostname DNS resolution successful"
  else
    echo "⚠️  $hostname DNS resolution failed (may be environment-specific)"
  fi

  echo "---"
done < "$seed_file"

if [ $failed -gt 0 ]; then
  echo "💥 $failed connectivity tests failed"
  echo "Infrastructure issues detected. Please check seed node status."
  exit 1
else
  echo "🎉 All seed nodes are healthy and reachable"
fi