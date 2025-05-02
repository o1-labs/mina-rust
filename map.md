# OpenMina Project Component Map

## Component Implementation Summary

| Component                         | State Machine Logic                               | Service / Library Logic                                         |
| :-------------------------------- | :------------------------------------------------ | :-------------------------------------------------------------- |
| P2P Networking                  | Peer/Connection/Channel State Management          | Network I/O (Service), Protocol Handling (Service)              |
| Block Production                | Scheduling Logic, Production State                | Prover Interaction (Service)                                    |
| SNARK Work/Verification         | Job Tracking, Pool Management, Worker State       | Verification/Proving (Service), External Comm (Service)         |
| Ledger Interaction              | Transaction/Block Application Control Flow        | Core Ledger Ops (Library), DB I/O (Service), Proofs (Service) |
| RPC Interface                   | Request State Tracking                            | HTTP Transport (Service), Request Processing (Service)          |
| Transition Frontier (Consensus) | Consensus Rules, Block Selection, Chain State     | *(Relies on Ledger/P2P services)*                               |
| Transaction Pool                | Pool State Management (Pending Txs)               | Validation Logic (Ledger Library via Service)                   |

---

This document outlines the high-level components of the OpenMina node and indicates whether they are primarily implemented within the core state machine (Redux-style store) or as separate services/modules interacting with the state machine, following the architecture described in [`ARCHITECTURE.md`](ARCHITECTURE.md).

## Core Node ([`node/`](node/))

*   **State Machine Core ([`node/src/state.rs`](node/src/state.rs), [`node/src/reducer.rs`](node/src/reducer.rs), [`node/src/action.rs`](node/src/action.rs))**: Manages the overall node state, transitions, and dispatches actions.
    *   **Implemented:** Within the State Machine
*   **Service Layer ([`node/src/service.rs`](node/src/service.rs))**: Orchestrates various services and interacts with the state machine via actions and state access.
    *   **Implemented:** Service (acts as a coordinator)
*   **P2P Integration ([`node/src/p2p/`](node/src/p2p/))**: State machine logic for handling P2P events and dispatching P2P actions within the main node context.
    *   **Implemented:** Within the State Machine
*   **Block Producer ([`node/src/block_producer/`](node/src/block_producer/), [`node/src/block_producer_effectful/`](node/src/block_producer_effectful/))**: Stateful logic for block production scheduling and state management (`block_producer/`), and effectful logic for interacting with services like provers (`block_producer_effectful/`).
    *   **Implemented:** State Machine & Service Interaction
*   **SNARK Integration ([`node/src/snark/`](node/src/snark/))**: State machine logic for handling SNARK-related events (like verification results) and dispatching actions to the SNARK component.
    *   **Implemented:** Within the State Machine
*   **External SNARK Worker Coordination ([`node/src/external_snark_worker/`](node/src/external_snark_worker/), [`node/src/external_snark_worker_effectful/`](node/src/external_snark_worker_effectful/))**: Stateful logic for managing external worker state (`external_snark_worker/`) and effectful logic for communication (`external_snark_worker_effectful/`).
    *   **Implemented:** State Machine & Service Interaction
*   **RPC ([`node/src/rpc/`](node/src/rpc/), [`node/src/rpc_effectful/`](node/src/rpc_effectful/))**: Stateful logic for handling RPC request state (`rpc/`) and effectful logic for processing requests and interacting with the underlying transport (`rpc_effectful/`).
    *   **Implemented:** State Machine & Service Interaction
*   **Ledger Interaction ([`node/src/ledger/`](node/src/ledger/), [`node/src/ledger_effectful/`](node/src/ledger_effectful/))**: Stateful logic for ledger-related operations (`ledger/`) and effectful logic for interacting with the ledger service/database (`ledger_effectful/`).
    *   **Implemented:** State Machine & Service Interaction
*   **Snark Pool ([`node/src/snark_pool/`](node/src/snark_pool/))**: Manages the pool of available SNARK work.
    *   **Implemented:** Within the State Machine
*   **Transaction Pool ([`node/src/transaction_pool/`](node/src/transaction_pool/))**: Manages the pool of pending transactions. The state machine tracks the transactions in the pool, while the core validation logic (determining which transactions are valid according to the current ledger state) resides in the [`ledger/`](ledger/) library and is invoked via service calls.
    *   **Implemented:** State Machine & Library/Service Interaction
*   **Transition Frontier ([`node/src/transition_frontier/`](node/src/transition_frontier/))**: Manages the blockchain state, including consensus and block application.
    *   **Implemented:** Within the State Machine
*   **Event Source ([`node/src/event_source/`](node/src/event_source/))**: Handles incoming events from services.
    *   **Implemented:** Within the State Machine
