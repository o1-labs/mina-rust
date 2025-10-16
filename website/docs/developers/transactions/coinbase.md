---
sidebar_position: 4
title: Coinbase rewards
description: Understanding coinbase reward transactions in the Mina protocol
slug: /developers/transactions/coinbase
---

# Coinbase rewards

## Overview

Coinbase transactions are protocol-generated rewards issued to block producers
for successfully producing a block. They represent new MINA tokens entering
circulation as compensation for maintaining the network.

A coinbase consists of:

- **Receiver**: The block producer receiving the reward
- **Amount**: The coinbase reward amount (720 MINA by default)
- **Fee transfer**: Optional fee transfer to SNARK worker

## Transaction structure

Coinbase transactions have a simple structure with an optional fee transfer:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L438-L442 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L438-L442
```

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L416-L419 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L416-L419
```

### Creating coinbase

Coinbase transactions are constructed with validation logic:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L452-L473 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L452-L473
```

## Transaction application

### First pass

During the first pass
([`apply_transaction_first_pass`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L190)),
the coinbase:

1. **Applies coinbase reward**
   - Adds coinbase amount to receiver's balance
   - Creates receiver account if it doesn't exist
   - Deducts account creation fee if creating account

2. **Applies fee transfer (if present)**
   - Adds fee to fee transfer receiver's balance
   - Deducts fee from coinbase receiver's reward
   - Creates fee transfer receiver account if needed

**Implementation:**
[`ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs)

### No fee payment

Like fee transfers, coinbase transactions do not pay fees:

- No nonce increment
- No fee payer
- No signature required
- Cannot be submitted by users

## Coinbase amount

The coinbase amount is configured in constraint constants:

```rust
pub struct ConstraintConstants {
    pub coinbase_amount: u64,  // 720_000_000_000 (720 MINA)
    pub supercharged_coinbase_factor: u32,  // 2x for supercharged rewards
    // ...
}
```

### Supercharged coinbase

Block producers who include SNARK work may receive a supercharged coinbase (2x
the base amount). This incentivizes SNARK work production and inclusion.

## Fee transfer interaction

### Splitting the reward

When a coinbase includes a fee transfer, the reward is split:

- **Coinbase receiver**: Gets `coinbase_amount - fee_transfer_amount`
- **Fee transfer receiver**: Gets `fee_transfer_amount`

This mechanism allows block producers to share rewards with SNARK workers.

### Same receiver optimization

If the fee transfer receiver equals the coinbase receiver, the fee transfer is
removed:

```rust
// Fee transfer to self is removed
let coinbase = Coinbase::create(
    amount,
    alice_pk.clone(),
    Some(CoinbaseFeeTransfer::create(alice_pk.clone(), fee))
).unwrap();

assert!(coinbase.fee_transfer.is_none());
```

This prevents unnecessary complexity when the reward would go to the same
account.

## Account creation

If receiver accounts don't exist, they are created automatically:

### Coinbase receiver creation

```rust
// Coinbase receiver gets: coinbase_amount - account_creation_fee
let expected_balance = Balance::from_u64(
    coinbase_amount.as_u64().saturating_sub(account_creation_fee)
);
```

### Fee transfer receiver creation

The fee transfer receiver follows standard account creation rules (fee transfer
must be sufficient to cover the account creation fee).

## Examples

### Coinbase without fee transfer

From the test suite (`tests/test_transaction_logic_first_pass_coinbase.rs:76`):

```rust
// Create coinbase of 720 MINA to Alice with no fee transfer
let coinbase_amount = Amount::from_u64(720_000_000_000);
let coinbase = Coinbase::create(coinbase_amount, alice_pk.clone(), None).unwrap();

let result = apply_transaction_first_pass(
    constraint_constants,
    Slot::from_u32(0),
    &state_view,
    &mut ledger,
    &Transaction::Coinbase(coinbase),
);

assert!(result.is_ok());

// Verify state changes:
// - Alice's balance increased by 720 MINA
// - Alice's nonce unchanged (coinbase doesn't affect nonces)
```

### Coinbase with fee transfer

From the test suite (`tests/test_transaction_logic_first_pass_coinbase.rs:135`):

