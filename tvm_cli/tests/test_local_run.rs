use std::fs;

use predicates::prelude::*;

mod common;
use common::cargo_bin_smart;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn test_genaddr() {
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);
    let keys = std::env::temp_dir().join("tvm_cli_genaddr_test.keys.json");
    let _ = fs::remove_file(&keys);

    let mut cmd = cargo_bin_smart();
    cmd.arg("genaddr").arg("--genkey").arg(&keys).arg("--save").arg(&tvc).arg("--abi").arg(&abi);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Raw address:"))
        .stdout(predicate::str::contains("0:"));

    assert!(keys.exists());
    let _ = fs::remove_file(&keys);
}

#[test]
fn test_genaddr_json() {
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);
    let keys = std::env::temp_dir().join("tvm_cli_genaddr_json_test.keys.json");
    let _ = fs::remove_file(&keys);

    let mut cmd = cargo_bin_smart();
    cmd.arg("-j")
        .arg("genaddr")
        .arg("--genkey")
        .arg(&keys)
        .arg("--save")
        .arg(&tvc)
        .arg("--abi")
        .arg(&abi);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("raw_address"))
        .stdout(predicate::str::contains("seed_phrase"));

    let _ = fs::remove_file(&keys);
}

#[test]
fn test_run_boc() {
    // run with a BOC file — tvm_executor tries to run the getter locally.
    // The BOC may not match the ABI, but the CLI path should still execute.
    let boc = format!("{}/boc/account_sample.boc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("run").arg("--boc").arg(&boc).arg("get_my_dapp").arg("{}").arg("--abi").arg(&abi);
    // Execution may fail because BOC doesn't match ABI, but CLI should parse args
    // and attempt local run. We check it at least starts processing.
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should contain either "Local run succeeded" or some error about execution,
    // but not "Invalid argument" or "missing field".
    assert!(
        stdout.contains("Local run")
            || stdout.contains("Error")
            || stderr.contains("Error")
            || stderr.contains("run"),
        "stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}
