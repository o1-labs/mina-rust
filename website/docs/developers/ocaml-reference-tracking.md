---
sidebar_position: 11
title: OCaml reference tracking
description:
  System for tracking correspondence between Rust and OCaml implementations
slug: /developers/ocaml-reference-tracking
---

# OCaml reference tracking

This document describes the system for tracking correspondence between Rust code
in this repository and the original OCaml implementation in the
[Mina Protocol repository](https://github.com/MinaProtocol/mina).

## Overview

As mina-rust is a reimplementation of the Mina OCaml client, we maintain inline
comments that reference the corresponding OCaml code. This helps developers:

1. Understand the original implementation context
2. Verify that the Rust implementation matches the OCaml behavior
3. Track changes in the OCaml codebase that may require updates in Rust

## Comment format

OCaml references use GitHub permalinks to reference the corresponding OCaml
code. These comments are added as doc comments directly above the Rust type or
function:

```rust
/// OCaml reference: https://github.com/MinaProtocol/mina/blob/55582d249cdb225f722dbbb3b1420ce7570d501f/src/lib/mina_base/transaction_status.ml#L9-L113
/// Last verified: 2025-10-08
pub enum TransactionFailure {
    // ...
}
```

### Format specification

- **Line 1**: `/// OCaml reference: <permalink>`
  - `<permalink>`: GitHub permalink to the exact code in the Mina repository
  - Format:
    `https://github.com/MinaProtocol/mina/blob/<commit>/<path>#L<start>-L<end>`
  - The permalink combines repository, commit, path, and line range
- **Line 2**: `/// Last verified: <YYYY-MM-DD>`
  - Date when the reference was last verified to be accurate

The permalink format makes it easy to click directly to the referenced code and
ensures the reference points to a specific commit.

## Validation script

The `.github/scripts/check-ocaml-refs.sh` script validates all OCaml references:

```bash
# Validate against compatible branch (default)
./.github/scripts/check-ocaml-refs.sh

# Validate against a specific branch
./.github/scripts/check-ocaml-refs.sh --branch develop

# Validate against a specific repository
./.github/scripts/check-ocaml-refs.sh --repo https://github.com/MinaProtocol/mina.git --branch develop

# Automatically update stale commit hashes
./.github/scripts/check-ocaml-refs.sh --update
```

### What the script checks

1. **File existence**: Verifies the OCaml file exists at the specified path
2. **Line ranges**: Validates that line ranges don't exceed the file length
3. **Code consistency**: Verifies that the code at the referenced commit matches
   the code on the current branch (ensures the reference is still accurate)
4. **Commit staleness**: Checks if the commit hash matches the current HEAD

### Exit codes

- `0`: All references are valid or only stale commits (warning)
- `1`: Invalid references found (missing files or invalid line ranges)

## Automated verification

A GitHub Actions workflow runs weekly to:

1. Validate all OCaml references against the latest `compatible` branch
2. Automatically update stale commit hashes and verification dates
3. Create a pull request with the updates

The workflow can also be triggered manually via the Actions tab.

## Adding new references

When implementing new features from the OCaml codebase:

1. Add the OCaml reference comment above your Rust type/function
2. Use the current commit hash from the Mina repository
3. Set the verification date to today
4. Include line ranges to make it easy to find the exact code

Example:

```rust
/// OCaml reference: src/lib/mina_base/fee_transfer.ml L:19-45
/// Commit: 55582d249cdb225f722dbbb3b1420ce7570d501f
/// Last verified: 2025-10-08
#[derive(Debug, Clone, PartialEq)]
pub struct SingleFeeTransfer {
    pub receiver_pk: CompressedPubKey,
    pub fee: Fee,
    pub fee_token: TokenId,
}
```

## Finding the correct line range

To find the line range for an OCaml reference:

1. Navigate to the file in the Mina repository
2. Find the relevant type or function definition
3. Note the starting and ending line numbers
4. Use format `L:<start>-<end>`

For single-line references, use the same number: `L:42-42`

## Best practices

1. **Be specific**: Include line ranges to point to exact definitions
2. **Verify regularly**: Run the validation script before committing
3. **Update when needed**: If you update Rust code based on OCaml changes,
   update the commit hash and date
4. **Document differences**: If the Rust implementation intentionally differs,
   add a note explaining why

## Example references

See
[`ledger/src/scan_state/transaction_logic/mod.rs`](https://github.com/o1-labs/mina-rust/blob/develop/ledger/src/scan_state/transaction_logic/mod.rs)
for examples of properly formatted OCaml references.
