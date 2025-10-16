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
   - Checks account exists (transaction rejected if it doesn't)
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

### Fee payer account must exist

The fee payer (sender) account **must already exist** in the ledger. If it
doesn't exist, the transaction is rejected with error "The fee-payer account
does not exist".

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L1341-L1343 -->

```rust reference title="ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L1341-L1343
```

### Receiver account creation

If the receiver account doesn't exist, a new account is created automatically.
The account creation fee (1 MINA by default) is deducted from the amount being
transferred to the receiver:

```rust
// Constraint constants
account_creation_fee: 1_000_000_000  // 1 MINA in nanomina
```

**Example:**

- Sender sends 10 MINA to a new account with 1 MINA fee
- Sender pays: `10 MINA + 1 MINA (fee) = 11 MINA`
- Receiver gets: `10 MINA - 1 MINA (account creation) = 9 MINA`

## Token support

While the payment structure supports custom tokens via `TokenId`, currently only
the default token (MINA) can be used for fee payments. Custom token payments
must use zkApp commands.

## Testing

Comprehensive tests are available in
`tests/test_transaction_logic_first_pass.rs`:

- `test_apply_payment_success` - Successful payment between existing accounts
- `test_apply_payment_creates_receiver_account` - Payment creating new receiver
  account
- `test_apply_payment_insufficient_balance` - Insufficient funds for payment
  (fee charged)
- `test_apply_payment_invalid_nonce` - Nonce mismatch error
- `test_apply_payment_nonexistent_fee_payer` - Nonexistent sender error
  (transaction rejected)

## Related files

- `ledger/src/scan_state/transaction_logic/signed_command.rs` - Payment type
  definitions
- `ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs` -
  Payment application logic
- `tests/test_transaction_logic_first_pass.rs` - Payment transaction tests
