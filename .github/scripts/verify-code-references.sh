#!/usr/bin/env bash
# Verify that code references in documentation match actual source code
# This script extracts CODE_REFERENCE comments from markdown files and validates
# that the referenced code still exists at the specified line numbers

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOCS_DIR="${REPO_ROOT}/website/docs"
EXIT_CODE=0
TOTAL_REFS=0
VALID_REFS=0
INVALID_REFS=0

echo "Verifying code references in documentation..."
echo "Repository root: ${REPO_ROOT}"
echo ""

# Find all markdown files with CODE_REFERENCE comments
while IFS= read -r doc_file; do
    # Extract CODE_REFERENCE comments from this file
    while IFS= read -r line_num; do
        # Get the actual line content
        comment_line=$(sed -n "${line_num}p" "$doc_file")

        # Extract file path and line range from comment
        # Format: <!-- CODE_REFERENCE: path/to/file.rs#L123-L456 -->
        if [[ $comment_line =~ CODE_REFERENCE:\ *([^#]+)#L([0-9]+)-L([0-9]+) ]]; then
            file_path="${BASH_REMATCH[1]}"
            start_line="${BASH_REMATCH[2]}"
            end_line="${BASH_REMATCH[3]}"

            # Trim whitespace from file_path
            file_path=$(echo "$file_path" | xargs)

            TOTAL_REFS=$((TOTAL_REFS + 1))

            # Check if the source file exists
            source_file="${REPO_ROOT}/${file_path}"
            if [[ ! -f "$source_file" ]]; then
                echo -e "${RED}✗${NC} Invalid reference in ${doc_file}:${line_num}"
                echo "  File not found: ${file_path}"
                INVALID_REFS=$((INVALID_REFS + 1))
                EXIT_CODE=1
                continue
            fi

            # Check if the line range is valid
            total_lines=$(wc -l < "$source_file")
            if [[ $end_line -gt $total_lines ]]; then
                echo -e "${RED}✗${NC} Invalid reference in ${doc_file}:${line_num}"
                echo "  Line range L${start_line}-L${end_line} exceeds file length (${total_lines} lines)"
                echo "  File: ${file_path}"
                INVALID_REFS=$((INVALID_REFS + 1))
                EXIT_CODE=1
                continue
            fi

            # Extract the actual code from source file
            actual_code=$(sed -n "${start_line},${end_line}p" "$source_file")

            # Find the corresponding code block in the markdown (should be right after the comment)
            # Look for ```rust reference block within next 5 lines
            code_block_start=$((line_num + 1))
            code_block_end=$((line_num + 10))

            # Extract GitHub URL from the reference block
            github_url=$(sed -n "${code_block_start},${code_block_end}p" "$doc_file" | grep "github.com" | head -1)

            if [[ -n "${github_url}" ]]; then
                # Verify the GitHub URL contains correct line range
                line_range_pattern="#L${start_line}-L${end_line}"
                if [[ "${github_url}" =~ ${line_range_pattern} ]]; then
                    # Extract GitHub raw URL from the reference
                    # Convert: https://github.com/o1-labs/mina-rust/blob/develop/path/to/file.rs#L10-L20
                    # To: https://raw.githubusercontent.com/o1-labs/mina-rust/develop/path/to/file.rs
                    if [[ "${github_url}" =~ github\.com/([^/]+)/([^/]+)/blob/([^/]+)/([^#]+) ]]; then
                        org="${BASH_REMATCH[1]}"
                        repo="${BASH_REMATCH[2]}"
                        branch="${BASH_REMATCH[3]}"
                        gh_file_path="${BASH_REMATCH[4]}"

                        raw_url="https://raw.githubusercontent.com/${org}/${repo}/${branch}/${gh_file_path}"

                        # Fetch the code from GitHub
                        github_code=$(curl -s "${raw_url}" | sed -n "${start_line},${end_line}p")

                        # Compare local code with GitHub code
                        if [[ "${actual_code}" == "${github_code}" ]]; then
                            echo -e "${GREEN}✓${NC} Valid reference in ${doc_file}:${line_num}"
                            echo "  ${file_path}#L${start_line}-L${end_line}"
                            echo "  Local code matches GitHub (${branch})"
                            VALID_REFS=$((VALID_REFS + 1))
                        else
                            echo -e "${RED}✗${NC} Code mismatch in ${doc_file}:${line_num}"
                            echo "  ${file_path}#L${start_line}-L${end_line}"
                            echo "  Local code differs from GitHub (${branch})"
                            echo "  This may indicate uncommitted changes or branch divergence"
                            INVALID_REFS=$((INVALID_REFS + 1))
                            EXIT_CODE=1
                        fi
                    else
                        echo -e "${YELLOW}⚠${NC} Could not parse GitHub URL in ${doc_file}:${line_num}"
                        echo "  URL: ${github_url}"
                        INVALID_REFS=$((INVALID_REFS + 1))
                        EXIT_CODE=1
                    fi
                else
                    echo -e "${YELLOW}⚠${NC} Mismatched line range in ${doc_file}:${line_num}"
                    echo "  CODE_REFERENCE comment specifies: L${start_line}-L${end_line}"
                    echo "  But GitHub URL has different line range"
                    INVALID_REFS=$((INVALID_REFS + 1))
                    EXIT_CODE=1
                fi
            else
                echo -e "${YELLOW}⚠${NC} No GitHub URL found for reference in ${doc_file}:${line_num}"
                echo "  Expected rust reference block with GitHub URL"
                INVALID_REFS=$((INVALID_REFS + 1))
                EXIT_CODE=1
            fi
        fi
    done < <(grep -n "CODE_REFERENCE:" "$doc_file" | cut -d: -f1)
done < <(find "$DOCS_DIR" -name "*.md" -o -name "*.mdx")

echo ""
echo "================================"
echo "Code Reference Verification Summary"
echo "================================"
echo -e "Total references checked: ${TOTAL_REFS}"
echo -e "${GREEN}Valid references: ${VALID_REFS}${NC}"
if [[ $INVALID_REFS -gt 0 ]]; then
    echo -e "${RED}Invalid references: ${INVALID_REFS}${NC}"
else
    echo -e "${GREEN}Invalid references: ${INVALID_REFS}${NC}"
fi
echo ""

if [[ $EXIT_CODE -eq 0 ]]; then
    echo -e "${GREEN}✓ All code references are valid!${NC}"
else
    echo -e "${RED}✗ Some code references are invalid. Please update the documentation.${NC}"
fi

exit $EXIT_CODE
