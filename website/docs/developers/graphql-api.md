---
title: GraphQL API
description: Complete reference for Mina Rust GraphQL API endpoints and queries
sidebar_position: 6
---

import CodeBlock from "@theme/CodeBlock"; import Tabs from "@theme/Tabs"; import
TabItem from "@theme/TabItem"; import TestBasicConnectivity from
"!!raw-loader!./scripts/graphql-api/queries/curl/basic-connectivity.sh"; import
QuerySyncStatus from
"!!raw-loader!./scripts/graphql-api/queries/curl/sync-status.sh"; import
QueryNodeInfo from
"!!raw-loader!./scripts/graphql-api/queries/curl/node-info.sh"; import
QueryRecentActivity from
"!!raw-loader!./scripts/graphql-api/queries/curl/recent-activity.sh"; import
QueryAccountBalance from
"!!raw-loader!./scripts/graphql-api/queries/curl/account-balance.sh"; import
QueryBlock from "!!raw-loader!./scripts/graphql-api/queries/curl/block.sh";
import QueryGenesisBlock from
"!!raw-loader!./scripts/graphql-api/queries/curl/genesis-block.sh"; import
QueryGenesisConstants from
"!!raw-loader!./scripts/graphql-api/queries/curl/genesis-constants.sh"; import
QueryPooledUserCommands from
"!!raw-loader!./scripts/graphql-api/queries/curl/pooled-user-commands.sh";
import QueryPooledZkappCommands from
"!!raw-loader!./scripts/graphql-api/queries/curl/pooled-zkapp-commands.sh";
import QueryTransactionStatus from
"!!raw-loader!./scripts/graphql-api/queries/examples/transaction-status.sh";
import QuerySnarkPool from
"!!raw-loader!./scripts/graphql-api/queries/curl/snark-pool.sh"; import
QueryPendingSnarkWork from
"!!raw-loader!./scripts/graphql-api/queries/curl/pending-snark-work.sh"; import
QueryCurrentSnarkWorker from
"!!raw-loader!./scripts/graphql-api/queries/curl/current-snark-worker.sh";
import QueryDaemonStatus from
"!!raw-loader!./scripts/graphql-api/queries/curl/daemon-status.sh"; import
QueryNetworkID from
"!!raw-loader!./scripts/graphql-api/queries/curl/network-id.sh"; import
QueryVersion from "!!raw-loader!./scripts/graphql-api/queries/curl/version.sh";
import QueryBestChain from
"!!raw-loader!./scripts/graphql-api/queries/curl/best-chain.sh"; import
QueryAccount from "!!raw-loader!./scripts/graphql-api/queries/curl/account.sh";
import BasicConnectivityQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/basic-connectivity.graphql";
import SyncStatusQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/sync-status.graphql"; import
NodeInfoQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/node-info.graphql"; import
RecentActivityQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/recent-activity.graphql";
import AccountBalanceQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/account-balance.graphql";
import DaemonStatusQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/daemon-status.graphql"; import
NetworkIDQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/network-id.graphql"; import
VersionQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/version.graphql"; import
BestChainQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/best-chain.graphql"; import
BlockQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/block.graphql"; import
GenesisBlockQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/genesis-block.graphql"; import
GenesisConstantsQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/genesis-constants.graphql";
import AccountQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/account.graphql"; import
PooledUserCommandsQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/pooled-user-commands.graphql";
import PooledZkappCommandsQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/pooled-zkapp-commands.graphql";
import TransactionStatusQuery from
"!!raw-loader!./scripts/graphql-api/queries/examples/transaction-status.graphql";
import SnarkPoolQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/snark-pool.graphql"; import
PendingSnarkWorkQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/pending-snark-work.graphql";
import CurrentSnarkWorkerQuery from
"!!raw-loader!./scripts/graphql-api/queries/query/current-snark-worker.graphql";
import SendPaymentMutation from
"!!raw-loader!./scripts/graphql-api/mutations/query/send-payment.graphql";
import SendDelegationMutation from
"!!raw-loader!./scripts/graphql-api/mutations/query/send-delegation.graphql";
import SendZkappMutation from
"!!raw-loader!./scripts/graphql-api/mutations/query/send-zkapp.graphql"; import
MutationSendPayment from
"!!raw-loader!./scripts/graphql-api/mutations/curl/send-payment.sh"; import
MutationSendDelegation from
"!!raw-loader!./scripts/graphql-api/mutations/curl/send-delegation.sh"; import
MutationSendZkapp from
"!!raw-loader!./scripts/graphql-api/mutations/curl/send-zkapp.sh";

# GraphQL API Reference

