// 2022-2026 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//

use std::sync::Arc;
use std::time::Duration;

use axum::Json;
use axum::Router;
use axum::extract::Query;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;
use serde_json::json;
use tokio::task::JoinHandle;

use crate::ClientConfig;
use crate::ClientContext;
use crate::account;
use crate::account::ParamsOfGetAccount;
use crate::error::ClientResult;
use crate::net::NetworkConfig;

const ACC_HEX: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const DAPP_HEX: &str = "2222222222222222222222222222222222222222222222222222222222222222";

/// A minimal GraphQL /graphql handler that returns the given server version
/// string, satisfying the `Endpoint::resolve` info-query.
fn graphql_info_handler(version: &'static str) -> axum::routing::MethodRouter {
    get(move || async move {
        Json(json!({
            "data": {
                "info": {
                    "version": version,
                    "time": 1700000000_i64,
                    "latency": 1_i64,
                    "rempEnabled": false
                }
            }
        }))
    })
}

async fn mock_v2_server(port: u16) -> JoinHandle<()> {
    #[derive(Deserialize)]
    struct Params {
        address: String,
    }
    let app = Router::new()
        // GraphQL info endpoint so Endpoint::resolve succeeds (version < 1.0.0).
        .route("/graphql", graphql_info_handler("0.9.0"))
        .route(
            "/v2/account",
            get(|headers: HeaderMap, Query(p): Query<Params>| async move {
                let auth = headers.get("Authorization").map(|v| v.to_str().unwrap_or(""));
                // Legacy server: expects workchain-prefixed address; response
                // contains optional dapp_id and no account_id.
                match (p.address.as_str(), auth) {
                    (addr, Some("Bearer secret")) if addr == format!("0:{}", ACC_HEX) =>
                        Json(json!({ "boc": "te6ccAAS", "dapp_id": null, "state_timestamp": null }))
                            .into_response(),
                    (_, Some("Bearer secret")) =>
                        (StatusCode::NOT_FOUND, "not found").into_response(),
                    _ => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                }
            }),
        );
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
    handle
}

async fn mock_v3_server(port: u16) -> JoinHandle<()> {
    #[derive(Deserialize)]
    struct Params {
        account_id: String,
        dapp_id: String,
    }
    let app = Router::new()
        // GraphQL info endpoint so Endpoint::resolve succeeds (version >= 1.0.0).
        .route("/graphql", graphql_info_handler("1.0.0"))
        .route(
            "/v2/account",
            get(|headers: HeaderMap, Query(p): Query<Params>| async move {
                let auth = headers.get("Authorization").map(|v| v.to_str().unwrap_or(""));
                // New server: expects account_id (no workchain) and dapp_id.
                match (p.account_id.as_str(), p.dapp_id.as_str(), auth) {
                    (a, d, Some("Bearer secret")) if a == ACC_HEX && d == DAPP_HEX =>
                        Json(json!({
                            "boc": "te6ccAAS",
                            "dapp_id": DAPP_HEX,
                            "state_timestamp": 1700000000u64,
                            "account_id": ACC_HEX,
                        })).into_response(),
                    _ => (StatusCode::NOT_FOUND, "not found").into_response(),
                }
            }),
        );
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
    handle
}

fn make_client(port: u16) -> Arc<ClientContext> {
    let config = ClientConfig {
        network: NetworkConfig {
            endpoints: Some(vec![format!("http://127.0.0.1:{port}")]),
            api_token: Some("secret".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    Arc::new(ClientContext::new(config).unwrap())
}

/// Triggers endpoint resolution so the server version gets populated from the
/// /graphql info response before any REST calls are made.
async fn resolve_endpoint_version(client: &Arc<ClientContext>) {
    let sl = client.get_server_link().unwrap();
    // Ignore the result — we just want the side effect of populating the
    // query_endpoint slot with its server_version.
    let _ = sl.state().get_query_endpoint().await;
}

#[tokio::test]
async fn get_account_v2_returns_account_id_and_dapp_id_from_params() -> ClientResult<()> {
    let handle = mock_v2_server(18610).await;
    let client = make_client(18610);
    // Resolve endpoint — the mock server reports version 0.9.0 (< 1.0.0),
    // so supports_dapp_id() will return false and v2 wire format is used.
    resolve_endpoint_version(&client).await;

    let params =
        ParamsOfGetAccount { account_id: ACC_HEX.to_string(), dapp_id: DAPP_HEX.to_string() };
    let res = account::get_account(client.clone(), params).await?;

    assert_eq!(res.boc, "te6ccAAS");
    assert_eq!(res.account_id, ACC_HEX); // derived from params (server returned none)
    assert_eq!(res.dapp_id, DAPP_HEX); // derived from params (server returned null)
    assert!(res.state_timestamp.is_none());

    handle.abort();
    Ok(())
}

#[tokio::test]
async fn get_account_v3_returns_server_account_id_and_dapp_id() -> ClientResult<()> {
    let handle = mock_v3_server(18611).await;
    let client = make_client(18611);
    // Resolve endpoint — the mock server reports version 1.0.0 (>= 1.0.0),
    // so supports_dapp_id() will return true and v3 wire format is used.
    resolve_endpoint_version(&client).await;

    let params =
        ParamsOfGetAccount { account_id: ACC_HEX.to_string(), dapp_id: DAPP_HEX.to_string() };
    let res = account::get_account(client.clone(), params).await?;

    assert_eq!(res.boc, "te6ccAAS");
    assert_eq!(res.account_id, ACC_HEX);
    assert_eq!(res.dapp_id, DAPP_HEX);
    assert_eq!(res.state_timestamp, Some(1700000000));

    handle.abort();
    Ok(())
}

#[tokio::test]
async fn get_account_rejects_account_id_with_workchain() {
    let client = make_client(18612);
    let params = ParamsOfGetAccount {
        account_id: format!("0:{}", &ACC_HEX[..62]), // contains ':'
        dapp_id: DAPP_HEX.to_string(),
    };
    let err = account::get_account(client, params).await.unwrap_err();
    assert!(err.message().contains("account_id"));
}

#[tokio::test]
async fn get_account_rejects_empty_dapp_id() {
    // Use a v3 server so that dapp_id validation is triggered.
    let handle = mock_v3_server(18613).await;
    let client = make_client(18613);
    resolve_endpoint_version(&client).await; // resolves to 1.0.0 → v3 mode

    let params = ParamsOfGetAccount { account_id: ACC_HEX.to_string(), dapp_id: String::new() };
    let err = account::get_account(client, params).await.unwrap_err();
    assert!(err.message().contains("dapp_id"));
    handle.abort();
}
