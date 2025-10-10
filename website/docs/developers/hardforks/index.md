---
sidebar_position: 1
---

# Introduction

Mina Protocol evolves through hardforks that introduce new features and protocol
improvements. This section tracks the implementation of hardfork features in the
Rust node.

## Overview

A hardfork is a protocol upgrade that is not backward-compatible with previous
versions. All nodes on the network must upgrade to continue participating in
consensus.

The Mina Rust node maintains compatibility with the OCaml node by implementing
the same protocol changes. Each hardfork page in this section documents:

- The features introduced in the hardfork
- Links to official specifications and blog posts
- Implementation tracking via GitHub issues and pull requests
- OCaml node patches that serve as reference implementations

## Hardfork history

### Berkeley (June 2024)

The first hardfork of the Mina Protocol, deployed in June 2024. Berkeley
introduced zkApps and programmable smart contracts to Mina.

See the [Berkeley hardfork page](./berkeley.md) for details.

### Mesa (Upcoming)

The second hardfork of the Mina Protocol. Mesa enhances zkApp capabilities with
increased limits for account updates, events/actions, and on-chain state.

See the [Mesa hardfork page](./mesa.md) for implementation tracking.
