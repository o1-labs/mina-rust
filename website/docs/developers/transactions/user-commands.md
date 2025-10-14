---
sidebar_position: 0
title: User commands
description: Understanding user-initiated transactions in the Mina protocol
slug: /developers/transactions/user-commands
---

# User commands

## Overview

User commands are transactions initiated and signed by users to interact with
the Mina blockchain. Unlike protocol transactions (fee transfers and coinbase),
user commands require user signatures and pay transaction fees.

The Mina protocol supports two categories of user commands:

- **Signed commands**: Simple transactions (payments and stake delegations)
- **zkApp commands**: Complex multi-account zero-knowledge operations

## Transaction structure

User commands are represented by the `UserCommand` enum:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L701-L704 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L701-L704
```

## Signed commands

Signed commands are straightforward transactions signed by a single user. The
structure includes a payload with common fields and a body containing either a
payment or stake delegation.

**Structure details:** See
[Payment transactions](./payments#transaction-structure) for the full type
definitions of `SignedCommand`, `SignedCommandPayload`, `Body`, and `Common`.

### Payment transactions

Transfer MINA tokens between accounts.

**Details:** [Payment transactions](./payments)

### Stake delegation

Delegate stake to block producers.

**Details:** [Stake delegation](./delegations)

## zkApp commands

zkApp commands enable complex multi-account operations with zero-knowledge proof
authorization:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2708-L2712 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2708-L2712
```

zkApp commands can:

- Update multiple accounts atomically
- Use zero-knowledge proofs for authorization
- Specify complex preconditions
- Emit events and actions
- Create and manage custom tokens

**Details:** [zkApp commands](./zkapps)

## Transaction fees

All user commands require transaction fees:

### Fee payment

Fees are paid by:

- **Signed commands**: The fee payer (usually the sender)
- **zkApp commands**: The fee payer specified in the command

### Fee distribution

Transaction fees are:

1. Deducted from the fee payer's balance
2. Collected by the block producer
3. Distributed via fee transfer transactions

### Fee requirements

Minimum fee requirements:

- Sufficient to incentivize block producers
- Cover account creation fees if creating new accounts
- Scale with transaction complexity (zkApp commands)

## Nonce management

User accounts maintain nonces to prevent replay attacks.

### Nonce rules

1. **Starts at zero**: New accounts have nonce 0
2. **Must match**: Transaction nonce must equal current account nonce
3. **Increments**: Successful transactions increment the nonce
4. **Sequential**: Nonces must be used in order

### Example

```rust
// Account nonce: 0
let tx1 = create_payment(&alice_pk, &bob_pk, amount, fee, 0); // Valid
let tx2 = create_payment(&alice_pk, &bob_pk, amount, fee, 1); // Valid after tx1
let tx3 = create_payment(&alice_pk, &bob_pk, amount, fee, 3); // Invalid - skips nonce 2
```

## Signatures

### Signing signed commands

Signed commands use a single signature. The signature covers:

- Transaction payload (common fields + body)
- Network identifier (mainnet/testnet)

### Signing zkApp commands

zkApp commands support multiple authorization types:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1769-L1777 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1769-L1777
```

Each account update can have different authorization.

## Valid until

Transactions specify an expiration slot in the `Common` struct.

### Expiration behavior

- **Before expiration**: Transaction can be included in blocks
- **After expiration**: Transaction becomes invalid
- **Set to max**: Transaction never expires (not recommended)

This prevents old transactions from being included indefinitely.

## Memos

Transactions can include optional 32-byte memos for auxiliary data.

### Use cases

- Payment references
- Application identifiers
- Arbitrary data
- Empty (all zeros)

Memos are included in transaction commitments and affect signatures.

## Transaction application

### Two-phase processing

User commands are applied in two phases:

1. **First pass**: Validates preconditions, applies fee payment, initiates state
   changes
2. **Second pass**: Verifies proofs (zkApps), finalizes state changes

### Status

After application, transactions have a status (see
[Transaction Status](../transactions#transaction-status)).

**Signed commands**: Either fully succeed or fully fail

**zkApp commands**: Can partially succeed (some account updates fail)

## Account creation

User commands can create new accounts:

### Automatic creation

- **Payments**: Create receiver account if it doesn't exist
- **zkApp commands**: Create accounts specified with
  `implicit_account_creation_fee`

### Account creation fee

Creating accounts costs 1 MINA (by default) as specified in constraint
constants. The fee is deducted from:

- **Payments**: The amount being transferred
- **zkApp commands**: The balance change of the created account

## Token support

### Default token (MINA)

All user commands support the default token (MINA), represented by `TokenId`.

### Custom tokens

Custom tokens are only supported via zkApp commands:

- Create new tokens
- Transfer custom tokens
- Manage token permissions

Signed commands (payments, delegations) only support the default token.

## Comparison

| Feature         | Signed commands  | zkApp commands            |
| --------------- | ---------------- | ------------------------- |
| Signature       | Single signature | Multiple authorizations   |
| Complexity      | Simple           | Complex                   |
| Account updates | 1-2 accounts     | Multiple accounts         |
| Authorization   | Signature only   | Signature, proof, or none |
| Preconditions   | Basic            | Advanced                  |
| Token support   | Default only     | Custom tokens             |
| Partial success | No               | Yes                       |

## Testing

User command tests are available in multiple files:

### Signed commands

- `tests/test_transaction_logic_first_pass.rs` - Payment tests
- `tests/test_transaction_logic_first_pass_delegation.rs` - Delegation tests

### zkApp commands

- `tests/test_transaction_logic_first_pass_zkapp.rs` - zkApp command tests
- `tests/test_zkapp.rs` - Additional zkApp tests

## Related files

- `ledger/src/scan_state/transaction_logic/mod.rs` - User command type
  definitions
- `ledger/src/scan_state/transaction_logic/signed_command.rs` - Signed command
  implementation
- `ledger/src/scan_state/transaction_logic/zkapp_command/` - zkApp command
  implementation
- `ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs` -
  Transaction application logic

## See also

- [Transactions overview](../transactions)
- [Payment transactions](./payments)
- [Stake delegation](./delegations)
- [zkApp commands](./zkapps)
- [Fee transfers](./fee-transfers)
- [Coinbase rewards](./coinbase)
