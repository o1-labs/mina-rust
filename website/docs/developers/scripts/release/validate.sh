#!/bin/bash
set -e

echo "Validating codebase for release..."

echo "Running tests..."
make test

echo "Running tests in release mode..."
make test-release

echo "Checking code formatting..."
make check-format

echo "Fixing trailing whitespace..."
make fix-trailing-whitespace

echo "Verifying no trailing whitespace remains..."
make check-trailing-whitespace

echo "Release validation completed successfully!"