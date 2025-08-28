---
sidebar_position: 4
title: P2P Tests
description: Peer-to-peer networking tests
slug: /developers/testing/p2p-tests
---

# P2P Tests

## Overview

Peer-to-peer networking tests validate the networking layer functionality,
including connection establishment, message routing, and network resilience.

## Running P2P Tests

### Basic P2P Testing

```bash
# Run P2P tests
make test-p2p

# Run with cargo-nextest
make nextest-p2p
```

## P2P Testing Scenarios

### Connection establishment

Low-level networking testing covers:

- **Basic peer connections**: Establishing peer-to-peer connections
- **Handshake protocol**: Connection initialization and authentication
- **Transport layer**: Both libp2p and WebRTC transport validation
- **Peer discovery**: Finding and connecting to network peers

### Message routing

Message handling and propagation:

- **Message delivery**: Proper message routing between peers
- **Gossip protocols**: Message propagation through the network
- **Request/response**: Direct peer-to-peer communication
- **Message prioritization**: Handling different message types

### Network resilience

Testing network fault tolerance:

- **Network partitions**: Handling network splits and merges
- **Connection recovery**: Reconnection after temporary failures
- **Peer churn**: Handling peers joining and leaving the network
- **Load testing**: Performance under high message volume

## Dual Transport Architecture

The Mina Rust node supports dual transport layers:

### libp2p transport

Traditional blockchain networking:

- **TCP/IP based**: Standard internet protocols
- **DHT integration**: Distributed hash table for peer discovery
- **NAT traversal**: Connection through firewalls and NATs
- **Protocol multiplexing**: Multiple protocols over single connection

### WebRTC transport

Browser-compatible networking:

- **UDP based**: Lower latency communication
- **Browser compatibility**: Direct browser-to-node connections
- **ICE protocol**: Interactive Connectivity Establishment
- **STUN/TURN**: NAT traversal support

## P2P testing components

### Connection management

- **Peer pool**: Managing active connections
- **Connection limits**: Respecting maximum connection counts
- **Bandwidth management**: Controlling data flow rates
- **Connection quality**: Monitoring connection health

### Message types

Different message categories tested:

- **Block messages**: New block announcements and requests
- **Transaction messages**: Transaction propagation
- **Consensus messages**: Consensus-related communication
- **Peer messages**: Peer discovery and management

### Network debugger integration

P2P tests integrate with the network debugger for:

- **Connection inspection**: Real-time connection monitoring
- **Message tracing**: Following messages through the network
- **Performance metrics**: Bandwidth and latency measurements
- **Failure analysis**: Debugging connection issues

## Advanced P2P Features

### Custom channel abstractions

The framework provides channel abstractions for different message types:

```rust
// Example: Message channel usage
let block_channel = p2p.get_channel::<BlockMessage>();
block_channel.broadcast(new_block).await;

let response = p2p.request::<TransactionRequest>(peer_id, request).await;
```

### Peer discovery mechanisms

Multiple discovery methods:

- **Bootstrap nodes**: Initial connection points
- **DHT discovery**: Distributed peer discovery
- **mDNS discovery**: Local network discovery
- **Gossip discovery**: Learning peers through existing connections

### Network policies

Configurable networking behavior:

- **Connection policies**: Which peers to connect to
- **Message filtering**: What messages to accept/forward
- **Rate limiting**: Preventing spam and DoS attacks
- **Reputation systems**: Peer quality scoring

## WebRTC Specific Testing

### Signaling tests

WebRTC requires signaling for connection establishment:

- **Offer/answer exchange**: SDP negotiation
- **ICE candidate exchange**: Connection path discovery
- **Signaling server**: Central coordination point
- **Browser integration**: Testing browser-based connections

### NAT traversal

WebRTC's NAT traversal capabilities:

- **STUN server**: NAT type detection
- **TURN server**: Relay connections when direct fails
- **ICE gathering**: Finding connection paths
- **Connection fallback**: Graceful degradation

## Performance testing

### Throughput testing

Measuring network performance:

- **Message throughput**: Messages per second
- **Bandwidth usage**: Data transfer rates
- **Latency measurement**: Message delivery times
- **Connection scaling**: Performance with many peers

### Load testing

Testing under stress:

- **High peer count**: Many simultaneous connections
- **Message flooding**: High message rates
- **Resource usage**: Memory and CPU under load
- **Degradation patterns**: How performance degrades

## Best practices

### Test design

1. **Network isolation**: Use isolated test networks
2. **Deterministic tests**: Control network timing where possible
3. **Error injection**: Test failure scenarios
4. **Resource cleanup**: Properly close connections

### Debugging P2P issues

1. **Network debugger**: Use sidecar for connection inspection
2. **Connection logs**: Monitor connection establishment
3. **Message tracing**: Follow message paths
4. **Performance metrics**: Monitor bandwidth and latency

### Integration considerations

1. **Scenario integration**: P2P tests within broader scenarios
2. **Multi-transport**: Test both libp2p and WebRTC
3. **Cross-implementation**: Test with OCaml nodes
4. **Real network conditions**: Test with actual internet conditions

## Related Documentation

- [Testing Framework Overview](testing-framework): Main testing documentation
- [Scenario Tests](scenario-tests): Integration testing scenarios
- [P2P Networking](../p2p-networking): P2P architecture details
- [WebRTC](../webrtc): WebRTC implementation details
