//! # Network Configuration
//!
//! This module provides network-specific configurations for different Mina blockchain networks.
//! OpenMina supports multiple networks, each with distinct parameters that define their behavior,
//! security settings, and connectivity requirements.
//!
//! ## Overview
//!
//! Network configurations define all the parameters needed to participate in a specific Mina
//! blockchain network. Each network configuration includes:
//!
//! - **Chain ID**: Unique identifier computed from network parameters
//! - **Protocol Constants**: Timing, fees, and consensus parameters
//! - **Circuit Configurations**: zkSNARK proving keys and circuit hashes
//! - **Seed Peers**: Initial connection points for network discovery
//! - **Cryptographic Parameters**: Signature schemes and hash functions
//!
//! ## Supported Networks
//!
//! ### Devnet
//!
//! Devnet is the primary development and testing network for Mina Protocol:
//!
//! - **Network ID**: `TESTNET` (0x00)
//! - **Purpose**: Development, testing, and integration
//! - **Characteristics**: Regular resets, development-friendly parameters
//! - **Seed Nodes**: o1Labs-operated infrastructure
//! - **Chain ID**: `29936104443aaf264a7f0192ac64b1c7173198c1ed404c1bcff5e562e05eb7f6`
//!
//! ### Mainnet
//!
//! Mainnet is the production Mina blockchain where real value transactions occur:
//!
//! - **Network ID**: `MAINNET` (0x01)
//! - **Purpose**: Production blockchain operations
//! - **Characteristics**: Stable, long-term operation with production security
//! - **Seed Nodes**: Community-operated, distributed infrastructure
//! - **Chain ID**: `a7351abc7ddf2ea92d1b38cc8e636c271c1dfd2c081c637f62ebc2af34eb7cc1`
//!
//! ## Usage
//!
//! ### Basic Configuration
//!
//! Network configuration must be initialized once at application startup:
//!
//! ```rust
//! use openmina_core::network::NetworkConfig;
//!
//! # fn main() -> Result<(), String> {
//! // Initialize for devnet
//! NetworkConfig::init("devnet")?;
//!
//! // Or initialize for mainnet (don't do both in the same program)
//! // NetworkConfig::init("mainnet")?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Accessing Configuration
//!
//! After initialization, access the global configuration:
//!
//! ```rust
//! use openmina_core::network::NetworkConfig;
//! # NetworkConfig::init("devnet").unwrap();
//!
//! let config = NetworkConfig::global();
//! println!("Connected to: {}", config.name);
//! println!("Network ID: {:?}", config.network_id);
//! ```
//!
//! ### Chain ID Usage
//!
//! Chain IDs uniquely identify network configurations:
//!
//! ```rust
//! use openmina_core::{ChainId, DEVNET_CHAIN_ID, MAINNET_CHAIN_ID};
//!
//! // Use predefined chain IDs
//! let devnet_id = DEVNET_CHAIN_ID;
//! println!("Devnet Chain ID: {}", devnet_id.to_hex());
//!
//! // Generate preshared key for libp2p
//! let psk = devnet_id.preshared_key();
//! println!("PSK length: {}", psk.len());
//! ```
//!
//! ### Network Differences
//!
//! Networks differ in several key parameters:
//!
//! | Parameter | Devnet | Mainnet |
//! |-----------|--------|---------|
//! | Signature Prefix | `CodaSignature` | `MinaSignatureMainnet` |
//! | zkApp Hash Param | `TestnetZkappBody` | `MainnetZkappBody` |
//! | Network ID | `TESTNET` (0x00) | `MAINNET` (0x01) |
//!
//! ## Implementation Details
//!
//! ### Chain ID Computation
//!
//! Chain IDs are computed deterministically from network parameters:
//!
//! ```rust
//! use openmina_core::ChainId;
//! use mina_p2p_messages::v2::{StateHash, UnsignedExtendedUInt32StableV1};
//! # use openmina_core::constants::PROTOCOL_CONSTANTS;
//! # use openmina_core::network::devnet;
//! # use openmina_core::constants::{PROTOCOL_TRANSACTION_VERSION, PROTOCOL_NETWORK_VERSION, TX_POOL_MAX_SIZE};
//!
//! // Example computation (simplified)
//! let genesis_state_hash: StateHash = "3NL93SipJfAMNDBRfQ8Uo8LPovC74mnJZfZYB5SK7mTtkL72dsPx".parse().unwrap();
//! let chain_id = ChainId::compute(
//!     &devnet::CONSTRAINT_SYSTEM_DIGESTS,
//!     &genesis_state_hash,
//!     &PROTOCOL_CONSTANTS,
//!     PROTOCOL_TRANSACTION_VERSION,
//!     PROTOCOL_NETWORK_VERSION,
//!     &UnsignedExtendedUInt32StableV1::from(TX_POOL_MAX_SIZE),
//! );
//! ```
//!
//! ### Circuit Configuration
//!
//! Each network specifies its zkSNARK circuits:
//!
//! ```rust
//! use openmina_core::network::{NetworkConfig, CircuitsConfig};
//! # NetworkConfig::init("devnet").unwrap();
//!
//! let config = NetworkConfig::global();
//! let circuits = config.circuits_config;
//! println!("Circuit directory: {}", circuits.directory_name);
//! println!("Transaction step circuit: {}", circuits.step_transaction_gates);
//! ```
//!
//! ## Best Practices
//!
//! ### Development
//!
//! 1. **Use Devnet**: Always develop and test on devnet first
//! 2. **Verify Chain ID**: Ensure your node connects to the expected network
//! 3. **Check Peer Connectivity**: Verify seed peers are accessible
//! 4. **Monitor Network Changes**: Devnet may reset; monitor announcements
//!
//! ### Production
//!
//! 1. **Use Mainnet**: Only use mainnet for production applications
//! 2. **Diversify Peers**: Don't rely on single seed peer providers
//! 3. **Validate Configuration**: Double-check network parameters before deployment
//! 4. **Monitor Chain ID**: Ensure consistency across your infrastructure
//!
//! ### Configuration Management
//!
//! 1. **Initialize Early**: Set network configuration at application startup
//! 2. **Handle Errors**: Properly handle initialization failures
//! 3. **Log Network Info**: Log which network you're connecting to
//! 4. **Avoid Double Init**: Don't attempt to reinitialize network configuration
//!
//! ## Troubleshooting
//!
//! ### Common Issues
//!
//! **Wrong Network**: If you see unexpected behavior, verify the chain ID matches your intended network:
//!
//! ```rust
//! use openmina_core::network::NetworkConfig;
//! use openmina_core::{DEVNET_CHAIN_ID, MAINNET_CHAIN_ID};
//! # NetworkConfig::init("devnet").unwrap();
//!
//! // Check current network
//! let config = NetworkConfig::global();
//! println!("Network: {}", config.name);
//! println!("Network ID: {:?}", config.network_id);
//!
//! // Verify chain ID
//! let expected_chain_id = match config.name {
//!     "devnet" => DEVNET_CHAIN_ID,
//!     "mainnet" => MAINNET_CHAIN_ID,
//!     _ => panic!("Unknown network"),
//! };
//! println!("Chain ID: {}", expected_chain_id.to_hex());
//! ```
//!
//! **Peer Connection Failures**: Check that seed peers are accessible from your network location.
//!
//! **Circuit Mismatches**: Ensure your proving keys match the network's constraint system digests.
//!
//! **Double Initialization**: Only call `NetworkConfig::init()` once per application instance.
//!
//! ## OCaml Compatibility
//!
//! OpenMina's network configurations maintain compatibility with the OCaml Mina implementation.
//! Related OCaml configuration files can be found at:
//! <https://github.com/MinaProtocol/mina/tree/compatible/src/config>

