#!/bin/bash

# Connect from within the Docker environment
# Note: postgres-mina-rust is the container name from the archive node setup
# See: https://o1-labs.github.io/mina-rust/node-operators/archive-node
docker exec -it postgres-mina-rust psql -U postgres -d archive
