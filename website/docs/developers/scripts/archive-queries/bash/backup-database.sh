#!/bin/bash

# Create database dump
docker exec postgres-mina-rust pg_dump -U postgres archive > archive_backup.sql