use once_cell::sync::OnceCell;
use poseidon::hash::{
    legacy,
    params::{CODA_SIGNATURE, MAINNET_ZKAPP_BODY, MINA_SIGNATURE_MAINNET, TESTNET_ZKAPP_BODY},
};

use crate::constants::ConstraintConstants;

/// Network identifier used to distinguish between different Mina networks.
/// This enum is derived from mina-signer to avoid dependency.
#[derive(Debug, Clone)]
pub enum NetworkId {
    /// Identifier for all testnet networks (devnet, etc.)
    /// Used for development and testing purposes
    TESTNET = 0x00,

    /// Identifier for the main production network
    /// Used for the live Mina blockchain
    MAINNET = 0x01,
}

/// Complete network configuration for a Mina network.
///
/// This struct contains all the network-specific parameters needed to configure
/// a node for a particular Mina network (mainnet, devnet, etc.).
#[derive(Debug)]
pub struct NetworkConfig {
    /// Human-readable network name (e.g., "mainnet", "devnet")
    pub name: &'static str,

    /// Network identifier distinguishing mainnet from testnets
    pub network_id: NetworkId,

    /// Poseidon hash parameter for current signature scheme
    pub signature_prefix: &'static poseidon::hash::LazyParam,

    /// Poseidon hash parameter for legacy signature compatibility
    pub legacy_signature_prefix: &'static poseidon::hash::LazyParam,

