---
title: Wallet operations
description: CLI wallet commands for managing accounts and sending transactions
sidebar_position: 15
---

# Wallet operations

The Mina CLI provides wallet functionality for sending transactions and managing
accounts. All wallet operations use encrypted key files and interact with a
running Mina node via GraphQL.

## Prerequisites

Before using wallet commands, you need:

- An encrypted private key file
- The password to decrypt the key
- A running Mina node (local or remote) - only required for sending transactions
  and checking balances

## Generate key pair

Generate a new encrypted key pair for use with Mina.

### Basic usage

```bash
mina wallet generate --output /path/to/key
```

### Arguments

**Required:**

- `--output <PATH>` - Path where the encrypted key file will be saved

**Optional:**

- `[PASSWORD]` - Password to encrypt the key. Can be provided as an argument or
  via the `MINA_PRIVKEY_PASS` environment variable (recommended for security)

### Example

```bash
# Generate new key with environment variable for password
export MINA_PRIVKEY_PASS="my-secret-password"
mina wallet generate --output ./keys/my-new-wallet
```

This command generates a new random keypair, encrypts the private key with the
provided password, and saves it to the specified path. It also creates a `.pub`
file containing the public key.

## Get address from key file

Extract the public address from an encrypted key file.

### Basic usage

```bash
mina wallet address --from /path/to/encrypted/key
```

### Arguments

**Required:**

- `--from <PATH>` - Path to encrypted key file

**Optional:**

- `[PASSWORD]` - Password to decrypt the key. Can be provided as an argument or
  via the `MINA_PRIVKEY_PASS` environment variable (recommended for security)

### Example

```bash
# Get address from encrypted key
mina wallet address --from ./keys/my-wallet

# Using environment variable for password
export MINA_PRIVKEY_PASS="my-secret-password"
mina wallet address --from ./keys/my-wallet
```

This command simply decrypts the key file and displays the associated public
address. It does not require a connection to a node.

## Check account balance

Query the balance of an account using GraphQL.

### Basic usage

```bash
# Check balance using key file
mina wallet balance --from /path/to/encrypted/key

# Check balance using public address
mina wallet balance --address <PUBLIC_KEY>
```

### Arguments

**Required (one of):**

- `--from <PATH>` - Path to encrypted key file
- `--address <PUBLIC_KEY>` - Public key to query directly

**Optional:**

- `[PASSWORD]` - Password to decrypt the key (only required when using
  `--from`). Can be provided as an argument or via the `MINA_PRIVKEY_PASS`
  environment variable (recommended for security)
- `--endpoint <URL>` - GraphQL endpoint URL (default:
  `http://localhost:3000/graphql`)
- `--format <FORMAT>` - Output format: `text` (default) or `json`

### Examples

#### Check balance using key file

```bash
export MINA_PRIVKEY_PASS="my-secret-password"
mina wallet balance --from ./keys/my-wallet
```

#### Check balance using public address

```bash
mina wallet balance \
  --address B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy
```

#### Check balance on remote node

```bash
mina wallet balance \
  --address B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --endpoint https://node.example.com:3000/graphql
```

#### Get balance in JSON format

```bash
mina wallet balance \
  --address B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --format json
```

### Output

The balance command displays:

- **Total balance** - Total amount of MINA in the account (both nanomina and
  MINA)
- **Liquid balance** - Amount available for spending
- **Locked balance** - Amount locked due to vesting schedule
- **Nonce** - Current account nonce
- **Delegate** - Public key of the delegate (if set)

#### Text format (default)

```
Account: B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy

Balance:
  Total:  1000.000000000 MINA
  Liquid: 800.000000000 MINA
  Locked: 200.000000000 MINA

Nonce: 5

Delegate: B62qkfHpLpELqpMK6ZvUTJ5wRqKDRF3UHyJ4Kv3FU79Sgs4qpBnx5RG
```

#### JSON format

