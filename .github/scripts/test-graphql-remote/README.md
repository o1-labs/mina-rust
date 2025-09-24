# GraphQL Remote Test Scripts

This directory contains individual test scripts that can be run against any GraphQL endpoint to verify compatibility with the Mina GraphQL API.

## Usage

### Run All Tests

```bash
# Test against the default endpoint
./run-all-tests.sh

# Test against a custom endpoint
./run-all-tests.sh http://your-node:3085/graphql

# Using environment variable
GRAPHQL_ENDPOINT=http://your-node:3085/graphql ./run-all-tests.sh
```

### Run Individual Tests

Each test script can be run independently:

```bash
# Test basic connectivity
./check-endpoint-availability.sh http://localhost:3085/graphql

# Test specific queries
./test-sync-status.sh http://localhost:3085/graphql
./test-daemon-status.sh http://localhost:3085/graphql
./test-best-chain.sh http://localhost:3085/graphql
```

## Test Scripts

### Core Functionality
- `check-endpoint-availability.sh` - Verify GraphQL endpoint is accessible
- `test-sync-status.sh` - Test syncStatus query
- `test-network-info.sh` - Test networkID and version queries

### Blockchain Data
- `test-daemon-status.sh` - Test daemonStatus query
- `test-best-chain.sh` - Test bestChain query
- `test-block-by-height.sh` - Test block query by height
- `test-genesis-block.sh` - Test genesisBlock query
- `test-genesis-constants.sh` - Test genesisConstants query

### Advanced Tests
- `test-complex-nested-query.sh` - Test complex nested GraphQL queries
- `test-error-handling.sh` - Test GraphQL error handling

### Test Runner
- `run-all-tests.sh` - Master script that runs all tests in sequence

## Prerequisites

- `curl` - For making HTTP requests
- `jq` - For parsing JSON responses
- Access to the website GraphQL scripts (must run from repo root)

## CI Integration

These scripts are used by the GitHub Actions workflow in `.github/workflows/test-graphql-compatibility.yml` to test GraphQL compatibility between Rust and OCaml node implementations.

To run the CI workflow locally using `act`:

```bash
# Install act via gh CLI
gh extension install https://github.com/nektos/gh-act

# Run the workflow
gh act --workflows .github/workflows/test-graphql-compatibility.yml

# Run specific job
gh act --workflows .github/workflows/test-graphql-compatibility.yml --job test-ocaml-node-graphql
```

## Tested Endpoints

These scripts test only the GraphQL endpoints that are implemented in the Rust node:

**Queries:**
- `syncStatus` - Node synchronization status
- `networkID` - Network identifier
- `version` - Node version
- `daemonStatus` - Comprehensive daemon status
- `bestChain` - Best chain blocks
- `block` - Individual block by height/hash
- `genesisBlock` - Genesis block
- `genesisConstants` - Network constants
- `account` - Account information
- `pooledUserCommands` - User commands in transaction pool
- `pooledZkappCommands` - zkApp commands in transaction pool
- `snarkPool` - Completed SNARK work
- `pendingSnarkWork` - Pending SNARK work
- `currentSnarkWorker` - Current SNARK worker info

**Mutations:** (tested via website scripts)
- `sendPayment` - Send payment transactions
- `sendDelegation` - Send delegation transactions
- `sendZkapp` - Send zkApp transactions

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Output Format

The scripts provide colored output:
- 🟢 Green: Test passed
- 🔴 Red: Test failed
- 🟡 Yellow: Test information/headers
- 🔵 Blue: Section headers

## Debugging

To debug failed tests:

1. Run individual test scripts to isolate issues
2. Check the GraphQL endpoint is accessible
3. Verify the node is running and synced
4. Check the raw curl commands in the scripts
5. Use the website's GraphQL scripts directly for comparison