    /// Hash parameter for zkApp account updates
    pub account_update_hash_param: &'static poseidon::hash::LazyParam,

    /// MD5 digests of constraint systems used for circuit verification.
    /// Contains digests for: [transaction-merge, transaction-base, blockchain-step]
    pub constraint_system_digests: &'static [[u8; 16]; 3],

    /// List of default seed peers for initial network bootstrapping
    pub default_peers: Vec<&'static str>,

    /// Circuit-specific configuration including proving key filenames
    pub circuits_config: &'static CircuitsConfig,

    /// Protocol constants that define blockchain behavior
    pub constraint_constants: &'static ConstraintConstants,
}

/// Configuration for zkSNARK circuits and proving keys.
///
/// This struct contains the filenames and directory information for all
/// the proving keys required for different types of SNARK proofs.
#[derive(Debug)]
pub struct CircuitsConfig {
    /// Directory name containing the proving keys for this network
    pub directory_name: &'static str,

    /// Proving key for transaction SNARK step circuit
    pub step_transaction_gates: &'static str,

    /// Proving key for transaction SNARK wrap circuit
    pub wrap_transaction_gates: &'static str,

    /// Proving key for transaction merge step circuit
    pub step_merge_gates: &'static str,

    /// Proving key for blockchain step circuit
    pub step_blockchain_gates: &'static str,

    /// Proving key for blockchain wrap circuit
    pub wrap_blockchain_gates: &'static str,

    /// Proving key for optional signed transaction step circuit
    pub step_transaction_opt_signed_opt_signed_gates: &'static str,

    /// Proving key for optional signed transaction circuit
    pub step_transaction_opt_signed_gates: &'static str,

    /// Proving key for proved transaction circuit
    pub step_transaction_proved_gates: &'static str,
}

static CONFIG: OnceCell<NetworkConfig> = OnceCell::new();

