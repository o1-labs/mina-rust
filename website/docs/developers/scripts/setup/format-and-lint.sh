# Set up SQLite database for heartbeats processor (required for make check/lint)
echo "Setting up SQLite database for heartbeats processor..."
sqlite3 /tmp/heartbeats.db < tools/heartbeats-processor/schema.sql
export DATABASE_URL="sqlite:///tmp/heartbeats.db"

# Format code (required before commits)
echo "Formatting code..."
make format

# Check formatting
echo "Checking code formatting..."
make check-format

# Run linter (clippy)
echo "Running linter..."
make lint

# Fix trailing whitespaces (mandatory before commits)
echo "Fixing trailing whitespaces..."
make fix-trailing-whitespace

# Check for trailing whitespaces
echo "Checking for trailing whitespaces..."
make check-trailing-whitespace
