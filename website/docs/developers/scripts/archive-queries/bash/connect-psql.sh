#!/bin/bash

# Connect using psql (requires PostgreSQL client installed)
psql -h localhost -p 5432 -U postgres -d archive
