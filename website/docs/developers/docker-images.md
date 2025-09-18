---
title: Docker Images
description:
  Learn about Mina Rust Docker images, versioning, and how to use them for
  development and deployment
---

# Mina Rust Docker Images

The Mina Rust project provides Docker images for easy deployment and testing.

## Available Images

Docker images are available at Docker Hub under the `o1labs` organization:

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

### For Development and Testing

For accessing the latest development features, use the `develop` tag:

<!-- prettier-ignore-start -->

:::warning Unstable Development Version

The `develop` tag points to the latest code from the development branch and may
be unstable. Only use this for development, testing, or accessing the newest
features. For production use, always use version tags.

:::

<!-- prettier-ignore-stop -->

```bash
# Latest development version (unstable)
docker pull o1labs/mina-rust:develop
docker pull o1labs/mina-rust-frontend:develop
```

### Latest Tag

The `latest` tag always corresponds to the latest commit on the main branch,
which represents the current stable release state.

### Automatic Publishing

Images are automatically built and pushed to Docker Hub:

- **On develop branch**: When commits are pushed to `develop`, images are tagged
  with the commit hash (8 characters) and also tagged as `develop` for easy
  access to the latest development version
- **On release branches**: When commits are pushed to branches starting with
  `release/`, images are tagged with the branch name (e.g., `release/v1.5.0`) -
  useful for testing release candidates
- **On tags**: When version tags are created, images are tagged with the tag
  name

### Finding Available Tags

You can find available tags at:

- [o1labs/mina-rust on Docker Hub](https://hub.docker.com/r/o1labs/mina-rust/tags)
- [o1labs/mina-rust-frontend on Docker Hub](https://hub.docker.com/r/o1labs/mina-rust-frontend/tags)

## Quick Start with Docker Compose

The easiest way to get started is using the provided docker compose
configuration:

```bash
# Clone the repository
git clone https://github.com/o1-labs/mina-rust.git
cd mina-rust

# Start node and frontend
docker compose up -d

# Access the frontend at http://localhost:8070 (currently being fixed)
```

## Local Development

For local development and testing, you can build images using the Makefile:

```bash
# Build images locally
make docker-build-mina
make docker-build-frontend

# Push to registry (requires Docker Hub login)
make docker-push-mina
make docker-push-frontend
```

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

## For Node Operators

For detailed usage instructions including running block producers, archive
nodes, and configuration examples, see:

[→ Node Operators Docker Usage Guide](../node-operators/docker-usage)
