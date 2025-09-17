# GitHub Copilot Instructions for Mina Rust Repository

## Project Overview

Mina is a Rust implementation of the Mina Protocol, a lightweight blockchain
using zero-knowledge proofs. It follows a Redux-style state machine architecture
for predictable, debuggable behavior.

## Architecture

### State Machine Pattern

The codebase follows Redux principles:

- **State** - Centralized, immutable data structure
- **Actions** - Events that trigger state changes
- **Enabling Conditions** - Guards that prevent invalid state transitions
- **Reducers** - Functions that update state and dispatch new actions
- **Effects** - Thin wrappers for service calls
- **Services** - Separate threads handling I/O and heavy computation

### Key Components

- **node/** - Main node logic (block production, transaction pool, consensus)
- **p2p/** - Networking layer (libp2p and WebRTC transports)
- **snark/** - Proof verification orchestration
- **ledger/** - Ledger implementation and transaction logic
- **core/** - Shared types and utilities

## Code Patterns

### File Organization

- `*_state.rs` - State definitions
- `*_actions.rs` - Action types
- `*_reducer.rs` - State transitions
- `*_effects.rs` - Service interactions
- `*_service.rs` - Service interfaces

### Defensive Programming

- Use `bug_condition!` macro for theoretically unreachable code paths
- Always add enabling condition checks before state transitions
- Extract complex logic from reducers into state methods

## Development Workflow

### Before Making Changes

1. Understand the Redux-style architecture
2. Identify the correct component and layer for your changes
3. Check existing patterns in similar files
4. Run tests to understand current behavior

### Code Changes

1. Make minimal, surgical modifications
2. Follow existing patterns and conventions
3. Update documentation if directly related to changes
4. Preserve working functionality

### Formatting and Quality

**MANDATORY before any commit:**

```bash
# Format Rust and TOML files
make format

# Format markdown and MDX files
make format-md

# Fix trailing whitespaces (CRITICAL)
make fix-trailing-whitespace

# Verify formatting
make check-format
make check-md
make check-trailing-whitespace
```

### Testing

```bash
# Run all tests
make test

# Run specific test suites
make test-p2p
make test-ledger
make nextest  # Faster test execution

# Build project
make build
make build-release
```

### Documentation

- Update `CHANGELOG.md` for significant user-facing changes
- Use format: `- **Category**: Description ([#PR](github-link))`
- Component docs are in individual `summary.md` files
- Website docs are in `website/docs/`

## Commit Guidelines

**CRITICAL RULES:**

- **NEVER** add any AI assistant as co-author in commit messages
- **NEVER** use emojis in commit messages
- **ALWAYS** wrap commit titles at 80 characters
- **ALWAYS** run `make fix-trailing-whitespace` before committing

### Commit Message Format

```
Brief description of change (80 chars max)

Longer description if needed, wrapped at 80 characters.
Reference issues/PRs when relevant.
```

## Code Style

### Rust Code

- Follow existing `rustfmt` configuration
- Use `clippy` suggestions: `make lint`
- Prefer explicit error handling over panics
- Use descriptive variable names
- Add comments only when they add value beyond the code

### Documentation Style

- Use lowercase for section headings ("test design", "resource management")
- Use lowercase for bullet point labels
- Wrap Docusaurus admonitions with prettier-ignore comments:

  ```mdx
  <!-- prettier-ignore-start -->

  :::note Content here :::

  <!-- prettier-ignore-stop -->
  ```

## Common Tasks

### Adding New Features

1. Define actions in appropriate `*_actions.rs`
2. Add state transitions in `*_reducer.rs`
3. Implement effects in `*_effects.rs` if needed
4. Add service methods in `*_service.rs` if needed
5. Update tests and documentation

### Debugging

- Use existing test scenarios in `node/testing/`
- Check component `summary.md` files for known issues
- Follow the Redux flow: Action → Reducer → Effect → Service

### Dependencies

- Update `Cargo.toml` files carefully
- Run `make check` after dependency changes
- Consider impact on WebAssembly build (`make build-wasm`)

## Important Files

- `node/src/state.rs` - Global state structure
- `node/src/action.rs` - Top-level action enum
- `node/src/reducer.rs` - Main reducer dispatch
- `Makefile` - Build and development commands
- `CLAUDE.md` - Comprehensive codebase navigation guide
- `ARCHITECTURE.md` - Migration guide for old vs new style

## Resources

- Architecture docs: `docs/handover/`
- Component docs: Find `summary.md` in component directories
- Website: `make docs-serve` (http://localhost:3000)
- Build commands: `make help`

Remember: This is a complex blockchain implementation. Take time to understand
the Redux architecture and existing patterns before making changes.
