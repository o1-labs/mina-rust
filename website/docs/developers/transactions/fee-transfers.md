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
creation fee (1 MINA by default) is deducted from the fee being transferred.

## Single vs. double transfers

### Single transfer

Distributes fees to one block producer. See
[test_apply_single_fee_transfer_success](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L94-L95)
for an example:

<!-- CODE_REFERENCE: ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L94-L95 -->

```rust reference title="ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L94-L95
```

### Double transfer

Distributes fees to two block producers (e.g., coinbase producer and snark work
producer). See
[test_apply_double_fee_transfer_success](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L169-L174)
for an example:

<!-- CODE_REFERENCE: ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L169-L174 -->

```rust reference title="ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L169-L174
```

## Token constraints

Fee transfers must use the default token. For double transfers, both tokens must
match. The validation is implemented in
[FeeTransfer::of_singles](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L400):

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L400-L408 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L400-L408
```

See
[test_apply_fee_transfer_incompatible_tokens](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L333-L337)
for an example creating fee transfers with incompatible tokens:

<!-- CODE_REFERENCE: ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L333-L337 -->

```rust reference title="ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L333-L337
```

The validation fails with an error:

<!-- CODE_REFERENCE: ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L344-L346 -->

```rust reference title="ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L344-L346
```

## Testing

Comprehensive tests are available in
[`ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs):

- [`test_apply_single_fee_transfer_success`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L75) -
  Single fee transfer to existing account
- [`test_apply_double_fee_transfer_success`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L137) -
  Double fee transfer to two receivers
- [`test_apply_fee_transfer_creates_account`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L237) -
  Fee transfer creating new account
- [`test_apply_fee_transfer_incompatible_tokens`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs#L316) -
  Token compatibility validation

## Related files

- [`ledger/src/scan_state/transaction_logic/mod.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs) -
  Fee transfer type definitions
- [`ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs) -
  Fee transfer application logic
- [`ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_fee_transfer.rs) -
  Fee transfer tests
