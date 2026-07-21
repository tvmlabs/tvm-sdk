use predicates::prelude::*;

mod common;
use common::cargo_bin_smart;
use common::with_mock_server;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn missing_abi_is_reported() {
    let mut cmd = cargo_bin_smart();
    cmd.arg("body").arg("--abi").arg("tests/fixtures/does-not-exist.abi.json").arg("add").arg("{}");
    cmd.assert().failure().stdout(predicate::str::contains("Error"));
}

#[test]
fn malformed_method_parameters_are_rejected() {
    let abi = format!("{FIXTURES_DIR}/boc/contract.abi.json");
    let mut cmd = cargo_bin_smart();
    cmd.arg("body").arg("--abi").arg(abi).arg("add").arg("not-json");
    cmd.assert().failure().stdout(predicate::str::contains("Error"));
}

#[test]
fn invalid_seed_phrase_is_rejected() {
    let output = std::env::temp_dir().join("tvm-cli-invalid-keys.json");
    let mut cmd = cargo_bin_smart();
    cmd.arg("getkeypair")
        .arg("--output")
        .arg(&output)
        .arg("--phrase")
        .arg("not a valid seed phrase");
    cmd.assert().failure().stdout(predicate::str::contains("Error"));
    let _ = std::fs::remove_file(output);
}

#[test]
fn invalid_public_key_is_rejected() {
    let mut cmd = cargo_bin_smart();
    cmd.arg("nodeid").arg("--pubkey").arg("not-hex");
    cmd.assert().failure().stdout(predicate::str::contains("Error"));
}

#[test]
fn garbage_sendfile_is_rejected_before_network_access() {
    let path = std::env::temp_dir().join("tvm-cli-garbage-message.boc");
    std::fs::write(&path, b"not a boc").unwrap();
    let mut cmd = cargo_bin_smart();
    cmd.arg("sendfile").arg(&path);
    cmd.assert().failure().stdout(predicate::str::contains("Error"));
    let _ = std::fs::remove_file(path);
}

#[test]
fn connection_errors_are_reported() {
    // This local port is not used by the test mock. Keep retries disabled so
    // this negative test remains fast and deterministic.
    let mut cmd = cargo_bin_smart();
    cmd.arg("--url")
        .arg("http://127.0.0.1:1")
        .arg("--retries")
        .arg("0")
        .arg("account")
        .arg("0:0000000000000000000000000000000000000000000000000000000000000000");
    cmd.assert().failure().stdout(predicate::str::contains("Error"));
}

#[test]
fn graphql_errors_are_reported() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url").arg(url).arg("query-raw").arg("accounts").arg("id force_graphql_error");
        cmd.assert().failure().stdout(predicate::str::contains("fixture GraphQL error"));
    });
}

#[test]
fn account_requests_can_still_use_the_mock_after_an_error_case() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(url)
            .arg("account")
            .arg("0:06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1");
        cmd.assert().success();
    });
}
