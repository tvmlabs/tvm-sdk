use std::fs;

use predicates::prelude::*;

mod common;
use common::cargo_bin_smart;
use common::with_mock_server;

const TEST_ADDRESS_LEGACY: &str =
    "0:06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1";
const TEST_ADDRESS_V3: &str = "0000000000000000000000000000000000000000000000000000000000000000::06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1";
const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn test_query_raw_accounts_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("query-raw")
            .arg("accounts")
            .arg("id")
            .arg("--limit")
            .arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(TEST_ADDRESS_LEGACY))
            .stdout(predicate::str::contains("Active"));
    });
}

#[test]
fn test_query_raw_accounts_filter_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("query-raw")
            .arg("accounts")
            .arg("id")
            .arg("--filter")
            .arg(r#"{"id":{"eq":"0:06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1"}}"#)
            .arg("--limit")
            .arg("1");
        cmd.assert().success().stdout(predicate::str::contains(TEST_ADDRESS_LEGACY));
    });
}

#[test]
fn test_query_raw_transactions_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("query-raw")
            .arg("transactions")
            .arg("id status")
            .arg("--limit")
            .arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Finalized"))
            .stdout(predicate::str::contains("balance_delta"));
    });
}

#[test]
fn test_query_raw_blocks_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("query-raw")
            .arg("blocks")
            .arg("id seq_no")
            .arg("--limit")
            .arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("42"))
            .stdout(predicate::str::contains("seq_no"));
    });
}

#[test]
fn test_account_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url").arg(&url).arg("account").arg(TEST_ADDRESS_V3);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Active"))
            .stdout(predicate::str::contains("balance:"));
    });
}

#[test]
fn test_account_json_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("-j").arg("--url").arg(&url).arg("account").arg(TEST_ADDRESS_V3);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(r#""acc_type": "Active""#))
            .stdout(predicate::str::contains(r#""balance""#));
    });
}

#[test]
fn test_fee_storage_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("fee")
            .arg("storage")
            .arg(TEST_ADDRESS_V3)
            .arg("--period")
            .arg("86400");
        cmd.assert().success().stdout(predicate::str::contains("Storage fee per 86400 seconds"));
    });
}

#[test]
fn test_fee_storage_json_with_mock() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("-j")
            .arg("--url")
            .arg(&url)
            .arg("fee")
            .arg("storage")
            .arg(TEST_ADDRESS_V3)
            .arg("--period")
            .arg("3600");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("storage_fee"))
            .stdout(predicate::str::contains("period"));
    });
}

#[test]
fn test_fetch_with_mock() {
    with_mock_server(|url| {
        let output = std::env::temp_dir().join("tvm_cli_fetch_test.jsonl");
        let _ = fs::remove_file(&output);

        let mut cmd = cargo_bin_smart();
        cmd.arg("--url").arg(&url).arg("fetch").arg(TEST_ADDRESS_LEGACY).arg(&output);
        cmd.assert().success().stdout(predicate::str::contains("Succeeded"));

        // Verify output file exists and contains zerostate
        assert!(output.exists());
        let content = fs::read_to_string(&output).unwrap();
        assert!(content.contains(TEST_ADDRESS_LEGACY));
        let _ = fs::remove_file(&output);
    });
}

#[test]
fn test_deploy_with_mock() {
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);
    let keys = format!("{}/boc/contract.keys.json", FIXTURES_DIR);
    let config = std::env::temp_dir().join("tvm_cli_deploy_mock.conf.json");
    let _ = fs::remove_file(&config);

    with_mock_server(move |url| {
        // Set config with mock URL
        let mut cmd = cargo_bin_smart();
        cmd.arg("--config").arg(&config).arg("config").arg("--url").arg(&url);
        cmd.assert().success();

        // Deploy with mock server
        let mut cmd = cargo_bin_smart();
        cmd.arg("--config")
            .arg(&config)
            .arg("deploy")
            .arg(&tvc)
            .arg("{}")
            .arg("--abi")
            .arg(&abi)
            .arg("--sign")
            .arg(&keys)
            .arg("--dst-dapp-id")
            .arg("0000000000000000000000000000000000000000000000000000000000000000");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Deploying"))
            .stdout(predicate::str::contains("Transaction succeeded"));

        let _ = fs::remove_file(&config);
    });
}

#[test]
#[ignore = "fetch_block requires complex block BOC fixture"]
fn test_fetch_block_with_mock() {
    with_mock_server(|url| {
        let output = std::env::temp_dir().join("tvm_cli_fetch_block_test.boc");
        let _ = fs::remove_file(&output);

        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("fetch-block")
            .arg("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
            .arg(&output);
        cmd.assert().success();

        let _ = fs::remove_file(&output);
    });
}

#[test]
fn test_sendfile_with_mock() {
    let msg_path = format!("{}/boc/message_sample.boc", FIXTURES_DIR);
    with_mock_server(move |url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("sendfile")
            .arg("--dst-dapp-id")
            .arg("0000000000000000000000000000000000000000000000000000000000000000")
            .arg(&msg_path);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Sending message"))
            .stdout(predicate::str::contains("Succeded"));
    });
}

#[test]
fn test_sendfile_json_with_mock() {
    let msg_path = format!("{}/boc/message_sample.boc", FIXTURES_DIR);
    with_mock_server(move |url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("-j")
            .arg("--url")
            .arg(&url)
            .arg("sendfile")
            .arg("--dst-dapp-id")
            .arg("0000000000000000000000000000000000000000000000000000000000000000")
            .arg(&msg_path);
        // sendfile in JSON mode returns empty stdout (no output, just success)
        cmd.assert().success();
    });
}
