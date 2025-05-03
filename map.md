# OpenMina Project Component Map

This document outlines the high-level components of the OpenMina node and indicates whether they are primarily implemented within the core state machine (Redux-style store) or as separate services/modules interacting with the state machine, following the architecture described in [`ARCHITECTURE.md`](ARCHITECTURE.md).

## State Machine vs. Service Components

The architecture distinguishes between components primarily responsible for managing the application's state and logic (State Machine Components) and those handling interactions with the external environment or performing heavy tasks (Service Components). This separation is reinforced by the distinction between *stateful* and *effectful* actions.

*   **State Machine Components (Stateful Actions):**
    *   Manage the core application `State` through *stateful* `Actions`.
    *   Logic resides in `reducer` functions which receive a `Substate` context (providing controlled access to state and the dispatcher) to compute the next state based on the action.
    *   Contain the primary business logic and control flow, designed for determinism.
    *   Interact with services indirectly by dispatching *effectful* actions.

*   **Service Components (Effectful Actions):**
    *   Handle interactions with the "outside world" (e.g., network, disk, intensive computations, external processes).
    *   Interactions are triggered by *effectful* `Actions` dispatched from state machine components.
    *   The `effects` functions associated with effectful actions act as thin wrappers that call the appropriate `Service` methods.
    *   Services themselves contain the I/O or computational logic and aim for minimal internal state; decision-making belongs in state machine components.
    *   Communicate results or occurrences back to the state machine primarily through `Events`, which are then wrapped in actions (like `EventSourceNewEventAction`) and dispatched.

This division ensures core state management is deterministic and testable, while side effects are handled controllably and decoupled.

## Testing Framework and the Role of the State Machine

The testing framework aims to ensure the node's correctness, security, and performance, with a strong focus on P2P interactions and OCaml node interoperability. It employs several approaches:

*   **Scenarios:** Specific network setups and event sequences defined in code (`node/testing/src/scenarios/`) to verify particular behaviors (e.g., connection handling, peer discovery).
*   **Simulator:** A network simulator (`node/testing/src/simulator/`) creates controlled environments with multiple nodes (Rust and potentially mocked/real OCaml) for deterministic testing of complex interactions.
*   **Solo Node Tests:** Deploying a single Rust node against an existing OCaml network (like a testnet) to validate real-world interoperability.
*   **Multi-Node Tests:** Deploying networks of only Rust nodes to test Rust-specific interactions in isolation.

The state machine architecture is fundamental to this testing approach:

*   **Determinism:** Because state transitions are purely determined by the current state and the incoming action (`reducer(state, action) -> state`), the state machine's behavior is predictable. Given the same initial state and sequence of inputs (actions, events), it will always produce the same final state and sequence of effects.
*   **Input Recording & Replay:** The deterministic nature allows all inputs (events, time-based triggers) to the state machine to be recorded. These recordings can be replayed later (`cli/src/commands/replay/`), exactly reproducing a specific execution flow. This is invaluable for debugging complex scenarios or failures observed in testing or production.
*   **Separation of Concerns:** The clear distinction between pure state logic (reducers for stateful actions) and side effects (effects functions for effectful actions calling services) makes testing easier. State logic can be tested in isolation without needing real services, while service interactions can be mocked or tested separately.
*   **Testable State:** Tests can directly inspect the state machine's state (`p2p/src/`, `node/src/state.rs`) at various points during a scenario to verify that components like peer management, Kademlia, or connection states are correct according to the expected behavior.
*   **Invariant Checking:** The testing framework also utilizes invariant checks. Invariants are rules or conditions about the system's state that must always hold true. During tests (especially fuzzing, see `tools/fuzzing/`), the framework checks the state *before* and *after* operations. It verifies that the state transitions performed by the state machine adhere to fundamental protocol rules (e.g., ensuring account modifications respect permissions, as seen in `tools/fuzzing/src/transaction_fuzzer/invariants.rs`). The `InvariantService` trait (`core/src/invariants.rs`) provides infrastructure for managing invariant-related state within the testing environment. If an invariant is violated, it indicates a bug in the state transition logic.

