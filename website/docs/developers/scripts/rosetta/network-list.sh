#!/bin/bash

# Query Rosetta /network/list endpoint
# Usage: ./network-list.sh <ROSETTA_URL>
# Example: ./network-list.sh https://devnet-rosetta.gcp.o1test.net

ROSETTA_URL="${1:-https://devnet-rosetta.gcp.o1test.net}"

curl -s -X POST "${ROSETTA_URL}/network/list" \
  -H "Content-Type: application/json" \
  -d '{
    "metadata": {}
  }'
