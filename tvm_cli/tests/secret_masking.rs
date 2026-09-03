use assert_cmd::Command;
use predicates::prelude::*;
use testdir::testdir;

const BIN_NAME: &str = "tvm-cli";

/// A throwaway phrase, already used as a test vector elsewhere in this repo.
const PHRASE: &str =
    "multiply extra monitor fog rocket defy attack right night jaguar hollow enlist";
/// The secret key the phrase above derives to.
const SECRET_KEY: &str = "c4415c03aa9d824e89ff4555cd12497aef1d5123f839803b0268e27ba6052354";

const TVC: &str = "tests/decode_fields.tvc";
const ABI: &str = "tests/test_abi_v2.1.abi.json";
/// Self-rooted `dapp_id::account_id`: the strict form the CLI requires of
/// external input. The alias test does not care which address it stores, so
/// it should not spell one the tool is meant to reject.
const ADDRESS: &str = "ece57bcc6c530283becbbd8a3b24d3c5987cdddc3c8b7b33be6e4a6312490415\
                       ::ece57bcc6c530283becbbd8a3b24d3c5987cdddc3c8b7b33be6e4a6312490415";

/// The repo carries no valid contract image, so genaddr cannot reach the
/// address calculation here. It does not need to: the `Input arguments:` block
/// is printed before any validation, which is exactly the point at which a
/// secret must already be safe. Success is therefore not asserted -- the
/// property under test is that the phrase never reaches stdout, on the failure
/// path just as much as on the happy one.
#[test]
fn genaddr_setkey_does_not_echo_the_phrase() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("genaddr").arg(TVC).arg("--abi").arg(ABI).arg("--setkey").arg(PHRASE);
    cmd.assert()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains("keys: <seed phrase>"));
    Ok(())
}

#[test]
fn getkeypair_does_not_echo_the_phrase() -> Result<(), Box<dyn std::error::Error>> {
    let key_file = testdir!().join("key.json");
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("getkeypair").arg("-o").arg(&key_file).arg("-p").arg(PHRASE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains("phrase: <seed phrase>"));
    Ok(())
}

#[test]
fn getkeypair_does_not_echo_a_raw_secret_key() -> Result<(), Box<dyn std::error::Error>> {
    let key_file = testdir!().join("key.json");
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("getkeypair").arg("-o").arg(&key_file).arg("-p").arg(SECRET_KEY);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(SECRET_KEY).not())
        .stdout(predicate::str::contains("phrase: <secret key>"));
    Ok(())
}

/// The config file is written by hand because `config --keys` no longer
/// accepts a phrase -- see `tests/config_key_source.rs`. A file written before
/// that refusal existed still holds one, and printing it is exactly when the
/// phrase must not reappear.
#[test]
fn a_phrase_stored_in_the_config_is_not_echoed() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = testdir!().join("masking.config");
    std::fs::write(&config_path, format!(r#"{{"retries": 1, "keys_path": "{PHRASE}"}}"#))?;
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("--config").arg(&config_path).arg("config").arg("--list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains(r#""keys_path": "<seed phrase>""#));
    Ok(())
}

#[test]
fn a_phrase_stored_in_an_alias_is_not_echoed() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = testdir!().join("masking_alias.config");
    std::fs::write(
        &config_path,
        format!(
            r#"{{"config": {{"retries": 1}}, "aliases": {{"msig": {{"address": "{ADDRESS}", "abi_path": "{ABI}", "key_path": "{PHRASE}"}}}}}}"#
        ),
    )?;
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("--config").arg(&config_path).arg("config").arg("alias").arg("print");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains(r#""key_path": "<seed phrase>""#));
    Ok(())
}

/// `config --keys` accepts a path with a space in it, and a stored value its
/// owner cannot read back is worse than a masked diagnostic. A path separator
/// settles what the value is: no wordlist holds a word with one.
#[test]
fn a_path_with_a_space_is_not_masked_as_a_phrase() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = testdir!().join("spaced_path.config");
    std::fs::write(&config_path, r#"{"retries": 1}"#)?;
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("--config")
        .arg(&config_path)
        .arg("config")
        .arg("--keys")
        .arg("./My Wallets/msig.keys.json");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""keys_path": "./My Wallets/msig.keys.json""#));
    Ok(())
}

/// A phrase with one mistyped word is still almost the whole wallet: the BIP39
/// checksum narrows the missing word to a trivial search. It must not appear in
/// the error either.
#[test]
fn an_invalid_phrase_is_not_echoed_by_the_error() -> Result<(), Box<dyn std::error::Error>> {
    let typo = format!("{PHRASE}X");
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("genpubkey").arg(&typo);
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(&typo).not())
        .stdout(predicate::str::contains("extra monitor fog").not())
        .stdout(predicate::str::contains("Invalid bip39 phrase"));
    Ok(())
}
