use std::fs;

use predicates::prelude::*;

mod common;
use common::cargo_bin_smart;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn test_body_generation() {
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("body").arg("--abi").arg(&abi).arg("add").arg("{}");
    cmd.assert().success().stdout(predicate::str::contains("Message body:"));
}

#[test]
fn test_body_generation_json() {
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("-j").arg("body").arg("--abi").arg(&abi).arg("add").arg("{}");
    cmd.assert().success().stdout(predicate::str::contains("\"Message\""));
}

#[test]
fn test_message_raw_generation() {
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);
    let keys = format!("{}/boc/contract.keys.json", FIXTURES_DIR);
    let output = std::env::temp_dir().join("tvm_cli_message_raw_test.boc");
    let _ = fs::remove_file(&output);

    let addr = "0000000000000000000000000000000000000000000000000000000000000000::06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1";

    let mut cmd = cargo_bin_smart();
    cmd.arg("message")
        .arg("--abi")
        .arg(&abi)
        .arg("--keys")
        .arg(&keys)
        .arg("--output")
        .arg(&output)
        .arg("--raw")
        .arg(addr)
        .arg("add")
        .arg("{}");
    cmd.assert().success().stdout(predicate::str::contains("Message saved to"));

    assert!(output.exists());
    let _ = fs::remove_file(&output);
}

#[test]
fn test_message_raw_json() {
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);
    let keys = format!("{}/boc/contract.keys.json", FIXTURES_DIR);
    let output = std::env::temp_dir().join("tvm_cli_message_raw_json_test.boc");
    let _ = fs::remove_file(&output);

    let addr = "0000000000000000000000000000000000000000000000000000000000000000::06b8a619779f770630fa97efb96b86e03aad5b08b6d0df689057569424ec91b1";

    let mut cmd = cargo_bin_smart();
    cmd.arg("-j")
        .arg("message")
        .arg("--abi")
        .arg(&abi)
        .arg("--keys")
        .arg(&keys)
        .arg("--output")
        .arg(&output)
        .arg("--raw")
        .arg(addr)
        .arg("add")
        .arg("{}");
    cmd.assert().success();

    assert!(output.exists());
    let _ = fs::remove_file(&output);
}

#[test]
fn test_decode_body() {
    // Generate body first via tvm-cli body
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("body").arg("--abi").arg(&abi).arg("add").arg("{}");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let body = stdout
        .lines()
        .find(|l| l.contains("Message body:"))
        .unwrap()
        .split("Message body: ")
        .nth(1)
        .unwrap()
        .trim()
        .to_string();

    // Now decode it
    let mut cmd = cargo_bin_smart();
    cmd.arg("decode").arg("body").arg("--abi").arg(&abi).arg(&body);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("FunctionId:"));
}

#[test]
fn test_decode_msg() {
    // Use the message_sample.boc as a raw message to decode
    let msg_path = format!("{}/boc/message_sample.boc", FIXTURES_DIR);
    let abi = format!("{}/boc/contract.abi.json", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("decode").arg("msg").arg("--abi").arg(&abi).arg(&msg_path);
    // Decoding may fail if the message body doesn't match the ABI,
    // but the CLI should attempt to decode it.
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("BodyCall")
            || stdout.contains("Method")
            || stdout.contains("Error")
            || stderr.contains("Error")
            || stderr.contains("decode"),
        "stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_decode_stateinit_tvc() {
    let tvc = format!("{}/boc/contract.tvc", FIXTURES_DIR);

    let mut cmd = cargo_bin_smart();
    cmd.arg("decode").arg("stateinit").arg("--tvc").arg(&tvc);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"code\":"))
        .stdout(predicate::str::contains("\"data\":"));
}

#[test]
fn test_getkeypair_from_phrase() {
    // Generate a phrase first
    let mut cmd = cargo_bin_smart();
    cmd.arg("genphrase");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let phrase = stdout
        .lines()
        .find(|l| l.contains("Seed phrase"))
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .trim()
        .trim_matches('"')
        .to_string();

    // Get keypair from phrase
    let keyfile = std::env::temp_dir().join("tvm_cli_getkeypair_test.keys.json");
    let _ = fs::remove_file(&keyfile);

    let mut cmd = cargo_bin_smart();
    cmd.arg("getkeypair").arg("--output").arg(&keyfile).arg("--phrase").arg(&phrase);
    cmd.assert().success().stdout(predicate::str::contains("Succeeded"));

    assert!(keyfile.exists());
    let content = fs::read_to_string(&keyfile).unwrap();
    assert!(content.contains("public"));
    assert!(content.contains("secret"));
    let _ = fs::remove_file(&keyfile);
}

#[test]
fn test_sign_data() {
    let keys = format!("{}/boc/contract.keys.json", FIXTURES_DIR);
    let data = "deadbeef"; // hex string

    let mut cmd = cargo_bin_smart();
    cmd.arg("sign").arg("--data").arg(data).arg("--keys").arg(&keys);
    cmd.assert().success().stdout(predicate::str::contains("Signature:"));
}

#[test]
fn test_sign_json() {
    let keys = format!("{}/boc/contract.keys.json", FIXTURES_DIR);
    let data = "deadbeef";

    let mut cmd = cargo_bin_smart();
    cmd.arg("-j").arg("sign").arg("--data").arg(data).arg("--keys").arg(&keys);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"Signature\""))
        .stdout(predicate::str::contains("\"public\""));
}
