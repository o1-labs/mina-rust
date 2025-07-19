# Ledger Crate Summary

The ledger crate is the most complex component in the codebase. For architecture overview and design details, see [docs/handover/ledger-crate.md](../docs/handover/ledger-crate.md).

## Quick Reference

**Core Ledger**
- `src/base.rs` - BaseLedger trait (fundamental interface)
- `src/database/` - In-memory account storage
- `src/mask/` - Layered ledger views with Arc-based sharing
- `src/tree.rs` - Merkle tree operations

**Transaction Processing**
- `src/transaction_pool.rs` - Mempool with fee-based ordering
- `src/staged_ledger/` - Block validation and transaction application
- `src/scan_state/` - SNARK work coordination and parallel scan

**Proof System**
- `src/proofs/` - Transaction, block, and zkApp proof generation/verification
- `src/sparse_ledger/` - Minimal ledger representation for proofs
- `src/zkapps/` - zkApp transaction processing

**Account Management**
- `src/account/` - Account structures, balances, permissions

## Critical Issues

**Error Handling**
- Too many `.unwrap()` and `.expect()` calls in production code paths (excluding tests)
- Critical transaction processing paths could panic the node
- Inconsistent error handling across modules
- Verification key lookup bug fix from upstream Mina Protocol needs to be ported (https://github.com/MinaProtocol/mina/pull/16699)

**Monolithic Structure**
- Single massive crate with files exceeding 6,000+ lines
- Deep coupling between components that should be independent
- Hard to maintain, test, and develop in parallel

**Performance**
- Excessive cloning of large structures in hot paths:
  - `SparseLedger::of_ledger_subset_exn()` calls `oledger.copy()` creating unnecessary deep clones for sparse ledger construction
  - Transaction pool operations clone transaction objects with acknowledged TODO comments about performance
- No memory pooling or reuse strategies (could help with memory fragmentation in WASM)

**Memory Management**
- Memory-only implementation, no persistence for production
- There's an unused `ondisk` implementation but we were planning a more comprehensive global solution (see persistence.md)
- Thread-local caching holds memory indefinitely

## Refactoring Plan

**Phase 1: Safety**
- Replace `.unwrap()` with proper error propagation in production code
- Reduce cloning in hot paths
- Standardize error types

**Phase 2: Decomposition**
Break into focused crates: `mina-account`, `mina-ledger`, `mina-transaction-logic`, `mina-scan-state`, `mina-transaction-pool`, `mina-proofs`

Changes must maintain strict OCaml compatibility while improving performance for production.