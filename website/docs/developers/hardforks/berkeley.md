---
sidebar_position: 2
---

# Berkeley upgrade

The Berkeley upgrade was the first hardfork of the Mina Protocol, deployed in
June 2024. It introduced zkApps (zero-knowledge applications) and programmable
smart contracts to Mina.

## Overview

The Berkeley upgrade implemented three Mina Improvement Proposals (MIPs):

### MIP-1: Remove supercharged rewards

Removes the supercharged coinbase rewards mechanism from the protocol.

- **MIP**:
  [MIP-1](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0001-remove-supercharged-rewards.md)

### MIP-3: Kimchi

Introduces Kimchi, an updated version of the PLONK proof system with improved
performance and capabilities.

- **MIP**:
  [MIP-3](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0003-kimchi.md)

### MIP-4: zkApps

Introduces zkApps, enabling programmable smart contracts on Mina with
zero-knowledge proofs.

- **MIP**:
  [MIP-4](https://github.com/MinaProtocol/MIPs/blob/main/MIPS/mip-0004-zkapps.md)

## TODO

This page needs to be updated with detailed implementation tracking for the
Berkeley features in the Rust node, including:

- Links to OCaml node implementation patches for each feature
- Links to Rust node implementation PRs
- Implementation status and testing coverage
- Any Berkeley-specific configuration or compatibility notes
