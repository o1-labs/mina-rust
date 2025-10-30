---
sidebar_position: 5
title: zkApp commands
description: Understanding zkApp command transactions in the Mina protocol
slug: /developers/transactions/zkapps
---

# zkApp commands

## Overview

zkApp commands are transactions that interact with zkApps (zero-knowledge
applications) - smart contracts on the Mina blockchain that leverage
zero-knowledge proofs for private, verifiable computation. Unlike traditional
smart contracts that execute on-chain, zkApp logic executes off-chain and
produces a zero-knowledge proof that the computation was performed correctly.

A zkApp command consists of:

- **Fee payer**: The account paying transaction fees
- **Account updates**: A forest of account modifications
- **Memo**: Optional 32-byte data field

## What is a zkApp?

A zkApp is a smart contract on Mina that:

- **Stores state**: 8 field elements of app state per account
- **Executes off-chain**: Logic verified by zero-knowledge proofs
- **Multi-account**: Can interact with multiple accounts atomically
- **Flexible permissions**: Custom authorization rules and verification keys

This architecture keeps the blockchain lightweight by only verifying proofs
on-chain, not executing the full computation.

## Transaction structure

zkApp commands have a complex structure supporting multiple account updates:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2708-L2712 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2708-L2712
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2701-L2704 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2701-L2704
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2692-L2697 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2692-L2697
```

### Account updates

Each account update specifies modifications to an account:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1881-L1887 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1881-L1887
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1573-L1587 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1573-L1587
```

### Call forest structure

Account updates are organized in a tree structure (forest):

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2305-L2305 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2305-L2305
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2262-L2266 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L2262-L2266
```

This allows nested account update calls, similar to contract call chains.

## Authorization methods

Account updates can be authorized in different ways:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1769-L1777 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1769-L1777
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1509-L1513 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1509-L1513
```

- **Proof**: Zero-knowledge proof of correct execution
- **Signature**: Traditional signature authorization
- **NoneGiven**: No authorization (for accounts with no permissions)

## Transaction application

### First pass

During the first pass (`apply_transaction_first_pass`), the zkApp command:

1. **Validates the fee payer**
   - Checks account exists
   - Verifies sufficient balance for fee
   - Validates nonce matches

2. **Applies the fee payment**
   - Deducts fee from fee payer's balance
   - Increments fee payer's nonce

3. **Processes account updates**
   - Validates preconditions for each update
   - Applies balance changes
   - Updates account state
   - Verifies authorizations (signatures/proofs)
   - Handles account creation fees

4. **Collects failures**
   - zkApp commands can partially succeed
   - Failed account updates are collected
   - Transaction marked as failed if any updates fail

**Implementation:**
`ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs`

### Second pass

The second pass verifies zero-knowledge proofs and finalizes state changes.

**Implementation:**
`ledger/src/scan_state/transaction_logic/transaction_applied.rs`

## Preconditions

Account updates can specify preconditions that must be satisfied. The
`Preconditions` struct contains network conditions, account conditions, and
validity windows.

### Network preconditions

Conditions on network state:

- Snarked ledger hash
- Blockchain length
- Global slot
- Staking epoch data

### Account preconditions

Conditions on account state:

- Balance constraints
- Nonce requirements
- State field values
- Delegate settings

## Balance changes

Account updates specify balance changes with sign (positive for addition,
negative for subtraction). Balance changes must sum to zero across all account
updates (conservation of funds).

## Account creation

zkApp commands can create accounts. When the `implicit_account_creation_fee`
field in the account update body is true:

- Account creation fee is automatically deducted
- New account is created with specified state
- Balance changes account for creation fee

## Examples

### Simple zkApp command

From the test suite (`tests/test_transaction_logic_first_pass_zkapp.rs:169`):

```rust
// Create zkApp command with Alice as fee payer
// and Bob's account as the update target (no balance change)
let fee = 10_000_000;  // 0.01 MINA
let nonce = 0;
let zkapp_command = create_simple_zkapp_command(&alice_pk, &bob_pk, fee, nonce);

let result = apply_transaction_first_pass(
    constraint_constants,
    Slot::from_u32(0),
    &state_view,
    &mut ledger,
    &Transaction::Command(UserCommand::ZkAppCommand(Box::new(zkapp_command))),
);

assert!(result.is_ok());

// Verify state changes:
// - Alice's balance decreased by fee
// - Alice's nonce incremented
// - Bob's account unchanged (no balance change in this example)
```

