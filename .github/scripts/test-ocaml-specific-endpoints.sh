#!/usr/bin/env bash

set -euo pipefail

MINA_BIN="${MINA_BIN:-./target/release/mina}"
# Convert to absolute path if it's a relative path
if [[ "${MINA_BIN}" != /* ]]; then
  MINA_BIN="$(pwd)/${MINA_BIN}"
fi

# Add the directory containing the mina binary to PATH
# so documentation scripts can find 'mina' command
MINA_DIR="$(dirname "${MINA_BIN}")"
export PATH="${MINA_DIR}:${PATH}"

OCAML_NODE="${OCAML_NODE:-https://devnet-plain-1.gcp.o1test.net/graphql}"
SCRIPT_DIR="website/docs/developers/scripts/cli"

echo "Testing OCaml-specific endpoints..."

# Test protocolState endpoint (OCaml-specific)
# Uses the command from the documentation script
SCRIPT_FILE="${SCRIPT_DIR}/graphql-run-ocaml-protocolstate.sh"
if [[ ! -f "${SCRIPT_FILE}" ]]; then
  echo "ERROR: Script file ${SCRIPT_FILE} not found"
  exit 1
fi

echo "Testing protocolState endpoint using ${SCRIPT_FILE}..."
# Extract the command from the script and replace mina with MINA_BIN and the node URL
COMMAND=$(grep -v "^#" "${SCRIPT_FILE}" | sed "s|mina|${MINA_BIN}|" | sed "s|https://devnet-plain-1.gcp.o1test.net/graphql|${OCAML_NODE}|")

if output=$(eval "${COMMAND}" 2>&1); then
  echo "  SUCCESS: protocolState endpoint query executed successfully"
else
  echo "ERROR: protocolState endpoint query failed:"
  echo "$output"
  exit 1
fi

echo "All OCaml-specific endpoint tests passed!"