This combination of simulation, real-world testing, and leveraging the state machine's properties allows for comprehensive validation of the node's behavior under diverse conditions.

## Core Node ([`node/`](node/))

### State Machine Components

*   **Core State/Reducer/Action ([`node/src/state.rs`](node/src/state.rs), [`node/src/reducer.rs`](node/src/reducer.rs), [`node/src/action.rs`](node/src/action.rs))**: Manages the overall node state, transitions, and dispatches actions.
    * Interacts with: P2P Service, Block Production Service, SNARK Service, Ledger Service, RPC Service, External SNARK Worker Service
*   **Block Producer ([`node/src/block_producer/`](node/src/block_producer/), [`node/src/block_producer_effectful/`](node/src/block_producer_effectful/))**: Manages block production scheduling and state (stateful part), and handles interactions with provers (effectful part).
    * Interacts with: Block Production Service
*   **External SNARK Worker Coordination ([`node/src/external_snark_worker/`](node/src/external_snark_worker/), [`node/src/external_snark_worker_effectful/`](node/src/external_snark_worker_effectful/))**: Manages external worker state (stateful part) and handles communication with external SNARK workers (effectful part).
    * Interacts with: External SNARK Worker Service
*   **RPC ([`node/src/rpc/`](node/src/rpc/), [`node/src/rpc_effectful/`](node/src/rpc_effectful/))**: Handles RPC request state (stateful part) and handles RPC request processing and transport interaction (effectful part).
    * Interacts with: RPC Service
*   **Ledger Interaction ([`node/src/ledger/`](node/src/ledger/), [`node/src/ledger_effectful/`](node/src/ledger_effectful/))**: Manages state related to ledger operations (stateful part) and handles interactions with the ledger database/library (effectful part).
    * Interacts with: Ledger Service
*   **Snark Pool ([`node/src/snark_pool/`](node/src/snark_pool/))**: Manages the pool of available SNARK work.
    * Interacts with: SNARK Service
*   **Transaction Pool ([`node/src/transaction_pool/`](node/src/transaction_pool/))**: Manages the state of pending transactions.
    * Interacts with: Ledger Service (for validation)
*   **Transition Frontier ([`node/src/transition_frontier/`](node/src/transition_frontier/))**: Manages the blockchain state, consensus, and block application.
    * Interacts with: Ledger Service, P2P Service
*   **Event Source ([`node/src/event_source/`](node/src/event_source/))**: Handles incoming events from services and integrates them into the state machine.
    * Interacts with: All Services (indirectly via the Event Source Service)
*   **Watched Accounts ([`node/src/watched_accounts/`](node/src/watched_accounts/))**: Manages accounts being watched for changes.
    * Interacts with: Ledger Service

### Service Components

*   **Block Production:**
    *   **Block Production Service ([`node/src/block_producer_effectful/block_producer_effectful_service.rs`](node/src/block_producer_effectful/block_producer_effectful_service.rs))**: Defines the interface for interacting with provers during block production.
    *   **Block Producer VRF Evaluator Service ([`node/src/block_producer_effectful/vrf_evaluator_effectful/block_producer_vrf_evaluator_effectful_service.rs`](node/src/block_producer_effectful/vrf_evaluator_effectful/block_producer_vrf_evaluator_effectful_service.rs))**: Defines the interface for evaluating VRFs during block production.
