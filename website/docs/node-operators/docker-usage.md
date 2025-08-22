---
title: Using Docker Images
description: Complete guide to running Mina Rust nodes using Docker images
sidebar_position: 2
---

# Using Docker Images

This guide covers how to run Mina Rust nodes using the pre-built Docker images.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed and running
- Basic familiarity with Docker commands
- At least 8GB RAM (16GB recommended for block producers)

## Available Images

<!-- prettier-ignore-start -->

:::warning Deprecated Repository

Docker images from the
[openmina/openmina](https://hub.docker.com/r/openmina/openmina) repository are
deprecated. Please use the
[o1labs/mina-rust](https://hub.docker.com/r/o1labs/mina-rust) images instead for
the latest updates and support.

:::

<!-- prettier-ignore-stop -->

- **Main Node**: `o1labs/mina-rust` - The core Mina Rust node
- **Frontend**: `o1labs/mina-rust-frontend` - Web dashboard and monitoring
  interface

## Image Tags and Versioning

### For Production Use

**Always use version tags for production deployments for stability.** Avoid
using `latest` tags as they may change unexpectedly.

- **Version tags**: `o1labs/mina-rust:v1.4.2` (recommended for stability)
- **Commit-based tags**: `o1labs/mina-rust:2b9e87b2` (available for accessing
  specific features during development, not recommended for general use)

Example:

```bash
# Use a version tag (recommended for stability)
docker pull o1labs/mina-rust:v1.4.2
docker pull o1labs/mina-rust-frontend:v1.4.2

# Commit hashes available for development/testing specific features
docker pull o1labs/mina-rust:2b9e87b2
docker pull o1labs/mina-rust-frontend:2b9e87b2
```

### Finding Available Tags

You can find available tags at:

- [o1labs/mina-rust on Docker Hub](https://hub.docker.com/r/o1labs/mina-rust/tags)
- [o1labs/mina-rust-frontend on Docker Hub](https://hub.docker.com/r/o1labs/mina-rust-frontend/tags)

## Architecture Support

All Docker images are built natively for multiple architectures to ensure
optimal performance:

- **`linux/amd64`** (x86_64) - For Intel/AMD processors
- **`linux/arm64`** (ARM64) - For ARM processors (Apple Silicon, AWS Graviton,
  etc.)

### Automatic Architecture Selection

Docker automatically pulls the correct architecture for your system:

```bash
# This automatically pulls the right architecture
docker pull o1labs/mina-rust:latest

# On Intel/AMD systems: gets linux/amd64
# On Apple Silicon: gets linux/arm64
# On ARM servers: gets linux/arm64
```

### Performance Benefits

- **Native builds**: Each architecture is compiled natively for optimal
  performance
- **No emulation overhead**: ARM users get native performance instead of x86
  emulation
- **Faster startup**: Native images start faster than emulated ones

## Running Nodes

### Basic Node

```bash
# Pull and run the main node
docker pull o1labs/mina-rust:latest
docker run -p 8302:8302 o1labs/mina-rust:latest
```

### Running a Block Producer

```bash
# Generate producer key (one-time setup)
docker run --rm -v $(pwd)/keys:/keys o1labs/mina-rust:latest \
  misc mina-encrypted-key --file /keys/producer-key

# Run block producer node (CHANGE THE COINBASE RECEIVER!)
docker run -d --name mina-block-producer \
  -p 8302:8302 \
  -v $(pwd)/keys:/keys:ro \
  -e MINA_PRIVKEY_PASS=your-password \
  o1labs/mina-rust:latest \
  node --network mainnet \
  --block-producer /keys/producer-key \
  --coinbase-receiver YOUR_MINA_ADDRESS_HERE
```

### Running an Archive Node

```bash
# Run archive node with local storage
docker run -d --name mina-archive \
  -p 8302:8302 -p 3007:3007 \
  -v $(pwd)/archive-data:/archive \
  o1labs/mina-rust:latest \
  node --network mainnet \
  --archive /archive

# Run archive node with PostgreSQL (requires external DB)
docker run -d --name mina-archive \
  -p 8302:8302 \
  -e PG_HOST=localhost \
  -e PG_PORT=5432 \
  -e PG_USER=mina \
  -e PG_PASSWORD=minamina \
  -e PG_DB=mina_archive \
  o1labs/mina-rust:latest \
  node --network mainnet \
  --archive-address http://localhost:3007
```

### Running with Frontend Dashboard

```bash
# Using Docker Compose (recommended)
# Download the latest release and use the provided docker-compose files

# Or run containers separately
docker run -d --name mina-node -p 8302:8302 o1labs/mina-rust:latest \
  node --network mainnet
docker run -d --name mina-frontend -p 8070:8070 o1labs/mina-rust-frontend:latest
```

### Advanced Configuration

```bash
# Run with custom network and verbosity
docker run -d --name mina-node \
  -p 8302:8302 \
  o1labs/mina-rust:latest \
  node --network devnet --verbosity debug

# Run with external IP and custom libp2p port
# Get your external IP first
EXTERNAL_IP=$(curl -s https://ifconfig.me/ip)
docker run -d --name mina-node \
  -p 9302:9302 \
  -e MINA_LIBP2P_EXTERNAL_IP=$EXTERNAL_IP \
  o1labs/mina-rust:latest \
  node --network mainnet --libp2p-port 9302
```

## Next Steps

For specific node configurations and detailed setup guides:

- [Block Producer Setup](block-producer) - Complete guide for running a block
  producer
- [Archive Node Setup](archive-node) - Complete guide for running an archive
  node
- [Docker Installation](../appendix/docker-installation) - Installing Docker on
  your system
