use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use serde_json::Value;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

/// Simple HTTP mock server for TVM CLI GraphQL and REST tests.
/// No external dependencies beyond tokio and serde_json.
pub struct MockGraphQLServer {
    listener: TcpListener,
    fixtures: Arc<Mutex<HashMap<String, Value>>>,
    port: u16,
}

impl MockGraphQLServer {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        Self { listener, fixtures: Arc::new(Mutex::new(HashMap::new())), port }
    }

    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub fn set_fixture(&self, key: &str, value: Value) {
        self.fixtures.lock().unwrap().insert(key.to_string(), value);
    }

    pub fn load_fixture(&self, path: &str) -> Value {
        let content = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    pub fn setup_default_fixtures(&self) {
        let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("graphql");

        let info = self.load_fixture(&base.join("info.json").to_string_lossy());
        let accounts = self.load_fixture(&base.join("accounts_active.json").to_string_lossy());
        let transactions = self.load_fixture(&base.join("transactions.json").to_string_lossy());
        let transactions_empty =
            self.load_fixture(&base.join("transactions_empty.json").to_string_lossy());
        let blocks = self.load_fixture(&base.join("blocks.json").to_string_lossy());
        let messages = self.load_fixture(&base.join("messages.json").to_string_lossy());
        let account_v2 = self.load_fixture(&base.join("account_v2.json").to_string_lossy());
        let zerostates = self.load_fixture(&base.join("zerostates.json").to_string_lossy());
        let aggregate = self.load_fixture(&base.join("aggregate_count.json").to_string_lossy());
        let error = self.load_fixture(&base.join("error.json").to_string_lossy());

        self.set_fixture("info", info);
        self.set_fixture("accounts", accounts);
        self.set_fixture("transactions", transactions);
        self.set_fixture("transactions_empty", transactions_empty);
        self.set_fixture("blocks", blocks);
        self.set_fixture("messages", messages);
        self.set_fixture("account_v2", account_v2);
        self.set_fixture("zerostates", zerostates);
        self.set_fixture("aggregate", aggregate);
        self.set_fixture("error", error);
    }

    pub async fn run(self) {
        let fixtures = self.fixtures.clone();
        tokio::spawn(async move {
            loop {
                let (stream, _) = self.listener.accept().await.unwrap();
                let fixtures = fixtures.clone();
                tokio::spawn(handle_connection(stream, fixtures));
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, fixtures: Arc<Mutex<HashMap<String, Value>>>) {
    // A single read is not guaranteed to contain the complete POST body.
    // Read the headers first, then honor Content-Length before routing it.
    let mut request_bytes = Vec::with_capacity(16 * 1024);
    let header_end = loop {
        let mut chunk = [0u8; 8192];
        let n = match stream.read(&mut chunk).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };
        request_bytes.extend_from_slice(&chunk[..n]);
        if let Some(end) = request_bytes.windows(4).position(|window| window == b"\r\n\r\n") {
            break end + 4;
        }
        if request_bytes.len() > 1024 * 1024 {
            return;
        }
    };

    let header_text = String::from_utf8_lossy(&request_bytes[..header_end]);
    let content_length = header_text
        .lines()
        .find_map(|line| {
            line.strip_prefix("Content-Length:").or_else(|| line.strip_prefix("content-length:"))
        })
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(0);
    while request_bytes.len() < header_end + content_length {
        let mut chunk = [0u8; 8192];
        let n = match stream.read(&mut chunk).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };
        request_bytes.extend_from_slice(&chunk[..n]);
    }

    let request = String::from_utf8_lossy(&request_bytes);

    let response_body = {
        let fixtures = fixtures.lock().unwrap();
        if request.starts_with("GET /graphql?query=%7Binfo") {
            fixtures
                .get("info")
                .cloned()
                .unwrap_or(serde_json::json!({"data":{"info":{"version":"0.0.0"}}}))
        } else if request.starts_with("GET /graphql") {
            fixtures
                .get("info")
                .cloned()
                .unwrap_or(serde_json::json!({"data":{"info":{"version":"0.0.0"}}}))
        } else if request.starts_with("POST /graphql") {
            // Route based on GraphQL query string in the request body
            let body = extract_body(&request);
            let query = body.as_ref().and_then(|b| get_query_from_body(b));
            let (op_type, collection) = query.map_or(("query", "unknown"), |q| detect_operation(q));
            if query.is_some_and(|query| query.contains("force_graphql_error")) {
                fixtures
                    .get("error")
                    .cloned()
                    .unwrap_or(serde_json::json!({"errors":[{"message":"fixture GraphQL error"}]}))
            } else {
                match op_type {
                    "aggregate" => fixtures
                        .get("aggregate")
                        .cloned()
                        .unwrap_or(serde_json::json!({"data":{"aggregateTransactions":[]}})),
                    "query" => match collection {
                        "zerostates" => fixtures
                            .get("zerostates")
                            .cloned()
                            .unwrap_or(serde_json::json!({"data":{"zerostates":[]}})),
                        "accounts" => fixtures
                            .get("accounts")
                            .cloned()
                            .unwrap_or(serde_json::json!({"data":{"accounts":[]}})),
                        "transactions" => fixtures
                            .get("transactions")
                            .cloned()
                            .unwrap_or(serde_json::json!({"data":{"transactions":[]}})),
                        "transactions_empty" => fixtures
                            .get("transactions_empty")
                            .cloned()
                            .unwrap_or(serde_json::json!({"data":{"transactions":[]}})),
                        "blocks" => fixtures
                            .get("blocks")
                            .cloned()
                            .unwrap_or(serde_json::json!({"data":{"blocks":[]}})),
                        "messages" => fixtures
                            .get("messages")
                            .cloned()
                            .unwrap_or(serde_json::json!({"data":{"messages":[]}})),
                        _ => fixtures
                            .get("accounts")
                            .cloned()
                            .unwrap_or(serde_json::json!({"data":{"accounts":[]}})),
                    },
                    _ => fixtures
                        .get("accounts")
                        .cloned()
                        .unwrap_or(serde_json::json!({"data":{"accounts":[]}})),
                }
            }
        } else if request.starts_with("POST /v2/messages")
            || request.starts_with("POST /v3/messages")
        {
            serde_json::json!({
                "result": {
                    "message_hash": "29cf2a52fe0154c8f03f3a4a29dab80f9fb308b9c4dfb1649cb3f08047deb049",
                    "block_hash": null,
                    "tx_hash": null,
                    "return_value": null,
                    "aborted": false,
                    "exit_code": 0,
                    "thread_id": "0000000000000000000000000000000000000000000000000000000000000000",
                    "producers": ["127.0.0.1"],
                    "current_time": "1760103403",
                    "account_id": "06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1",
                    "dapp_id": "0000000000000000000000000000000000000000000000000000000000000000"
                }
            })
        } else if request.starts_with("GET /v2/account") {
            fixtures
                .get("account_v2")
                .cloned()
                .unwrap_or(serde_json::json!({"boc":"","account_id":"","dapp_id":""}))
        } else if request.starts_with("GET /v3/account") {
            fixtures
                .get("account_v2")
                .cloned()
                .unwrap_or(serde_json::json!({"boc":"","account_id":"","dapp_id":""}))
        } else {
            serde_json::json!({"data":{}})
        }
    };

    let body = response_body.to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n\
         {}",
        body.len(),
        body
    );

    let _ = stream.write_all(response.as_bytes()).await;
}

fn extract_body(request: &str) -> Option<Value> {
    let parts: Vec<&str> = request.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return None;
    }
    let body = parts[1..].join("\r\n\r\n");
    serde_json::from_str(&body).ok()
}

