# OpenMina Codebase Navigation Guide

This file helps understand and navigate the OpenMina codebase structure.

## Project Overview

OpenMina is a Rust implementation of the Mina Protocol, a lightweight blockchain
using zero-knowledge proofs. It follows a Redux-style state machine architecture
for predictable, debuggable behavior.

_For detailed architecture documentation, see
[`docs/handover/`](docs/handover/)_

## Architecture Overview

### State Machine Pattern

The codebase follows Redux principles:

- **State** - Centralized, immutable data structure
- **Actions** - Events that trigger state changes
- **Enabling Conditions** - Guards that prevent invalid state transitions
- **Reducers** - Functions that update state and dispatch new actions
- **Effects** - Thin wrappers for service calls
- **Services** - Separate threads handling I/O and heavy computation

### Architecture Styles

- **New Style**: Unified reducers that handle both state updates and action
  dispatch
- **Old Style**: Separate reducer and effects files (being migrated)

## Project Structure

### Core Components

**node/** - Main node logic

- `block_producer/` - Block production
- `transaction_pool/` - Transaction mempool
- `transition_frontier/` - Consensus and blockchain state
- `snark_pool/` - SNARK work management
- `ledger/` - Ledger operations
- `rpc/` - External API
- `event_source/` - Event routing
- `service/` - Service implementations

**p2p/** - Networking layer

- Dual transport: libp2p and WebRTC
- Channel abstractions for message types
- Peer discovery and connection management

**snark/** - Proof verification orchestration

**ledger/** - Ledger implementation, transaction logic, core transaction pool
logic, staged ledger, scan state, and proof verification

**core/** - Shared types and utilities

## Code Organization

### File Patterns

- `*_state.rs` - State definitions
- `*_actions.rs` - Action types
- `*_reducer.rs` - State transitions
- `*_effects.rs` - Service interactions
- `*_service.rs` - Service interfaces
- `summary.md` - Component documentation and technical debt notes

### Key Files

- `node/src/state.rs` - Global state structure
- `node/src/action.rs` - Top-level action enum
- `node/src/reducer.rs` - Main reducer dispatch

## Understanding the Flow

1. **Actions** flow through the system as events
2. **Enabling conditions** check if actions are valid
3. **Reducers** process actions to update state
4. **Effects** trigger service calls when needed
5. **Services** handle async operations and send events back

## Key Patterns

### Defensive Programming

- `bug_condition!` macro marks theoretically unreachable code paths
- Used after enabling condition checks for invariant validation

### State Methods

- Complex logic extracted from reducers into state methods
- Keeps reducers focused on orchestration

### Callbacks

- Enable decoupled component communication
- Used for async operation completion

## Finding Code

```bash
# Find specific actions
rg "YourAction" --type rust

# Find reducers
rg "reducer.*Substate" --type rust

# Find services
rg "impl.*Service" --type rust

# Check component documentation
find . -name "summary.md" -path "*/component/*"
```

## Component Documentation

Each component directory contains a `summary.md` file documenting:

- Component purpose and responsibilities
- Known technical debt
- Implementation notes
- Refactoring plans

## Documentation Website

OpenMina includes a comprehensive documentation website built with Docusaurus:

### Quick Access
```bash
# Start local documentation server
make docs-serve

# Build documentation
make docs-build

# Other documentation commands
make help | grep docs
```

The website is available at http://localhost:3000 when running locally.

### Structure
- **Node Runners** (`website/docs/node-runners/`) - Installation and operation guides
- **Developers** (`website/docs/developers/`) - Architecture and contribution guides
- **Researchers** (`website/docs/researchers/`) - Protocol and cryptography documentation

### Adding Documentation
1. Create markdown files in the appropriate `website/docs/` subdirectory
2. Add frontmatter with title, description, and sidebar position
3. Update `website/sidebars.ts` if needed for navigation

The website supports versioning and will be automatically deployed when commits are made to `develop` or when tags are created.

## Additional Resources

- `docs/handover/` - Comprehensive architecture documentation
- `ARCHITECTURE.md` - Migration guide for old vs new style
- Component-specific `summary.md` files throughout the codebase
- `website/` - Docusaurus documentation website

## Claude Development Guidelines

This section contains specific instructions for Claude when working on this
project.

### Formatting Commands

After making any code modifications, run the appropriate formatting commands:

#### Markdown Files

- **Format**: Run `make format-md` after modifying any markdown files
- **Check**: Run `make check-md` to verify markdown files are formatted
  correctly

#### Rust and TOML Files

- **Format**: Run `make format` after modifying any Rust (.rs) or TOML (.toml)
  files
- **Check**: Run `make check-format` to verify Rust and TOML files are formatted
  correctly

### Commit Guidelines

**NEVER** add Claude as a co-author in commit messages. Do not include:

- `Co-Authored-By: Claude <noreply@anthropic.com>`
- Any other co-author attribution for Claude

**NEVER** use emojis in commit messages.

**Always** wrap commit message titles at 80 characters and body text at 80
characters.

Always verify commit messages before committing and remove any co-author lines
referencing Claude.

### Development Workflow

1. Make your code changes
2. Run the appropriate formatting command based on file types modified
3. Verify formatting with check commands if needed
4. **Verify commit message does not include Claude as co-author**
5. **Verify commit message contains no emojis and follows 80-character wrap**
6. Proceed with testing or committing changes
