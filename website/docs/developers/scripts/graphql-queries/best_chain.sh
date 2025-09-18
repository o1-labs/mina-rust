#!/bin/bash

curl -X POST "${GRAPHQL_ENDPOINT:-http://127.0.0.1:3085/graphql}" \
  -H "Content-Type: application/json" \
  -d '{"query":"{ bestChain(maxLength: 3) { stateHash protocolState { consensusState { blockHeight epoch slot } blockchainState { snarkedLedgerHash } } } }"}'