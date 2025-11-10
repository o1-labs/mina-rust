#!/bin/bash

# Get count of blocks in devnet archive
curl -s 'https://devnet-archive-node-api.gcp.o1test.net/?query=SELECT%20COUNT(*)%20FROM%20blocks' \
  | head -n 10
