#!/bin/bash

# Test mainnet archive node connectivity
curl -s 'https://archive-node-api.gcp.o1test.net/?query=SELECT%201' \
  | head -n 1
