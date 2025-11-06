#!/bin/bash

# Verify seed node address format
# This corresponds to the third step in test-docs-infrastructure.yaml
#
# Usage:
#   ./test-seed-node-format.sh
#
# This script verifies seed nodes are subset of official o1Labs seeds

echo "🔍 Verifying seed nodes are subset of official o1Labs seeds..."

# Download official seeds list
official_seeds_url="https://raw.githubusercontent.com/o1-labs/seeds/main/networks/devnet-webrtc.txt"
curl -s "$official_seeds_url" > /tmp/official-seeds.txt

if [ ! -s /tmp/official-seeds.txt ]; then
  echo "❌ Failed to download official seeds list"
  exit 1
fi

echo "Downloaded official seeds list"

# Read our seed nodes
seed_file="website/docs/developers/scripts/infrastructure/seed-nodes.txt"
our_seeds=$(cat "$seed_file")

# Check each of our seeds exists in official list
missing=0
while IFS= read -r seed; do
  if grep -Fxq "$seed" /tmp/official-seeds.txt; then
    echo "✅ Found in official list: $seed"
  else
    echo "❌ Missing from official list: $seed"
    missing=$((missing + 1))
  fi
done <<< "$our_seeds"

if [ $missing -gt 0 ]; then
  echo "💥 $missing seed node(s) not found in official seeds"
  echo "Official seeds list:"
  cat /tmp/official-seeds.txt
  exit 1
else
  echo "🎉 All seed nodes are present in official seeds list"
fi