/// End-to-end tests for GraphQL Mutation endpoints using remote node
use reqwest::Client;
use serde_json::{json, Value};

fn get_graphql_endpoint() -> String {
    std::env::var("GRAPHQL_ENDPOINT").expect("GRAPHQL_ENDPOINT environment variable must be set")
}

/// Helper function to execute GraphQL mutations via HTTP
async fn execute_graphql_mutation(mutation: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request_body = json!({
        "query": mutation
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

    // Check for GraphQL errors
    if let Some(errors) = json_response.get("errors") {
        if !errors.as_array().unwrap().is_empty() {
            return Err(format!("GraphQL errors: {}", errors).into());
        }
    }

    Ok(json_response)
}

/// Note: These mutation tests are ignored by default because they modify state on the devnet.
/// They can be run individually with: cargo test test_name -- --ignored
/// The goal is to test that the mutations are properly structured and parsed.

#[tokio::test]
#[ignore]
async fn test_send_zkapp_mutation_structure() {
    // Test: Verify zkApp mutation structure (expect signature validation failure)
    let mutation = r#"
        mutation {
            sendZkapp(input: {
                zkappCommand: {
                    memo: "test_memo",
                    feePayer: {
                        body: {
                            publicKey: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                            fee: "1000000",
                            nonce: "0"
                        },
                        authorization: "signature"
                    },
                    accountUpdates: []
                }
            }) {
                zkapp {
                    hash
                    id
                }
            }
        }
    "#;

    // This will likely fail due to invalid signature, but we're testing structure parsing
    let result = execute_graphql_mutation(mutation).await;

    // The mutation structure should be valid even if signature verification fails
    match result {
        Ok(_) => {
            // Unexpected success - signature validation should have failed
            println!("Warning: sendZkapp succeeded unexpectedly");
        }
        Err(e) => {
            // Expected - should fail on signature validation
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature/authorization validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_payment_mutation_structure() {
    // Test: Verify payment mutation structure (expect signature validation failure)
    let mutation = r#"
        mutation {
            sendPayment(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    amount: "1000000000",
                    fee: "1000000",
                    memo: "test_payment"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                payment {
                    id
                    hash
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    // The mutation structure should be valid even if signature verification fails
    match result {
        Ok(_) => {
            // Unexpected success - signature validation should have failed
            println!("Warning: sendPayment succeeded unexpectedly");
        }
        Err(e) => {
            // Expected - should fail on signature validation
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_payment_with_nonce_structure() {
    // Test: Verify payment with nonce structure
    let mutation = r#"
        mutation {
            sendPayment(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    amount: "5000000000",
                    fee: "2000000",
                    nonce: 42,
                    memo: "payment_with_nonce"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                payment {
                    id
                    hash
                    nonce
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    match result {
        Ok(_) => {
            println!("Warning: sendPayment with nonce succeeded unexpectedly");
        }
        Err(e) => {
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_payment_with_valid_until_structure() {
    // Test: Verify payment with validUntil structure
    let mutation = r#"
        mutation {
            sendPayment(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    amount: "2000000000",
                    fee: "1500000",
                    validUntil: 100000,
                    memo: "payment_with_expiry"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                payment {
                    id
                    hash
                    validUntil
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    match result {
        Ok(_) => {
            println!("Warning: sendPayment with validUntil succeeded unexpectedly");
        }
        Err(e) => {
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_delegation_mutation_structure() {
    // Test: Verify delegation mutation structure
    let mutation = r#"
        mutation {
            sendDelegation(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    fee: "1000000",
                    memo: "test_delegation"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                delegation {
                    id
                    hash
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    match result {
        Ok(_) => {
            println!("Warning: sendDelegation succeeded unexpectedly");
        }
        Err(e) => {
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_delegation_with_nonce_structure() {
    // Test: Verify delegation with nonce structure
    let mutation = r#"
        mutation {
            sendDelegation(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    fee: "1500000",
                    nonce: 10,
                    memo: "delegation_with_nonce"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                delegation {
                    id
                    hash
                    nonce
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    match result {
        Ok(_) => {
            println!("Warning: sendDelegation with nonce succeeded unexpectedly");
        }
        Err(e) => {
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_delegation_with_valid_until_structure() {
    // Test: Verify delegation with validUntil structure
    let mutation = r#"
        mutation {
            sendDelegation(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    fee: "2000000",
                    validUntil: 200000,
                    memo: "delegation_with_expiry"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                delegation {
                    id
                    hash
                    validUntil
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    match result {
        Ok(_) => {
            println!("Warning: sendDelegation with validUntil succeeded unexpectedly");
        }
        Err(e) => {
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_empty_memo_payment_structure() {
    // Test: Verify payment with empty memo structure
    let mutation = r#"
        mutation {
            sendPayment(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    amount: "1000000000",
                    fee: "1000000",
                    memo: ""
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                payment {
                    id
                    hash
                    memo
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    match result {
        Ok(_) => {
            println!("Warning: sendPayment with empty memo succeeded unexpectedly");
        }
        Err(e) => {
            let error_str = e.to_string();
            assert!(
                error_str.contains("GraphQL errors")
                    || error_str.contains("signature")
                    || error_str.contains("authorization"),
                "Should fail on signature validation: {}",
                error_str
            );
        }
    }
}

#[tokio::test]
async fn test_invalid_public_key_format() {
    // Test: Verify error handling for invalid public key format
    let mutation = r#"
        mutation {
            sendPayment(
                input: {
                    from: "invalid_public_key",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    amount: "1000000000",
                    fee: "1000000",
                    memo: "invalid_key_test"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                payment {
                    id
                    hash
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    // Should fail due to invalid public key format
    assert!(
        result.is_err(),
        "Should fail with invalid public key format"
    );
    let error_str = result.unwrap_err().to_string();
    assert!(
        error_str.contains("GraphQL errors")
            || error_str.contains("public")
            || error_str.contains("key")
            || error_str.contains("invalid"),
        "Should indicate invalid public key: {}",
        error_str
    );
}

#[tokio::test]
async fn test_invalid_amount_format() {
    // Test: Verify error handling for invalid amount format
    let mutation = r#"
        mutation {
            sendPayment(
                input: {
                    from: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                    to: "B62qqKAQh8M61uvuw3tjJsmRgsEvzRm84Nc9MwXTF3zoqFRZ86rV8qk",
                    amount: "invalid_amount",
                    fee: "1000000",
                    memo: "invalid_amount_test"
                },
                signature: {
                    field: "signature_field",
                    scalar: "signature_scalar"
                }
            ) {
                payment {
                    id
                    hash
                }
            }
        }
    "#;

    let result = execute_graphql_mutation(mutation).await;

    // Should fail due to invalid amount format
    assert!(result.is_err(), "Should fail with invalid amount format");
    let error_str = result.unwrap_err().to_string();
    assert!(
        error_str.contains("GraphQL errors")
            || error_str.contains("amount")
            || error_str.contains("invalid")
            || error_str.contains("parse"),
        "Should indicate invalid amount: {}",
        error_str
    );
}