```json
{
  "account": "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy",
  "balance": {
    "total": "1000000000000",
    "total_mina": "1000.000000000",
    "liquid": "800000000000",
    "liquid_mina": "800.000000000",
    "locked": "200000000000",
    "locked_mina": "200.000000000"
  },
  "nonce": "5",
  "delegate": "B62qkfHpLpELqpMK6ZvUTJ5wRqKDRF3UHyJ4Kv3FU79Sgs4qpBnx5RG"
}
```

The JSON format includes both nanomina (raw values) and formatted MINA values
for convenience.

## Send payment

Send a payment transaction to the network.

### Basic usage

```bash
mina wallet send \
  --from /path/to/encrypted/key \
  --to <receiver_public_key> \
  --amount <amount_in_nanomina> \
  --fee <fee_in_nanomina>
```

### Arguments

**Required:**

- `--from <PATH>` - Path to encrypted sender key file
- `--to <PUBLIC_KEY>` - Receiver's public key (Base58Check encoded)
- `--amount <AMOUNT>` - Amount in nanomina (1 MINA = 1,000,000,000 nanomina)
- `--fee <FEE>` - Transaction fee in nanomina

**Optional:**

- `[PASSWORD]` - Password to decrypt the sender key. Can be provided as an
  argument or via the `MINA_PRIVKEY_PASS` environment variable (recommended for
  security)
- `--memo <MEMO>` - Transaction memo (max 32 bytes, default: empty)
- `--nonce <NONCE>` - Transaction nonce (default: fetched from node)
- `--valid-until <SLOT>` - Slot until which transaction is valid (default: never
  expires)
- `--fee-payer <PUBLIC_KEY>` - Optional fee payer public key (default: sender
  pays)
- `--network <NETWORK>` - Network for signing: `mainnet` or `devnet` (default:
  `devnet`)
- `--node <URL>` - Node GraphQL endpoint (default: `http://localhost:3000`)

### Environment variables

You can set the following environment variables:

```bash
export MINA_PRIVKEY_PASS="your-password"
export MINA_NETWORK="mainnet"
```

### Examples

#### Send payment on devnet

```bash
mina wallet send \
  --from ./keys/my-wallet \
  --to B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --amount 1000000000 \
  --fee 10000000 \
  --network devnet
```

#### Send payment on mainnet with memo

```bash
mina wallet send \
  --from ./keys/my-wallet \
  --to B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --amount 5000000000 \
  --fee 10000000 \
  --memo "Payment for services" \
  --network mainnet
```

#### Send payment with separate fee payer

```bash
mina wallet send \
  --from ./keys/sender-wallet \
  --to B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --amount 1000000000 \
  --fee 10000000 \
  --fee-payer B62qkfHpLpELqpMK6ZvUTJ5wRqKDRF3UHyJ4Kv3FU79Sgs4qpBnx5RG
```

#### Send payment to remote node

```bash
mina wallet send \
  --from ./keys/my-wallet \
  --to B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --amount 1000000000 \
  --fee 10000000 \
  --node https://node.example.com:3000
```

#### Use environment variable for password

```bash
export MINA_PRIVKEY_PASS="my-secret-password"

mina wallet send \
  --from ./keys/my-wallet \
  --to B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --amount 1000000000 \
  --fee 10000000
```

## Understanding amounts

All amounts in the CLI are specified in **nanomina**:

- 1 MINA = 1,000,000,000 nanomina
- 0.1 MINA = 100,000,000 nanomina
- 0.01 MINA = 10,000,000 nanomina (common minimum fee)

## How it works

When you send a payment, the CLI:

1. **Decrypts your key** - Uses the provided password to decrypt your private
   key
2. **Fetches nonce** - Queries the node via GraphQL to get your current account
   nonce (if not specified)
3. **Creates payload** - Builds the payment transaction payload with all details
4. **Signs transaction** - Signs the transaction using your private key and the
   correct network ID
5. **Submits to node** - Sends the signed transaction to the node via GraphQL
   `sendPayment` mutation
