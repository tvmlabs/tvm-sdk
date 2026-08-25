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

#[test]
fn config_keys_does_not_echo_the_phrase() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = testdir!().join("masking.config");
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("--config").arg(&config_path).arg("config").arg("--keys").arg(PHRASE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains(r#""keys_path": "<seed phrase>""#));
    Ok(())
}

#[test]
fn config_alias_add_does_not_echo_the_phrase() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = testdir!().join("masking_alias.config");
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("--config")
        .arg(&config_path)
        .arg("config")
        .arg("alias")
        .arg("add")
        .arg("msig")
        .arg("--addr")
        .arg(ADDRESS)
        .arg("--abi")
        .arg(ABI)
        .arg("--keys")
        .arg(PHRASE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains(r#""key_path": "<seed phrase>""#));
    Ok(())
}
