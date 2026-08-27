// 2022-2026 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;

use axum::Json;
use axum::Router;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
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

/// v3 server. `with_graphql` controls whether it exposes a `/graphql`
/// endpoint; `complete` controls whether the account response carries
/// `account_id` / `dapp_id`. The `/v2/account` handler accepts only the v3
/// query form and 400s anything else.
async fn mock_v3_server(
    port: u16,
    with_graphql: bool,
    complete: bool,
) -> (JoinHandle<()>, Arc<AtomicUsize>) {
    let graphql_hits = Arc::new(AtomicUsize::new(0));
    let hits = graphql_hits.clone();
    let mut app = Router::new().route(
        "/v2/account",
        get(move |Query(q): Query<HashMap<String, String>>| async move {
            if !q.contains_key("account_id") || !q.contains_key("dapp_id") {
                return (StatusCode::BAD_REQUEST, "legacy form rejected").into_response();
            }
            let mut body = json!({ "boc": "te6ccAAS", "state_timestamp": 1_700_000_001_i64 });
            if complete {
                body["account_id"] = json!(ACC_HEX);
                body["dapp_id"] = json!(DAPP_HEX);
            }
            Json(body).into_response()
        }),
    );
    if with_graphql {
        app = app.route(
            "/graphql",
            get(move || {
                let hits = hits.clone();
                async move {
                    hits.fetch_add(1, Ordering::SeqCst);
                    Json(json!({"data": {"info": {
                        "version": "0.9.0", "time": 1_700_000_000_i64,
                        "latency": 1_i64, "rempEnabled": false
                    }}}))
                }
            }),
        );
    }
    (spawn_server(port, app).await, graphql_hits)
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_account_uses_v3_form_and_never_probes_graphql() -> ClientResult<()> {
    // The mock reports 0.9.0 and is ignored: version no longer selects a form,
    // and get_account must not call /graphql at all.
    let (handle, graphql_hits) = mock_v3_server(18611, true, true).await;
    let account = account::get_account(
        make_client(18611),
        ParamsOfGetAccount { account_id: ACC_HEX.to_owned(), dapp_id: DAPP_HEX.to_owned() },
    )
    .await?;
    assert_eq!(account.boc, "te6ccAAS");
    assert_eq!(account.account_id, ACC_HEX);
    assert_eq!(account.dapp_id, DAPP_HEX);
    assert_eq!(graphql_hits.load(Ordering::SeqCst), 0, "get_account must not probe GraphQL");
    handle.abort();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_account_works_without_graphql() -> ClientResult<()> {
    let (handle, _) = mock_v3_server(18612, false, true).await;
    let account = account::get_account(
        make_client(18612),
        ParamsOfGetAccount { account_id: ACC_HEX.to_owned(), dapp_id: DAPP_HEX.to_owned() },
    )
    .await?;
    assert_eq!(account.boc, "te6ccAAS");
    handle.abort();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_account_fills_ids_when_server_omits_them() -> ClientResult<()> {
    // Deliberate fallback: values come from `params` rather than being
    // returned empty inside a successful result.
    let (handle, _) = mock_v3_server(18613, true, false).await;
    let account = account::get_account(
        make_client(18613),
        ParamsOfGetAccount { account_id: ACC_HEX.to_owned(), dapp_id: DAPP_HEX.to_owned() },
    )
    .await?;
    assert_eq!(account.account_id, ACC_HEX);
    assert_eq!(account.dapp_id, DAPP_HEX);
    handle.abort();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_account_rejects_empty_dapp_id() {
    let (handle, _) = mock_v3_server(18614, true, true).await;
    let err = account::get_account(
        make_client(18614),
        ParamsOfGetAccount { account_id: ACC_HEX.to_owned(), dapp_id: String::new() },
    )
    .await
    .expect_err("empty dapp_id must be refused");
    assert!(err.message().contains("dapp_id"), "unexpected error: {}", err.message());
    handle.abort();
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
