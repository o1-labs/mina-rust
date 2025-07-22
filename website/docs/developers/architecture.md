---
sidebar_position: 1
title: Architecture Overview
description:
  Understand OpenMina's Redux-style state machine architecture and design
  principles
slug: /developers/architecture
---

# OpenMina Architecture

OpenMina follows a Redux-style state machine architecture for predictable,
debuggable behavior. This design ensures that all state changes are traceable
and the system behavior is deterministic.

## Core Principles

### State Machine Pattern

OpenMina implements Redux principles adapted for a blockchain node:

- **State** - Centralized, immutable data structure representing the entire node
  state
- **Actions** - Events that trigger state changes throughout the system
- **Enabling Conditions** - Guards that prevent invalid state transitions
- **Reducers** - Pure functions that update state and dispatch new actions
- **Effects** - Thin wrappers for service calls and side effects
- **Services** - Separate threads handling I/O and heavy computation

### Predictable State Management

Every state change in OpenMina follows the same pattern:

```rust
// 1. Action is dispatched
dispatch(SomeAction { data });

// 2. Enabling condition is checked
if enabling_condition_met(&state, &action) {
    // 3. Reducer processes the action
    let new_state = reducer(state, action);

    // 4. Effects may be triggered
    trigger_effects(&new_state, &action);
}
```

## Architecture Styles

The codebase contains two architectural styles:

### New Style (Recommended)

- **Unified reducers** that handle both state updates and action dispatch
- Single file per component containing all logic
- Cleaner separation of concerns

### Old Style (Being Migrated)

- Separate reducer and effects files
- Split between `*_reducer.rs` and `*_effects.rs`
- Gradually being converted to new style

## Component Structure

### Core Components

**Node** - Main node logic

- Block production and validation
- Transaction pool management
- Consensus and blockchain state
- RPC interface

**P2P** - Networking layer

- Dual transport: libp2p and WebRTC
- Peer discovery and connection management
- Message routing and validation

**Ledger** - Blockchain state

- Account state and transactions
- Proof verification
- Scan state management

**Core** - Shared utilities

- Common types and data structures
- Cryptographic primitives
- Configuration management

### File Organization

Each component follows consistent patterns:

- `*_state.rs` - State definitions and data structures
- `*_actions.rs` - Action types and event definitions
- `*_reducer.rs` - State transition logic
- `*_effects.rs` - Service interaction wrappers
- `*_service.rs` - Service interface definitions
- `summary.md` - Component documentation

## Data Flow

1. **External Events** (network messages, user commands) create actions
2. **Actions** flow through the dispatch system
3. **Enabling Conditions** validate whether actions can be processed
4. **Reducers** compute new state based on current state and action
5. **Effects** trigger service calls when state changes require external
   interaction
6. **Services** handle async operations and generate new events

## Key Benefits

- **Debuggability** - Complete state history and action replay
- **Testability** - Pure functions and predictable state changes
- **Maintainability** - Clear separation of concerns and data flow
- **Performance** - Efficient state updates and selective processing

## Development Guidelines

- Use `bug_condition!` macro for theoretically unreachable code paths
- Extract complex logic into state methods rather than bloating reducers
- Prefer enabling conditions over error handling in reducers
- Document component responsibilities in `summary.md` files

For detailed implementation examples, see the component-specific documentation
in the codebase.
