---
sidebar_position: 3
title: Fee transfers
description: Understanding fee transfer transactions in the Mina protocol
slug: /developers/transactions/fee-transfers
---

# Fee transfers

## Overview

Fee transfers are protocol-generated transactions that distribute collected
transaction fees to block producers. Unlike user commands, fee transfers are
created automatically during block production and do not require user
signatures.

A fee transfer consists of:

- **Receiver(s)**: One or two block producers receiving fees
- **Fee amount(s)**: The fee amount(s) to distribute
- **Token**: Must be the default token (MINA)

## Transaction structure

Fee transfers use a specialized structure that supports one or two receivers:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L357-L357 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L357-L357
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L330-L334 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L330-L334
```

### Creating fee transfers

Fee transfers are constructed using the `of_singles` method which validates
token compatibility:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L396-L412 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L396-L412
```

## Transaction application

### First pass

During the first pass (`apply_transaction_first_pass`), the fee transfer:

1. **Validates token compatibility**
   - All fees must be in the default token (MINA)
   - Rejects non-default tokens

2. **Processes single or double transfers**
   - For single transfers: adds fee to receiver's balance
   - For double transfers: adds respective fees to both receivers
   - Creates receiver accounts if they don't exist

3. **Deducts account creation fees**
   - If receiver account doesn't exist, charges account creation fee
   - Receiver gets: `fee_amount - account_creation_fee`

**Implementation:**
`ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs`

### No fee payment

Fee transfers do not pay fees themselves - they are protocol transactions:

- No nonce increment
- No fee payer
- No signature required
- Cannot be submitted by users

## Account creation

If a receiver account doesn't exist, it is created automatically. The account
creation fee (1 MINA by default) is deducted from the fee being transferred:

```rust
// Constraint constants
account_creation_fee: 1_000_000_000  // 1 MINA in nanomina
```

**Example:**

- Fee transfer of 2 MINA to a new account
- Receiver gets: `2 MINA - 1 MINA (account creation) = 1 MINA`

## Single vs. double transfers

### Single transfer

Distributes fees to one block producer:

```rust
let single = SingleFeeTransfer::create(receiver_pk, fee, TokenId::default());
let fee_transfer = FeeTransfer::of_singles(OneOrTwo::One(single))?;
```

### Double transfer

Distributes fees to two block producers (e.g., coinbase producer and snark work
producer):

```rust
let transfer1 = SingleFeeTransfer::create(receiver1_pk, fee1, TokenId::default());
let transfer2 = SingleFeeTransfer::create(receiver2_pk, fee2, TokenId::default());
let fee_transfer = FeeTransfer::of_singles(OneOrTwo::Two((transfer1, transfer2)))?;
```

## Token constraints

Fee transfers must use the default token:

```rust
if !fee_transfer.fee_tokens().all(TokenId::is_default) {
    return Err("Cannot pay fees in non-default tokens.");
}
```

For double transfers, both tokens must match:

```rust
if transfer1.fee_token != transfer2.fee_token {
    return Err("Cannot combine single fee transfers with incompatible tokens");
}
```

## Examples

### Single fee transfer

From the test suite
(`tests/test_transaction_logic_first_pass_fee_transfer.rs:76`):

```rust
// Transfer 0.01 MINA to Alice
let fee = Fee::from_u64(10_000_000);
let single_transfer = SingleFeeTransfer::create(
    alice_pk.clone(),
    fee,
    TokenId::default()
);
let fee_transfer = FeeTransfer::of_singles(OneOrTwo::One(single_transfer)).unwrap();

let result = apply_transaction_first_pass(
    constraint_constants,
    Slot::from_u32(0),
    &state_view,
    &mut ledger,
    &Transaction::FeeTransfer(fee_transfer),
);

assert!(result.is_ok());

// Verify state changes:
// - Alice's balance increased by 0.01 MINA
// - Alice's nonce unchanged (fee transfers don't affect nonces)
```

### Double fee transfer

From the test suite
(`tests/test_transaction_logic_first_pass_fee_transfer.rs:139`):

```rust
// Transfer 0.005 MINA to Alice and 0.007 MINA to Bob
let alice_fee = Fee::from_u64(5_000_000);
let bob_fee = Fee::from_u64(7_000_000);
let transfer1 = SingleFeeTransfer::create(alice_pk.clone(), alice_fee, TokenId::default());
let transfer2 = SingleFeeTransfer::create(bob_pk.clone(), bob_fee, TokenId::default());
let fee_transfer = FeeTransfer::of_singles(OneOrTwo::Two((transfer1, transfer2))).unwrap();

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_ok());

// Verify state changes:
// - Alice's balance increased by 0.005 MINA
// - Bob's balance increased by 0.007 MINA
// - Both nonces unchanged
```

### Fee transfer creating account

From the test suite
(`tests/test_transaction_logic_first_pass_fee_transfer.rs:237`):

```rust
// Transfer 2 MINA to Bob (who doesn't exist yet)
// Must be >= 1 MINA to cover account creation fee
let fee = Fee::from_u64(2_000_000_000);
let single_transfer = SingleFeeTransfer::create(bob_pk.clone(), fee, TokenId::default());
let fee_transfer = FeeTransfer::of_singles(OneOrTwo::One(single_transfer)).unwrap();

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_ok());

// Bob's account created with: 2 - 1 (account creation fee) = 1 MINA
```

### Incompatible tokens error

From the test suite
(`tests/test_transaction_logic_first_pass_fee_transfer.rs:310`):

```rust
// Create transfers with different tokens
let transfer1 = SingleFeeTransfer::create(alice_pk, alice_fee, TokenId::default());
let transfer2 = SingleFeeTransfer::create(bob_pk, bob_fee, TokenId::from(999999u64));

// Attempt to create fee transfer with incompatible tokens
let result = FeeTransfer::of_singles(OneOrTwo::Two((transfer1, transfer2)));

assert!(result.is_err());
assert!(result
    .unwrap_err()
    .contains("Cannot combine single fee transfers with incompatible tokens"));
```

## Block production workflow

### Fee collection

During block production:

1. Block producer collects fees from all transactions in the block
2. Fees are aggregated by recipient (block producer, snark workers)
3. Fee transfers are created to distribute the collected fees

### Coinbase interaction

Fee transfers often occur alongside coinbase transactions:

- Coinbase reward goes to block producer
- Fee transfers distribute transaction fees
- Both are applied in the same block

## Balance constraints

### Minimum for account creation

When creating accounts, fee amount must cover account creation fee:

```rust
if fee_amount < account_creation_fee {
    // Fee transfer may fail or burn the difference
}
```

### Receiver balance limits

The receiver's balance cannot overflow the maximum amount (2^64 nanomina).

## Testing

Comprehensive tests are available in
`tests/test_transaction_logic_first_pass_fee_transfer.rs`:

- `test_apply_single_fee_transfer_success` - Single fee transfer to existing
  account
- `test_apply_double_fee_transfer_success` - Double fee transfer to two
  receivers
- `test_apply_fee_transfer_creates_account` - Fee transfer creating new account
- `test_apply_fee_transfer_incompatible_tokens` - Token compatibility validation

## Related files

- `ledger/src/scan_state/transaction_logic/mod.rs` - Fee transfer type
  definitions
- `ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs` -
  Fee transfer application logic
- `tests/test_transaction_logic_first_pass_fee_transfer.rs` - Fee transfer tests

## See also

- [Transactions overview](../transactions)
- [Coinbase rewards](./coinbase)
- [Payment transactions](./payments)
