# Set up environment for tests
export DATABASE_URL="sqlite:///tmp/heartbeats.db"

sqlite3 /tmp/heartbeats.db < tools/heartbeats-processor/schema.sql 2>/dev/null || true

# Run basic tests (excluding those that require special setup)
echo "Running basic test suite..."
make test
