#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Error: TAG is required. Usage: $0 <tag>"
    echo "Example: $0 v1.2.3"
    exit 1
fi

TAG="$1"
DOCKER_ORG="${DOCKER_ORG:-o1labs}"

echo "Verifying multi-arch Docker images for tag: $TAG"

echo "Checking mina-rust image..."
docker manifest inspect "$DOCKER_ORG/mina-rust:$TAG"

echo "Checking mina-rust-frontend image..."
docker manifest inspect "$DOCKER_ORG/mina-rust-frontend:$TAG"

echo "Multi-arch Docker images verified successfully!"