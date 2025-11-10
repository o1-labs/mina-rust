#!/bin/bash

# Vacuum and analyze for performance
docker exec postgres-mina-rust psql -U postgres -d archive -c "VACUUM ANALYZE;"
