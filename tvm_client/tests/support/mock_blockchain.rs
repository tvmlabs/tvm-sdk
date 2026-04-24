use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::get;
use serde_json::Value;
use serde_json::json;
use tokio::task::JoinHandle;

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
        let app = Router::new().route("/graphql", get(info).post(graphql)).with_state(state);
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

async fn graphql(State(state): State<MockState>, Json(body): Json<Value>) -> Json<Value> {
    let query = body.get("query").and_then(Value::as_str).unwrap_or_default();
    if query.contains("blocks") {
        Json(json!({
            "data": {
                "blocks": [
                    { "id": "mock-block-1", "seq_no": 1 }
                ]
            }
        }))
    } else {
        Json(info_response(&state))
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