6. **Returns hash** - Displays the transaction hash, which can be used with
   `mina wallet status` to track the transaction

## Network selection

The `--network` flag controls which network the transaction is signed for:

- `devnet` - For development and testing (default)
- `mainnet` - For production transactions

**Important:** Make sure to use the correct network flag. A transaction signed
for devnet will not be valid on mainnet and vice versa.

## Fee payer

By default, the sender pays the transaction fee. However, you can specify a
different fee payer using the `--fee-payer` option:

- The sender's key is used to sign the payment
- The fee payer's public key is included in the transaction
- The fee payer must also sign the transaction (currently requires manual
  coordination)

This is useful for sponsored transactions where another party pays the fees.

## Check transaction status

Check the status of a submitted transaction using its hash.

### Basic usage

```bash
mina wallet status --hash <TRANSACTION_HASH>
```

### Arguments

**Required:**

- `--hash <HASH>` - Transaction hash to check

**Optional:**

- `--node <URL>` - Node GraphQL endpoint (default: `http://localhost:3000`)
- `--check-mempool` - Force checking the mempool even if transaction is found in
  blockchain

### Examples

#### Check transaction on local node

```bash
mina wallet status \
  --hash 5Ju4H4DTE1zkwrnLrQ8vb2sZR19b7eSMiAVbb4wQh4bfhh4aQNew
```

#### Check transaction on remote node

```bash
mina wallet status \
  --hash 5Ju4H4DTE1zkwrnLrQ8vb2sZR19b7eSMiAVbb4wQh4bfhh4aQNew \
  --node https://devnet-plain-1.gcp.o1test.net
```

### Output

The status command will:

1. First attempt to query the blockchain for the transaction status
2. If not found in the blockchain, automatically check the mempool (pending
   transactions)
3. Display transaction details if found in the mempool

#### Transaction found in mempool

```
Checking transaction status...
Transaction hash: 5Ju4H4DTE1zkwrnLrQ8vb2sZR19b7eSMiAVbb4wQh4bfhh4aQNew

Transaction not found in blockchain, checking mempool...

✓ Transaction found in mempool!

Transaction Details:
  Hash:   5Ju4H4DTE1zkwrnLrQ8vb2sZR19b7eSMiAVbb4wQh4bfhh4aQNew
  From:   B62qjtpVAMr7knjLxRLU887QgT7GPk3JYCg8NGdZsfMuaykAJ9C2Rem
  To:     B62qjtpVAMr7knjLxRLU887QgT7GPk3JYCg8NGdZsfMuaykAJ9C2Rem
  Amount: 1000000000 nanomina
  Fee:    1000000 nanomina
  Nonce:  0

Status: PENDING (waiting to be included in a block)
```

#### Transaction not found

```
Checking transaction status...
Transaction hash: 5Ju6ku4DY5McpfqPvduQyQASjv1iAF12Xn75W3f3kGL1wsgSRKBA

Transaction not found in blockchain, checking mempool...

✗ Transaction not found in mempool

The transaction may have:
  - Already been included in a block
  - Been rejected by the network
  - Not yet propagated to this node
```

### How it works

The status command automatically:

1. **Queries blockchain** - Attempts to query `transactionStatus` via GraphQL
2. **Falls back to mempool** - If not found or if the query fails, checks
   `pooledUserCommands` for pending transactions
3. **Displays results** - Shows transaction details if found, or helpful
   messages if not found

This is particularly useful immediately after sending a transaction to verify it
has been accepted into the mempool.

## GraphQL integration

The wallet commands use the node's GraphQL API:

- **Account query** - Fetches current nonce and account information (`balance`
  command)
- **sendPayment mutation** - Submits signed transactions to the network (`send`
  command)
- **transactionStatus query** - Checks if a transaction is included in the
  blockchain (`status` command)
- **pooledUserCommands query** - Lists pending transactions in the mempool
  (`status` command)

For more details on the GraphQL API, see the [GraphQL API](./graphql-api.md)
documentation.