impl NetworkConfig {
    /// Get the globally configured network configuration.
    ///
    /// If no network has been explicitly initialized via `init()`, this will
    /// default to the devnet configuration and log a warning.
    pub fn global() -> &'static Self {
        CONFIG.get_or_init(|| {
            let config = Self::default_config();
            crate::warn!(
                crate::log::system_time();
                kind = "network",
                message = "no network config initialized, using default config",
                config = config.name,
            );
            config
        })
    }

    /// Initialize the global network configuration.
    ///
    /// This must be called once at application startup to set the network
    /// configuration. Subsequent calls will return an error.
    ///
    /// # Arguments
    /// * `network_name` - Name of the network ("devnet" or "mainnet")
    ///
    /// # Returns
    /// * `Ok(())` if initialization succeeded
    /// * `Err(String)` if the network name is unknown or already initialized
    pub fn init(network_name: &str) -> Result<(), String> {
        let config = match network_name {
            "devnet" => Self::devnet_config(),
            "mainnet" => Self::mainnet_config(),
            other => Err(format!("Unknown network {other}"))?,
        };

        CONFIG
            .set(config)
            .map_err(|_| "Double network configuration initialization".to_owned())?;

        Ok(())
    }

    fn default_config() -> Self {
        Self::devnet_config()
    }

    fn mainnet_config() -> Self {
        Self {
            name: mainnet::NAME,
            network_id: mainnet::NETWORK_ID,
            signature_prefix: &MINA_SIGNATURE_MAINNET,
            legacy_signature_prefix: &legacy::params::MINA_SIGNATURE_MAINNET,
            account_update_hash_param: &MAINNET_ZKAPP_BODY,
            constraint_system_digests: &mainnet::CONSTRAINT_SYSTEM_DIGESTS,
            default_peers: mainnet::default_peers(),
            circuits_config: &mainnet::CIRCUITS_CONFIG,
            constraint_constants: &mainnet::CONSTRAINT_CONSTANTS,
        }
    }

    fn devnet_config() -> Self {
        Self {
            name: devnet::NAME,
            network_id: devnet::NETWORK_ID,
            signature_prefix: &CODA_SIGNATURE,
            legacy_signature_prefix: &legacy::params::CODA_SIGNATURE,
            account_update_hash_param: &TESTNET_ZKAPP_BODY,
            constraint_system_digests: &devnet::CONSTRAINT_SYSTEM_DIGESTS,
            default_peers: devnet::default_peers(),
            circuits_config: &devnet::CIRCUITS_CONFIG,
            constraint_constants: &devnet::CONSTRAINT_CONSTANTS,
        }
    }
}

/// Network-specific constants and configurations.
///
/// This module contains the hardcoded parameters for each supported Mina network.
/// Each network has its own set of:
/// - Constraint system digests (for circuit verification)
/// - Protocol constants (timing, fees, etc.)
/// - Default peer addresses
/// - Circuit configurations
/// - Fork information
///
/// Devnet network configuration.
///
/// Devnet is the primary development and testing network for Mina Protocol.
/// It uses testnet parameters and is regularly reset for testing new features.
///
/// Key characteristics:
/// - Uses TESTNET network ID
/// - Has development-friendly timing parameters
/// - Connects to o1Labs-hosted seed nodes
/// - Uses CodaSignature prefix for compatibility
///
/// Related OCaml configuration: <https://github.com/MinaProtocol/mina/tree/compatible/src/config>
pub mod devnet {
    use super::{CircuitsConfig, NetworkId};
    use crate::constants::{ConstraintConstants, ForkConstants};
    use mina_hasher::Fp;

    /// Network identifier for devnet (uses testnet ID)
    pub const NETWORK_ID: NetworkId = NetworkId::TESTNET;

    /// Human-readable name for this network
    pub const NAME: &str = "devnet";

    /// Signature prefix used for Poseidon hashing (legacy compatibility)
    pub const SIGNATURE_PREFIX: &str = "CodaSignature";

    /// Hash parameter for zkApp account updates on testnet
    pub const ACCOUNT_UPDATE_HASH_PARAM: &str = "TestnetZkappBody";

    /// MD5 digests of the constraint systems used for devnet circuit verification.
    ///
    /// These digests must match the circuits used by the network to ensure
    /// compatibility. The array contains digests for:
    /// - Index 0: transaction-merge circuit
    /// - Index 1: transaction-base circuit
    /// - Index 2: blockchain-step circuit
    pub const CONSTRAINT_SYSTEM_DIGESTS: [[u8; 16]; 3] = [
        // transaction-merge
        [
            0xb8, 0x87, 0x9f, 0x67, 0x7f, 0x62, 0x2a, 0x1d, 0x86, 0x64, 0x80, 0x30, 0x70, 0x1f,
            0x43, 0xe1,
        ],
        // transaction-base
        [
            0x3b, 0xf6, 0xbb, 0x8a, 0x97, 0x66, 0x5f, 0xe7, 0xa9, 0xdf, 0x6f, 0xc1, 0x46, 0xe4,
            0xf9, 0x42,
        ],
        // blockchain-step
        [
            0xd0, 0x24, 0xa9, 0xac, 0x78, 0xd4, 0xc9, 0x3a, 0x88, 0x8b, 0x63, 0xfc, 0x85, 0xee,
            0xb6, 0x6a,
        ],
    ];

