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
const OTHER_HEX: &str = "3333333333333333333333333333333333333333333333333333333333333333";

/// A minimal GraphQL `/graphql` handler returning the given server version,
/// satisfying the `Endpoint::resolve` info-query so the probe succeeds.
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

/// Legacy v2 server. `with_graphql` controls whether it exposes a `/graphql`
/// endpoint reporting version 0.9.0 (so the probe resolves to a v2 verdict).
async fn mock_v2_server(port: u16, with_graphql: bool) -> JoinHandle<()> {
    #[derive(Deserialize)]
    struct Params {
        address: String,
    }
    let mut app = Router::new().route(
        "/v2/account",
        get(|headers: HeaderMap, Query(p): Query<Params>| async move {
            let auth = headers.get("Authorization").map(|v| v.to_str().unwrap_or(""));
            // Legacy server: expects a workchain-prefixed address; response
            // contains an optional dapp_id and no account_id.
            match (p.address.as_str(), auth) {
                (addr, Some("Bearer secret")) if addr == format!("0:{}", ACC_HEX) => {
                    Json(json!({ "boc": "te6ccAAS", "dapp_id": null, "state_timestamp": null }))
                        .into_response()
                }
                (_, Some("Bearer secret")) => (StatusCode::NOT_FOUND, "not found").into_response(),
                _ => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            }
        }),
    );
    if with_graphql {
        app = app.route("/graphql", graphql_info_handler("0.9.0"));
    }
    spawn_server(port, app).await
}

/// New v3 server. `with_graphql` controls whether it exposes a `/graphql`
/// endpoint reporting version 1.0.0. Its `/v2/account` handler deserializes
/// `account_id`+`dapp_id`, so a legacy `?address=...` request is rejected with
/// 400 — which drives the v2->v3 fallback when GraphQL is unavailable.
async fn mock_v3_server(port: u16, with_graphql: bool) -> JoinHandle<()> {
    #[derive(Deserialize)]
    struct Params {
        account_id: String,
        dapp_id: String,
    }
    let mut app = Router::new().route(
        "/v2/account",
        get(|headers: HeaderMap, Query(p): Query<Params>| async move {
            let auth = headers.get("Authorization").map(|v| v.to_str().unwrap_or(""));
            match (p.account_id.as_str(), p.dapp_id.as_str(), auth) {
                (a, d, Some("Bearer secret")) if a == ACC_HEX && d == DAPP_HEX => Json(json!({
                    "boc": "te6ccAAS",
                    "dapp_id": DAPP_HEX,
                    "state_timestamp": 1700000000u64,
                    "account_id": ACC_HEX,
                }))
                .into_response(),
                _ => (StatusCode::NOT_FOUND, "not found").into_response(),
            }
        }),
    );
    if with_graphql {
        app = app.route("/graphql", graphql_info_handler("1.0.0"));
    }
    spawn_server(port, app).await
}

async fn spawn_server(port: u16, app: Router) -> JoinHandle<()> {
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

#[tokio::test]
async fn get_account_uses_v2_when_graphql_reports_v2() -> ClientResult<()> {
    // GraphQL reports 0.9.0 (< 1.0.0) -> authoritative v2; the v2 form is used.
    let handle = mock_v2_server(18610, true).await;
    let client = make_client(18610);

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
async fn get_account_uses_v3_when_graphql_reports_v3() -> ClientResult<()> {
    // GraphQL reports 1.0.0 -> authoritative v3; the v3 form is used directly
    // (the v2 form is never attempted — this mock would 400 it).
    let handle = mock_v3_server(18611, true).await;
    let client = make_client(18611);

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
async fn get_account_falls_back_to_v2_when_graphql_unavailable() -> ClientResult<()> {
    // No GraphQL: the probe fails, so we default to v2; the legacy node accepts
    // the v2 form on the first try.
    let handle = mock_v2_server(18612, false).await;
    let client = make_client(18612);

    let params =
        ParamsOfGetAccount { account_id: ACC_HEX.to_string(), dapp_id: DAPP_HEX.to_string() };
    let res = account::get_account(client.clone(), params).await?;

    assert_eq!(res.boc, "te6ccAAS");
    assert_eq!(res.account_id, ACC_HEX);

    handle.abort();
    Ok(())
}

#[tokio::test]
async fn get_account_falls_back_to_v3_when_graphql_unavailable_and_v2_rejected() -> ClientResult<()>
{
    // No GraphQL + v3-only node: the probe fails, the v2 form is rejected (400),
    // so get_account retries the v3 form and succeeds.
    let handle = mock_v3_server(18613, false).await;
    let client = make_client(18613);

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
async fn get_account_trusts_graphql_v2_verdict_and_does_not_retry_v3() {
    // GraphQL says v2; a v2 error (account not found) is genuine and must be
    // returned as-is, NOT retried as v3.
    let handle = mock_v2_server(18614, true).await;
    let client = make_client(18614);

    let params =
        ParamsOfGetAccount { account_id: OTHER_HEX.to_string(), dapp_id: DAPP_HEX.to_string() };
    let err = account::get_account(client, params).await.unwrap_err();
    // The v2 endpoint returns 404 "not found"; a wrong v3 retry would instead
    // hit a 400 from the v2 mock's Query extractor.
    assert!(err.message().contains("not found"), "got: {}", err.message());

    handle.abort();
}

#[tokio::test]
async fn get_account_v2_works_without_dapp_id() -> ClientResult<()> {
    // A v2-only caller may leave dapp_id empty: GraphQL says v2, the v2 form
    // carries no dapp_id, and no fallback is needed.
    let handle = mock_v2_server(18615, true).await;
    let client = make_client(18615);

    let params = ParamsOfGetAccount { account_id: ACC_HEX.to_string(), dapp_id: String::new() };
    let res = account::get_account(client.clone(), params).await?;

    assert_eq!(res.boc, "te6ccAAS");
    assert_eq!(res.account_id, ACC_HEX);
    assert_eq!(res.dapp_id, ""); // none from server, empty from params

    handle.abort();
    Ok(())
}

#[tokio::test]
async fn get_account_rejects_account_id_with_workchain() {
    let client = make_client(18616);
    let params = ParamsOfGetAccount {
        account_id: format!("0:{}", &ACC_HEX[..62]), // contains ':'
        dapp_id: DAPP_HEX.to_string(),
    };
    let err = account::get_account(client, params).await.unwrap_err();
    assert!(err.message().contains("account_id"));
}

#[tokio::test]
async fn get_account_rejects_empty_dapp_id_on_v3() {
    // GraphQL says v3, so a dapp_id is required up front; an empty one is
    // reported with a dapp_id validation error.
    let handle = mock_v3_server(18617, true).await;
    let client = make_client(18617);

    let params = ParamsOfGetAccount { account_id: ACC_HEX.to_string(), dapp_id: String::new() };
    let err = account::get_account(client, params).await.unwrap_err();
    assert!(err.message().contains("dapp_id"));
    handle.abort();
}
