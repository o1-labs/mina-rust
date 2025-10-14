---
sidebar_position: 1
title: Payment transactions
description: Understanding payment transactions in the Mina protocol
slug: /developers/transactions/payments
---

# Payment transactions

## Overview

Payment transactions transfer MINA tokens from one account to another. They are
the most common type of user-initiated transaction in the Mina blockchain.

A payment consists of:

- **Sender**: The account sending tokens (pays the fee)
- **Receiver**: The account receiving tokens
- **Amount**: The quantity of tokens to transfer
- **Fee**: The transaction fee paid to block producers
- **Nonce**: The sender's transaction counter (prevents replay attacks)

## Transaction structure

Payment transactions are a type of `SignedCommand` with a `Payment` body:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L152-L158 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L152-L158
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L95-L100 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L95-L100
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L82-L87 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L82-L87
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L36-L41 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L36-L41
```

### Common fields

The `Common` struct contains fields shared by all signed commands:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L21-L32 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L21-L32
```

## Transaction application

### First pass

During the first pass (`apply_transaction_first_pass`), the payment transaction:

1. **Validates the fee payer account**
   - Checks account exists
   - Verifies sufficient balance for fee
   - Validates nonce matches

2. **Applies the fee payment**
   - Deducts fee from sender's balance
   - Increments sender's nonce
   - Updates receipt chain hash

3. **Applies the payment**
   - Deducts amount from sender's balance
   - Adds amount to receiver's balance
   - Creates receiver account if it doesn't exist (charges account creation fee)

**Implementation:**
`ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs`

### Second pass

The second pass finalizes the transaction after SNARK proof verification.

**Implementation:**
`ledger/src/scan_state/transaction_logic/transaction_applied.rs`

## Account creation

If the receiver account doesn't exist, a new account is created automatically.
The account creation fee (1 MINA by default) is deducted from the amount being
transferred:

```rust
// Constraint constants
account_creation_fee: 1_000_000_000  // 1 MINA in nanomina
```

**Example:**

- Sender sends 10 MINA to a new account with 1 MINA fee
- Sender pays: `10 MINA + 1 MINA (fee) + 1 MINA (account creation) = 12 MINA`
- Receiver gets: `10 MINA - 1 MINA (account creation) = 9 MINA`

## Balance constraints

### Sufficient funds for payment

The sender must have sufficient balance to cover:

```
sender_balance >= amount + fee + (account_creation_fee if creating account)
```

If insufficient, the transaction fails with error: `"insufficient funds"`

### Receiver balance limits

The receiver's balance cannot overflow the maximum amount (2^64 nanomina).

## Examples

### Successful payment

From the test suite (`tests/test_transaction_logic_first_pass.rs:76`):

```rust
// Create sender account with 10 MINA
let alice_id = AccountId::new(alice_pk, TokenId::default());
let alice_account = Account::create_with(alice_id.clone(), Balance::from_u64(10_000_000_000));

// Create receiver account with 5 MINA
let bob_id = AccountId::new(bob_pk, TokenId::default());
let bob_account = Account::create_with(bob_id.clone(), Balance::from_u64(5_000_000_000));

// Create payment: 3 MINA from Alice to Bob with 0.1 MINA fee
let payment = create_payment(&alice_pk, &bob_pk, 3_000_000_000, 100_000_000, 0);

// Apply transaction
let result = apply_transaction_first_pass(
    constraint_constants,
    Slot::from_u32(0),
    &state_view,
    &mut ledger,
    &Transaction::Command(UserCommand::SignedCommand(Box::new(payment))),
);

assert!(result.is_ok());

// Verify balances:
// Alice: 10 - 3 - 0.1 = 6.9 MINA
// Bob: 5 + 3 = 8 MINA
```

### Payment creating new account

From the test suite (`tests/test_transaction_logic_first_pass.rs:186`):

```rust
// Send 5 MINA to Bob (who doesn't exist yet) with 0.1 MINA fee
let payment = create_payment(&alice_pk, &bob_pk, 5_000_000_000, 100_000_000, 0);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_ok());

// Bob's account is created with: 5 - 1 (account creation fee) = 4 MINA
```

### Insufficient balance

From the test suite (`tests/test_transaction_logic_first_pass.rs:249`):

```rust
// Alice has 1 MINA but tries to send 5 MINA
let payment = create_payment(&alice_pk, &bob_pk, 5_000_000_000, 100_000_000, 0);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(result.unwrap_err(), "insufficient funds");

// Ledger remains unchanged - no fee charged
```

### Invalid nonce

From the test suite (`tests/test_transaction_logic_first_pass.rs:292`):

```rust
// Alice's nonce is 0, but transaction specifies nonce 5
let payment = create_payment(&alice_pk, &bob_pk, 3_000_000_000, 100_000_000, 5);

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_err());
assert_eq!(
    result.unwrap_err(),
    "Nonce in account Nonce(0) different from nonce in transaction Nonce(5)"
);

// Ledger remains unchanged
```

## Fee calculation

Users should calculate required funds as:

```rust
let required_balance = if receiver_exists {
    amount + fee
} else {
    amount + fee + account_creation_fee
};
```

Block producers receive fees through fee transfer transactions that are
automatically generated during block production.

## Token support

While the payment structure supports custom tokens via `TokenId`, currently only
the default token (MINA) can be used for fee payments. Custom token payments
must use zkApp commands.

## Testing

Comprehensive tests are available in
`tests/test_transaction_logic_first_pass.rs`:

- `test_apply_payment_success` - Successful payment between existing accounts
- `test_apply_payment_creates_account` - Payment creating a new account
- `test_apply_payment_insufficient_balance` - Insufficient funds error
- `test_apply_payment_invalid_nonce` - Nonce mismatch error
- `test_apply_payment_nonexistent_source` - Nonexistent sender error
- `test_apply_payment_receiver_overflow` - Receiver balance overflow error

## Related files

- `ledger/src/scan_state/transaction_logic/signed_command.rs` - Payment type
  definitions
- `ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs` -
  Payment application logic
- `tests/test_transaction_logic_first_pass.rs` - Payment transaction tests

## See also

- [Transactions overview](../transactions)
- [Stake delegation](./delegations)
- [Fee transfers](./fee-transfers)
