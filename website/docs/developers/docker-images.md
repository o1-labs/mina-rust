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

### Automatic Publishing

Images are automatically built and pushed to Docker Hub:

- **On develop branch**: When commits are pushed to `develop`, images are tagged
  with the commit hash (8 characters)
- **On release branches**: When commits are pushed to branches starting with
  `release/`, images are tagged with the branch name (e.g., `release/v1.5.0`) -
  useful for testing release candidates
- **On tags**: When version tags are created, images are tagged with the tag
  name

### Finding Available Tags

You can find available tags at:

- [o1labs/mina-rust on Docker Hub](https://hub.docker.com/r/o1labs/mina-rust/tags)
- [o1labs/mina-rust-frontend on Docker Hub](https://hub.docker.com/r/o1labs/mina-rust-frontend/tags)

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

### Verifying Architecture

You can verify which architecture you're running:

```bash
docker run --rm o1labs/mina-rust:latest uname -m
# x86_64 on Intel/AMD systems
# aarch64 on ARM systems
```

## Using Docker Images

### Running a Basic Node

```bash
# Pull and run the main node
docker pull o1labs/mina-rust:latest
docker run -p 8302:8302 o1labs/mina-rust:latest
```

### Running with Frontend Dashboard

```bash
# Using Docker Compose (recommended)
# Download the latest release and use the provided docker-compose files

# Or run containers separately
docker run -d --name mina-node -p 8302:8302 o1labs/mina-rust:latest
docker run -d --name mina-frontend -p 8070:8070 o1labs/mina-rust-frontend:latest
```

For complete setup guides, see the
[Node Operators](../node-operators/getting-started) section.
