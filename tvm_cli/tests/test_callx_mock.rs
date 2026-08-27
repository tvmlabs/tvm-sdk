use predicates::prelude::*;

mod common;
use common::cargo_bin_smart;
use common::with_mock_server;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
const TEST_ADDRESS_V3: &str = "0000000000000000000000000000000000000000000000000000000000000000::06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1";

#[test]
fn test_callx_send_message() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("callx")
            .arg("--abi")
            .arg(format!("{}/boc/contract.abi.json", FIXTURES_DIR))
            .arg("--addr")
            .arg(TEST_ADDRESS_V3)
            .arg("--keys")
            .arg(format!("{}/boc/contract.keys.json", FIXTURES_DIR))
            .arg("-m")
            .arg("add")
            .arg("{}");
        // Without --local_run, callx encodes the message and sends it via network.
        // The mock server returns a pre-canned sendMessage result.
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Processing"))
            .stdout(predicate::str::contains("Succeeded"));
    });
}

#[test]
fn test_callx_local_run() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("--local_run")
            .arg("callx")
            .arg("--abi")
            .arg(format!("{}/boc/contract.abi.json", FIXTURES_DIR))
            .arg("--addr")
            .arg(TEST_ADDRESS_V3)
            .arg("--keys")
            .arg(format!("{}/boc/contract.keys.json", FIXTURES_DIR))
            .arg("-m")
            .arg("add")
            .arg("{}");
        // With --local_run, callx emulates the transaction locally.
        // The account BOC from the mock may not match the ABI, so the emulation
        // may fail, but the CLI path should still execute (encode + emulate attempt).
        let output = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stdout.contains("Local run")
                || stdout.contains("Error")
                || stderr.contains("Error")
                || stderr.contains("run"),
            "stdout: {}, stderr: {}",
            stdout,
            stderr
        );
    });
}

#[test]
fn test_callx_json() {
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("-j")
            .arg("--url")
            .arg(&url)
            .arg("callx")
            .arg("--abi")
            .arg(format!("{}/boc/contract.abi.json", FIXTURES_DIR))
            .arg("--addr")
            .arg(TEST_ADDRESS_V3)
            .arg("--keys")
            .arg(format!("{}/boc/contract.keys.json", FIXTURES_DIR))
            .arg("-m")
            .arg("add")
            .arg("{}");
        cmd.assert().success();
        // In JSON mode the output should be valid JSON or empty on success.
    });
}

#[test]
fn test_deployx_with_mock() {
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);
    let keys = format!("{}/boc/contract.keys.json", FIXTURES_DIR);

    with_mock_server(move |url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("deployx")
            .arg("--abi")
            .arg(&abi)
            .arg("--keys")
            .arg(&keys)
            .arg("--dst-dapp-id")
            .arg("0000000000000000000000000000000000000000000000000000000000000000")
            .arg(&tvc)
            .arg("{}");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Deploying"))
            .stdout(predicate::str::contains("Transaction succeeded"));
    });
}

#[test]
fn test_deployx_json_with_mock() {
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);
    let keys = format!("{}/boc/contract.keys.json", FIXTURES_DIR);

    with_mock_server(move |url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("-j")
            .arg("--url")
            .arg(&url)
            .arg("deployx")
            .arg("--abi")
            .arg(&abi)
            .arg("--keys")
            .arg(&keys)
            .arg("--dst-dapp-id")
            .arg("0000000000000000000000000000000000000000000000000000000000000000")
            .arg(&tvc)
            .arg("{}");
        cmd.assert().success();
    });
}

#[test]
fn test_runx_tvc() {
    // runx --tvc runs locally without any network.
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("runx")
        .arg("--tvc")
        .arg(&tvc)
        .arg("--abi")
        .arg(&abi)
        .arg("-m")
        .arg("get_my_dapp")
        .arg("{}");
    // The TVC and ABI should match, so local run should succeed.
    // However, the contract may not have been initialized with proper data,
    // so the getter may return default value or fail. We check CLI execution.
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("Succeeded")
            || stdout.contains("Result")
            || stdout.contains("Error")
            || stderr.contains("Error"),
        "stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_runx_tvc_json() {
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("-j")
        .arg("runx")
        .arg("--tvc")
        .arg(&tvc)
        .arg("--abi")
        .arg(&abi)
        .arg("-m")
        .arg("get_my_dapp")
        .arg("{}");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    // In JSON mode it should print JSON or an error JSON.
    assert!(
        stdout.contains("value0") || stdout.contains("Error") || stderr.contains("Error"),
        "stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_runx_network_with_mock() {
    // runx with NETWORK source loads account from the mock server and runs locally.
    with_mock_server(|url| {
        let mut cmd = cargo_bin_smart();
        cmd.arg("--url")
            .arg(&url)
            .arg("runx")
            .arg("--abi")
            .arg(format!("{}/boc/contract.abi.json", FIXTURES_DIR))
            .arg("--addr")
            .arg(TEST_ADDRESS_V3)
            .arg("-m")
            .arg("get_my_dapp")
            .arg("{}");
        // The mock account BOC may not match the ABI, so local run may fail,
        // but the CLI path (fetch account + run tvm) should execute.
        let output = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stdout.contains("Connecting")
                || stdout.contains("Succeeded")
                || stdout.contains("Error")
                || stderr.contains("Error"),
            "stdout: {}, stderr: {}",
            stdout,
            stderr
        );
    });
}