    pub const CONSTRAINT_CONSTANTS: ConstraintConstants = ConstraintConstants {
        sub_windows_per_window: 11,
        ledger_depth: 35,
        work_delay: 2,
        block_window_duration_ms: 180000,
        transaction_capacity_log_2: 7,
        pending_coinbase_depth: 5,
        coinbase_amount: 720000000000,
        supercharged_coinbase_factor: 1,
        account_creation_fee: 1000000000,
        // TODO(tizoc): This should come from the config file, but
        // it affects the circuits. Since we cannot produce the circuits
        // ourselves right now, we cannot react to changes in this value,
        // so it will be hardcoded for now.
        fork: Some(ForkConstants {
            state_hash: ark_ff::field_new!(
                Fp,
                "7908066420535064797069631664846455037440232590837253108938061943122344055350"
            ),
            blockchain_length: 296371,
            global_slot_since_genesis: 445860,
        }),
    };

    pub const CIRCUITS_CONFIG: CircuitsConfig = CircuitsConfig {
        directory_name: "3.0.1devnet",

        step_transaction_gates: "step-step-proving-key-transaction-snark-transaction-0-c33ec5211c07928c87e850a63c6a2079",
        wrap_transaction_gates:
            "wrap-wrap-proving-key-transaction-snark-b9a01295c8cc9bda6d12142a581cd305",
        step_merge_gates:
            "step-step-proving-key-transaction-snark-merge-1-ba1d52dfdc2dd4d2e61f6c66ff2a5b2f",
        step_blockchain_gates:
            "step-step-proving-key-blockchain-snark-step-0-55f640777b6486a6fd3fdbc3fcffcc60",
        wrap_blockchain_gates:
            "wrap-wrap-proving-key-blockchain-snark-bbecaf158ca543ec8ac9e7144400e669",
        step_transaction_opt_signed_opt_signed_gates: "step-step-proving-key-transaction-snark-opt_signed-opt_signed-2-48925e6a97197028e1a7c1ecec09021d",
        step_transaction_opt_signed_gates:
            "step-step-proving-key-transaction-snark-opt_signed-3-9eefed16953d2bfa78a257adece02d47",
        step_transaction_proved_gates:
            "step-step-proving-key-transaction-snark-proved-4-0cafcbc6dffccddbc82f8c2519c16341",
    };

    /// Returns the list of default seed peers for devnet bootstrapping.
    ///
    /// These are o1Labs-operated seed nodes that help new nodes discover
    /// the devnet network. DNS addresses are preferred over IP addresses
    /// for resilience to infrastructure changes.
    pub fn default_peers() -> Vec<&'static str> {
        vec![
            "/dns4/seed-1.devnet.gcp.o1test.net/tcp/10003/p2p/12D3KooWAdgYL6hv18M3iDBdaK1dRygPivSfAfBNDzie6YqydVbs",
            "/dns4/seed-2.devnet.gcp.o1test.net/tcp/10003/p2p/12D3KooWLjs54xHzVmMmGYb7W5RVibqbwD1co7M2ZMfPgPm7iAag",
            "/dns4/seed-3.devnet.gcp.o1test.net/tcp/10003/p2p/12D3KooWEiGVAFC7curXWXiGZyMWnZK9h8BKr88U8D5PKV3dXciv",
            // "/ip4/34.45.167.81/tcp/10003/p2p/12D3KooWAdgYL6hv18M3iDBdaK1dRygPivSfAfBNDzie6YqydVbs",
            // "/ip4/34.28.194.121/tcp/10003/p2p/12D3KooWLjs54xHzVmMmGYb7W5RVibqbwD1co7M2ZMfPgPm7iAag",
            // "/ip4/34.44.189.148/tcp/10003/p2p/12D3KooWEiGVAFC7curXWXiGZyMWnZK9h8BKr88U8D5PKV3dXciv",
        ]
    }
}

