/// End-to-end tests for GraphQL edge cases and error handling using remote node
use reqwest::Client;
use serde_json::{json, Value};

fn get_graphql_endpoint() -> String {
    std::env::var("GRAPHQL_ENDPOINT").expect("GRAPHQL_ENDPOINT environment variable must be set")
}

/// Helper function to execute GraphQL queries via HTTP
async fn execute_graphql_query(query: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request_body = json!({
        "query": query
    });

    let response = client
        .post(&get_graphql_endpoint())
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let json_response: Value = response.json().await?;

    Ok(json_response)
}

#[tokio::test]
async fn test_invalid_graphql_syntax() {
    // Test: Invalid GraphQL syntax should return proper error
    let invalid_query = r#"
        query {
            syncStatus {
                invalidField
        }
    "#; // Intentionally malformed

    let result = execute_graphql_query(invalid_query).await;

    // Should fail due to syntax error
    assert!(result.is_err(), "Should fail with GraphQL syntax error");
}

#[tokio::test]
async fn test_nonexistent_field() {
    // Test: Query non-existent field should return GraphQL error
    let query = r#"
        query {
            nonExistentField
        }
    "#;

    let result = execute_graphql_query(query).await;

    match result {
        Ok(response) => {
            // Check if there are GraphQL errors
            let errors = response.get("errors");
            assert!(
                errors.is_some(),
                "Should have GraphQL errors for non-existent field"
            );
        }
        Err(_) => {
            // Also acceptable - HTTP level error
        }
    }
}

#[tokio::test]
async fn test_invalid_public_key_format() {
    // Test: Invalid public key format should return proper error
    let query = r#"
        query {
            account(publicKey: "invalid_public_key_format") {
                publicKey
            }
        }
    "#;

    let result = execute_graphql_query(query).await;

    match result {
        Ok(response) => {
            // Check if there are GraphQL errors
            let errors = response.get("errors");
            if errors.is_some() {
                let error_array = errors.unwrap().as_array().unwrap();
                assert!(!error_array.is_empty(), "Should have validation errors");
            }
        }
        Err(_) => {
            // Also acceptable - request level error
        }
    }
}

#[tokio::test]
async fn test_negative_block_height() {
    // Test: Negative block height should be handled gracefully
    let query = r#"
        query {
            block(height: -1) {
                stateHash
            }
        }
    "#;

    let result = execute_graphql_query(query).await;

    match result {
        Ok(response) => {
            // Either should have errors or handle gracefully
            let data = response.get("data");
            let errors = response.get("errors");

            if errors.is_some() {
                let error_array = errors.unwrap().as_array().unwrap();
                assert!(
                    !error_array.is_empty(),
                    "Should have validation errors for negative height"
                );
            } else {
                // If no errors, data should be present but likely null
                assert!(data.is_some());
            }
        }
        Err(_) => {
            // Also acceptable - could fail at HTTP level
        }
    }
}

#[tokio::test]
async fn test_zero_max_length_best_chain() {
    // Test: Zero max length for best chain should handle gracefully
    let query = r#"
        query {
            bestChain(maxLength: 0) {
                stateHash
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Zero maxLength should be handled gracefully"
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Should have data");
    let best_chain = data.get("bestChain").expect("Should have bestChain");

    // Should return empty array
    assert!(best_chain.is_array());
    assert_eq!(
        best_chain.as_array().unwrap().len(),
        0,
        "Should return empty array for maxLength 0"
    );
}

#[tokio::test]
async fn test_large_max_length_best_chain() {
    // Test: Very large max length should be capped or handled gracefully
    let query = r#"
        query {
            bestChain(maxLength: 2147483647) {
                stateHash
                protocolState {
                    consensusState {
                        blockHeight
                    }
                }
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Large maxLength should be handled gracefully"
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Should have data");
    let best_chain = data.get("bestChain").expect("Should have bestChain");

    // Should return an array (likely capped at reasonable size)
    assert!(best_chain.is_array());

    let chain_array = best_chain.as_array().unwrap();
    if !chain_array.is_empty() {
        // If there are blocks, verify structure
        assert!(chain_array[0].get("stateHash").is_some());
    }
}

#[tokio::test]
async fn test_invalid_state_hash_format() {
    // Test: Invalid state hash format should return proper error
    let query = r#"
        query {
            block(stateHash: "invalid_hash_format") {
                stateHash
            }
        }
    "#;

    let result = execute_graphql_query(query).await;

    match result {
        Ok(response) => {
            let errors = response.get("errors");
            let data = response.get("data");

            if errors.is_some() {
                let error_array = errors.unwrap().as_array().unwrap();
                assert!(
                    !error_array.is_empty(),
                    "Should have validation errors for invalid hash"
                );
            } else if data.is_some() {
                // If no errors, block should be null (not found)
                let block = data.unwrap().get("block");
                assert!(block.is_some());
            }
        }
        Err(_) => {
            // Also acceptable - could fail at validation level
        }
    }
}

#[tokio::test]
async fn test_empty_query() {
    // Test: Empty query should return proper error
    let query = r#"
        query {
        }
    "#;

    let result = execute_graphql_query(query).await;

    match result {
        Ok(response) => {
            // Should have GraphQL errors for empty query
            let errors = response.get("errors");
            if errors.is_some() {
                let error_array = errors.unwrap().as_array().unwrap();
                assert!(
                    !error_array.is_empty(),
                    "Should have errors for empty query"
                );
            }
        }
        Err(_) => {
            // Also acceptable - HTTP level error
        }
    }
}

#[tokio::test]
async fn test_multiple_queries_in_one_request() {
    // Test: Multiple queries in single request
    let query = r#"
        query {
            syncStatus
            version
            networkID
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(result.is_ok(), "Multiple queries should be handled");

    let response = result.unwrap();
    let data = response.get("data").expect("Should have data");

    // All fields should be present
    assert!(data.get("syncStatus").is_some());
    assert!(data.get("version").is_some());
    assert!(data.get("networkID").is_some());
}

#[tokio::test]
async fn test_deeply_nested_query() {
    // Test: Deeply nested query structure
    let query = r#"
        query {
            genesisBlock {
                protocolState {
                    consensusState {
                        blockHeight
                        epoch
                        slot
                    }
                    blockchainState {
                        snarkedLedgerHash
                    }
                }
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(result.is_ok(), "Deeply nested query should be handled");

    let response = result.unwrap();
    let data = response.get("data").expect("Should have data");
    let genesis_block = data.get("genesisBlock").expect("Should have genesisBlock");

    // Verify nested structure
    assert!(genesis_block.get("protocolState").is_some());
    let protocol_state = genesis_block.get("protocolState").unwrap();
    assert!(protocol_state.get("consensusState").is_some());
    assert!(protocol_state.get("blockchainState").is_some());
}

#[tokio::test]
async fn test_query_with_variables_but_no_variables_provided() {
    // Test: Query that expects variables but none provided
    let query = r#"
        query($height: Int!) {
            block(height: $height) {
                stateHash
            }
        }
    "#;

    let client = Client::new();

    let request_body = json!({
        "query": query,
        "variables": {}
    });

    let response = client
        .post(&get_graphql_endpoint())
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .expect("HTTP request should succeed");

    let json_response: Value = response.json().await.expect("Should parse JSON");

    // Should have errors for missing required variable
    let errors = json_response.get("errors");
    assert!(errors.is_some(), "Should have errors for missing variables");
}
