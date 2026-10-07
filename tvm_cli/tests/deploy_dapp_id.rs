use std::sync::Arc;

use assert_cmd::Command;
use axum::Router;
use axum::body::Body;
use axum::body::to_bytes;
use axum::routing::post;
use serde_json::json;
use testdir::testdir;
use tokio::sync::Mutex;

const BIN_NAME: &str = "tvm-cli";
// Hello has an empty constructor and needs no keypair, so `{}` deploys as-is.
// The other fixtures in this directory are not deployable images — see
// `params_file.rs:12-16`.
const TVC: &str = "tests/Hello.tvc";
const ABI: &str = "tests/Hello.abi.json";

/// Serves `/v2/messages`, records the payload, and answers with a minimal
/// success result. Binds to port 0 so repeated and parallel runs do not
/// collide.
async fn mock_messages_endpoint() -> (String, Arc<Mutex<Option<serde_json::Value>>>) {
    let captured: Arc<Mutex<Option<serde_json::Value>>> = Arc::new(Mutex::new(None));
    let captured_handle = captured.clone();
    let app = Router::new().route(
        "/v2/messages",
        post(move |body: Body| {
            let captured = captured_handle.clone();
            async move {
                let bytes = to_bytes(body, usize::MAX).await.unwrap();
                *captured.lock().await = Some(serde_json::from_slice(&bytes).unwrap());
                axum::Json(json!({
                    "result": { "message_hash": "deadbeef", "thread_id": null, "producers": [] },
                    "error": null,
                    "ext_message_token": null
                }))
            }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (url, captured)
}

/// `--url` overrides only `endpoints` and `url`. Everything else — `wc`,
/// `local_run`, `keys_path`, retry settings — comes from a config file, and
/// `default_config_name()` resolves against the current directory, where
/// `tvm_cli/tvm-cli.conf.json` already sits. Without an explicit `--config`
/// the test would silently inherit it.
fn isolated_config(url: &str) -> std::path::PathBuf {
    let dir = testdir!();
    let path = dir.join("tvm-cli.conf.json");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&json!({
            "url": url,
            "endpoints": [url],
            "wc": 0,
            "local_run": false,
            "async_call": false,
            "is_json": true
        }))
        .unwrap(),
    )
    .unwrap();
    path
}

/// Multi-threaded on purpose: the mock runs as a task while `assert_cmd`
/// blocks the thread on the child process. A current-thread runtime would
/// starve the server and deadlock instead of failing.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn deploy_sends_dapp_id_equal_to_account_id() {
    let (url, captured) = mock_messages_endpoint().await;
    let config = isolated_config(&url);

    let output = tokio::task::spawn_blocking(move || {
        Command::cargo_bin(BIN_NAME)
            .unwrap()
            .arg("--config")
            .arg(&config)
            .arg("deploy")
            .arg(TVC)
            .arg("{}")
            .arg("--abi")
            .arg(ABI)
            .output()
            .unwrap()
    })
    .await
    .unwrap();

    let body = captured.lock().await.clone().expect("no message reached /v2/messages");
    let item = &body[0];
    let dapp_id = item["dapp_id"].as_str().expect("dapp_id in payload");
    let account_id = item["account_id"].as_str().expect("account_id in payload");

    assert_eq!(dapp_id, account_id, "a CLI deploy roots its own dapp");
    assert_eq!(dapp_id.len(), 64);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&format!("{account_id}::{account_id}")),
        "printed address must match the wire ids: {stdout}"
    );
}

/// Pins the workchain decision: `--wc 1` still sends and only then fails on
/// the printed address, exactly as before this change. A guard against
/// deriving through `strip_workchain`, which would move the failure ahead of
/// the send.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn deploy_with_non_zero_workchain_still_sends_before_failing() {
    let (url, captured) = mock_messages_endpoint().await;
    let config = isolated_config(&url);

    let output = tokio::task::spawn_blocking(move || {
        Command::cargo_bin(BIN_NAME)
            .unwrap()
            .arg("--config")
            .arg(&config)
            .arg("deploy")
            .arg(TVC)
            .arg("{}")
            .arg("--abi")
            .arg(ABI)
            .arg("--wc")
            .arg("1")
            .output()
            .unwrap()
    })
    .await
    .unwrap();

    assert!(captured.lock().await.is_some(), "the message must still be sent");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stderr.contains("non-zero workchain not supported")
            || stdout.contains("non-zero workchain not supported"),
        "expected the existing workchain error; stdout={stdout} stderr={stderr}"
    );
}
