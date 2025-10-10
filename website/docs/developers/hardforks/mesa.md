---
sidebar_position: 3
---

# Mesa upgrade

The Mesa upgrade is the second hardfork of the Mina Protocol that introduces
several enhancements to zkApp capabilities. This page tracks the implementation
of these features in the Rust node.

## Overview

The Mesa upgrade consists of four main protocol improvements:

### MIP-6: Slot reduction to 90 seconds

Reduces the slot time from 180 seconds to 90 seconds, enabling faster block
production and improved network responsiveness.

- **MIP**:
  [MIP-6](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0006-slot-reduction-90s.md)
- **Blog post**:
  [Road to Mesa: Performance Dialed Up for zkApps](https://www.o1labs.org/blog/account-update-limit)
- **Implementation status**: To be tracked

### MIP-7: Increase state size limit

Expands zkApp account state from 8 to 32 fields, allowing developers to store
more data directly on-chain and reducing the need for external storage
workarounds.

- **MIP**:
  [MIP-7](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0007-increase-state-size-limit.md)
- **Blog post**:
  [Road to Mesa: Expanding zkApp State with Fewer Constraints](https://www.o1labs.org/blog/increasing-zkapp-state)
- **Implementation status**: To be tracked

### MIP-8: Increase events and actions limit

Increases the field element limit from 100 to 1024 for both events and actions
per transaction, and removes the per-event/per-action mini-cap of 16 field
elements. This allows zkApps to carry more information and instructions in a
single transaction.

- **MIP**:
  [MIP-8](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0008-increase-events-actions-limit.md)
- **Blog post**:
  [Road to Mesa: Preparing for the Next Chapter with More Use Cases](https://www.o1labs.org/blog/mip-8-events-actions)
- **Implementation tracking**:
  [#1261](https://github.com/o1-labs/mina-rust/issues/1261)

### MIP-9: Increase zkApp account update limit

Increases the number of account updates allowed per transaction from 10
signature-based and 5 proof-based updates to roughly triple those limits. This
enables more complex zkApp logic to be executed in fewer transactions.

- **MIP**:
  [MIP-9](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0009-increase-zkapp-account-update-limit.md)
- **Blog post**:
  [Road to Mesa: Performance Dialed Up for zkApps](https://www.o1labs.org/blog/account-update-limit)
- **Implementation status**: To be tracked

## Implementation tracking

All Mesa upgrade work is tracked in
[#1259](https://github.com/o1-labs/mina-rust/issues/1259). Each feature has its
own sub-issue that includes:

- Links to relevant MIPs (Mina Improvement Proposals)
- OCaml node patches implementing the feature
- Rust node implementation PRs
- Performance benchmarks and tests
