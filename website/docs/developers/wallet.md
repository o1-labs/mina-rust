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
- `--network <NETWORK>` - Network for signing: `mainnet` or `testnet` (default:
  `testnet`)
- `--node <URL>` - Node GraphQL endpoint (default: `http://localhost:3000`)

### Environment variables

You can set the following environment variables:

```bash
export MINA_PRIVKEY_PASS="your-password"
export MINA_NETWORK="mainnet"
```

### Examples

#### Send payment on testnet

```bash
mina wallet send \
  --from ./keys/my-wallet \
  --to B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy \
  --amount 1000000000 \
  --fee 10000000 \
  --network testnet
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
6. **Returns hash** - Displays the transaction hash for tracking

## Network selection

The `--network` flag controls which network the transaction is signed for:

- `testnet` - For development and testing (default)
- `mainnet` - For production transactions

**Important:** Make sure to use the correct network flag. A transaction signed
for testnet will not be valid on mainnet and vice versa.

## Fee payer

By default, the sender pays the transaction fee. However, you can specify a
different fee payer using the `--fee-payer` option:

- The sender's key is used to sign the payment
- The fee payer's public key is included in the transaction
- The fee payer must also sign the transaction (currently requires manual
  coordination)

This is useful for sponsored transactions where another party pays the fees.

## Transaction status

After submitting a transaction, you'll receive a transaction hash. You can check
its status by querying the node's GraphQL endpoint:

```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { transactionStatus(payment: \"<base64_transaction>\") }"
  }'
```

## GraphQL integration

The wallet commands use the node's GraphQL API:

- **Account query** - Fetches current nonce and account information
- **sendPayment mutation** - Submits signed transactions to the network

For more details on the GraphQL API, see the [GraphQL API](./graphql-api.md)
documentation.
