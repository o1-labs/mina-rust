#!/bin/bash

# Check database size
docker exec postgres-mina-rust psql -U postgres -d archive -c "
  SELECT
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(tablename::text)) as size
  FROM pg_tables
  WHERE schemaname = 'public'
  ORDER BY pg_total_relation_size(tablename::text) DESC;
"