fn get_query_from_body(body: &Value) -> Option<&str> {
    body.get("query").and_then(|q| q.as_str())
}

fn detect_operation(query: &str) -> (&str, &str) {
    // Returns (operation_type, collection_name)
    // e.g. ("aggregate", "transactions") or ("query", "accounts")
    if query.contains("aggregateTransactions") {
        ("aggregate", "transactions")
    } else if query.contains("aggregateAccounts") {
        ("aggregate", "accounts")
    } else if query.contains("aggregateBlocks") {
        ("aggregate", "blocks")
    } else if query.contains("aggregateMessages") {
        ("aggregate", "messages")
    } else if query.contains("orderBy") {
        // fetch with pagination — return empty to break loop
        if query.contains("transactions") {
            ("query", "transactions_empty")
        } else {
            ("query", "unknown")
        }
    } else if query.contains("transactions(") || query.contains("transactions {") {
        ("query", "transactions")
    } else if query.contains("accounts(") || query.contains("accounts {") {
        ("query", "accounts")
    } else if query.contains("blocks(") || query.contains("blocks {") {
        ("query", "blocks")
    } else if query.contains("messages(") || query.contains("messages {") {
        ("query", "messages")
    } else if query.contains("zerostates(") || query.contains("zerostates {") {
        ("query", "zerostates")
    } else {
        ("query", "unknown")
    }
}
