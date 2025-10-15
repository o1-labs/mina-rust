#!/usr/bin/env bash
# Script to list all workspace packages with their test requirements
# Outputs JSON suitable for GitHub Actions matrix strategy

set -euo pipefail

# Packages that require nightly Rust
NIGHTLY_PACKAGES=(
  "mina-tree"
  "vrf"
  "mina-p2p-messages"
  "transaction_fuzzer"
)

# Packages that don't have unit tests or shouldn't be tested
# cli is tested via wallet-tests (end-to-end tests)
SPECIAL_PACKAGES=(
  "cli"
)

# Helper function to check if an element is in an array
contains_element() {
  local element="$1"
  shift
  local array=("$@")
  for item in "${array[@]}"; do
    if [[ "$item" == "$element" ]]; then
      return 0
    fi
  done
  return 1
}

# Get all workspace packages with their features
packages_data=$(cargo metadata --no-deps --format-version 1 2>/dev/null | \
  jq -r '.packages[] | "\(.name)|\(.features | keys | length)"')

matrix_entries=()

while IFS='|' read -r package_name feature_count; do
  # Skip special packages that have dedicated CI jobs
  if contains_element "$package_name" "${SPECIAL_PACKAGES[@]}"; then
    continue
  fi

  # Determine toolchain
  toolchain="stable"
  if contains_element "$package_name" "${NIGHTLY_PACKAGES[@]}"; then
    toolchain="nightly"
  fi

  # Determine if --all-features is needed
  all_features="false"
  if [[ "$feature_count" -gt 0 ]]; then
    all_features="true"
  fi

  # Create matrix entry
  entry=$(jq -n \
    --arg name "$package_name" \
    --arg toolchain "$toolchain" \
    --arg all_features "$all_features" \
    '{
      package: $name,
      toolchain: $toolchain,
      all_features: ($all_features == "true")
    }')

  matrix_entries+=("$entry")
done <<< "$packages_data"

# Combine all entries into a JSON array (compact format for GitHub Actions)
if [[ ${#matrix_entries[@]} -eq 0 ]]; then
  echo '{"include":[]}'
else
  printf '%s\n' "${matrix_entries[@]}" | jq -s -c '{include: .}'
fi