The Mina Rust node provides a comprehensive GraphQL API for querying blockchain
data, account information, transaction status, and network statistics. The API
is built using [Juniper](https://github.com/graphql-rust/juniper) and is
available at `http://localhost:3000/graphql` when running a node.

You can also use one of the nodes deployed by o1Labs. See the
[Infrastructure](../node-operators/infrastructure/plain-nodes) section for
available nodes and connection details.

## Quick Start

### Testing the API

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/basic-connectivity.graphql"

> {BasicConnectivityQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/test-basic-connectivity.sh"

> {TestBasicConnectivity} </CodeBlock>

</TabItem>
</Tabs>

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/sync-status.graphql"

> {SyncStatusQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/sync-status.sh"

> {QuerySyncStatus} </CodeBlock>

</TabItem>
</Tabs>

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

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/daemon-status.graphql"

> {DaemonStatusQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/daemon-status.sh"

> {QueryDaemonStatus} </CodeBlock>

</TabItem>
</Tabs>

##### `networkID`

Get the network identifier.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/network-id.graphql"

> {NetworkIDQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/network-id.sh"

> {QueryNetworkID} </CodeBlock>

</TabItem>
</Tabs>

##### `version`

Get the node version (git commit hash).

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/version.graphql"

> {VersionQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/version.sh"

> {QueryVersion} </CodeBlock>

</TabItem>
</Tabs>

#### Blockchain Data

##### `bestChain(maxLength: Int!)`

Get the best chain blocks up to specified length.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/best-chain.graphql"

> {BestChainQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/best-chain.sh"

> {QueryBestChain} </CodeBlock>

</TabItem>
</Tabs>

##### `block(height: Int, stateHash: String)`

Get a specific block by height or state hash.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/block.graphql"

> {BlockQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/block.sh"

> {QueryBlock} </CodeBlock>

</TabItem>
</Tabs>

##### `genesisBlock`

Get the genesis block.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/genesis-block.graphql"

> {GenesisBlockQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/genesis-block.sh"

> {QueryGenesisBlock} </CodeBlock>

</TabItem>
</Tabs>

##### `genesisConstants`

Get genesis constants and network parameters.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/genesis-constants.graphql"

> {GenesisConstantsQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/genesis-constants.sh"

> {QueryGenesisConstants} </CodeBlock>

</TabItem>
</Tabs>

#### Account Information

##### `account(publicKey: String!, token: String)`

Get account information by public key.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/account.graphql"

> {AccountQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/account.sh"

> {QueryAccount} </CodeBlock>

</TabItem>
</Tabs>

#### Transaction Pool

##### `pooledUserCommands(publicKey: String, hashes: [String], ids: [String])`

Get pending user commands (payments/delegations) from the transaction pool.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/pooled-user-commands.graphql"

> {PooledUserCommandsQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/pooled-user-commands.sh"

> {QueryPooledUserCommands} </CodeBlock>

</TabItem>
</Tabs>

##### `pooledZkappCommands(publicKey: String, hashes: [String], ids: [String])`

Get pending zkApp commands from the transaction pool.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/pooled-zkapp-commands.graphql"

> {PooledZkappCommandsQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/pooled-zkapp-commands.sh"

> {QueryPooledZkappCommands} </CodeBlock>

</TabItem>
</Tabs>

#### Transaction Status

##### `transactionStatus(payment: String, zkappTransaction: String)`

Get the status of a specific transaction.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/examples/transaction-status.graphql"

> {TransactionStatusQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/examples/transaction-status.sh"

> {QueryTransactionStatus} </CodeBlock>

</TabItem>
</Tabs>

#### SNARK Work

##### `snarkPool`

Get completed SNARK work from the pool.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/snark-pool.graphql"

> {SnarkPoolQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/snark-pool.sh"

> {QuerySnarkPool} </CodeBlock>

</TabItem>
</Tabs>

##### `pendingSnarkWork`

Get pending SNARK work that needs to be completed.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/pending-snark-work.graphql"

> {PendingSnarkWorkQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/pending-snark-work.sh"

> {QueryPendingSnarkWork} </CodeBlock>

</TabItem>
</Tabs>

##### `currentSnarkWorker`

Get information about the currently configured SNARK worker.

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/current-snark-worker.graphql"

> {CurrentSnarkWorkerQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/current-snark-worker.sh"

> {QueryCurrentSnarkWorker} </CodeBlock>

</TabItem>
</Tabs>

### Mutation Endpoints

#### Transaction Submission

##### `sendPayment`

Submit a payment transaction.

<Tabs>
<TabItem value="graphql" label="GraphQL Mutation" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/mutations/query/send-payment.graphql"

> {SendPaymentMutation} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/mutations/curl/send-payment.sh"

> {MutationSendPayment} </CodeBlock>

</TabItem>
</Tabs>

##### `sendDelegation`

Submit a delegation transaction.

<Tabs>
<TabItem value="graphql" label="GraphQL Mutation" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/mutations/query/send-delegation.graphql"

> {SendDelegationMutation} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/mutations/curl/send-delegation.sh"

> {MutationSendDelegation} </CodeBlock>

</TabItem>
</Tabs>

##### `sendZkapp`

Submit a zkApp transaction.

<Tabs>
<TabItem value="graphql" label="GraphQL Mutation" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/mutations/query/send-zkapp.graphql"

> {SendZkappMutation} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/mutations/curl/send-zkapp.sh"

> {MutationSendZkapp} </CodeBlock>

</TabItem>
</Tabs>

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

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/node-info.graphql"

> {NodeInfoQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/node-info.sh"

> {QueryNodeInfo} </CodeBlock>

</TabItem>
</Tabs>

### Get Recent Blockchain Activity

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/recent-activity.graphql"

> {RecentActivityQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/recent-activity.sh"

> {QueryRecentActivity} </CodeBlock>

</TabItem>
</Tabs>

### Check Account Balance

<Tabs>
<TabItem value="graphql" label="GraphQL Query" default>

<CodeBlock language="graphql"
title="website/docs/developers/scripts/graphql-api/queries/query/account-balance.graphql"

> {AccountBalanceQuery} </CodeBlock>

</TabItem>
<TabItem value="curl" label="Curl Command">

<CodeBlock language="bash"
title="website/docs/developers/scripts/graphql-api/queries/curl/account-balance.sh"

> {QueryAccountBalance} </CodeBlock>

</TabItem>
</Tabs>

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
