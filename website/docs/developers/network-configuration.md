---
title: Network Configuration
description: Understanding how OpenMina configures different Mina networks
sidebar_position: 6
---

# Network Configuration

OpenMina supports multiple Mina networks (mainnet, devnet) with distinct
configurations that define their behavior, security parameters, and
connectivity. This page provides an overview and links to the detailed API
documentation.

## Overview

Network configurations in OpenMina define all the parameters needed to
participate in a specific Mina blockchain network, including:

- **Chain ID**: Unique identifier computed from network parameters
- **Protocol Constants**: Timing, fees, and consensus parameters
- **Circuit Configurations**: zkSNARK proving keys and circuit hashes
- **Seed Peers**: Initial connection points for network discovery
- **Cryptographic Parameters**: Signature schemes and hash functions

## API Documentation

For comprehensive documentation including usage examples, implementation
details, and troubleshooting guides, see the Rust API documentation:

**[📚 Network Configuration API Documentation](https://o1-labs.github.io/openmina/api-docs/openmina_core/network/index.html)**

The API documentation includes:

- **Complete Usage Examples**: Working code snippets for all features
- **Network Differences**: Detailed comparison between mainnet and devnet
- **Chain ID Computation**: How unique network identifiers are generated
- **Circuit Configuration**: zkSNARK proving keys and verification
- **Best Practices**: Guidelines for development and production use
- **Troubleshooting**: Common issues and solutions

## Quick Start

Here's a basic example of how to initialize network configuration:

```rust
use openmina_core::network::NetworkConfig;

// Initialize for devnet
NetworkConfig::init("devnet")?;

// Access the configuration
let config = NetworkConfig::global();
println!("Connected to: {}", config.name);
```

## Supported Networks

### Devnet

- **Purpose**: Development and testing
- **Network ID**: TESTNET (0x00)
- **Chain ID**:
  `29936104443aaf264a7f0192ac64b1c7173198c1ed404c1bcff5e562e05eb7f6`

### Mainnet

- **Purpose**: Production blockchain
- **Network ID**: MAINNET (0x01)
- **Chain ID**:
  `a7351abc7ddf2ea92d1b38cc8e636c271c1dfd2c081c637f62ebc2af34eb7cc1`

## Further Reading

- [Architecture Overview](./architecture.md)
- [Network Configuration API Documentation](https://o1-labs.github.io/openmina/api-docs/openmina_core/network/index.html)
- [Chain ID API Documentation](https://o1-labs.github.io/openmina/api-docs/openmina_core/chain_id/index.html)
- [OCaml Mina Configuration](https://github.com/MinaProtocol/mina/tree/compatible/src/config)
