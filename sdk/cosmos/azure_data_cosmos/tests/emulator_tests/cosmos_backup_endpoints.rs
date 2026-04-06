// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for backup endpoint fallback.
//! Verifies that the SDK can boot using a backup endpoint when the global endpoint is unavailable.

#![cfg(feature = "key_auth")]

use super::framework;

use azure_data_cosmos::{
    ConnectionString, CosmosAccountEndpoint, CosmosAccountReference, CosmosClient, RoutingStrategy,
};
use framework::{TestClient, CONNECTION_STRING_ENV_VAR, HUB_REGION};
use std::error::Error;

/// Tests that the SDK can initialize when the global endpoint is unreachable
/// but a valid backup endpoint is provided.
///
/// The test uses a RFC 5737 TEST-NET address (guaranteed non-routable) as the
/// global endpoint and sets the real account endpoint as a backup. The client
/// should fail to reach the global endpoint, fall back to the backup endpoint,
/// and initialize successfully.
#[tokio::test]
async fn client_boots_via_backup_when_global_endpoint_unreachable() -> Result<(), Box<dyn Error>> {
    TestClient::run(async |_run_context| {
        let Ok(env_var) = std::env::var(CONNECTION_STRING_ENV_VAR) else {
            eprintln!("Skipping: no connection string");
            return Ok(());
        };

        let connection_string: ConnectionString = env_var.parse()?;
        let real_endpoint: CosmosAccountEndpoint = connection_string.account_endpoint.parse()?;

        // RFC 5737 TEST-NET address — guaranteed non-routable for deterministic failure.
        let fake_endpoint: CosmosAccountEndpoint = "https://192.0.2.1:443/".parse()?;

        let client = CosmosClient::builder()
            .with_backup_endpoints(vec![real_endpoint])
            .build(
                CosmosAccountReference::with_master_key(
                    fake_endpoint,
                    connection_string.account_key.clone(),
                ),
                RoutingStrategy::ProximityTo(HUB_REGION),
            )
            .await;

        assert!(
            client.is_ok(),
            "client should initialize via backup endpoint, but got: {:?}",
            client.err()
        );

        Ok(())
    })
    .await
}

/// Tests that the SDK can list databases after initializing via a backup endpoint.
#[tokio::test]
async fn operations_work_after_backup_endpoint_boot() -> Result<(), Box<dyn Error>> {
    TestClient::run(async |_run_context| {
        let Ok(env_var) = std::env::var(CONNECTION_STRING_ENV_VAR) else {
            eprintln!("Skipping: no connection string");
            return Ok(());
        };

        let connection_string: ConnectionString = env_var.parse()?;
        let real_endpoint: CosmosAccountEndpoint = connection_string.account_endpoint.parse()?;

        let fake_endpoint: CosmosAccountEndpoint = "https://192.0.2.1:443/".parse()?;

        let client = CosmosClient::builder()
            .with_backup_endpoints(vec![real_endpoint])
            .build(
                CosmosAccountReference::with_master_key(
                    fake_endpoint,
                    connection_string.account_key.clone(),
                ),
                RoutingStrategy::ProximityTo(HUB_REGION),
            )
            .await?;

        // Verify the client can list databases (a basic read operation)
        use futures::TryStreamExt;
        let query = azure_data_cosmos::Query::from("SELECT * FROM root r");
        let mut pager = client.query_databases(query, None)?;
        let page = pager.try_next().await;
        assert!(
            page.is_ok(),
            "should be able to list databases after backup boot: {:?}",
            page.err()
        );

        Ok(())
    })
    .await
}
