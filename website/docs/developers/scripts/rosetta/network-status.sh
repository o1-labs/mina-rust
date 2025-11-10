#!/bin/bash

# Query Rosetta /network/status endpoint
# Usage: ./network-status.sh <ROSETTA_URL> <NETWORK> <BLOCKCHAIN>
# Example: ./network-status.sh https://devnet-rosetta.gcp.o1test.net mina devnet

ROSETTA_URL="${1:-https://devnet-rosetta.gcp.o1test.net}"
NETWORK="${2:-mina}"
BLOCKCHAIN="${3:-devnet}"

curl -s -X POST "${ROSETTA_URL}/network/status" \
  -H "Content-Type: application/json" \
  -d "{
    \"network_identifier\": {
      \"blockchain\": \"${BLOCKCHAIN}\",
      \"network\": \"${NETWORK}\"
    },
    \"metadata\": {}
  }"
