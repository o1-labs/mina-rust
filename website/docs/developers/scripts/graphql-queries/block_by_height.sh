#!/bin/bash

HEIGHT="$1"

if [ -z "$HEIGHT" ]; then
  echo "Usage: $0 <height>"
  echo "Example: $0 12345"
  exit 1
fi

curl -X POST "${GRAPHQL_ENDPOINT:-http://127.0.0.1:3085/graphql}" \
  -H "Content-Type: application/json" \
  -d "{\"query\":\"{ block(height: $HEIGHT) { stateHash protocolState { consensusState { blockHeight epoch slot } blockchainState { snarkedLedgerHash } } transactions { userCommands { id } zkappCommands { id } } } }\"}"