*   **External SNARK Worker Service ([`node/src/external_snark_worker_effectful/external_snark_worker_service.rs`](node/src/external_snark_worker_effectful/external_snark_worker_service.rs))**: Defines the interface for communicating with external SNARK workers.
*   **RPC Service ([`node/src/rpc_effectful/rpc_service.rs`](node/src/rpc_effectful/rpc_service.rs))**: Defines the interface for handling RPC requests.
*   **Ledger Service ([`node/src/ledger/ledger_service.rs`](node/src/ledger/ledger_service.rs))**: Defines the interface for interacting with the ledger database and core logic.
*   **SNARK Pool Service ([`node/src/snark_pool/snark_pool_service.rs`](node/src/snark_pool/snark_pool_service.rs))**: Defines the interface for interacting with the SNARK pool (e.g., adding work, getting work).
*   **Transaction Pool Service ([`node/src/transaction_pool/transaction_pool_service.rs`](node/src/transaction_pool/transaction_pool_service.rs))**: Defines the interface for interacting with the transaction pool (e.g., adding transactions).
*   **Transition Frontier:**
    *   **Sync Ledger Staged Service ([`node/src/transition_frontier/sync/ledger/staged/transition_frontier_sync_ledger_staged_service.rs`](node/src/transition_frontier/sync/ledger/staged/transition_frontier_sync_ledger_staged_service.rs))**: Defines the interface for syncing the staged ledger part of the transition frontier.
    *   **Sync Ledger Snarked Service ([`node/src/transition_frontier/sync/ledger/snarked/transition_frontier_sync_ledger_snarked_service.rs`](node/src/transition_frontier/sync/ledger/snarked/transition_frontier_sync_ledger_snarked_service.rs))**: Defines the interface for syncing the SNARKed ledger part of the transition frontier.
    *   **Archive Service ([`node/src/transition_frontier/archive/archive_service.rs`](node/src/transition_frontier/archive/archive_service.rs))**: Defines the interface for interacting with the transition frontier archive.
    *   **Genesis Service ([`node/src/transition_frontier/genesis_effectful/transition_frontier_genesis_service.rs`](node/src/transition_frontier/genesis_effectful/transition_frontier_genesis_service.rs))**: Defines the interface for handling genesis-related operations for the transition frontier.
*   **Event Source Service ([`node/src/event_source/event_source_service.rs`](node/src/event_source/event_source_service.rs))**: Defines the interface for receiving events from various node services.
*   **Error Sink Service ([`node/src/error_sink/error_sink_service.rs`](node/src/error_sink/error_sink_service.rs))**: Defines the interface for reporting and handling internal errors.

## P2P ([`p2p/`](p2p/))

### State Machine Components

*   **Core State/Reducer/Action ([`p2p/src/p2p_state.rs`](p2p/src/p2p_state.rs), [`p2p/src/p2p_reducer.rs`](p2p/src/p2p_reducer.rs), [`p2p/src/p2p_actions.rs`](p2p/src/p2p_actions.rs))**: Manages P2P-specific state (peers, connections, channels).
    * Interacts with: P2P Service Layer
*   **Connection Management ([`p2p/src/connection/`](p2p/src/connection/))**: Manages state related to peer connections.
    * Interacts with: P2P Service Layer
*   **Disconnection ([`p2p/src/disconnection/`](p2p/src/disconnection/), [`p2p/src/disconnection_effectful/`](p2p/src/disconnection_effectful/))**: Manages disconnection state (stateful part) and handles service interactions for disconnection (effectful part).
    * Interacts with: P2P Disconnection Service
*   **Channels ([`p2p/src/channels/`](p2p/src/channels/))**: Manages P2P communication channels (Best Practices, RPC, etc.).
    * Interacts with: P2P Service Layer
*   **Libp2p Identify Protocol ([`p2p/src/identify/`](p2p/src/identify/))**: Handles the state and logic for the standard libp2p Identify protocol exchange.
    * Interacts with: P2P Service Layer
*   **Peer Management ([`p2p/src/peer/`](p2p/src/peer/))**: Manages state related to known peers.
    * Interacts with: P2P Service Layer
