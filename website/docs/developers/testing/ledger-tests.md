---
sidebar_position: 3
title: Ledger Tests
description: Comprehensive ledger functionality testing
slug: /developers/testing/ledger-tests
---

# Ledger Tests

## Overview

Comprehensive ledger functionality testing covers transaction processing, state
management, and ledger operations. These tests require nightly Rust due to
advanced features and dependencies.

## Running Ledger Tests

### Basic Ledger Testing

```bash
# Build ledger tests
make build-ledger

# Run ledger tests
make test-ledger

# Run with cargo-nextest
make nextest-ledger
```

**Note**: Ledger tests require nightly Rust toolchain.

## VRF Tests

```bash
# Run VRF tests
make test-vrf

# Run with cargo-nextest
make nextest-vrf
```

**Note**: VRF tests also require nightly Rust toolchain.

## Block Replayer Tool

### Purpose

The block replayer validates transaction and ledger logic by replaying
historical blockchain data:

- **Transaction Validation**: Verify transaction processing correctness
- **Ledger State**: Ensure ledger state transitions are accurate
- **Protocol Compliance**: Check adherence to protocol rules
- **Performance Analysis**: Measure processing speeds and resource usage

### Usage

```bash
# Replay blocks from chain data
./target/release/mina replay --blocks-file chain.json

# Validate specific block range
./target/release/mina replay --start-block 1000 --end-block 2000
```

### Future development

Continued development of the block replayer is recommended for:

- **Enhanced validation**: More comprehensive transaction validation
- **Performance testing**: Benchmark ledger operation performance
- **Protocol evolution**: Test new protocol features against historical data
- **Regression testing**: Ensure changes don't break existing functionality

## Ledger testing components

### Transaction processing

Tests cover:

- **User commands**: Payment and delegation transactions
- **Fee transfers**: Block producer rewards and fees
- **Coinbase**: Block reward distribution
- **Account creation**: New account initialization

### State management

Validation includes:

- **Account updates**: Balance and nonce changes
- **Merkle tree operations**: Ledger tree consistency
- **State transitions**: Valid state progression
- **Rollback handling**: Proper state restoration

### Scan state testing

The scan state manages pending transactions:

- **Transaction queuing**: Proper ordering and batching
- **SNARK work integration**: Proof generation coordination
- **Staged ledger**: Intermediate state management
- **Fork resolution**: Handling competing chains

## Performance Considerations

### Resource management

1. **Memory usage**: Ledger tests can be memory intensive
2. **Disk I/O**: State persistence operations
3. **CPU usage**: Cryptographic operations and proof verification
4. **Test duration**: Comprehensive tests may take significant time

### Optimization strategies

1. **Parallel execution**: Use cargo-nextest for faster test runs
2. **Test filtering**: Run specific test subsets during development
3. **Mock services**: Use lightweight mocks where appropriate
4. **Resource cleanup**: Ensure proper cleanup after tests

## Best practices

### Test design

1. **Isolation**: Use fresh ledger state for each test
2. **Deterministic**: Tests should produce consistent results
3. **Coverage**: Test both success and failure scenarios
4. **Edge cases**: Include boundary conditions and error cases

### Debugging ledger tests

1. **State inspection**: Examine ledger state at test failure points
2. **Transaction tracing**: Track transaction processing steps
3. **Merkle tree validation**: Verify tree consistency
4. **Account state**: Check individual account changes

## Integration with other components

### Connection to P2P tests

Ledger tests integrate with P2P testing for:

- **Transaction propagation**: Ensuring transactions spread correctly
- **Block validation**: Verifying received blocks
- **Consensus integration**: Coordinating with consensus mechanisms

### Scenario test integration

Ledger functionality is tested within broader scenarios:

- **Multi-node synchronization**: Consistent ledger state across nodes
- **Fork resolution**: Proper handling of competing ledgers
- **Performance testing**: Ledger operations under load

## Related Documentation

- [Testing Framework Overview](testing-framework): Main testing documentation
- [Unit Tests](unit-tests): Basic component testing
- [Scenario Tests](scenario-tests): Integration testing scenarios
- [Architecture](../architecture): Overall system architecture including ledger
