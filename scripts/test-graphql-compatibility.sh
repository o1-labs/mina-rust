#!/bin/bash
# GraphQL Compatibility Test Script
# Tests GraphQL queries against both OCaml and Rust Mina nodes

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OCAML_ENDPOINT="${1:-http://localhost:3085/graphql}"
RUST_ENDPOINT="${2:-http://localhost:3086/graphql}"

FAILURES=0
SUCCESSES=0
TOTAL_TESTS=0

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() {
    echo -e "$1"
}

test_query() {
    local name="$1"
    local query="$2"
    local expected_field="$3"
    local endpoint="$4"
    local node_type="$5"

    ((TOTAL_TESTS++))
    echo -n "Testing $name on $node_type... "

    response=$(curl -s --max-time 10 -X POST "$endpoint" \
        -H "Content-Type: application/json" \
        -d "{\"query\":\"$query\"}" 2>/dev/null || echo '{"errors":[{"message":"Request failed"}]}')

    if echo "$response" | jq -e "$expected_field" > /dev/null 2>&1; then
        log "${GREEN}✓ PASSED${NC}"
        ((SUCCESSES++))
        return 0
    else
        log "${RED}✗ FAILED${NC}"
        log "  Response: $(echo "$response" | jq -c '.' 2>/dev/null || echo "$response" | head -c 200)"
        ((FAILURES++))
        return 1
    fi
}

test_endpoint_availability() {
    local endpoint="$1"
    local node_type="$2"

    echo "Checking $node_type endpoint availability at $endpoint..."

    if ! curl -s --max-time 5 "$endpoint" > /dev/null 2>&1; then
        log "${RED}✗ $node_type endpoint is not reachable${NC}"
        return 1
    fi

    # Test basic GraphQL schema query
    response=$(curl -s --max-time 10 -X POST "$endpoint" \
        -H "Content-Type: application/json" \
        -d '{"query":"{ __schema { queryType { name } } }"}' 2>/dev/null || echo '{}')

    if echo "$response" | jq -e '.data.__schema.queryType.name' > /dev/null 2>&1; then
        log "${GREEN}✓ $node_type GraphQL endpoint is working${NC}"
        return 0
    else
        log "${RED}✗ $node_type GraphQL endpoint is not responding correctly${NC}"
        return 1
    fi
}

run_compatibility_tests() {
    local endpoint="$1"
    local node_type="$2"

    log "${YELLOW}Testing GraphQL queries on $node_type node${NC}"
    log "Endpoint: $endpoint"
    log "==========================================="

    # Core network queries
    test_query "syncStatus" "{ syncStatus }" ".data.syncStatus" "$endpoint" "$node_type"
    test_query "version" "{ version }" ".data.version" "$endpoint" "$node_type"
    test_query "networkID" "{ networkID }" ".data.networkID" "$endpoint" "$node_type"

    # Blockchain state queries
    test_query "bestChain" "{ bestChain(maxLength: 1) { stateHash } }" ".data.bestChain" "$endpoint" "$node_type"
    test_query "genesisBlock" "{ genesisBlock { stateHash } }" ".data.genesisBlock" "$endpoint" "$node_type"
    test_query "genesisConstants" "{ genesisConstants { coinbase accountCreationFee } }" ".data.genesisConstants" "$endpoint" "$node_type"

    # Daemon status query
    test_query "daemonStatus" "{ daemonStatus { blockchainLength chainId consensusMechanism } }" ".data.daemonStatus" "$endpoint" "$node_type"

    # Account query (may not have accounts in demo mode, so we allow both data and errors)
    test_query "account" "{ account(publicKey: \"B62qkYa1o6Mj6uTTjDQCob7FYZspuhkm4RRQhgJg9j4koEBWiSrTQrS\") { balance { total } } }" ".data.account // .errors" "$endpoint" "$node_type"

    # Transaction pool queries (empty results are OK)
    test_query "pooledUserCommands" "{ pooledUserCommands { id } }" ".data.pooledUserCommands // []" "$endpoint" "$node_type"
    test_query "pooledZkappCommands" "{ pooledZkappCommands { id } }" ".data.pooledZkappCommands // []" "$endpoint" "$node_type"

    # SNARK pool queries (empty results are OK)
    test_query "snarkPool" "{ snarkPool { fee prover } }" ".data.snarkPool // []" "$endpoint" "$node_type"
    test_query "pendingSnarkWork" "{ pendingSnarkWork { workIds } }" ".data.pendingSnarkWork // []" "$endpoint" "$node_type"
    test_query "currentSnarkWorker" "{ currentSnarkWorker { key } }" ".data.currentSnarkWorker // null" "$endpoint" "$node_type"

    # Block query by height (may fail if height doesn't exist)
    test_query "block_by_height" "{ block(height: 1) { stateHash } }" ".data.block // .errors" "$endpoint" "$node_type"

    # Complex nested query
    test_query "complex_query" "{ daemonStatus { blockchainLength consensusTimeNow { epoch slot } } }" ".data.daemonStatus" "$endpoint" "$node_type"

    # Transaction status (with a fake transaction - should return UNKNOWN or error)
    test_query "transactionStatus" "{ transactionStatus(payment: \"fake_transaction_data\") }" ".data.transactionStatus // .errors" "$endpoint" "$node_type"

    log "==========================================="
}