*   **Network Layer**: Components managing interactions with the underlying libp2p network stack.
    *   **Private Network (Pnet) ([`p2p/src/network/pnet/`](p2p/src/network/pnet/), [`p2p/src/network/pnet_effectful/`](p2p/src/network/pnet_effectful/))**: Manages state for private network configuration (stateful part) and interacts with the network layer for enforcement (effectful part).
        * Interacts with: Network Layer Service
    *   **Network Scheduler ([`p2p/src/network/scheduler/`](p2p/src/network/scheduler/), [`p2p/src/network/scheduler_effectful/`](p2p/src/network/scheduler_effectful/))**: Manages state for scheduling network events (stateful part) and interacts with the underlying timer/event system (effectful part).
        * Interacts with: Network Layer Service
    *   **Kademlia DHT ([`p2p/src/network/kad/`](p2p/src/network/kad/))**: Manages state related to peer discovery and routing via Kademlia (e.g., routing table).
        * Interacts with: Network Layer Service
    *   **Gossipsub ([`p2p/src/network/pubsub/`](p2p/src/network/pubsub/))**: Manages state related to message broadcasting and topic subscriptions.
        * Interacts with: Network Layer Service
    *   **Network RPC ([`p2p/src/network/rpc/`](p2p/src/network/rpc/))**: Manages state for ongoing RPC requests/responses within the network layer.
        * Interacts with: Network Layer Service
    *   **Network Identify ([`p2p/src/network/identify/`](p2p/src/network/identify/))**: Manages state related to the libp2p Identify protocol execution (e.g., received peer info).
        * Interacts with: Network Layer Service
    *   **Noise Protocol ([`p2p/src/network/noise/`](p2p/src/network/noise/))**: Manages state during the Noise protocol handshake for secure channel establishment.
        * Interacts with: Network Layer Service
    *   **Protocol Selection ([`p2p/src/network/select/`](p2p/src/network/select/))**: Manages state during protocol negotiation (e.g., multistream-select).
        * Interacts with: Network Layer Service
    *   **Yamux Multiplexing ([`p2p/src/network/yamux/`](p2p/src/network/yamux/))**: Manages state for multiplexed streams over a single connection.
        * Interacts with: Network Layer Service

### Service Components

*   **P2P Service Layer ([`p2p/src/p2p_service.rs`](p2p/src/p2p_service.rs), [`p2p/src/service_impl/`](p2p/src/service_impl/))**: Implements the P2P service interface, handling network I/O and protocol multiplexing.
*   **P2P Channels Service ([`p2p/src/channels/p2p_channels_service.rs`](p2p/src/channels/p2p_channels_service.rs))**: Defines the interface for managing P2P communication channels.
*   **P2P Disconnection Service ([`p2p/src/disconnection_effectful/p2p_disconnection_effectful_service.rs`](p2p/src/disconnection_effectful/p2p_disconnection_effectful_service.rs))**: Defines the interface for triggering peer disconnections.
*   **Network Layer Service ([`p2p/src/network/p2p_network_service.rs`](p2p/src/network/p2p_network_service.rs))**: Defines the service interface for interacting with the underlying libp2p network stack.

## SNARK ([`snark/`](snark/))

### State Machine Components

*   **Core State/Reducer/Action ([`snark/src/snark_state.rs`](snark/src/snark_state.rs), [`snark/src/snark_reducer.rs`](snark/src/snark_reducer.rs), [`snark/src/snark_actions.rs`](snark/src/snark_actions.rs))**: Manages overall SNARK-related state within this crate.
    * Interacts with: SNARK Verifier/Prover Services
*   **Block Verification ([`snark/src/block_verify/`](snark/src/block_verify/), [`snark/src/block_verify_effectful/`](snark/src/block_verify_effectful/))**: Manages block verification state (stateful part) and calls the verifier service (effectful part).
    * Interacts with: SNARK Block Verifier Service (likely part of Ledger Service)
*   **Work Verification ([`snark/src/work_verify/`](snark/src/work_verify/), [`snark/src/work_verify_effectful/`](snark/src/work_verify_effectful/))**: Manages SNARK work verification state (stateful part) and calls the verifier service (effectful part).
    * Interacts with: SNARK Work Verifier Service
*   **User Command Verification ([`snark/src/user_command_verify/`](snark/src/user_command_verify/), [`snark/src/user_command_verify_effectful/`](snark/src/user_command_verify_effectful/))**: Manages transaction verification state (stateful part) and calls the verifier service (effectful part).
    * Interacts with: SNARK User Command Verifier Service

