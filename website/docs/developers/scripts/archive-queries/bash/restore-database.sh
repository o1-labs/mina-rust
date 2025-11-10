#!/bin/bash

# Restore from backup
docker exec -i postgres-mina-rust psql -U postgres archive < archive_backup.sql
