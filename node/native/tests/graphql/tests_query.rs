/// End-to-end tests for GraphQL Query endpoints using remote node
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

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

    // Check for GraphQL errors
    if let Some(errors) = json_response.get("errors") {
        if !errors.as_array().unwrap().is_empty() {
            return Err(format!("GraphQL errors: {}", errors).into());
        }
    }

    Ok(json_response)
}

/// Helper function to get the current best chain height
async fn get_current_height() -> Result<u64, Box<dyn std::error::Error>> {
    let query = r#"
        query {
            bestChain(maxLength: 1) {
                protocolState {
                    consensusState {
                        blockHeight
                    }
                }
            }
        }
    "#;

    let response = execute_graphql_query(query).await?;
    let height = response
        .get("data")
        .and_then(|data| data.get("bestChain"))
        .and_then(|chain| chain.as_array())
        .and_then(|chain| chain.first())
        .and_then(|block| block.get("protocolState"))
        .and_then(|ps| ps.get("consensusState"))
        .and_then(|cs| cs.get("blockHeight"))
        .and_then(|h| h.as_u64())
        .ok_or("Failed to extract block height")?;

    Ok(height)
}

#[tokio::test]
async fn test_account_query() {
    // Test: Retrieve account information for a given public key
    let query = r#"
        query {
            account(publicKey: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV") {
                publicKey
                balance {
                    total
                }
                nonce
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Account query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let account = data.get("account");

    // Account might not exist, but the query structure should be valid
    if account.is_some() && !account.unwrap().is_null() {
        assert!(account.unwrap().get("publicKey").is_some());
    }
}

#[tokio::test]
async fn test_account_with_token_query() {
    // Test: Retrieve account with custom token ID
    let query = r#"
        query {
            account(
                publicKey: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV",
                token: "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"
            ) {
                publicKey
                tokenId
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Account with token query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    assert!(data.get("account").is_some());
}

#[tokio::test]
async fn test_sync_status_query() {
    // Test: Get current synchronization status
    let query = r#"
        query {
            syncStatus
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Sync status query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let sync_status = data.get("syncStatus").expect("Should have syncStatus");

    // Verify it's a valid sync status string
    let status_str = sync_status.as_str().unwrap();
    assert!(
        ["SYNCED", "CATCHUP", "BOOTSTRAP", "CONNECTING"].contains(&status_str),
        "Sync status should be valid: {}",
        status_str
    );
}

#[tokio::test]
async fn test_best_chain_query() {
    // Test: Retrieve the best chain of blocks
    let query = r#"
        query {
            bestChain(maxLength: 3) {
                stateHash
                protocolState {
                    consensusState {
                        blockHeight
                        epoch
                        slot
                    }
                }
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Best chain query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let best_chain = data
        .get("bestChain")
        .expect("Should have bestChain")
        .as_array()
        .unwrap();

    // Should return at least one block
    assert!(!best_chain.is_empty(), "Best chain should not be empty");

    // Verify first block structure
    let first_block = &best_chain[0];
    assert!(first_block.get("stateHash").is_some());
    assert!(first_block
        .get("protocolState")
        .unwrap()
        .get("consensusState")
        .unwrap()
        .get("blockHeight")
        .is_some());
}

#[tokio::test]
async fn test_daemon_status_query() {
    // Test: Get daemon status information
    let query = r#"
        query {
            daemonStatus {
                chainId
                commitId
                blockProductionKeys
                uptimeSecs
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Daemon status query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let daemon_status = data.get("daemonStatus").expect("Should have daemonStatus");

    // Verify essential fields are present
    assert!(daemon_status.get("chainId").is_some());
    assert!(daemon_status.get("commitId").is_some());
}

#[tokio::test]
async fn test_genesis_constants_query() {
    // Test: Retrieve genesis configuration constants
    let query = r#"
        query {
            genesisConstants {
                accountCreationFee
                coinbase
                genesisStateTimestamp
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Genesis constants query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let genesis_constants = data
        .get("genesisConstants")
        .expect("Should have genesisConstants");

    // Verify essential fields are present
    assert!(genesis_constants.get("accountCreationFee").is_some());
    assert!(genesis_constants.get("coinbase").is_some());
}

#[tokio::test]
async fn test_transaction_status_payment_query() {
    // Test: Check status of a payment transaction (expect not found)
    let query = r#"
        query {
            transactionStatus(payment: "test_payment_base64")
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Transaction status query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    // Transaction likely won't exist, but query should be valid
    assert!(data.get("transactionStatus").is_some());
}

#[tokio::test]
async fn test_transaction_status_zkapp_query() {
    // Test: Check status of a zkApp transaction (expect not found)
    let query = r#"
        query {
            transactionStatus(zkappTransaction: "test_zkapp_base64")
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Transaction status query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    // Transaction likely won't exist, but query should be valid
    assert!(data.get("transactionStatus").is_some());
}

#[tokio::test]
async fn test_block_by_height_query() {
    // Test: Retrieve block by current height
    let current_height = get_current_height()
        .await
        .expect("Should get current height");
    let test_height = current_height.saturating_sub(1); // Use height - 1 to ensure block exists

    let query = format!(
        r#"
        query {{
            block(height: {}) {{
                stateHash
                protocolState {{
                    consensusState {{
                        blockHeight
                        epoch
                        slot
                    }}
                    blockchainState {{
                        snarkedLedgerHash
                    }}
                }}
                transactions {{
                    userCommands {{
                        id
                    }}
                    zkappCommands {{
                        id
                    }}
                }}
            }}
        }}
    "#,
        test_height
    );

    let result = execute_graphql_query(&query).await;
    assert!(
        result.is_ok(),
        "Block by height query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let block = data.get("block").expect("Should have block");

    if !block.is_null() {
        // Verify block structure
        assert!(block.get("stateHash").is_some());
        let consensus_state = block
            .get("protocolState")
            .unwrap()
            .get("consensusState")
            .unwrap();
        let block_height = consensus_state
            .get("blockHeight")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(block_height, test_height);
    }
}

#[tokio::test]
async fn test_block_by_hash_query() {
    // Test: First get a valid block hash, then query by it
    let query_best_chain = r#"
        query {
            bestChain(maxLength: 1) {
                stateHash
            }
        }
    "#;

    let result = execute_graphql_query(query_best_chain).await;
    assert!(result.is_ok(), "Should get best chain");

    let response = result.unwrap();
    let state_hash = response
        .get("data")
        .unwrap()
        .get("bestChain")
        .unwrap()
        .as_array()
        .unwrap()[0]
        .get("stateHash")
        .unwrap()
        .as_str()
        .unwrap();

    let query = format!(
        r#"
        query {{
            block(stateHash: "{}") {{
                stateHash
                protocolState {{
                    consensusState {{
                        blockHeight
                    }}
                }}
            }}
        }}
    "#,
        state_hash
    );

    let result = execute_graphql_query(&query).await;
    assert!(
        result.is_ok(),
        "Block by hash query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let block = data.get("block").expect("Should have block");

    // Verify returned block has the correct hash
    if !block.is_null() {
        let returned_hash = block.get("stateHash").unwrap().as_str().unwrap();
        assert_eq!(returned_hash, state_hash);
    }
}

#[tokio::test]
async fn test_pooled_user_commands_query() {
    // Test: Query pending user commands in transaction pool
    let query = r#"
        query {
            pooledUserCommands {
                id
                hash
                kind
                memo
                feeToken
                source {
                    publicKey
                }
                receiver {
                    publicKey
                }
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Pooled user commands query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let pooled_commands = data
        .get("pooledUserCommands")
        .expect("Should have pooledUserCommands");

    // Pool might be empty, but query should succeed
    assert!(pooled_commands.is_array());
}

#[tokio::test]
async fn test_pooled_user_commands_with_public_key_query() {
    // Test: Query pooled commands by public key
    let query = r#"
        query {
            pooledUserCommands(publicKey: "B62qpD75xH5R19wxZG2uz8whNsHPTioVoYcPV3zfjjSbzTmaHQHKKEV") {
                id
                hash
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Pooled user commands with publicKey query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    assert!(data.get("pooledUserCommands").unwrap().is_array());
}

#[tokio::test]
async fn test_pooled_zkapp_commands_query() {
    // Test: Query pending zkApp commands in transaction pool
    let query = r#"
        query {
            pooledZkappCommands {
                id
                hash
                zkappCommand {
                    memo
                }
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Pooled zkapp commands query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let pooled_zkapp_commands = data
        .get("pooledZkappCommands")
        .expect("Should have pooledZkappCommands");

    // Pool might be empty, but query should succeed
    assert!(pooled_zkapp_commands.is_array());
}

#[tokio::test]
async fn test_genesis_block_query() {
    // Test: Retrieve the genesis block
    let query = r#"
        query {
            genesisBlock {
                stateHash
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
    assert!(
        result.is_ok(),
        "Genesis block query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let genesis_block = data.get("genesisBlock").expect("Should have genesisBlock");

    // Verify genesis block structure
    assert!(genesis_block.get("stateHash").is_some());
    let consensus_state = genesis_block
        .get("protocolState")
        .unwrap()
        .get("consensusState")
        .unwrap();

    // Genesis block should have height 0
    let block_height = consensus_state
        .get("blockHeight")
        .unwrap()
        .as_u64()
        .unwrap();
    assert_eq!(block_height, 0, "Genesis block should have height 0");
}

#[tokio::test]
async fn test_snark_pool_query() {
    // Test: Get completed SNARK jobs
    let query = r#"
        query {
            snarkPool {
                workIds
                fee
                prover
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Snark pool query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let snark_pool = data.get("snarkPool").expect("Should have snarkPool");

    // Pool might be empty, but query should succeed
    assert!(snark_pool.is_array());
}

#[tokio::test]
async fn test_pending_snark_work_query() {
    // Test: Get pending SNARK work items
    let query = r#"
        query {
            pendingSnarkWork {
                workIds
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Pending snark work query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let pending_snark_work = data
        .get("pendingSnarkWork")
        .expect("Should have pendingSnarkWork");

    // Pool might be empty, but query should succeed
    assert!(pending_snark_work.is_array());
}

#[tokio::test]
async fn test_network_id_query() {
    // Test: Get chain-agnostic network identifier
    let query = r#"
        query {
            networkID
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Network ID query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let network_id = data.get("networkID").expect("Should have networkID");

    // Should be a string
    assert!(network_id.is_string(), "Network ID should be a string");
    assert!(
        !network_id.as_str().unwrap().is_empty(),
        "Network ID should not be empty"
    );
}

#[tokio::test]
async fn test_version_query() {
    // Test: Get node version (git commit hash)
    let query = r#"
        query {
            version
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Version query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let version = data.get("version").expect("Should have version");

    // Should be a string (commit hash)
    assert!(version.is_string(), "Version should be a string");
    assert!(
        !version.as_str().unwrap().is_empty(),
        "Version should not be empty"
    );
}

#[tokio::test]
async fn test_current_snark_worker_query() {
    // Test: Get current SNARK worker configuration
    let query = r#"
        query {
            currentSnarkWorker {
                key
                fee
            }
        }
    "#;

    let result = execute_graphql_query(query).await;
    assert!(
        result.is_ok(),
        "Current snark worker query should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let data = response.get("data").expect("Response should have data");
    let snark_worker = data.get("currentSnarkWorker");

    // Worker might not be configured, but query should succeed
    assert!(snark_worker.is_some());
}