/// Mainnet network configuration.
///
/// Mainnet is the production Mina blockchain network where real value transactions
/// occur. It uses production-grade parameters and connects to a distributed
/// set of community and foundation-operated seed nodes.
///
/// Key characteristics:
/// - Uses MAINNET network ID
/// - Has production timing and security parameters
/// - Connects to diverse community-operated seed nodes
/// - Uses MinaSignatureMainnet prefix
///
/// Related OCaml configuration: <https://github.com/MinaProtocol/mina/tree/compatible/src/config>
pub mod mainnet {
    use super::{CircuitsConfig, NetworkId};
    use crate::constants::{ConstraintConstants, ForkConstants};
    use mina_hasher::Fp;

    /// Network identifier for mainnet (uses mainnet ID)
    pub const NETWORK_ID: NetworkId = NetworkId::MAINNET;

    /// Human-readable name for this network
    pub const NAME: &str = "mainnet";

    /// Signature prefix used for Poseidon hashing on mainnet
    pub const SIGNATURE_PREFIX: &str = "MinaSignatureMainnet";

    /// Hash parameter for zkApp account updates on mainnet
    pub const ACCOUNT_UPDATE_HASH_PARAM: &str = "MainnetZkappBody";

    /// MD5 digests of the constraint systems used for mainnet circuit verification.
    ///
    /// These digests must match the circuits used by the network to ensure
    /// compatibility. The array contains digests for:
    /// - Index 0: transaction-merge circuit
    /// - Index 1: transaction-base circuit
    /// - Index 2: blockchain-step circuit
    ///
    /// Note: These differ from devnet to ensure network separation.
    pub const CONSTRAINT_SYSTEM_DIGESTS: [[u8; 16]; 3] = [
        // transaction-merge
        [
            0xb8, 0x87, 0x9f, 0x67, 0x7f, 0x62, 0x2a, 0x1d, 0x86, 0x64, 0x80, 0x30, 0x70, 0x1f,
            0x43, 0xe1,
        ],
        // transaction-base
        [
            0xd3, 0x19, 0x48, 0xe6, 0x61, 0xcc, 0x66, 0x26, 0x75, 0xb0, 0xc0, 0x79, 0x45, 0x8f,
            0x71, 0x4a,
        ],
        // blockchain-step
        [
            0x14, 0xab, 0x55, 0x62, 0xed, 0x29, 0x2d, 0xe7, 0xa3, 0xde, 0xb9, 0xe1, 0x2f, 0x00,
            0xae, 0xc0,
        ],
    ];

    pub const CONSTRAINT_CONSTANTS: ConstraintConstants = ConstraintConstants {
        sub_windows_per_window: 11,
        ledger_depth: 35,
        work_delay: 2,
        block_window_duration_ms: 180000,
        transaction_capacity_log_2: 7,
        pending_coinbase_depth: 5,
        coinbase_amount: 720000000000,
        supercharged_coinbase_factor: 1,
        account_creation_fee: 1000000000,
        // TODO(tizoc): This should come from the config file, but
        // it affects the circuits. Since we cannot produce the circuits
        // ourselves right now, we cannot react to changes in this value,
        // so it will be hardcoded for now.
        fork: Some(ForkConstants {
            state_hash: ark_ff::field_new!(
                Fp,
                "24465973112608446515163575794792913472627621028836869800891179577915755065526"
            ),
            blockchain_length: 359604,
            global_slot_since_genesis: 564480,
        }),
    };

