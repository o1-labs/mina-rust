#!/bin/bash

# Test devnet archive node connectivity
curl -s 'https://devnet-archive-node-api.gcp.o1test.net/?query=SELECT%201' \
  | head -n 1
