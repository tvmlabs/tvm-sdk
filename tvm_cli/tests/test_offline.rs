use std::fs;

use predicates::prelude::*;

mod common;
use common::cargo_bin_smart;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn test_decode_account_boc() {
    let boc_path = format!("{}/boc/account_sample.boc", FIXTURES_DIR);
    let mut cmd = cargo_bin_smart();
    cmd.arg("decode").arg("account").arg("boc").arg(&boc_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("acc_type:"))
        .stdout(predicate::str::contains("balance:"))
        .stdout(predicate::str::contains("last_paid:"));
}

#[test]
fn test_decode_account_boc_json() {
    let boc_path = format!("{}/boc/account_sample.boc", FIXTURES_DIR);
    let mut cmd = cargo_bin_smart();
    cmd.arg("-j").arg("decode").arg("account").arg("boc").arg(&boc_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""acc_type""#))
        .stdout(predicate::str::contains(r#""balance""#));
}

#[test]
fn test_config_set_and_clear() {
    let temp_dir = std::env::temp_dir().join("tvm_cli_test_config");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let config_path = temp_dir.join("test.conf.json");

    // Set config with URL
    let mut cmd = cargo_bin_smart();
    cmd.arg("--config").arg(&config_path).arg("config").arg("--url").arg("http://localhost:12345");
    cmd.assert().success();

    // Verify config exists and contains URL
    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("http://localhost:12345"));

    // Clear config URL
    let mut cmd = cargo_bin_smart();
    cmd.arg("--config").arg(&config_path).arg("config").arg("clear").arg("--url");
    cmd.assert().success();

    // Verify URL is cleared (reverted to default)
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(!content.contains("http://localhost:12345"));
}

#[test]
fn test_version() {
    let mut cmd = cargo_bin_smart();
    cmd.arg("version");
    cmd.assert().success().stdout(predicate::str::contains("tvm-cli"));
}

#[test]
fn test_nodeid_from_pubkey() {
    // 64-character hex public key
    let mut cmd = cargo_bin_smart();
    cmd.arg("nodeid")
        .arg("--pubkey")
        .arg("4c7c546dce0f664b4d33c72dd9e30a7e1c8e89c9a9e1a7f4b7f5d3e2a1c0b9f8");
    cmd.assert().success().stdout(predicate::str::contains("babd")); // nodeid is a hex hash
}

#[test]
fn test_genphrase() {
    let mut cmd = cargo_bin_smart();
    cmd.arg("genphrase");
    cmd.assert().success().stdout(predicate::str::contains("Seed phrase"));
}

#[test]
fn test_genphrase_and_genpubkey() {
    // Generate phrase
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

    // Generate pubkey from phrase
    let mut cmd = cargo_bin_smart();
    cmd.arg("genpubkey").arg(&phrase);
    cmd.assert().success().stdout(predicate::str::contains("Public key"));
}
