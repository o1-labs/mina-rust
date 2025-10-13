#!/usr/bin/env bash
# Script to validate OCaml reference comments in Rust code
# Usage: ./.github/scripts/check-ocaml-refs.sh [--repo REPO_URL] [--branch BRANCH] [--update]

set -euo pipefail

# Default configuration
OCAML_REPO="${OCAML_REPO:-https://github.com/MinaProtocol/mina.git}"
OCAML_BRANCH="${OCAML_BRANCH:-compatible}"
UPDATE_MODE="${UPDATE_MODE:-false}"
RUST_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --repo)
            OCAML_REPO="$2"
            shift 2
            ;;
        --branch)
            OCAML_BRANCH="$2"
            shift 2
            ;;
        --update)
            UPDATE_MODE="true"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: ./.github/scripts/check-ocaml-refs.sh [--repo REPO_URL] [--branch BRANCH] [--update]"
            exit 1
            ;;
    esac
done

echo "Checking OCaml references against ${OCAML_REPO} (branch: ${OCAML_BRANCH})"

# Create temporary directory
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

# Extract GitHub owner and repo from URL (e.g., https://github.com/MinaProtocol/mina.git)
GITHUB_URL_PATTERN="https://github.com/([^/]+)/(.+)"
if [[ "$OCAML_REPO" =~ $GITHUB_URL_PATTERN ]]; then
    GITHUB_OWNER="${BASH_REMATCH[1]}"
    GITHUB_REPO="${BASH_REMATCH[2]%.git}"  # Remove .git suffix if present
else
    echo "Error: Repository URL must be a GitHub URL"
    exit 1
fi