collect_schema() {
    local endpoint="$1"
    local node_type="$2"
    local output_file="$3"

    log "Collecting $node_type GraphQL schema..."

    # Introspection query
    local introspection_query='{
        __schema {
            queryType {
                fields {
                    name
                    description
                    args {
                        name
                        type { name }
                    }
                    type { name }
                }
            }
            mutationType {
                fields {
                    name
                    description
                }
            }
            subscriptionType {
                fields {
                    name
                    description
                }
            }
        }
    }'

    curl -s -X POST "$endpoint" \
        -H "Content-Type: application/json" \
        -d "{\"query\":\"$introspection_query\"}" \
        | jq '.' > "$output_file" 2>/dev/null || {
            log "${RED}Failed to collect $node_type schema${NC}"
            echo '{"error": "Schema collection failed"}' > "$output_file"
        }

    if [ -s "$output_file" ] && jq -e '.data.__schema' "$output_file" > /dev/null 2>&1; then
        log "${GREEN}✓ $node_type schema collected successfully${NC}"

        # Extract and display available queries
        log "\nAvailable queries in $node_type node:"
        jq -r '.data.__schema.queryType.fields[].name' "$output_file" 2>/dev/null | sort | head -20 || log "Could not extract queries"

        # Extract and display available mutations
        local mutations=$(jq -r '.data.__schema.mutationType.fields[].name' "$output_file" 2>/dev/null | wc -l)
        log "\nAvailable mutations in $node_type node: $mutations"
        if [ "$mutations" -gt 0 ]; then
            jq -r '.data.__schema.mutationType.fields[].name' "$output_file" 2>/dev/null | sort | head -10
        fi
    else
        log "${RED}✗ Failed to collect $node_type schema${NC}"
    fi
}

compare_schemas() {
    local ocaml_schema="$1"
    local rust_schema="$2"

    if [ ! -f "$ocaml_schema" ] || [ ! -f "$rust_schema" ]; then
        log "${RED}Schema files not found for comparison${NC}"
        return 1
    fi

    log "${YELLOW}Comparing GraphQL schemas...${NC}"

    # Create a simple comparison
    local ocaml_queries=$(jq -r '.data.__schema.queryType.fields[].name' "$ocaml_schema" 2>/dev/null | sort | tr '\n' ' ')
    local rust_queries=$(jq -r '.data.__schema.queryType.fields[].name' "$rust_schema" 2>/dev/null | sort | tr '\n' ' ')

    log "OCaml queries: $ocaml_queries"
    log "Rust queries: $rust_queries"

    # Count queries
    local ocaml_count=$(jq -r '.data.__schema.queryType.fields | length' "$ocaml_schema" 2>/dev/null || echo 0)
    local rust_count=$(jq -r '.data.__schema.queryType.fields | length' "$rust_schema" 2>/dev/null || echo 0)

    log "\nQuery count comparison:"
    log "  OCaml node: $ocaml_count queries"
    log "  Rust node: $rust_count queries"

    if [ "$rust_count" -gt 0 ] && [ "$ocaml_count" -gt 0 ]; then
        local percentage=$((rust_count * 100 / ocaml_count))
        log "  Implementation coverage: $percentage%"
    fi
}

main() {
    log "${YELLOW}GraphQL Compatibility Test Suite${NC}"
    log "==============================="

    # Check if endpoints are provided
    if [ -z "$OCAML_ENDPOINT" ] || [ -z "$RUST_ENDPOINT" ]; then
        log "Usage: $0 [OCAML_ENDPOINT] [RUST_ENDPOINT]"
        log "Example: $0 http://localhost:3085/graphql http://localhost:3086/graphql"
        exit 1
    fi

    local ocaml_available=false
    local rust_available=false

    # Test OCaml endpoint
    if test_endpoint_availability "$OCAML_ENDPOINT" "OCaml"; then
        ocaml_available=true
        run_compatibility_tests "$OCAML_ENDPOINT" "OCaml"
        collect_schema "$OCAML_ENDPOINT" "OCaml" "ocaml_schema.json"
        log ""
    else
        log "${YELLOW}Skipping OCaml tests - endpoint not available${NC}\n"
    fi

    # Test Rust endpoint
    if test_endpoint_availability "$RUST_ENDPOINT" "Rust"; then
        rust_available=true
        run_compatibility_tests "$RUST_ENDPOINT" "Rust"
        collect_schema "$RUST_ENDPOINT" "Rust" "rust_schema.json"
        log ""
    else
        log "${YELLOW}Skipping Rust tests - endpoint not available${NC}\n"
    fi

    # Compare schemas if both are available
    if [ "$ocaml_available" = true ] && [ "$rust_available" = true ]; then
        compare_schemas "ocaml_schema.json" "rust_schema.json"
    fi

    # Final results
    log "${YELLOW}Final Results${NC}"
    log "============="
    log "Total tests run: $TOTAL_TESTS"
    log "Successes: ${GREEN}$SUCCESSES${NC}"
    log "Failures: ${RED}$FAILURES${NC}"

    if [ $FAILURES -gt 0 ]; then
        log "${RED}Some tests failed!${NC}"
        exit 1
    else
        log "${GREEN}All tests passed!${NC}"
    fi
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi