---
sidebar_position: 2
title: Stake delegation
description: Understanding stake delegation transactions in the Mina protocol
slug: /developers/transactions/delegations
---

# Stake delegation

## Overview

Stake delegation transactions allow token holders to delegate their stake to
block producers without transferring token ownership. This enables users to
participate in consensus and earn staking rewards while maintaining control of
their funds.

A delegation consists of:

- **Delegator**: The account delegating stake (pays the fee)
- **Delegate**: The block producer receiving the delegation
- **Fee**: The transaction fee paid to block producers
- **Nonce**: The delegator's transaction counter

## Transaction structure

Delegation transactions are a type of `SignedCommand` with a `StakeDelegation`
body:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L152-L158 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L152-L158
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L82-L87 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L82-L87
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L49-L55 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L49-L55
```

### Common fields

Like payments, delegations use the `Common` struct for fee, nonce, and other
shared fields (see Payment transactions for details).

## Transaction application

### First pass

During the first pass (`apply_transaction_first_pass`), the delegation
transaction:

1. **Validates the fee payer account**
   - Checks account exists
   - Verifies sufficient balance for fee
   - Validates nonce matches

2. **Applies the fee payment**
   - Deducts fee from delegator's balance
   - Increments delegator's nonce
   - Updates receipt chain hash

3. **Sets the delegate**
   - Updates the delegator's `delegate` field
   - No funds are transferred
   - Delegate account doesn't need to exist

**Implementation:**
`ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs`

### Second pass

The second pass finalizes the transaction after SNARK proof verification.

**Implementation:**
`ledger/src/scan_state/transaction_logic/transaction_applied.rs`

## Key characteristics

### No fund transfer

Unlike payments, delegations do not transfer any tokens. Only the delegation
pointer is updated:

- Delegator retains full ownership of their tokens
- Delegator can spend tokens freely
- Delegator can change delegation at any time (after nonce increment)

### Delegate account

The delegate account:

- Does not need to exist at delegation time
- Does not receive any notification of the delegation
- Cannot reject the delegation
- Receives staking weight from all delegators

### Balance requirements

The delegator must only have sufficient balance for the fee:

```
delegator_balance >= fee
```

No account creation fee is charged since no new accounts are created.

## Staking weight

After delegation, the delegate's staking weight for consensus includes:

- The delegate's own balance
- All balances delegated to them

This affects their probability of winning block production slots.

## Examples

### Successful delegation

From the test suite
(`tests/test_transaction_logic_first_pass_delegation.rs:114`):

```rust
// Alice delegates to Bob with 0.01 MINA fee
let delegation = create_delegation(&alice_pk, &bob_pk, 10_000_000, 0);

let result = apply_transaction_first_pass(
    constraint_constants,
    Slot::from_u32(0),
    &state_view,
    &mut ledger,
    &Transaction::Command(UserCommand::SignedCommand(Box::new(delegation))),
);

assert!(result.is_ok());

// Verify state changes:
// - Alice's balance decreased by fee only (no amount transferred)
// - Alice's nonce incremented
// - Alice's delegate set to Bob
// - Bob's balance unchanged
```

### Insufficient balance for fee

From the test suite
(`tests/test_transaction_logic_first_pass_delegation.rs:228`):

```rust
// Alice has 0.001 MINA but fee is 0.01 MINA
let delegation = create_delegation(&alice_pk, &bob_pk, 10_000_000, 0);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(result.unwrap_err(), "insufficient funds");

// Ledger state unchanged:
// - No fee charged
// - Nonce not incremented
// - Delegate not changed
```

### Invalid nonce

From the test suite
(`tests/test_transaction_logic_first_pass_delegation.rs:312`):

```rust
// Alice's nonce is 0, but transaction specifies nonce 5
let delegation = create_delegation(&alice_pk, &bob_pk, 10_000_000, 5);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(
    result.unwrap_err(),
    "Nonce in account Nonce(0) different from nonce in transaction Nonce(5)"
);

// Ledger state unchanged
```

### Nonexistent fee payer

From the test suite
(`tests/test_transaction_logic_first_pass_delegation.rs:387`):

```rust
// Alice's account doesn't exist
let delegation = create_delegation(&alice_pk, &bob_pk, 10_000_000, 0);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(result.unwrap_err(), "The fee-payer account does not exist");

// No account created
```

## Delegation lifecycle

### Initial state

By default, accounts delegate to themselves:

```rust
pub struct Account {
    pub delegate: Option<CompressedPubKey>,
    // When None, delegates to self (public_key)
}
```

### Changing delegation

Users can change their delegation by submitting a new delegation transaction:

1. Verify balance covers new transaction fee
2. Wait for previous transaction to be confirmed (nonce must increment)
3. Submit new delegation with incremented nonce
4. New delegation takes effect immediately upon inclusion

### Removing delegation

To remove delegation (return to self-delegation), delegate to your own public
key:

```rust
let delegation = create_delegation(&alice_pk, &alice_pk, fee, nonce);
```

## Consensus implications

### Epoch boundaries

Delegations affect staking weight at epoch boundaries:

- **Staking epoch**: Determines who can produce blocks
- **Next epoch**: Determines delegation snapshots for future epochs

The exact timing depends on the current slot and epoch progression.

### Snarked ledger

Delegations use the snarked ledger state for computing staking distributions.
Recent delegations may not affect current epoch block production.

## Fee calculation

Users should ensure sufficient balance:

```rust
let required_balance = fee;

if delegator_balance < required_balance {
    return Err("insufficient funds");
}
```

## Testing

Comprehensive tests are available in
`tests/test_transaction_logic_first_pass_delegation.rs`:

- `test_apply_delegation_success` - Successful delegation to block producer
- `test_apply_delegation_insufficient_balance` - Insufficient funds for fee
- `test_apply_delegation_invalid_nonce` - Nonce mismatch error
- `test_apply_delegation_nonexistent_fee_payer` - Nonexistent delegator error

## Related files

- `ledger/src/scan_state/transaction_logic/signed_command.rs` - Delegation type
  definitions
- `ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs` -
  Delegation application logic
- `tests/test_transaction_logic_first_pass_delegation.rs` - Delegation tests

## See also

- [Transactions overview](../transactions)
- [Payment transactions](./payments)