    pub const CIRCUITS_CONFIG: CircuitsConfig = CircuitsConfig {
        directory_name: "3.0.0mainnet",

        step_transaction_gates: "step-step-proving-key-transaction-snark-transaction-0-b421ac835a0e73935f3d3569ff87f484",
        wrap_transaction_gates:
            "wrap-wrap-proving-key-transaction-snark-93928b62a1803f78b59f698ee4d36e63",
        step_merge_gates:
            "step-step-proving-key-transaction-snark-merge-1-ba1d52dfdc2dd4d2e61f6c66ff2a5b2f",
        step_blockchain_gates:
            "step-step-proving-key-blockchain-snark-step-0-281a97b76f28a0b850065190cbb892af",
        wrap_blockchain_gates:
            "wrap-wrap-proving-key-blockchain-snark-26c8a899619ad2682c077b0fecef87f8",
        step_transaction_opt_signed_opt_signed_gates: "step-step-proving-key-transaction-snark-opt_signed-opt_signed-2-a84fb2a46cf4f9b58857ea5922f23266",
        step_transaction_opt_signed_gates:
            "step-step-proving-key-transaction-snark-opt_signed-3-a7e0f70d44ac6f0dd0afd3478e2b38ac",
        step_transaction_proved_gates:
            "step-step-proving-key-transaction-snark-proved-4-7bb3855dfcf14da4b3ffa7091adc0143",
    };

