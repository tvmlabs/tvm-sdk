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

async fn mock_server() -> JoinHandle<()> {
    #[derive(Deserialize)]
    struct Params {
        address: String,
    }

    let app = Router::new().route(
        "/v2/account",
        get(|headers: HeaderMap, Query(params): Query<Params>| async move {
            let auth = headers.get("Authorization").map(|v| v.to_str().unwrap_or(""));

            match (params.address.as_str(), auth) {
                ("0:11111", Some("Bearer secret")) => {
                    Json(json!({ "boc": "te6ccAAS" })).into_response()
                }
                ("0:55555", Some("Bearer secret")) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
                (_, Some("Bearer secret")) => (StatusCode::NOT_FOUND, "not found").into_response(),
                (_, _) => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8600").await.unwrap();
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    // Allow the Axum server to start before continuing testing.
    tokio::time::sleep(Duration::from_secs(1)).await;
    handle
}

#[tokio::test]
async fn test_get_account() -> ClientResult<()> {
    let handle = mock_server().await;
    let contract_boc = "te6ccAAS";

    let mut config = ClientConfig {
        network: NetworkConfig {
            endpoints: Some(vec!["http://127.0.0.1".to_string()]),
            api_token: Some("secret".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let client = Arc::new(ClientContext::new(config.clone()).unwrap());

    // Correct request
    let params = ParamsOfGetAccount { address: "0:11111".to_string() };
    let boc = account::get_account(client.clone(), params).await?.boc;
    assert_eq!(boc, contract_boc);

    // Not found address
    let params = ParamsOfGetAccount { address: "0:12344567890".to_string() };
    match account::get_account(client.clone(), params).await {
        Ok(_) => panic!("Expected an error but got Ok"),
        Err(e) => {
            assert_eq!(e.code, crate::net::ErrorCode::NotFound as u32);
        }
    }

    // Internal error
    let params = ParamsOfGetAccount { address: "0:55555".to_string() };
    match account::get_account(client.clone(), params).await {
        Ok(_) => panic!("Expected an error but got Ok"),
        Err(e) => {
            assert_eq!(e.code, crate::net::ErrorCode::InvalidServerResponse as u32);
        }
    }

    // Request with wrong authorization
    config.network.api_token = Some("wrong_token".to_string());
    let client = Arc::new(ClientContext::new(config.clone()).unwrap());
    let params = ParamsOfGetAccount { address: "0:12232323".to_string() };
    let response = account::get_account(client.clone(), params).await;
    match response {
        Ok(_) => panic!("Expected an error but got Ok"),
        Err(e) => {
            assert_eq!(e.code, crate::net::ErrorCode::Unauthorized as u32);
        }
    };

    // Request without authorization
    config.network.api_token = None;
    let client = Arc::new(ClientContext::new(config).unwrap());
    let params = ParamsOfGetAccount { address: "0:12232323".to_string() };
    let response = account::get_account(client.clone(), params).await;
    match response {
        Ok(_) => panic!("Expected an error but got Ok"),
        Err(e) => {
            assert_eq!(e.code, crate::net::ErrorCode::Unauthorized as u32);
        }
    };
    handle.abort();

    Ok(())
}
