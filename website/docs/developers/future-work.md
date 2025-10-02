---
sidebar_position: 1
title: Overview
description:
  Overview of planned enhancements and design proposals for the Mina Rust node
slug: /developers/future-work
---

# Future Work

This section outlines planned enhancements and design proposals for the Mina
Rust node that are not yet implemented but are critical for the project's
continued development and mainnet readiness.

## Overview

The Mina Rust node is an active development project with several key areas
identified for future enhancement. These proposals represent thoughtful designs
for addressing current limitations and preparing the node for production
deployment at scale.

## Key Areas

### Mainnet Readiness

The [Mainnet Readiness](mainnet-readiness) document provides a comprehensive
overview of all requirements for production deployment. This includes critical
blockers, performance requirements, and operational considerations for running
the Mina Rust node on mainnet. See
[Issue #1354](https://github.com/o1-labs/mina-rust/issues/1354) for tracking
progress.

### P2P Networking Evolution

The [P2P Evolution Plan](p2p-evolution) builds on the successful pull-based
design already implemented for webnodes. Key planned improvements include:

- **QUIC Transport Integration**: Adding QUIC as an alternative transport to
  WebRTC for improved performance and reduced complexity
- **Block Propagation Optimization**: Reducing bandwidth usage by sending only
  block headers and missing data
- **OCaml Node Integration**: Potential unification of networking layers across
  Mina implementations

### Persistent Storage

The [Persistence Design](persistence-design) addresses one of the most critical
requirements for mainnet readiness. Current challenges include:

- **Memory Usage**: The entire ledger is currently kept in memory, creating
  scalability issues
- **Startup Time**: Nodes must reconstruct the full ledger, which is
  time-consuming
- **SNARK Verification**: Redundant verification work across blocks and pools

## Implementation Status

**Important**: All items in this section are design proposals only and are not
yet implemented. These documents serve as:

- **Technical Specifications**: Detailed designs for future implementation
- **Discussion Starting Points**: Basis for technical discussions and
  refinements
- **Roadmap Guidance**: Priority areas for development effort

## Contribution

These design proposals benefit from community review and input. Developers
interested in contributing to these areas should:

1. **Review the designs** thoroughly to understand the proposed approaches
2. **Check the
   [project dashboard](https://github.com/orgs/o1-labs/projects/24/)** for
   current development status and active work items
3. **Provide feedback** on technical feasibility and implementation details
4. **Participate in discussions** about priorities and trade-offs
5. **Contribute to implementation** when development begins

The designs may evolve based on community feedback, technical constraints, and
changing requirements as the Mina ecosystem develops.

## Timeline Considerations

While these documents provide detailed implementation plans, actual development
timelines depend on:

- **Resource allocation** and team priorities
- **Community needs** and feedback
- **Technical dependencies** and prerequisites
- **Coordination requirements** with other Mina implementations

The goal is to ensure these enhancements are implemented thoughtfully and with
proper consideration for the broader Mina ecosystem.
