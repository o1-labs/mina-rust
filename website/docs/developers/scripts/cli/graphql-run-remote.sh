# shellcheck shell=bash
# Execute a GraphQL query on a remote node
mina internal graphql run \
  'query { syncStatus }' \
  --node http://mina-rust-plain-3.gcp.o1test.net/graphql
