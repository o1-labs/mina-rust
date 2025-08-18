---
sidebar_position: 7
title: Network Configuration
description:
  Learn about Mina Rust Node's network configuration and how to contribute to
  the default peer list
slug: /node-operators/network-configuration
---

# Network Configuration

This guide explains the Mina Rust Node's network configuration and how you can
contribute to improving the network's connectivity.

## Default Peers

The Mina Rust Node includes hardcoded default peers for both mainnet and devnet
networks to ensure reliable initial connectivity. These peers are defined in the
[`devnet::default_peers()`](https://o1-labs.github.io/mina-rust/api-docs/openmina_core/network/devnet/fn.default_peers.html)
and
[`mainnet::default_peers()`](https://o1-labs.github.io/mina-rust/api-docs/openmina_core/network/mainnet/fn.default_peers.html)
functions in `core/src/network.rs`.

### Mainnet Default Peers

The mainnet configuration includes a curated list of reliable seed nodes
operated by various community members and organizations. These peers help new
nodes bootstrap their connection to the Mina Protocol network.

### Devnet Default Peers

The devnet configuration includes seed nodes specifically for development and
testing purposes, allowing developers to connect to the test network.

## Contributing to Default Peers

The Mina community welcomes contributions to improve network connectivity by
adding reliable seed nodes to the default peer lists.

### Requirements for Peer Contribution

To contribute a seed node to the default peer list, your node should meet these
criteria:

- **High Availability**: Node should have >99% uptime
- **Stable Network**: Reliable internet connection with sufficient bandwidth
- **Proper Configuration**: Correctly configured Mina Protocol node
- **Long-term Commitment**: Willingness to maintain the node for extended
  periods
- **Network Security**: Secure server setup and regular maintenance

### Contribution Process

#### 1. Via Rust Node Repository

For Rust node-specific peer additions:

1. Fork the [Rust node repository](https://github.com/o1-labs/mina-rust)
2. Edit the appropriate default peer list in `core/src/network.rs`:
   - For mainnet: Update the
     [`mainnet::default_peers()`](https://o1-labs.github.io/mina-rust/api-docs/openmina_core/network/mainnet/fn.default_peers.html)
     function
   - For devnet: Update the
     [`devnet::default_peers()`](https://o1-labs.github.io/mina-rust/api-docs/openmina_core/network/devnet/fn.default_peers.html)
     function
3. Add your peer's multiaddr in the format:
   ```rust
   "/ip4/YOUR_IP/tcp/PORT/p2p/YOUR_PEER_ID",
   ```
4. Submit a pull request with:
   - Description of your node setup
   - Expected uptime and maintenance commitment
   - Contact information for node operator

#### 2. Via Official Seeds Repository

For broader Mina Protocol peer contributions, consider using the official seeds
repository:

1. Visit the
   [Mina Foundation Seeds Repository](https://github.com/o1-labs/seeds)
2. Follow their contribution guidelines:
   - Fork the repository
   - Add your seed to the appropriate network list file
   - Ensure all pre-commit checks pass
   - Submit a pull request with detailed explanation
   - Wait for maintainer review and potential merge

The seeds repository maintains the canonical list of seed nodes accessible at:

- https://bootnodes.minaprotocol.com/networks/mainnet.txt for mainnet
- https://bootnodes.minaprotocol.com/networks/devnet.txt for devnet

### Peer Address Format

Peer addresses use the multiaddr format:

```
/dns4/hostname/tcp/port/p2p/peer_id
/ip4/ip_address/tcp/port/p2p/peer_id
```

Where:

- `hostname` or `ip_address`: Your node's network address
- `port`: The port your node can be reached out to
- `peer_id`: Your node's unique peer identifier

### Testing Your Contribution

Before submitting a peer contribution:

1. **Verify Connectivity**: Ensure your node is accessible from the internet
2. **Test Peer Discovery**: Confirm other nodes can discover and connect to
   yours
3. **Check Network Compatibility**: Verify your node works with both OCaml and
   Rust implementations
4. **Monitor Performance**: Ensure stable performance under normal network load

### Node Operation Best Practices

When running a seed node:

- **Monitor Uptime**: Use monitoring tools to track node availability
- **Regular Updates**: Keep your node software up to date
- **Resource Management**: Ensure adequate CPU, memory, and bandwidth
- **Security Measures**: Implement proper firewall and security configurations
- **Documentation**: Maintain documentation of your node's configuration

### Community Resources

- **GitHub Discussions**: Join discussions on the Mina Rust Node repository
- **Discord**: Connect with the community on the
  [Mina Protocol Discord](https://discord.com/channels/484437221055922177/1290662938734231552)
- **Forums**: Participate in
  [Mina Protocol Forums](https://forums.minaprotocol.com/)

## Advanced Network Configuration

### Custom Peer Lists

While the Mina Rust Node uses default peers for initial connectivity, you can
configure your node to use custom peer lists using command-line options:

#### Adding Individual Peers

Use the `--peers` (or `-P`) argument to add individual peers:

```bash
openmina node \
  --peers /ip4/192.168.1.100/tcp/8302/p2p/12D3KooW... \
  --peers /dns4/seed.example.com/tcp/8302/p2p/12D3KooW...
```

#### Loading Peers from File

Use the `--peer-list-file` argument to load peers from a local file:

```bash
openmina node --peer-list-file /path/to/peers.txt
```

The file should contain one multiaddr per line:

```
/ip4/192.168.1.100/tcp/8302/p2p/12D3KooWExample1...
/dns4/seed1.example.com/tcp/8302/p2p/12D3KooWExample2...
/ip4/10.0.0.50/tcp/8302/p2p/12D3KooWExample3...
```

#### Loading Peers from URL

Use the `--peer-list-url` argument to load peers from a remote URL:

```bash
openmina node --peer-list-url https://example.com/peers.txt
```

This is useful for loading community-maintained peer lists or using dynamic peer
discovery services.

#### Seed Mode

Use the `--seed` flag to run your node as a seed node without connecting to
default peers:

```bash
openmina node --seed
```

This is useful when running your own network or when you want to rely entirely
on custom peer lists.

### Network Monitoring

Monitor your node's network connectivity through:

- The Mina Rust Node's built-in dashboard
- Peer connection metrics
- Network synchronization status

## Troubleshooting

### Common Connectivity Issues

- **Firewall Blocking**: Ensure the libp2p port is open
- **NAT Configuration**: Configure port forwarding if behind NAT
- **Peer Discovery**: Check if your node appears in other nodes' peer lists
- **Network Segmentation**: Verify connectivity to multiple network regions

For additional troubleshooting and support, visit our
[GitHub repository](https://github.com/o1-labs/mina-rust) or reach out to the
community for assistance.
