---
sidebar_position: 3
title: Persistence Design
description: Design proposal for persistent ledger storage
slug: /developers/persistence-design
---

# Persistence Design (Not Yet Implemented)

This document outlines the proposed design for persisting the Mina ledger and
other critical state to disk, reducing memory usage and enabling faster node
restarts.

**Status**: Not yet implemented - this is a design proposal only.

**Critical for Mainnet**: This is one of the most important changes required to
make the webnode mainnet-ready.

## Overview

Currently, the Mina Rust node keeps the entire ledger in memory, which creates
scalability issues for mainnet deployment where the ledger can be large. A
persistent storage solution is needed to:

- Reduce memory usage for both server-side nodes and webnodes
- Enable faster node restarts by avoiding full ledger reconstruction
- Deduplicate SNARK verification work across blocks and pools
- Support partial ledger storage for light clients

## Design Reference

A draft design for the persistence database is outlined in
[Issue #522](https://github.com/o1-labs/mina-rust/issues/522), which proposes an
approach for efficiently storing, updating, and retrieving accounts and hashes.

**Note**: There is a very old implementation for on-disk storage in
`ledger/src/ondisk/` that was never used - a lightweight key-value store
implemented to avoid the RocksDB dependency.

**Database Design Resources**: For those implementing persistence, "Database
Internals" and "Designing Data-Intensive Applications" are excellent books on
database design and implementation.

## Key Design Principles

Based on [Issue #522](https://github.com/o1-labs/mina-rust/issues/522), the
persistence design follows these principles:

1. **Simplicity First**: The design prioritizes simplicity over optimal
   performance
2. **Fixed-Size Storage**: Most data (except zkApp accounts) uses fixed-size
   slots for predictable access patterns
3. **Sequential Account Creation**: Mina creates accounts sequentially, filling
   leaves from left to right in the Merkle tree, enabling an append-only design
4. **Selective Persistence**: Only epoch ledgers and the root ledger need
   persistence; masks can remain in-memory
5. **Infrequent Updates**: Root ledger updates occur only when the transition
   frontier root moves
6. **Hashes in Memory**: All Merkle tree hashes remain in RAM for quick access
7. **Recoverable**: Data corruption is not catastrophic as ledgers can be
   reconstructed from the network

## Problems to be Solved

### Memory Usage

**Current State**: The entire ledger is kept in memory, which can be substantial
on mainnet:

- Account data includes balances, nonces, zkApp state
- Merkle tree structure for cryptographic proofs
- Multiple ledger versions for different blockchain heights

**Solution**: Move account data to persistent storage while keeping frequently
accessed data (like Merkle tree hashes) in memory.

### Startup Time

**Current State**: Nodes must reconstruct the full ledger from genesis or sync
from peers, which is time-consuming.

**Solution**: Persist confirmed ledger states to enable fast startup by loading
from disk rather than network reconstruction.

### SNARK Verification Deduplication

**Current State**: The same SNARK work may be verified multiple times across
different blocks and transaction pools.

**Solution**: Cache verification results persistently to avoid redundant
computation.

## Proposed Architecture

### Storage Layers

#### 1. Root Ledger Storage

- **Purpose**: Store the confirmed ledger at the root of the transition frontier
- **Update frequency**: Only when transition frontier advances
- **Data**: Account balances, nonces, zkApp state
- **Access pattern**: Random reads, infrequent writes

#### 2. Epoch Ledger Storage

- **Purpose**: Store ledger snapshots for staking epoch calculations
- **Update frequency**: Once per epoch
- **Data**: Complete ledger state at epoch boundaries
- **Access pattern**: Sequential reads during epoch transitions

#### 3. Verification Cache

- **Purpose**: Store SNARK verification results
- **Update frequency**: High during block processing
- **Data**: Verification status keyed by work specification
- **Access pattern**: High read/write frequency

### Data Structures

#### Account Storage Format

```rust
struct PersistedAccount {
    public_key: PublicKey,          // 32 bytes
    balance: u64,                   // 8 bytes
    nonce: u32,                     // 4 bytes
    delegate: Option<PublicKey>,    // 33 bytes (1 + 32)
    voting_for: StateHash,          // 32 bytes
    zkapp_state: Option<ZkAppState>, // Variable size
    // ... other fields
}
```

#### Index Structure

- **Account Index**: Maps public keys to storage locations
- **Merkle Index**: Maps tree positions to account locations
- **Height Index**: Maps blockchain heights to ledger versions

### Memory vs Disk Trade-offs

#### Keep in Memory

- **Merkle Tree Hashes**: Fast cryptographic proof generation
- **Recent Transactions**: Active processing requirements
- **Connection State**: Network and consensus data
- **Indices**: Fast lookup structures

#### Move to Disk

- **Account Data**: Large, infrequently accessed in bulk
- **Historical Ledgers**: Epoch snapshots and old states
- **Verification Cache**: Large datasets with locality

## Implementation Strategy

### Phase 1: Foundation

1. **Storage Interface**: Define abstract storage traits
2. **Account Serialization**: Implement efficient encoding/decoding
3. **Index Management**: Create lookup structures
4. **Testing Framework**: Comprehensive test suite

### Phase 2: Basic Persistence

1. **Root Ledger Storage**: Implement basic account persistence
2. **Startup Recovery**: Load ledger from disk on startup
3. **Incremental Updates**: Efficient account modifications
4. **Corruption Recovery**: Handle storage failures gracefully

### Phase 3: Advanced Features

1. **Epoch Ledgers**: Historical snapshot storage
2. **Verification Cache**: SNARK result persistence
3. **Compaction**: Optimize storage usage over time
4. **Partial Loading**: Support for light client scenarios

### Phase 4: Optimization

1. **Performance Tuning**: Optimize for real-world usage patterns
2. **Memory Management**: Fine-tune memory vs disk balance
3. **Concurrent Access**: Support multiple readers/writers
4. **Monitoring**: Add persistence-related metrics

## Technical Considerations

### Storage Backend Options

#### File-Based Storage

**Pros**: Simple, no external dependencies, full control **Cons**: Must
implement indexing, compression, concurrent access

#### Embedded Database (e.g., RocksDB)

**Pros**: Battle-tested, efficient indexing, concurrent access **Cons**:
Additional dependency, larger binary size

#### Custom Key-Value Store

**Pros**: Optimized for Mina's specific needs, lightweight **Cons**: More
development effort, needs thorough testing

### Consistency Guarantees

- **Atomic Updates**: Ensure ledger state changes are atomic
- **Crash Recovery**: Handle interruptions during writes
- **Checksum Validation**: Detect storage corruption
- **Version Management**: Track ledger version compatibility

### Performance Requirements

- **Read Latency**: Account lookups must remain fast
- **Write Throughput**: Handle block processing rates
- **Memory Usage**: Significant reduction from current levels
- **Startup Time**: Faster than network reconstruction

## Migration Strategy

### Development Phase

1. **Parallel Implementation**: Build alongside current in-memory system
2. **Feature Flags**: Enable persistence selectively
3. **Testing**: Extensive testing with mainnet data
4. **Benchmarking**: Performance comparison with current system

### Deployment Phase

1. **Opt-in**: Initially optional for testing
2. **Gradual Rollout**: Enable for specific node types
3. **Full Migration**: Make persistence default
4. **Legacy Support**: Maintain fallback to in-memory mode

## Success Metrics

### Memory Usage

- **Target**: 50-80% reduction in memory usage
- **Measurement**: RSS and heap size monitoring
- **Threshold**: Must support mainnet ledger sizes

### Performance

- **Startup Time**: &lt;5 minutes for full ledger load
- **Query Latency**: &lt;1ms for account lookups
- **Block Processing**: No degradation in processing speed

### Reliability

- **Data Integrity**: Zero data loss during normal operation
- **Crash Recovery**: &lt;30 seconds to restore consistent state
- **Storage Corruption**: Graceful degradation and recovery

## Risks and Mitigation

### Technical Risks

- **Performance Degradation**: Mitigate with extensive benchmarking
- **Data Corruption**: Implement checksums and validation
- **Storage Space**: Monitor and optimize storage usage

### Operational Risks

- **Migration Complexity**: Provide clear upgrade paths
- **Backup Requirements**: Document backup and recovery procedures
- **Monitoring Needs**: Add persistence-specific observability

## Related Work

### Existing Implementations

- **OCaml Node**: Uses RocksDB for ledger persistence
- **Other Blockchains**: Study approaches from Ethereum, Bitcoin
- **Database Systems**: Learn from established database designs

### Design References

- [Issue #522](https://github.com/o1-labs/mina-rust/issues/522): Original
  persistence design proposal
- [Ledger Implementation](https://o1-labs.github.io/mina-rust/api-docs/ledger/):
  Current in-memory ledger code
- [Database Internals](https://databass.dev/): Database design principles
- [DDIA](https://dataintensive.net/): Data-intensive application patterns

## Conclusion

Implementing persistent storage is critical for the Mina Rust node's mainnet
readiness. The proposed design balances simplicity with performance, enabling
significant memory usage reduction while maintaining the fast query performance
required for blockchain operations.

The phased implementation approach allows for careful validation and
optimization, ensuring that persistence improves rather than degrades node
performance. Success in this area will enable the Mina Rust node to scale to
mainnet requirements and support a broader range of deployment scenarios.