### Service Components

*   **SNARK Block Verify Service ([`snark/src/block_verify_effectful/snark_block_verify_service.rs`](snark/src/block_verify_effectful/snark_block_verify_service.rs))**: Defines the interface for verifying SNARKs related to blocks.
*   **SNARK Work Verifier Service ([`snark/src/work_verify_effectful/snark_work_verify_service.rs`](snark/src/work_verify_effectful/snark_work_verify_service.rs))**: Defines the interface for verifying completed SNARK work.
*   **SNARK User Command Verifier Service ([`snark/src/user_command_verify_effectful/snark_user_command_verify_service.rs`](snark/src/user_command_verify_effectful/snark_user_command_verify_service.rs))**: Defines the interface for verifying SNARKs within user commands (transactions).
*   *(Note: The SNARK Block Verifier service interface is defined above. Block verification is tightly coupled with ledger state but has its own service interface within the `snark` crate.)*

## Interfaces

*   **CLI ([`cli/`](cli/))**: Command-line interface. Interacts with the node primarily via the RPC service.
    *   **Implemented:** Interface (External Application)
*   **Frontend ([`frontend/`](frontend/))**: Web-based dashboard. Interacts with the node via the RPC service.
    *   **Implemented:** Interface (External Application)

## Supporting Components

*   **Ledger ([`ledger/`](ledger/))**: Standalone crate providing the library for managing the blockchain's account state (Merkle tree), transaction application logic, scan state, zkApp handling, and ledger proofs/verification.
    *   **Core State & Database ([`ledger/src/base.rs`](ledger/src/base.rs), [`ledger/src/database/`](ledger/src/database/), [`ledger/src/sparse_ledger/`](ledger/src/sparse_ledger/), [`ledger/src/mask/`](ledger/src/mask/))**: Library code for Merkle tree structure, account data ([`ledger/src/account/`](ledger/src/account/)), and database interaction logic.
        *   **Implemented:** Library (Data structures & DB logic called via Service Interaction)
    *   **Staged Ledger & Transaction Application ([`ledger/src/staged_ledger/`](ledger/src/staged_ledger/), [`ledger/src/scan_state/`](ledger/src/scan_state/))**: Library code defining how transactions are applied, diffs created, and scan state managed.
        *   **Implemented:** Library (Logic invoked by State Machine)
    *   **zkApp Logic ([`ledger/src/zkapps/`](ledger/src/zkapps/))**: Library code for handling zkApp transactions and state updates.
        *   **Implemented:** Library (Logic invoked by State Machine)
    *   **Ledger Proofs ([`ledger/src/proofs/`](ledger/src/proofs/))**: Library code for generating/handling ledger-specific proofs.
        *   **Implemented:** Library (Called via Service Interaction)
    *   **Ledger Verifier ([`ledger/src/verifier/`](ledger/src/verifier/))**: Library code for verifying ledger-specific proofs.
        *   **Implemented:** Library (Called via Service Interaction)
*   **Core Types ([`core/`](core/))**: Basic data structures and types.
    *   **Implemented:** Library
*   **Cryptography ([`vrf/`](vrf/), [`poseidon/`](poseidon/))**: Cryptographic primitives.
    *   **Implemented:** Library
*   **Serialization ([`mina-p2p-messages/`](mina-p2p-messages/))**: Network message definitions and serialization.
    *   **Implemented:** Library

**Note:** The distinction between "State Machine" and "Service/Library" reflects the primary mode of operation. The state machine ([`node/src/`](node/src/)) controls the overall flow and logic, dispatching actions. Effectful actions ([`node/src/*_effectful/`](node/src/*_effectful/)) trigger interactions with Services (like P2P networking, external provers) or Libraries (like the [`ledger/`](ledger/) crate). Heavy computations or I/O within libraries (like ledger DB access or proof generation) are typically handled via service interactions.
