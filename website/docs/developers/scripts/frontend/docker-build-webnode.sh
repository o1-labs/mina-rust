# Build Docker image for WebNode

# Generate .env.docker with current git information (required for WASM build)
frontend/docker/generate-docker-env.sh

# Build the Docker image (from project root)
docker build -t mina-frontend -f frontend/Dockerfile .