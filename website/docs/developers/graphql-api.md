---
title: GraphQL API
description: Complete reference for Mina Rust GraphQL API endpoints and queries
sidebar_position: 6
---

# GraphQL API Reference

The Mina Rust node provides a comprehensive GraphQL API for querying blockchain
data, account information, transaction status, and network statistics. The API
is built using [Juniper](https://github.com/graphql-rust/juniper) and is
available at `http://localhost:3000/graphql` when running a node.

## Quick Start

### Testing the API

```bash
# Basic connectivity test
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __typename }"}'

# Get sync status
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ syncStatus }"}'
```

### Interactive Exploration

The node provides interactive GraphQL explorers:

- **GraphQL Playground**: http://localhost:3000/playground
- **GraphiQL**: http://localhost:3000/graphiql

## API Endpoints

### Query Endpoints

#### Network and Node Status

##### `syncStatus`

Get the current synchronization status of the node.

```graphql
query {
  syncStatus # Returns: CONNECTING | LISTENING | OFFLINE | BOOTSTRAP | SYNCED | CATCHUP
}
```

##### `daemonStatus`

Get comprehensive daemon status information.

```graphql
query {
  daemonStatus {
    blockchainLength
    chainId
    commitId
    stateHash
    numAccounts
    globalSlotSinceGenesisBestTip
    ledgerMerkleRoot
    coinbaseReceiver
  }
}
```

##### `networkID`

Get the network identifier.

```graphql
query {
  networkID # Returns: "mina:devnet" or similar
}
```

##### `version`

Get the node version (git commit hash).

```graphql
query {
  version # Returns: git commit hash
}
```

#### Blockchain Data

##### `bestChain(maxLength: Int!)`

Get the best chain blocks up to specified length.

```graphql
query RecentBlocks {
  bestChain(maxLength: 10) {
    stateHash
    protocolState {
      consensusState {
        blockHeight
        slotSinceGenesis
      }
      previousStateHash
    }
    transactions {
      userCommands {
        id
        fee
        amount
        memo
      }
    }
  }
}
```

##### `block(height: Int, stateHash: String)`

Get a specific block by height or state hash.

```graphql
query GetBlock {
  block(height: 455450) {
    stateHash
    protocolState {
      consensusState {
        blockHeight
      }
    }
    creator
    transactions {
      userCommands {
        amount
        fee
        from
        to
      }
    }
  }
}
```

##### `genesisBlock`

Get the genesis block.

```graphql
query {
  genesisBlock {
    stateHash
    protocolState {
      consensusState {
        blockHeight
      }
    }
  }
}
```

##### `genesisConstants`

Get genesis constants and network parameters.

```graphql
query {
  genesisConstants {
    accountCreationFee
    genesisTimestamp
    coinbase
  }
}
```

#### Account Information

##### `account(publicKey: String!, token: String)`

Get account information by public key.

```graphql
query GetAccount($publicKey: String!) {
  account(publicKey: $publicKey) {
    balance {
      total
      liquid
      locked
    }
    nonce
    delegateAccount {
      publicKey
    }
    votingFor
    receiptChainHash
    publicKey
    token
    tokenSymbol
    zkappUri
    zkappState
    permissions {
      editState
      send
      receive
      access
      setDelegate
      setPermissions
      setVerificationKey
      setZkappUri
      editActionState
      setTokenSymbol
      incrementNonce
      setVotingFor
    }
  }
}
```

#### Transaction Pool

##### `pooledUserCommands(publicKey: String, hashes: [String], ids: [String])`

Get pending user commands (payments/delegations) from the transaction pool.

```graphql
query PooledUserCommands($publicKey: String) {
  pooledUserCommands(publicKey: $publicKey) {
    id
    amount
    fee
    from
    to
    nonce
    memo
    isDelegation
    hash
    kind
  }
}
```

##### `pooledZkappCommands(publicKey: String, hashes: [String], ids: [String])`

Get pending zkApp commands from the transaction pool.

```graphql
query PooledZkApps($publicKey: String) {
  pooledZkappCommands(publicKey: $publicKey) {
    id
    hash
    zkappCommand {
      feePayer {
        body {
          publicKey
          fee
          nonce
        }
      }
      accountUpdates {
        body {
          publicKey
          balanceChange
        }
      }
    }
  }
}
```

#### Transaction Status

##### `transactionStatus(payment: String, zkappTransaction: String)`

Get the status of a specific transaction.

```graphql
query TransactionStatus($transactionId: String!) {
  transactionStatus(payment: $transactionId) # Returns: PENDING | INCLUDED | UNKNOWN
}
```

#### SNARK Work

##### `snarkPool`

Get completed SNARK work from the pool.

```graphql
query {
  snarkPool {
    fee
    prover
  }
}
```

##### `pendingSnarkWork`

Get pending SNARK work that needs to be completed.

```graphql
query {
  pendingSnarkWork {
    workBundle {
      sourceFirstPassLedgerHash
      targetFirstPassLedgerHash
      sourceSecondPassLedgerHash
      targetSecondPassLedgerHash
      workId
    }
  }
}
```

##### `currentSnarkWorker`

Get information about the currently configured SNARK worker.

```graphql
query {
  currentSnarkWorker {
    key
    fee
    account {
      publicKey
      balance {
        total
      }
    }
  }
}
```

### Mutation Endpoints

#### Transaction Submission

##### `sendPayment`

Submit a payment transaction.

```graphql
mutation SendPayment(
  $input: SendPaymentInput!
  $signature: UserCommandSignature!
) {
  sendPayment(input: $input, signature: $signature) {
    payment {
      id
      hash
      amount
      fee
      from
      to
    }
  }
}
```

##### `sendDelegation`

Submit a delegation transaction.

```graphql
mutation SendDelegation(
  $input: SendDelegationInput!
  $signature: UserCommandSignature!
) {
  sendDelegation(input: $input, signature: $signature) {
    delegation {
      id
      hash
      delegator
      delegate
      fee
    }
  }
}
```

##### `sendZkapp`

Submit a zkApp transaction.

```graphql
mutation SendZkApp($input: SendZkAppInput!) {
  sendZkapp(input: $input) {
    zkapp {
      id
      hash
      zkappCommand {
        feePayer {
          body {
            publicKey
            fee
          }
        }
      }
    }
  }
}
```

## Implementation Details

The GraphQL API is implemented in the following source files:

### Core Implementation

- **Main module**:
  [`node/native/src/graphql/mod.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/mod.rs) -
  Root GraphQL schema and query/mutation implementations
- **HTTP routing**:
  [`node/native/src/http_server.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/http_server.rs) -
  HTTP server setup and GraphQL endpoint routing

### Type Implementations

- **Account types**:
  [`node/native/src/graphql/account.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/account.rs) -
  Account queries and balance information
- **Block types**:
  [`node/native/src/graphql/block.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/block.rs) -
  Block queries and blockchain data
- **Transaction types**:
  [`node/native/src/graphql/transaction.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/transaction.rs) -
  Transaction status and submission
- **User commands**:
  [`node/native/src/graphql/user_command.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/user_command.rs) -
  Payments and delegations
- **zkApp types**:
  [`node/native/src/graphql/zkapp.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/zkapp.rs) -
  zkApp transactions and smart contracts
- **SNARK types**:
  [`node/native/src/graphql/snark.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/snark.rs) -
  SNARK work and proof data
- **Constants**:
  [`node/native/src/graphql/constants.rs`](https://github.com/o1-labs/mina-rust/blob/develop/node/native/src/graphql/constants.rs) -
  Network constants and genesis parameters

## Common Patterns

### Error Handling

All GraphQL queries return either successful data or structured errors:

```json
{
  "data": null,
  "errors": [
    {
      "message": "Could not find block with height: `999999` in transition frontier",
      "locations": [{ "line": 2, "column": 3 }]
    }
  ]
}
```

### Pagination and Limits

Many queries accept `maxLength` or similar parameters:

```graphql
query {
  bestChain(maxLength: 100) # Get up to 100 recent blocks
}
```

### Optional Parameters

Most queries accept optional parameters for filtering:

```graphql
query {
  # Get all pooled commands
  pooledUserCommands

  # Get commands from specific sender
  pooledUserCommands(publicKey: "B62q...")

  # Get specific commands by hash
  pooledUserCommands(hashes: ["5Jt8..."])
}
```

## Example Queries

### Get Node Information

```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query NodeInfo {
      syncStatus
      networkID
      version
      daemonStatus {
        blockchainLength
        peers
        uptimeSecs
      }
    }"
  }'
```

### Get Recent Blockchain Activity

```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query RecentActivity {
      bestChain(maxLength: 5) {
        stateHash
        protocolState {
          consensusState {
            blockHeight
          }
        }
        transactions {
          userCommands {
            amount
            fee
            from
            to
          }
        }
      }
    }"
  }'
```

### Check Account Balance

```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query GetBalance($publicKey: String!) {
      account(publicKey: $publicKey) {
        balance {
          total
          liquid
          locked
        }
        nonce
        delegate
      }
    }",
    "variables": {
      "publicKey": "B62qp3B9VW1ir5qL1MWRwr6ecjC2NZbGr8vysGeme9vXGcFXTMNXb2t"
    }
  }'
```

## Development and Testing

## Schema Introspection

Get the complete schema information:

```graphql
query IntrospectionQuery {
  __schema {
    queryType {
      fields {
        name
        description
        args {
          name
          type {
            name
          }
        }
        type {
          name
        }
      }
    }
    mutationType {
      fields {
        name
        description
      }
    }
  }
}
```

## Rate Limiting and Performance

The GraphQL API shares the same resources as the node's internal operations.
Consider:

- **Complex queries**: Deep nested queries may impact performance
- **Large result sets**: Use appropriate `maxLength` parameters
- **Concurrent requests**: The API handles concurrent requests but intensive
  queries may affect node performance

## Next Steps

- [Node Architecture](./architecture) - Understanding the node's internal
  structure
- [Archive Database Queries](./archive-database-queries) - SQL queries and
  database analysis
- [Network Configuration](../node-operators/network-configuration) - Configuring
  your node for different networks
