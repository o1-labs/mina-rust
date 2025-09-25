#!/bin/bash

set -e

# Function to build frontend based on environment
build_frontend() {
    local environment="$1"
    echo "Building frontend for environment: $environment"

    cd /app/frontend

    # Source NVM to make Node.js and npm available
    export NVM_DIR="$HOME/.nvm"
    # shellcheck disable=SC1091
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

    # Map environment to Makefile targets
    case "$environment" in
        "local")
            make build-local
            ;;
        "fuzzing")
            make build-fuzzing
            ;;
        "prod"|"production")
            make build-prod
            ;;
        "producer")
            make build-producer
            ;;
        "webnodelocal"|"webnode")
            make build-webnodelocal
            ;;
        "leaderboard")
            # Use production build for leaderboard environment
            make build-leaderboard
            ;;
        *)
            echo "Error: Unknown environment '$environment'"
            echo "Available environments: local, fuzzing, prod, producer, webnodelocal, leaderboard"
            exit 1
            ;;
    esac

    # Copy built files to Apache document root
    echo "Copying built files to Apache document root..."
    cp -r dist/frontend/browser/* /usr/local/apache2/htdocs/

    echo "Frontend build complete for environment: $environment"
    cd /app
}

# Validate that MINA_FRONTEND_ENVIRONMENT is set
if [ -z "$MINA_FRONTEND_ENVIRONMENT" ]; then
    echo "Error: MINA_FRONTEND_ENVIRONMENT environment variable is required."
    echo "Available environments: local, fuzzing, prod, producer, webnodelocal, leaderboard"
    echo "Example: docker run -e MINA_FRONTEND_ENVIRONMENT=webnodelocal mina-frontend"
    exit 1
fi

echo "Using environment: $MINA_FRONTEND_ENVIRONMENT"

# Build the frontend for the specified environment
build_frontend "$MINA_FRONTEND_ENVIRONMENT"

# Environment file is now copied by Makefile build targets
echo "Environment configuration set during build process"

echo "Starting Apache..."
exec "$@"