### Insufficient balance

From the test suite (`tests/test_transaction_logic_first_pass_zkapp.rs:243`):

```rust
// Alice has 0.001 MINA but fee is 0.01 MINA
let zkapp_command = create_simple_zkapp_command(&alice_pk, &bob_pk, 10_000_000, 0);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(result.unwrap_err(), "[[Overflow]]");

// Ledger state unchanged
```

### Invalid nonce

From the test suite (`tests/test_transaction_logic_first_pass_zkapp.rs:322`):

```rust
// Alice's nonce is 0, but transaction specifies nonce 5
let zkapp_command = create_simple_zkapp_command(&alice_pk, &bob_pk, 10_000_000, 5);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(result.unwrap_err(), "[[AccountNoncePreconditionUnsatisfied]]");

// Ledger state unchanged
```

### Nonexistent fee payer

From the test suite (`tests/test_transaction_logic_first_pass_zkapp.rs:392`):

```rust
// Alice's account doesn't exist
let zkapp_command = create_simple_zkapp_command(&alice_pk, &bob_pk, 10_000_000, 0);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(
    result.unwrap_err(),
    "[[Overflow, AmountInsufficientToCreateAccount]]"
);

// No account created
```

## Error handling

zkApp commands use a list-based error format:

```rust
"[[ErrorType1, ErrorType2, ...]]"
```

Common error types:

- **Overflow**: Balance overflow or underflow
- **AccountNoncePreconditionUnsatisfied**: Nonce mismatch
- **AmountInsufficientToCreateAccount**: Cannot afford account creation
- **PreconditionUnsatisfied**: Other precondition failures

## Transaction commitments

zkApp commands use two types of commitments for signing:

### Partial commitment

Hash of account updates only (used for account update signatures):

```rust
pub fn compute_account_update_digest(account_update: &AccountUpdate) -> Fp {
    // Hash account update fields
}
```

### Full commitment

Includes memo and fee payer (used for fee payer signature):

```rust
pub fn compute_full_commitment(zkapp_command: &ZkAppCommand) -> Fp {
    // Hash fee payer + account updates + memo
}
```

Account updates specify which commitment to use via `use_full_commitment`.

## State updates

zkApp accounts can store up to 8 field elements of app state. The `Update`
struct allows updating app state, delegate, verification key, permissions, zkApp
URI, token symbol, timing, and voting fields. Each field can be updated
independently through account updates.

## Permissions

zkApp accounts have granular permissions controlling which operations require
which authorizations. The `Permissions` struct specifies permission levels
(None, Either, Proof, Signature, or Impossible) for operations like editing
state, sending/receiving funds, setting delegate, modifying permissions,
updating verification keys, and managing zkApp metadata.

## Events and actions

zkApp commands can emit events and actions. Events are emitted for off-chain
indexing, while actions are used for reducer pattern state management. Both are
represented as vectors of field elements.

## Token support

zkApp commands support custom tokens:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1813-L1823 -->

```rust reference title="ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/zkapp_command/mod.rs#L1813-L1823
```

This enables:

- Creating new tokens
- Transferring custom tokens
- Token permission management

## Testing

Comprehensive tests are available in
`tests/test_transaction_logic_first_pass_zkapp.rs`:

- `test_apply_zkapp_command_success` - Successful zkApp command
- `test_apply_zkapp_command_insufficient_balance` - Insufficient fee payer
  balance
- `test_apply_zkapp_command_invalid_nonce` - Nonce mismatch error
- `test_apply_zkapp_command_nonexistent_fee_payer` - Nonexistent fee payer error

## Related files

- `ledger/src/scan_state/transaction_logic/zkapp_command/` - zkApp command
  implementation
- `ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs` -
  zkApp command application
- `ledger/src/generators/zkapp_command_builder.rs` - zkApp command construction
- `tests/test_transaction_logic_first_pass_zkapp.rs` - zkApp command tests
- `tests/test_zkapp.rs` - Additional zkApp tests

## Further reading

For detailed zkApp development:

- [o1js Documentation](https://docs.minaprotocol.com/zkapps) - High-level zkApp
  development
- [zkApp Protocol Specification](https://docs.minaprotocol.com/zkapps/advanced)
  - Detailed protocol specification
- [Zero-Knowledge Proofs](https://docs.minaprotocol.com/zkapps/tutorials) -
  Understanding ZK proofs

## See also

- [Transactions overview](../transactions)
- [Payment transactions](./payments)
