# =============================================================================
# Mina Frontend Docker Image
# =============================================================================
#
# This Dockerfile creates a production-ready Apache HTTP server image that serves
# the Mina frontend application with pre-built WebAssembly (WASM) components.
#
# IMAGE STAGES:
#   1. build_frontend - Compiles Angular frontend application
#   2. build_wasm     - Builds WebAssembly files for web node functionality
#   3. wasm_files     - Extracts only WASM artifacts (size optimization)
#   4. final          - Apache HTTP server with frontend and WASM assets
#
# FEATURES:
#   - Multi-stage build for optimized image size
#   - Pre-built WASM files for Mina web node
#   - Environment-specific configuration support
#   - Caching optimization for web assets
#   - WebRTC signaling configuration
#
# BUILD ARGUMENTS:
#   BUILD_CONFIGURATION - Angular build configuration (default: production)
#
# USAGE:
#   # Build frontend image
#   docker build -t mina-frontend .
#
#   # Run with webnode environment
#   docker run -p 80:80 -e MINA_FRONTEND_ENVIRONMENT=webnode mina-frontend
#
# =============================================================================

# Build arguments
ARG BUILD_CONFIGURATION=production

# Build stage for Angular frontend
FROM node:23 AS build_frontend
ARG BUILD_CONFIGURATION
WORKDIR /app/frontend
COPY frontend/ .
RUN npm install && \
    node_modules/.bin/ng build --configuration \
        ${BUILD_CONFIGURATION} && \
    npm prune --production && \
    echo "---------- FRONTEND BUILD COMPLETE ----------"

# Build stage for WASM files
FROM rust:latest AS build_wasm
# IMPORTANT: The nightly version should match the version specified in:
# - Makefile (NIGHTLY_RUST_VERSION)
# - .github/workflows/ci.yaml
# - .github/workflows/fmt.yaml
# - .github/workflows/lint.yaml
# - frontend/Dockerfile (this file)

# Install system dependencies required for building WASM
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        libssl-dev \
        pkg-config \
        protobuf-compiler \
        sqlite3 && \
    rm -rf /var/lib/apt/lists/*

# Install Rust nightly and WASM target
RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli

WORKDIR /app
COPY . .
RUN make build-wasm && \
    echo "---------- WASM BUILD COMPLETE ----------"

# Extract WASM files stage
FROM scratch AS wasm_files
COPY --from=build_wasm /app/node/web/pkg /pkg

# Circuit files stage
FROM alpine:latest AS circuit_files
WORKDIR /circuit-blobs
COPY circuit-blobs/3.0.1devnet/ .
# Remove the symbolic link that might cause issues
RUN rm -f tests

# Final Apache image
FROM httpd:2.4

# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*

# Copy frontend assets
COPY --from=build_frontend /app/frontend/dist/frontend/browser \
    /usr/local/apache2/htdocs/
COPY --from=build_frontend /app/frontend/httpd.conf \
    /usr/local/apache2/conf/httpd.conf

# Copy WASM files to webnode assets directory
COPY --from=wasm_files /pkg \
    /usr/local/apache2/htdocs/assets/webnode/pkg

# Copy circuit files for webnode
RUN mkdir -p /usr/local/apache2/htdocs/assets/webnode/circuit-blobs/3.0.1devnet
COPY --from=circuit_files /circuit-blobs/ \
    /usr/local/apache2/htdocs/assets/webnode/circuit-blobs/3.0.1devnet/

# Add labels for image metadata
LABEL mina.circuit-files-included="true"
LABEL mina.image-type="frontend"
LABEL mina.components="angular,wasm,circuits"

CMD ["httpd-foreground"]
