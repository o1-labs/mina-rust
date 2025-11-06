# shellcheck shell=bash
# Execute a query against OCaml-specific protocolState endpoint
mina internal graphql run 'query { protocolState }' --node https://devnet-plain-1.gcp.o1test.net/graphql