*   **HTTP Server ([`node/src/http/`](node/src/http/))**: Provides the HTTP interface for RPC and other interactions.
    *   **Implemented:** Service
*   **Logger ([`node/src/logger/`](node/src/logger/))**: Handles logging.
    *   **Implemented:** Service
*   **Recorder ([`node/src/recorder/`](node/src/recorder/))**: Records state machine actions and events for replay/debugging.
    *   **Implemented:** Service
*   **Stats ([`node/src/stats/`](node/src/stats/))**: Gathers and potentially exposes node statistics.
    *   **Implemented:** Service
*   **Watched Accounts ([`node/src/watched_accounts/`](node/src/watched_accounts/))**: Manages accounts being watched for changes.
    *   **Implemented:** Within the State Machine

## P2P ([`p2p/`](p2p/))

*   **State Machine Core ([`p2p/src/p2p_state.rs`](p2p/src/p2p_state.rs), [`p2p/src/p2p_reducer.rs`](p2p/src/p2p_reducer.rs), [`p2p/src/p2p_actions.rs`](p2p/src/p2p_actions.rs))**: Manages P2P-specific state (peers, connections, channels).
    *   **Implemented:** Within the State Machine
*   **Service Layer ([`p2p/src/p2p_service.rs`](p2p/src/p2p_service.rs), [`p2p/src/service_impl/`](p2p/src/service_impl/))**: Implements the P2P service interface, handling network I/O.
    *   **Implemented:** Service
*   **Connection Management ([`p2p/src/connection/`](p2p/src/connection/))**: Stateful logic for connection states.
    *   **Implemented:** Within the State Machine (interacts heavily with Service)
*   **Disconnection ([`p2p/src/disconnection/`](p2p/src/disconnection/), [`p2p/src/disconnection_effectful/`](p2p/src/disconnection_effectful/))**: Stateful logic for disconnection (`disconnection/`) and effectful service interactions (`disconnection_effectful/`).
    *   **Implemented:** State Machine & Service Interaction
*   **Channels ([`p2p/src/channels/`](p2p/src/channels/))**: Manages P2P communication channels (Best Practices, RPC, etc.).
    *   **Implemented:** Within the State Machine
*   **Libp2p Identify Protocol ([`p2p/src/identify/`](p2p/src/identify/))**: Handles the state and logic for the standard libp2p Identify protocol exchange.
    *   **Implemented:** State Machine & Service Interaction
*   **Peer Identity Management ([`p2p/src/identity/`](p2p/src/identity/))**: Manages the node's cryptographic P2P identity (keys).
    *   **Implemented:** Service
*   **Network Layer ([`p2p/src/network/`](p2p/src/network/))**: Abstraction over the network transport layer, including lower-level identification.
    *   **Network Identify ([`p2p/src/network/identify/`](p2p/src/network/identify/))**: Handles identification aspects within the network transport layer.
        *   **Implemented:** Service
    *   **Implemented:** Service
*   **Peer Management ([`p2p/src/peer/`](p2p/src/peer/))**: Stateful logic for managing peer information.
    *   **Implemented:** Within the State Machine
*   **WebRTC ([`p2p/src/webrtc/`](p2p/src/webrtc/))**: Handles WebRTC signaling and connections.
    *   **Implemented:** Service

## SNARK ([`snark/`](snark/))

*   **State Machine Core ([`snark/src/snark_state.rs`](snark/src/snark_state.rs), [`snark/src/snark_reducer.rs`](snark/src/snark_reducer.rs), [`snark/src/snark_actions.rs`](snark/src/snark_actions.rs))**: Manages overall SNARK-related state.
    *   **Implemented:** Within the State Machine
*   **Block Verification ([`snark/src/block_verify/`](snark/src/block_verify/), [`snark/src/block_verify_effectful/`](snark/src/block_verify_effectful/))**: Stateful logic for tracking verification jobs (`block_verify/`) and effectful logic for calling the verifier service (`block_verify_effectful/`).
    *   **Implemented:** State Machine & Service Interaction
*   **Work Verification ([`snark/src/work_verify/`](snark/src/work_verify/), [`snark/src/work_verify_effectful/`](snark/src/work_verify_effectful/))**: Stateful logic for tracking SNARK work verification jobs (`work_verify/`) and effectful logic for calling the verifier service (`work_verify_effectful/`).
    *   **Implemented:** State Machine & Service Interaction
*   **User Command Verification ([`snark/src/user_command_verify/`](snark/src/user_command_verify/), [`snark/src/user_command_verify_effectful/`](snark/src/user_command_verify_effectful/))**: Stateful logic for tracking transaction verification jobs (`user_command_verify/`) and effectful logic for calling the verifier service (`user_command_verify_effectful/`).
    *   **Implemented:** State Machine & Service Interaction

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
