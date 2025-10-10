---
sidebar_position: 3
---

# Mesa upgrade

The Mesa upgrade is a hardfork of the Mina Protocol that introduces several
enhancements to zkApp capabilities. This page tracks the implementation of these
features in the Rust node.

## Overview

The Mesa upgrade consists of three main protocol improvements:

### Account update limit increases

Increases the number of account updates allowed per transaction from 10
signature-based and 5 proof-based updates to roughly triple those limits. This
enables more complex zkApp logic to be executed in fewer transactions.

- **Blog post**:
  [Road to Mesa: Performance Dialed Up for zkApps](https://www.o1labs.org/blog/account-update-limit)
- **Implementation status**: To be tracked

### Events and actions capacity expansion (MIP-8)

Increases the field element limit from 100 to 1024 for both events and actions
per transaction, and removes the per-event/per-action mini-cap of 16 field
elements. This allows zkApps to carry more information and instructions in a
single transaction.

- **Blog post**:
  [Road to Mesa: Preparing for the Next Chapter with More Use Cases](https://www.o1labs.org/blog/mip-8-events-actions)
- **Implementation tracking**:
  [#1261](https://github.com/o1-labs/mina-rust/issues/1261)

### zkApp state expansion

Expands zkApp account state from 8 to 32 fields, allowing developers to store
more data directly on-chain and reducing the need for external storage
workarounds.

- **Blog post**:
  [Road to Mesa: Expanding zkApp State with Fewer Constraints](https://www.o1labs.org/blog/increasing-zkapp-state)
- **Implementation status**: To be tracked

## Implementation tracking

All Mesa upgrade work is tracked in
[#1259](https://github.com/o1-labs/mina-rust/issues/1259). Each feature has its
own sub-issue that includes:

- Links to relevant MIPs (Mina Improvement Proposals)
- OCaml node patches implementing the feature
- Rust node implementation PRs
- Performance benchmarks and tests

## Additional resources

- [Mesa Hard Fork Project Board](https://www.notion.so/o1labs/Hard-Fork-MIPs-Project-1c9e79b1f910805fb44cdc2b9db2ee8e?p=1c9e79b1f910804da24ed4f96638ab2b&pm=s)
- [Mina Protocol Website](https://minaprotocol.com/)
