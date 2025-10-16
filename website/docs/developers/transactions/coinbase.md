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
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L452-L476
```

## Transaction application

### First pass

During the first pass
([`apply_coinbase`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L454)),
the coinbase:

1. **Applies coinbase reward**
   - Adds coinbase amount to receiver's balance
   - Creates receiver account if it doesn't exist
   - Deducts account creation fee if creating account

2. **Applies fee transfer (if present)**
   - Adds fee to fee transfer receiver's balance
   - Deducts fee from coinbase receiver's reward
   - Creates fee transfer receiver account if needed

### No fee payment

Like fee transfers, coinbase transactions do not pay fees:

- No nonce increment
- No fee payer
- No signature required
- Cannot be submitted by users

## Coinbase amount

The coinbase amount is configured in
[`ConstraintConstants`](https://github.com/o1-labs/mina-rust/blob/develop/core/src/constants.rs#L39-L42)
(`core/src/constants.rs`), which includes the base `coinbase_amount` and the
`supercharged_coinbase_factor` for rewards when SNARK work is included. The
specific values are defined in the network configurations:
[devnet](https://github.com/o1-labs/mina-rust/blob/develop/core/src/network.rs#L145-L146)
sets `coinbase_amount` to 720,000,000,000 nanomina (720 MINA) and
`supercharged_coinbase_factor` to 1, while
[mainnet](https://github.com/o1-labs/mina-rust/blob/develop/core/src/network.rs#L223-L224)
uses the same values.

### Supercharged coinbase

Supercharged rewards were designed to provide double block rewards (factor of 2)
to block producers staking with unlocked tokens during the early mainnet period
following the 2021 launch. This mechanism incentivized participation and orderly
markets after mainnet launch.

**Historical values**:

- Original mainnet: 2 (double rewards for unlocked tokens)
- Berkeley hardfork (June 2024): 1 (supercharged rewards removed via
  [MIP1](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0001-remove-supercharged-rewards.md))

The removal was decided by community vote on January 1, 2023, as proposed by
community member Gareth Davies. This change ensures uniform rewards for all
tokens and reduces inflation, promoting a sustainable economic model.

**References**:

- [Berkeley Upgrade](https://minaprotocol.com/blog/minas-berkeley-upgrade-what-to-expect)
- [Supercharged Rewards Removal](https://minaprotocol.com/blog/update-on-minas-supercharged-rewards-schedule)
- [Original Proposal](https://github.com/MinaProtocol/mina/issues/5753)

## Fee transfer interaction

### Splitting the reward

When a coinbase includes a fee transfer, the reward is split
([implementation](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L496-L498)):

- **Coinbase receiver**: Gets `coinbase_amount - fee_transfer_amount`
- **Fee transfer receiver**: Gets `fee_transfer_amount`

This mechanism allows block producers to share rewards with SNARK workers.

### Same receiver

If the fee transfer receiver equals the coinbase receiver, the fee transfer is
removed
([`Coinbase::create`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L452)):

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/mod.rs#L464-L472 -->

```rust reference title="ledger/src/scan_state/transaction_logic/mod.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs#L464-L472
```

## Account creation

If receiver accounts don't exist, they are created automatically:

### Coinbase receiver creation

When creating a new coinbase receiver account, the account creation fee is
deducted from the reward:

<!-- CODE_REFERENCE: ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L559-L561 -->

```rust reference title="ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs"
https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L559-L561
```

The
[`sub_account_creation_fee`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/transaction_partially_applied.rs#L653)
function handles the fee deduction logic.

### Fee transfer receiver creation

The fee transfer receiver follows standard account creation rules, and the fee
transfer must be sufficient to cover the account creation fee.

## Testing

Comprehensive tests are available in
[`ledger/tests/test_transaction_logic_first_pass_coinbase.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/tests/test_transaction_logic_first_pass_coinbase.rs):

- `test_apply_coinbase_without_fee_transfer` - Basic coinbase reward
- `test_apply_coinbase_with_fee_transfer` - Coinbase with SNARK work payment to
  existing account
- `test_apply_coinbase_with_fee_transfer_creates_account` - Fee transfer
  creating new account
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
