---
title: Transactions
description: Understanding how transactions modify the ledger in Mina
---

# Transactions

Transactions are the primary mechanism for modifying the ledger state in Mina.
This document explains the different types of transactions, their structures,
and how they interact with the ledger.

## Overview

The transaction system in Mina is implemented in the ledger crate, specifically
in
[`ledger/src/scan_state/transaction_logic/mod.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs).
Transactions modify account balances, permissions, and other state in the ledger
through a two-pass application process.

## Transaction types

All transactions in Mina are represented by the `Transaction` enum:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L996-L1001 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L996-L1001
```

### User commands

User commands are transactions initiated by users. They are represented by the
`UserCommand` enum:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L642-L646 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L642-L646
```

#### Signed commands

Signed commands are traditional transactions that transfer value or delegate
stake. A `SignedCommand` consists of a payload, signer, and signature:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L129-L144 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L129-L144
```

The payload includes common fields shared by all signed commands:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L15-L30 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L15-L30
```

The body can be one of two types:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L67-L76 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L67-L76
```

**Payment**

Transfers MINA tokens from the fee payer to a receiver:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L32-L39 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L32-L39
```

**Stake delegation**

Delegates the fee payer's stake to another account:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/signed_command.rs#L41-L51 -->

```rust reference title="ledger/src/scan_state/transaction_logic/signed_command.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/signed_command.rs#L41-L51
```

#### zkApp commands

zkApp commands are more complex transactions that can update multiple accounts
and execute zero-knowledge smart contracts. These are documented in detail in
the [zkApps documentation](./zkapps.md).

### Fee transfers

Fee transfers are created by block producers to collect fees from transactions
in a block. A
[`FeeTransfer`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L295-L308)
can contain one or two transfers:

```rust
pub struct FeeTransfer(pub(super) OneOrTwo<SingleFeeTransfer>);
```

Each
[`SingleFeeTransfer`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L268-L276)
includes:

- `receiver_pk`: Public key receiving the fee
- `fee`: Amount of the fee
- `fee_token`: Token ID for the fee (must be default token)

<!-- prettier-ignore-start -->

:::note

Fee transfers have an important invariant: when combining two single fee
transfers, they must use the same token. This is enforced in the
[`of_singles`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L335-L354)
method to ensure the transaction SNARK only handles fee excesses in a single
token.

:::

<!-- prettier-ignore-stop -->

### Coinbase

Coinbase transactions create new MINA tokens as block rewards. A
[`Coinbase`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L376-L384)
includes:

- `receiver`: Public key receiving the reward
- `amount`: Total coinbase amount
- `fee_transfer`: Optional fee transfer to pay the SNARK worker

The coinbase may include an optional
[`CoinbaseFeeTransfer`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L357-L375)
to compensate SNARK workers. If the receiver and fee transfer recipient are the
same, the fee transfer is removed.

## Transaction application

Transactions are applied to the ledger through a two-pass process implemented in
[`transaction_partially_applied.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs).

### First pass

The
[`apply_transaction_first_pass`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L49-L109)
function:

1. Records the previous ledger hash
2. Validates the transaction
3. Applies initial state changes
4. For signed commands and fee transfers, fully applies the transaction
5. For zkApp commands, performs the first phase of application

The function returns a
[`TransactionPartiallyApplied`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L23-L29)
which contains the transaction state after the first pass.

### Second pass

The
[`apply_transaction_second_pass`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L111-L158)
function:

1. For signed commands, fee transfers, and coinbase: returns the already-applied
   transaction
2. For zkApp commands: completes the second phase of application

This two-pass system allows zkApp commands to properly handle their complex
state transitions while keeping simpler transactions efficient.

### Applying user commands

User commands (signed commands) are applied through
[`apply_user_command`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L982-L999),
which:

1. Validates the transaction is not expired
2. Pays the fee from the fee payer's account
3. Updates the fee payer's nonce and receipt chain hash
4. Applies the command body (payment or stake delegation)
5. Returns the application status

## Transaction status and failures

Each transaction has a
[`TransactionStatus`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L181-L188)
indicating whether it succeeded or failed:

```rust
pub enum TransactionStatus {
    Applied,
    Failed(Vec<Vec<TransactionFailure>>),
}
```

When a transaction fails, it includes one or more
[`TransactionFailure`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L63-L113)
reasons:

### Common failures

