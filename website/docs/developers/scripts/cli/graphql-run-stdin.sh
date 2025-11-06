# shellcheck shell=bash
# Execute a GraphQL query from stdin
echo 'query { version }' | mina internal graphql run
