#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Error: VERSION is required. Usage: $0 <version>"
    echo "Example: $0 1.2.3"
    exit 1
fi

VERSION="$1"

echo "Updating version to $VERSION in Cargo.toml files..."

# Find all Cargo.toml files, exclude target directory, and update version
find . -name "Cargo.toml" -not -path "./target/*" -exec sed -i.bak 's/^version = "[^"]*"/version = "'"$VERSION"'"/' {} \;

# Clean up backup files
find . -name "*.bak" -delete

echo "Version updated to $VERSION in all Cargo.toml files"