- **`SourceNotPresent`**: Source account doesn't exist
- **`ReceiverNotPresent`**: Receiver account doesn't exist (for stake
  delegation)
- **`SourceInsufficientBalance`**: Source doesn't have enough funds
- **`AmountInsufficientToCreateAccount`**: Payment amount is less than account
  creation fee
- **`ReceiverAlreadyExists`**: Attempted to create an account that already
  exists
- **`Overflow`**: Arithmetic overflow in balance calculations

### Permission failures

- **`UpdateNotPermittedBalance`**: Account permissions don't allow balance
  changes
- **`UpdateNotPermittedAccess`**: Account permissions don't allow access
- **`UpdateNotPermittedDelegate`**: Account permissions don't allow delegate
  changes
- **`UpdateNotPermittedNonce`**: Account permissions don't allow nonce changes

### zkApp-specific failures

For zkApp commands, additional failures are possible related to preconditions,
app state, verification keys, and more. These are documented in the zkApps
documentation.

## Transaction fee handling

### Fee payment

Fees are paid from the fee payer's account before the transaction body is
applied. The
[`pay_fee`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L1001-L1030)
function:

1. Verifies the signer matches the fee payer
2. Verifies the fee token is the default token
3. Deducts the fee from the account balance
4. Increments the account nonce
5. Updates the receipt chain hash
6. Validates timing constraints

<!-- prettier-ignore-start -->

:::note

Even if a transaction fails, the fee is still deducted and the nonce is
incremented. This ensures the network is compensated for processing the
transaction and prevents replay attacks.

:::

<!-- prettier-ignore-stop -->

### Fee excess

The
[`fee_excess`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L1008-L1018)
method calculates the net change in token supply from a transaction:

- User commands: Positive (fee is paid)
- Fee transfers: Negative (fee is received)
- Coinbase: Zero (new supply equals coinbase amount)

Fee excesses are tracked to ensure blocks maintain proper token supply
constraints.

## Account creation

When a transaction references a non-existent account:

1. A new account is created with default permissions
2. An account creation fee is deducted from the transaction amount
3. The new account is added to the ledger

The account creation fee is defined in
[`constraint_constants.account_creation_fee`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L447-L466)
and ensures the ledger doesn't grow unbounded with dust accounts.

## Timing constraints

Accounts can have timing constraints that control when funds can be spent. The
[`validate_timing`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_union_payload.rs)
function (in `transaction_union_payload.rs`) checks:

- Whether the account is timed or untimed
- If timed, whether the current slot allows the withdrawal
- If the withdrawal amount exceeds the currently available balance

Timing validation is performed during transaction application and can cause
transactions to fail if constraints aren't met.

## Memo field

Every signed command includes a
[`Memo`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L485-L640)
field (34 bytes):

- Byte 0: Tag (0x00 for digest, 0x01 for bytes)
- Byte 1: Length
- Bytes 2-33: Data (or hash of data if too long)

Memos allow users to attach messages or identifiers to transactions. For strings
longer than 32 bytes, a Blake2b digest is stored instead.

## Integration with scan state

Transactions flow through the scan state for proof generation:

1. Transactions enter the staged ledger
2. They're added to the scan state's parallel scan tree
3. SNARK workers generate proofs for transaction correctness
4. Completed proofs allow transactions to be finalized
5. The ledger is updated with the final state

The scan state coordinates parallel proof generation to maximize throughput
while maintaining correctness. See the
[scan state documentation](../researchers/scan-state.md) for details.

## Code organization

The transaction logic is organized into several modules:

- **`mod.rs`**: Core transaction types and top-level logic
- **`signed_command.rs`**: Signed command structures and validation
- **`transaction_partially_applied.rs`**: Transaction application logic
- **`transaction_applied.rs`**: Applied transaction results
- **`transaction_union_payload.rs`**: Shared payload handling
- **`protocol_state.rs`**: Protocol state views for validation
- **`local_state.rs`**: Local state for zkApp execution
- **`zkapp_command.rs`**: zkApp command structures (documented separately)
- **`valid.rs`**: Validated transaction types
- **`verifiable.rs`**: Verifiable transaction types for proof generation

## Related documentation

- [zkApps](./zkapps.md): Documentation for zkApp transactions
- [Ledger crate](./ledger-crate.md): Overall ledger architecture
- [Scan state](../researchers/scan-state.md): How transactions are proven
- [Architecture](./architecture.md): State machine patterns used in transaction
  processing
