use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use testdir::testdir;

const BIN_NAME: &str = "tvm-cli";

const TVC: &str = "tests/decode_fields.tvc";
const ABI: &str = "tests/test_abi_v2.1.abi.json";

/// The help of `<PARAMS>` promises a filename is accepted in place of the json
/// itself. The repo carries no deployable contract image, so deploy cannot get
/// past address calculation here -- it does not need to: `Input arguments:` is
/// printed as soon as the arguments are resolved, which is exactly the point at
/// which a file must already have been read. Success is therefore not asserted.
#[test]
fn deploy_reads_constructor_args_from_a_file() -> Result<(), Box<dyn std::error::Error>> {
    let params = testdir!().join("params.json");
    fs::write(&params, r#"{"value":"0"}"#)?;

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deploy").arg(TVC).arg(&params).arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains(r#"params: {"value":"0"}"#));
    Ok(())
}

#[test]
fn deployx_reads_constructor_args_from_a_file() -> Result<(), Box<dyn std::error::Error>> {
    let params = testdir!().join("params.json");
    fs::write(&params, r#"{"value":"0"}"#)?;

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deployx").arg("--abi").arg(ABI).arg(TVC).arg(&params);
    cmd.assert().stdout(predicate::str::contains(r#"params: {"value":"0"}"#));
    Ok(())
}

/// A path that cannot be read must be reported as such, and must name the path
/// it stands for. Left to the json parser it turns into `expected value at line
/// 1 column 1`, which names neither the file nor the reason.
#[test]
fn deploy_reports_a_params_file_it_cannot_read() -> Result<(), Box<dyn std::error::Error>> {
    let missing = testdir!().join("no-such-params.json");

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deploy").arg(TVC).arg(&missing).arg("--abi").arg(ABI);
    cmd.assert()
        .stdout(predicate::str::contains("failed to load params from file"))
        .stdout(predicate::str::contains(missing.to_str().unwrap()));
    Ok(())
}