# Get current commit hash for the branch using GitHub API
echo "Fetching current commit from ${OCAML_BRANCH}..."
CURRENT_COMMIT=$(curl -s "https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/commits/${OCAML_BRANCH}" | grep -o '"sha": "[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$CURRENT_COMMIT" ]; then
    echo "Error: Could not fetch current commit for branch ${OCAML_BRANCH}"
    exit 1
fi

echo "Current OCaml commit: ${CURRENT_COMMIT}"

# Find all Rust files with OCaml references
cd "${RUST_ROOT}"
RUST_FILES=$(git grep -l -E "^/// OCaml reference:" "*.rs" "**/*.rs" || true)

if [ -z "$RUST_FILES" ]; then
    echo "No OCaml references found in Rust code"
    exit 0
fi

# Use temporary files to accumulate results
RESULTS_FILE="${TEMP_DIR}/results.txt"
touch "$RESULTS_FILE"

echo ""
echo "Validating references..."
echo "========================"

# Process each file
echo "$RUST_FILES" | while IFS= read -r rust_file; do
    # Extract OCaml reference comments from the file
    awk '
        /^\/\/\/ OCaml reference:/ {
            ref = $0
            getline
            if ($0 ~ /^\/\/\/ Commit:/) {
                commit = $0
                getline
                if ($0 ~ /^\/\/\/ Last verified:/) {
                    verified = $0
                    print ref
                    print commit
                    print verified
                    print "---"
                }
            }
        }
    ' "$rust_file" | while IFS= read -r line; do
        if [[ "$line" == "/// OCaml reference:"* ]]; then
            # Extract file path and line range
            # Format: src/lib/mina_base/transaction_status.ml L:9-113
            FULL_REF="${line#/// OCaml reference: }"
            OCAML_PATH="${FULL_REF%% L:*}"
            LINE_RANGE=$(echo "$FULL_REF" | grep -o 'L:[0-9-]*' | sed 's/L://' || echo "")

            # Read next two lines
            read -r commit_line
            read -r _verified_line
            read -r _separator

            COMMIT="${commit_line#/// Commit: }"
            # LAST_VERIFIED could be extracted from _verified_line if needed for future validation

            # Fetch the OCaml file from the current branch
            CURRENT_FILE="${TEMP_DIR}/current_${rust_file//\//_}_${OCAML_PATH//\//_}"
            CURRENT_URL="https://raw.githubusercontent.com/${GITHUB_OWNER}/${GITHUB_REPO}/${OCAML_BRANCH}/${OCAML_PATH}"

            if ! curl -sf "$CURRENT_URL" -o "$CURRENT_FILE"; then
                echo "INVALID|${rust_file}|${OCAML_PATH}|FILE_NOT_FOUND" >> "$RESULTS_FILE"
                echo "❌ INVALID: ${rust_file}"
                echo "   OCaml file not found: ${OCAML_PATH}"
            else
                # Validate line range if specified
                RANGE_VALID=true
                if [ -n "$LINE_RANGE" ]; then
                    FILE_LINES=$(wc -l < "$CURRENT_FILE")
                    # START_LINE is not currently used but could be useful for validation
                    # START_LINE=$(echo "$LINE_RANGE" | cut -d'-' -f1)
                    END_LINE=$(echo "$LINE_RANGE" | cut -d'-' -f2)

                    if [ "$END_LINE" -gt "$FILE_LINES" ]; then
                        echo "INVALID|${rust_file}|${OCAML_PATH}|LINE_RANGE_EXCEEDED|L:${LINE_RANGE}|${FILE_LINES}" >> "$RESULTS_FILE"
                        echo "❌ INVALID: ${rust_file}"
                        echo "   Line range L:${LINE_RANGE} exceeds file length (${FILE_LINES} lines): ${OCAML_PATH}"
                        RANGE_VALID=false
                    fi
                fi

                if [ "$RANGE_VALID" = "true" ]; then
                    # Verify that the code at the referenced commit matches the current branch
                    CODE_MATCHES=true
                    if [ -n "$LINE_RANGE" ]; then
                        START_LINE=$(echo "$LINE_RANGE" | cut -d'-' -f1)
                        END_LINE=$(echo "$LINE_RANGE" | cut -d'-' -f2)

                        # Fetch the file from the referenced commit
                        COMMIT_FILE="${TEMP_DIR}/commit_${rust_file//\//_}_${OCAML_PATH//\//_}"
                        COMMIT_URL="https://raw.githubusercontent.com/${GITHUB_OWNER}/${GITHUB_REPO}/${COMMIT}/${OCAML_PATH}"

                        if ! curl -sf "$COMMIT_URL" -o "$COMMIT_FILE"; then
                            echo "INVALID|${rust_file}|${OCAML_PATH}|COMMIT_NOT_FOUND|${COMMIT}" >> "$RESULTS_FILE"
                            echo "❌ INVALID: ${rust_file}"
                            echo "   Referenced commit does not exist: ${COMMIT}"
                            CODE_MATCHES=false
                        else
                            # Extract the specific line ranges from both files and compare
                            CURRENT_LINES=$(sed -n "${START_LINE},${END_LINE}p" "$CURRENT_FILE")
                            COMMIT_LINES=$(sed -n "${START_LINE},${END_LINE}p" "$COMMIT_FILE")

                            if [ "$CURRENT_LINES" != "$COMMIT_LINES" ]; then
                                echo "INVALID|${rust_file}|${OCAML_PATH}|CODE_MISMATCH|${COMMIT}" >> "$RESULTS_FILE"
                                echo "❌ INVALID: ${rust_file}"
                                echo "   Code at L:${LINE_RANGE} differs between commit ${COMMIT} and current branch"
                                echo "   Referenced: https://github.com/${GITHUB_OWNER}/${GITHUB_REPO}/blob/${COMMIT}/${OCAML_PATH}#L${START_LINE}-L${END_LINE}"
                                echo "   Current:    https://github.com/${GITHUB_OWNER}/${GITHUB_REPO}/blob/${OCAML_BRANCH}/${OCAML_PATH}#L${START_LINE}-L${END_LINE}"
                                CODE_MATCHES=false
                            fi
                        fi
                    fi

                    if [ "$CODE_MATCHES" = "true" ]; then
                        # Check if commit is stale
                        if [ "$COMMIT" != "$CURRENT_COMMIT" ]; then
                            echo "STALE|${rust_file}|${OCAML_PATH}|${COMMIT}|${LINE_RANGE}" >> "$RESULTS_FILE"
                            echo "✓ VALID: ${rust_file} -> ${OCAML_PATH} L:${LINE_RANGE}"
                            echo "  ⚠ STALE COMMIT: ${COMMIT} (current: ${CURRENT_COMMIT})"
                        else
                            echo "VALID|${rust_file}|${OCAML_PATH}|${LINE_RANGE}" >> "$RESULTS_FILE"
                            echo "✓ VALID: ${rust_file} -> ${OCAML_PATH} L:${LINE_RANGE}"
                        fi
                    fi
                fi
            fi
        fi
    done
done

# Count results
TOTAL_REFS=$(wc -l < "$RESULTS_FILE")
VALID_REFS=$(grep -c "^VALID|" "$RESULTS_FILE" || true)
INVALID_REFS=$(grep -c "^INVALID|" "$RESULTS_FILE" || true)
STALE_COMMITS=$(grep -c "^STALE|" "$RESULTS_FILE" || true)

echo ""
echo "Summary"
echo "======="
echo "Total references found: ${TOTAL_REFS}"
echo "Valid references: $((VALID_REFS + STALE_COMMITS))"
echo "Invalid references: ${INVALID_REFS}"
echo "Stale commits: ${STALE_COMMITS}"

if [ "$UPDATE_MODE" = "true" ] && [ "${STALE_COMMITS}" -gt 0 ]; then
    echo ""
    echo "Updating stale commit hashes and verification dates..."

    CURRENT_DATE=$(date +%Y-%m-%d)

    # Update each file with stale commits
    grep "^STALE|" "$RESULTS_FILE" | while IFS='|' read -r _status rust_file ocaml_path _old_commit _line_range; do
        echo "Updating ${rust_file}..."

        # Find and replace the old commit with the new one
        sed -i.bak \
            -e "/^\/\/\/ OCaml reference: ${ocaml_path//\//\\/}/,/^\/\/\/ Last verified:/ {
                s/^\/\/\/ Commit: .*/\/\/\/ Commit: ${CURRENT_COMMIT}/
                s/^\/\/\/ Last verified: .*/\/\/\/ Last verified: ${CURRENT_DATE}/
            }" \
            "${RUST_ROOT}/${rust_file}"
        rm -f "${RUST_ROOT}/${rust_file}.bak"
    done

    echo "Updated ${STALE_COMMITS} reference(s)"
fi

# Exit with error if there are invalid references
if [ "${INVALID_REFS}" -gt 0 ]; then
    echo ""
    echo "❌ Validation failed: ${INVALID_REFS} invalid reference(s) found"
    exit 1
fi

if [ "${STALE_COMMITS}" -gt 0 ] && [ "$UPDATE_MODE" = "false" ]; then
    echo ""
    echo "⚠ Warning: ${STALE_COMMITS} reference(s) have stale commits"
    echo "Run with --update to update them automatically"
    exit 0
fi

echo ""
echo "✓ All OCaml references are valid!"
