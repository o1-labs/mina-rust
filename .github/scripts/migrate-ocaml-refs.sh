#!/usr/bin/env bash
# Script to migrate OCaml reference comments from old format to permalink format
# Usage: ./.github/scripts/migrate-ocaml-refs.sh [--dry-run] [--branch BRANCH]

set -euo pipefail

# Default configuration
GITHUB_OWNER="${GITHUB_OWNER:-MinaProtocol}"
GITHUB_REPO="${GITHUB_REPO:-mina}"
OCAML_BRANCH="${OCAML_BRANCH:-compatible}"
DRY_RUN="${DRY_RUN:-false}"
RUST_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN="true"
            shift
            ;;
        --branch)
            OCAML_BRANCH="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: ./.github/scripts/migrate-ocaml-refs.sh [--dry-run] [--branch BRANCH]"
            exit 1
            ;;
    esac
done

if [ "$DRY_RUN" = "true" ]; then
    echo "DRY RUN MODE: No files will be modified"
fi

echo "Migrating OCaml references to permalink format..."
echo "Repository: ${GITHUB_OWNER}/${GITHUB_REPO}"
echo "Branch: ${OCAML_BRANCH}"

# Fetch current commit from the branch
echo "Fetching current commit from ${OCAML_BRANCH}..."
CURRENT_COMMIT=$(curl -s "https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/commits/${OCAML_BRANCH}" | grep -o '"sha": "[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$CURRENT_COMMIT" ]; then
    echo "Error: Could not fetch current commit for branch ${OCAML_BRANCH}"
    exit 1
fi

echo "Current commit: ${CURRENT_COMMIT}"
CURRENT_DATE=$(date +%Y-%m-%d)
echo "Verification date: ${CURRENT_DATE}"
echo ""

# Find all Rust files with old-style OCaml references
cd "${RUST_ROOT}"
OLD_FILES=$(git grep -l -E "/// (Commit: [0-9a-f]+|OCaml reference: src/)" "*.rs" "**/*.rs" || true)

if [ -z "$OLD_FILES" ]; then
    echo "No old-style OCaml references found. All references may already be migrated."
    exit 0
fi

MIGRATED_COUNT=0
SKIPPED_COUNT=0

# Process each file
echo "$OLD_FILES" | while IFS= read -r rust_file; do
    echo "Processing: ${rust_file}"

    # Create temporary file for the migrated content
    TEMP_FILE="${rust_file}.migrate.tmp"
    cp "$rust_file" "$TEMP_FILE"

    # Extract and convert references
    awk -v github_owner="$GITHUB_OWNER" -v github_repo="$GITHUB_REPO" \
        -v current_commit="$CURRENT_COMMIT" -v current_date="$CURRENT_DATE" '
    BEGIN {
        in_old_format = 0
    }

    # Detect OCaml reference lines (with or without leading whitespace)
    /\/\/\/ OCaml reference:/ {
        ref_line = $0
        # Extract indentation
        match($0, /^[ \t]*/)
        indent = substr($0, RSTART, RLENGTH)

        # Remove indentation and "/// OCaml reference: " prefix
        temp = $0
        sub(/^[ \t]*\/\/\/ OCaml reference: /, "", temp)
        ocaml_ref = temp

        # Check if this is already a permalink
        if (ocaml_ref ~ /^https:\/\/github\.com\//) {
            # Already migrated, output as-is
            print $0
            next
        }

        # Save current line for potential old format parsing
        getline next_line

        if (next_line ~ /\/\/\/ Commit:/) {
            # Three-line format with commit
            # Extract path and line range
            split(ocaml_ref, parts, " L:")
            ocaml_path = parts[1]
            line_range = parts[2]

            # Extract commit
            temp2 = next_line
            sub(/^[ \t]*\/\/\/ Commit: /, "", temp2)
            commit = temp2

            # Get verification date
            getline verified_line
            temp3 = verified_line
            sub(/^[ \t]*\/\/\/ Last verified: /, "", temp3)
            verified = temp3

            # Convert line range format
            split(line_range, range_parts, "-")
            start_line = range_parts[1]
            end_line = range_parts[2]

            if (end_line == "") {
                end_line = start_line
            }

            # Generate new permalink
            permalink = "https://github.com/" github_owner "/" github_repo "/blob/" commit "/" ocaml_path "#L" start_line "-L" end_line

            # Output new format with preserved indentation
            print indent "/// OCaml reference: " permalink
            print indent "/// Last verified: " verified
            next
        } else if (ocaml_ref ~ /^src\//) {
            # Simple format without commit info
            # Extract path and line range
            split(ocaml_ref, parts, " L:")
            ocaml_path = parts[1]
            line_range = parts[2]

            # Convert line range format
            split(line_range, range_parts, "-")
            start_line = range_parts[1]
            end_line = range_parts[2]

            if (end_line == "") {
                end_line = start_line
            }

            # Generate new permalink with current commit
            permalink = "https://github.com/" github_owner "/" github_repo "/blob/" current_commit "/" ocaml_path "#L" start_line "-L" end_line

            # Output new format with preserved indentation
            print indent "/// OCaml reference: " permalink
            print indent "/// Last verified: " current_date
            # Output the next line that we already read
            print next_line
            next
        } else {
            # Unknown format, output both lines as-is
            print indent "/// OCaml reference: " ocaml_ref
            print next_line
            next
        }
    }

    # Output all other lines as-is
    { print }
    ' "$TEMP_FILE" > "${TEMP_FILE}.new"

    # Check if any changes were made
    if ! diff -q "$TEMP_FILE" "${TEMP_FILE}.new" > /dev/null 2>&1; then
        echo "  ✓ Migrated references in ${rust_file}"
        MIGRATED_COUNT=$((MIGRATED_COUNT + 1))

        if [ "$DRY_RUN" = "false" ]; then
            mv "${TEMP_FILE}.new" "$rust_file"
        else
            echo "  (dry run - changes not applied)"
            # Show a sample of changes in dry run mode
            echo "  Sample changes:"
            diff -u "$TEMP_FILE" "${TEMP_FILE}.new" | head -30 || true
        fi
    else
        echo "  - No old-format references found, skipping"
        SKIPPED_COUNT=$((SKIPPED_COUNT + 1))
    fi

    # Cleanup
    rm -f "$TEMP_FILE" "${TEMP_FILE}.new"
done

echo ""
echo "Migration Summary"
echo "================="
echo "Files processed: $((MIGRATED_COUNT + SKIPPED_COUNT))"
echo "Files migrated: ${MIGRATED_COUNT}"
echo "Files skipped: ${SKIPPED_COUNT}"

if [ "$DRY_RUN" = "true" ]; then
    echo ""
    echo "This was a dry run. Run without --dry-run to apply changes."
elif [ "$MIGRATED_COUNT" -gt 0 ]; then
    echo ""
    echo "✓ Migration complete!"
    echo ""
    echo "Next steps:"
    echo "1. Review the changes: git diff"
    echo "2. Run the validation script: ./.github/scripts/check-ocaml-refs.sh"
    echo "3. Commit the changes if everything looks good"
fi
