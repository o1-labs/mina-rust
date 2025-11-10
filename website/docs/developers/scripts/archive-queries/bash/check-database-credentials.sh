#!/bin/bash

# Ensure proper database credentials
docker exec postgres-mina-rust psql -U postgres -l