```rust
// Create coinbase of 720 MINA to Alice with 10 MINA fee transfer to Bob
let coinbase_amount = Amount::from_u64(720_000_000_000);
let fee_transfer_amount = Fee::from_u64(10_000_000_000);
let fee_transfer = CoinbaseFeeTransfer::create(bob_pk.clone(), fee_transfer_amount);
let coinbase = Coinbase::create(coinbase_amount, alice_pk.clone(), Some(fee_transfer)).unwrap();

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_ok());

// Verify state changes:
// - Alice's balance increased by: 720 - 10 = 710 MINA
// - Bob's balance increased by: 10 MINA
// - Both nonces unchanged
```

### Coinbase with fee transfer to same account

From the test suite (`tests/test_transaction_logic_first_pass_coinbase.rs:231`):

```rust
// Create coinbase to Alice with fee transfer also to Alice
let coinbase_amount = Amount::from_u64(720_000_000_000);
let fee_transfer_amount = Fee::from_u64(10_000_000_000);
let fee_transfer = CoinbaseFeeTransfer::create(alice_pk.clone(), fee_transfer_amount);
let coinbase = Coinbase::create(coinbase_amount, alice_pk.clone(), Some(fee_transfer)).unwrap();

// Fee transfer is removed during creation
assert!(coinbase.fee_transfer.is_none());

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_ok());

// Alice receives only the coinbase amount (not coinbase + fee transfer)
// - Alice's balance increased by: 720 MINA
```

### Coinbase creating account

From the test suite (`tests/test_transaction_logic_first_pass_coinbase.rs:306`):

```rust
// Create coinbase of 720 MINA to Bob (who doesn't exist yet)
let coinbase_amount = Amount::from_u64(720_000_000_000);
let coinbase = Coinbase::create(coinbase_amount, bob_pk.clone(), None).unwrap();

let result = apply_transaction_first_pass(/* ... */);

assert!(result.is_ok());

// Bob's account created with: 720 - 1 (account creation fee) = 719 MINA
```

## Block production workflow

### Coinbase generation

During block production:

1. Block producer successfully produces a block
2. Coinbase transaction is created with the configured amount
3. Optional fee transfer is added for SNARK workers
4. Coinbase is applied as the first transaction in the block

### SNARK work incentive

The fee transfer mechanism incentivizes SNARK work:

- Block producers can allocate fees to SNARK workers
- Workers receive compensation for proof generation
- Creates a marketplace for SNARK work

## Supercharged coinbase

Blocks that include sufficient SNARK work may receive supercharged coinbase:

```rust
let supercharged_amount = if includes_snark_work {
    coinbase_amount * supercharged_coinbase_factor  // 720 * 2 = 1440 MINA
} else {
    coinbase_amount  // 720 MINA
};
```

This provides additional incentive for including SNARK work in blocks.

## Balance constraints

### Account creation

When creating coinbase receiver account:

```rust
if coinbase_amount < account_creation_fee {
    // Coinbase may fail or create account with zero balance
}
```

### Receiver balance limits

The receiver's balance cannot overflow the maximum amount (2^64 nanomina).

## Monetary policy

Coinbase transactions are the primary mechanism for:

- **Token issuance**: New MINA enters circulation
- **Block producer rewards**: Compensation for consensus participation
- **Network security**: Economic incentive for honest behavior

The coinbase amount and issuance schedule are defined in constraint constants
and may change through protocol upgrades.

## Testing

Comprehensive tests are available in
[`ledger/tests/test_transaction_logic_first_pass_coinbase.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_coinbase.rs):

- `test_apply_coinbase_without_fee_transfer` - Basic coinbase reward
- `test_apply_coinbase_with_fee_transfer` - Coinbase with SNARK work payment
- `test_apply_coinbase_with_fee_transfer_to_same_account` - Same receiver
  optimization
- `test_apply_coinbase_creates_account` - Coinbase creating new account

## Related files

- [`ledger/src/scan_state/transaction_logic/mod.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs) -
  Coinbase type definitions
- [`ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs) -
  Coinbase application logic
- [`ledger/tests/test_transaction_logic_first_pass_coinbase.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_coinbase.rs) -
  Coinbase tests

## See also

- [Transactions overview](../transactions)
- [Fee transfers](./fee-transfers)
