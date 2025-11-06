# shellcheck shell=bash
# Execute a GraphQL query with variables
# shellcheck disable=SC2016
mina internal graphql run \
  'query($maxLen: Int!) { bestChain(maxLength: $maxLen) { stateHash } }' \
  -v '{"maxLen": 5}'