    /// Returns the list of default seed peers for mainnet bootstrapping.
    ///
    /// These include a diverse set of community-operated and foundation-operated
    /// seed nodes to ensure decentralization and network resilience. The commented
    /// DNS addresses show the original peer addresses, while the active list uses
    /// IP addresses for immediate connectivity.
    pub fn default_peers() -> Vec<&'static str> {
        vec![
            // /dns4/mina-seed.etonec.com/tcp/8302/p2p/12D3KooWKQ1YVtqZFzxDmSw8RASCPZpDCQBywnFz76RbrvZCXk5T
            // /dns4/mina-mainnet-seed.obscura.network/tcp/5002/p2p/12D3KooWFRpU3giZDFjJjwoHSY8kdpv8ktvferGkyQRUHozsXw4X
            // /dns4/mina-mainnet-seed.staketab.com/tcp/10003/p2p/12D3KooWSDTiXcdBVpN12ZqXJ49qCFp8zB1NnovuhZu6A28GLF1J
            // /dns4/mina-seed-1.zkvalidator.com/tcp/8302/p2p/12D3KooWSfEfnVCqzpMbmyUmRY3ESEVmJaRcd1EkLbnvvERQxwtu
            // /dns4/mina-seed.bitcat365.com/tcp/10001/p2p/12D3KooWQzozNTDKL7MqUh6Nh11GMA4pQhRCAsNTRWxCAzAi4VbE
            // /dns4/production-mainnet-libp2p.minaprotocol.network/tcp/10000/p2p/12D3KooWPywsM191KGGNVGiNqN35nyyJg4W2BhhYukF6hP9YBR8q
            // /dns4/production-mainnet-libp2p.minaprotocol.network/tcp/10010/p2p/12D3KooWGB6mJ9Ub9qRBDgHhedNXH4FawWjGQGGN2tQKaKa3gK2h
            // /dns4/production-mainnet-libp2p.minaprotocol.network/tcp/10020/p2p/12D3KooWMvsPx6A1XNa4V8bTbNb6Fh7WHWf92Ezgfxt6UWxiNq5n
            // /dns4/production-mainnet-libp2p.minaprotocol.network/tcp/10030/p2p/12D3KooW9wL9iaj7qbCTBFspi4gCwdZFCdNRnwkRrdRfe4GBJ978
            // /dns4/production-mainnet-libp2p.minaprotocol.network/tcp/10040/p2p/12D3KooWL8SFDx6PSzpSLgBtRSK1brjKFqs8EvW2yX9zexQEefAo
            // /dns4/seed-1.mainnet.gcp.o1test.net/tcp/10003/p2p/12D3KooWCa1d7G3SkRxy846qTvdAFX69NnoYZ32orWVLqJcDVGHW
            // /dns4/seed-2.mainnet.gcp.o1test.net/tcp/10003/p2p/12D3KooWK4NfthViCTyLgVQa1WvqDC1NccVxGruCXCZUt3GqvFvn
            // /dns4/seed-4.mainnet.gcp.o1test.net/tcp/10003/p2p/12D3KooWEdBiTUQqxp3jeuWaZkwiSNcFxC6d6Tdq7u2Lf2ZD2Q6X
            // /dns4/seed-5.mainnet.gcp.o1test.net/tcp/10003/p2p/12D3KooWL1DJTigSwuKQRfQE3p7puFUqfbHjXbZJ9YBWtMNpr3GU
            // /dns4/seed.minaexplorer.com/tcp/8302/p2p/12D3KooWR7coZtrMHvsgsfiWq2GESYypac3i29LFGp6EpbtjxBiJ
            // /dns4/seed.minataur.net/tcp/8302/p2p/12D3KooWNyExDzG8T1BYXHpXQS66kaw3zi6qi5Pg9KD3GEyHW5FF
            // /dns4/seed.piconbello.com/tcp/10001/p2p/12D3KooWRFac2AztcTeen2DYNwnTrmVBvwNDsRiFpDVdTkwdFAHP
            "/ip4/138.201.11.249/tcp/8302/p2p/12D3KooWKQ1YVtqZFzxDmSw8RASCPZpDCQBywnFz76RbrvZCXk5T",
            "/ip4/51.178.128.35/tcp/5002/p2p/12D3KooWFRpU3giZDFjJjwoHSY8kdpv8ktvferGkyQRUHozsXw4X",
            "/ip4/138.201.53.35/tcp/10003/p2p/12D3KooWSDTiXcdBVpN12ZqXJ49qCFp8zB1NnovuhZu6A28GLF1J",
            "/ip4/37.27.121.141/tcp/8302/p2p/12D3KooWSfEfnVCqzpMbmyUmRY3ESEVmJaRcd1EkLbnvvERQxwtu",
            "/ip4/94.130.21.18/tcp/10001/p2p/12D3KooWQzozNTDKL7MqUh6Nh11GMA4pQhRCAsNTRWxCAzAi4VbE",
            "/ip4/44.236.52.227/tcp/10000/p2p/12D3KooWPywsM191KGGNVGiNqN35nyyJg4W2BhhYukF6hP9YBR8q",
            "/ip4/44.236.52.227/tcp/10010/p2p/12D3KooWGB6mJ9Ub9qRBDgHhedNXH4FawWjGQGGN2tQKaKa3gK2h",
            "/ip4/44.236.52.227/tcp/10020/p2p/12D3KooWMvsPx6A1XNa4V8bTbNb6Fh7WHWf92Ezgfxt6UWxiNq5n",
            "/ip4/44.236.52.227/tcp/10030/p2p/12D3KooW9wL9iaj7qbCTBFspi4gCwdZFCdNRnwkRrdRfe4GBJ978",
            "/ip4/44.236.52.227/tcp/10040/p2p/12D3KooWL8SFDx6PSzpSLgBtRSK1brjKFqs8EvW2yX9zexQEefAo",
            "/ip4/34.86.219.199/tcp/10003/p2p/12D3KooWCa1d7G3SkRxy846qTvdAFX69NnoYZ32orWVLqJcDVGHW",
            "/ip4/34.145.137.93/tcp/10003/p2p/12D3KooWK4NfthViCTyLgVQa1WvqDC1NccVxGruCXCZUt3GqvFvn",
            "/ip4/34.95.19.83/tcp/10003/p2p/12D3KooWEdBiTUQqxp3jeuWaZkwiSNcFxC6d6Tdq7u2Lf2ZD2Q6X",
            "/ip4/35.203.59.118/tcp/10003/p2p/12D3KooWL1DJTigSwuKQRfQE3p7puFUqfbHjXbZJ9YBWtMNpr3GU",
            "/ip4/65.21.20.43/tcp/8302/p2p/12D3KooWR7coZtrMHvsgsfiWq2GESYypac3i29LFGp6EpbtjxBiJ",
            "/ip4/37.27.118.159/tcp/8302/p2p/12D3KooWNyExDzG8T1BYXHpXQS66kaw3zi6qi5Pg9KD3GEyHW5FF",
            "/ip4/144.76.18.153/tcp/10001/p2p/12D3KooWRFac2AztcTeen2DYNwnTrmVBvwNDsRiFpDVdTkwdFAHP",
        ]
    }
}
