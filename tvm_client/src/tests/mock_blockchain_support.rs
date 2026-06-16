use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use tokio::task::JoinHandle;

const TEST_ACCOUNT_ID: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const TEST_DAPP_ID: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";

#[derive(Clone)]
struct MockState {
    version: String,
}

pub struct MockBlockchain {
    endpoint: String,
    handle: JoinHandle<()>,
}

impl MockBlockchain {
    pub async fn start() -> Self {
        let state = MockState { version: "1.2.3".to_owned() };
        let app = Router::new()
            .route("/graphql", get(info).post(graphql))
            .route("/v2/account", get(account))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        Self { endpoint: format!("http://{}", addr), handle }
    }

    pub fn endpoint(&self) -> String {
        self.endpoint.clone()
    }
}

impl Drop for MockBlockchain {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

async fn info(State(state): State<MockState>) -> Json<Value> {
    Json(info_response(&state))
}

async fn graphql(
    State(state): State<MockState>,
    Json(body): Json<Value>,
) -> axum::response::Response {
    let query = body.get("query").and_then(Value::as_str).unwrap_or_default();
    let body_text = body.to_string();

    if query.contains("forceUnauthorized") {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    if query.contains("forceHttp404") {
        return (StatusCode::NOT_FOUND, "not found").into_response();
    }

    if query.contains("forceMalformedJson") {
        return (StatusCode::OK, "not-json").into_response();
    }

    if query.contains("forceGraphqlError") {
        return Json(json!({
            "errors": [{
                "message": "forced graphql failure",
                "extensions": { "exception": { "code": 777 } }
            }]
        }))
        .into_response();
    }

    if query.contains("forcePayloadGraphqlError") {
        return Json(json!({
            "payload": {
                "errors": [{
                    "message": "forced payload graphql failure",
                    "extensions": { "code": 778 }
                }]
            }
        }))
        .into_response();
    }

    if query.contains("aggregateBlocks") {
        if body_text.contains("bad-aggregate") {
            return Json(json!({ "data": { "aggregateBlocks": { "count": "2" } } }))
                .into_response();
        }
        return Json(json!({ "data": { "aggregateBlocks": ["2"] } })).into_response();
    }

    if query.contains("counterparties") {
        let after_second = body_text.contains("cursor-2");
        let result = if after_second {
            json!([
                {
                    "counterparty": "0:counterparty-3",
                    "last_message_id": "message-3",
                    "cursor": "cursor-3"
                }
            ])
        } else {
            json!([
                {
                    "counterparty": "0:counterparty-1",
                    "last_message_id": "message-1",
                    "cursor": "cursor-1"
                },
                {
                    "counterparty": "0:counterparty-2",
                    "last_message_id": "message-2",
                    "cursor": "cursor-2"
                }
            ])
        };
        return Json(json!({ "data": { "counterparties": result } })).into_response();
    }

    if query.contains("blocks") {
        if query.contains("timeout") {
            if body_text.contains("777") {
                return Json(json!({ "data": { "blocks": [] } })).into_response();
            }
            return Json(json!({
                "data": {
                    "blocks": [
                        {
                            "id": "mock-wait-block",
                            "seq_no": 2
                        }
                    ]
                }
            }))
            .into_response();
        }
        if body_text.contains("999") {
            return Json(json!({ "data": { "blocks": [] } })).into_response();
        }
        if body_text.contains("13") {
            return Json(json!({ "data": { "blocks": { "id": "not-an-array" } } })).into_response();
        }
        if body_text.contains("42") {
            return Json(json!({ "data": { "other": [] } })).into_response();
        }
        return Json(json!({
            "data": {
                "blocks": [
                    { "id": "mock-block-1", "seq_no": 1 }
                ]
            }
        }))
        .into_response();
    }

    Json(info_response(&state)).into_response()
}

#[derive(Deserialize)]
struct AccountParamsV2 {
    address: String,
}

async fn account(
    headers: HeaderMap,
    query: axum::extract::Query<serde_json::Value>,
) -> axum::response::Response {
    let auth = headers.get("Authorization").and_then(|value| value.to_str().ok());

    // Check for v3 format (account_id + dapp_id)
    if let (Some(account_id), Some(dapp_id)) = (
        query.get("account_id").and_then(|v| v.as_str()),
        query.get("dapp_id").and_then(|v| v.as_str()),
    ) {
        return handle_account_v3(account_id, dapp_id, auth);
    }

    // Fall back to v2 format (address)
    let params = match serde_json::from_value::<AccountParamsV2>(query.0) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid parameters").into_response(),
    };

    handle_account_v2(params.address, auth)
}

fn handle_account_v2(address: String, auth: Option<&str>) -> axum::response::Response {
    match (address.as_str(), auth) {
        ("0:11111", Some("Bearer secret")) => Json(json!({
            "boc": "te6ccAAS",
            "dapp_id": "mock-dapp",
            "state_timestamp": 1_700_000_001_u64
        }))
        .into_response(),
        ("0:55555", Some("Bearer secret")) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
        ("0:bad-json", Some("Bearer secret")) => {
            Json(json!({ "unexpected": true })).into_response()
        }
        ("0:not-json", Some("Bearer secret")) => (StatusCode::OK, "not-json").into_response(),
        (_, Some("Bearer secret")) => (StatusCode::NOT_FOUND, "not found").into_response(),
        (_, _) => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}

fn handle_account_v3(
    account_id: &str,
    dapp_id: &str,
    auth: Option<&str>,
) -> axum::response::Response {
    match (account_id, dapp_id, auth) {
        (a, d, Some("Bearer secret")) if a == TEST_ACCOUNT_ID && d == TEST_DAPP_ID => Json(json!({
            "boc": "te6ccAAS",
            "dapp_id": TEST_DAPP_ID,
            "state_timestamp": 1_700_000_001_u64,
            "account_id": TEST_ACCOUNT_ID
        }))
        .into_response(),
        // Test case: not found - return 404
        (
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            _,
            Some("Bearer secret"),
        ) => (StatusCode::NOT_FOUND, "not found").into_response(),
        // Test case: invalid response - return 500
        (
            "5555555555555555555555555555555555555555555555555555555555555555",
            _,
            Some("Bearer secret"),
        ) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        // Test case: invalid shape - return 200 with unexpected JSON
        (
            "badjsonbadjsonbadjsonbadjsonbadjsonbadjsonbadjsonbadjsonbadjson",
            _,
            Some("Bearer secret"),
        ) => Json(json!({ "unexpected": true })).into_response(),
        // Test case: invalid json - return 200 with plain text
        (
            "notjsonnotjsonnotjsonnotjsonnotjsonnotjsonnotjsonnotjsonnotjson",
            _,
            Some("Bearer secret"),
        ) => (StatusCode::OK, "not-json").into_response(),
        (_, _, Some("Bearer secret")) => (StatusCode::NOT_FOUND, "not found").into_response(),
        (_, _, _) => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}

fn info_response(state: &MockState) -> Value {
    json!({
        "data": {
            "info": {
                "version": state.version,
                "time": 1_700_000_000_000_i64,
                "latency": 1,
                "rempEnabled": false
            }
        }
    })
}
