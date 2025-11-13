---
sidebar_position: 5
title: Transactions
description: Transaction types and processing in the Mina protocol
slug: /developers/transactions
---

# Transactions

Transactions in Mina represent state changes to the ledger. For detailed API
documentation with implementation specifics, see the
[transaction logic module rustdoc](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs).

## Transaction categories

### User commands

User-initiated transactions that are signed and submitted:

- **Payments** - Transfer MINA tokens between accounts
- **Stake delegation** - Delegate stake to block producers
- **zkApp commands** - Complex zero-knowledge application transactions

### Protocol transactions

System-generated transactions that do not require signatures:

- **Fee transfers** - Distribute transaction fees to block producers
- **Coinbase** - Issue block production rewards (720 MINA per block)

## Two-phase application

All transactions are processed through a two-phase model:

1. **First pass** - Validates preconditions, applies fees, creates accounts
2. **Second pass** - Completes application and finalizes state

Protocol transactions (fee transfers, coinbase) complete in the first pass. User
commands may require both passes.

## Account creation

Creating new accounts requires a 1 MINA account creation fee, deducted from the
amount transferred to the new account. This applies to all transaction types.

## Key concepts

### Fees

User transactions require fees to compensate block producers and prevent spam.
Fees are paid by the sender and distributed through fee transfer transactions.

### Nonces

User accounts maintain nonces to prevent replay attacks. Each transaction must
specify the correct nonce, which increments after successful application.

### Coinbase rewards

Block producers receive 720 MINA per block. Rewards can be split with SNARK
workers via an optional fee transfer component. See the
[Coinbase rustdoc](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs)
for implementation details including:

- Reward structure and distribution
- Fee transfer interaction
- Account creation handling
- Application logic and failure modes

## Implementation details

For comprehensive documentation including transaction structure, validation,
application logic, state transitions, and code examples, see the rustdoc in:

- `ledger/src/scan_state/transaction_logic/mod.rs` - Transaction types and
  overview
- `ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs` -
  Application logic
- `ledger/tests/test_transaction_logic_first_pass_*.rs` - Test examples
