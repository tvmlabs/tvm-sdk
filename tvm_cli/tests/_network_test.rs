use assert_cmd::Command;
mod common;
use common::BIN_NAME;
use common::GIVER_V2_ABI;
use common::GIVER_V2_ADDR;
use common::GIVER_V2_KEY;
use common::NETWORK;

/// Ignored: this needs a running Node SE, and running it without one is
/// destructive. It resets the developer's config -- `config clear` and
/// `config --global clear`, with no `--config` of its own, so it writes the
/// real `tvm-cli.conf.json` in the current directory and the global one next
/// to the binary. It also asserts nothing that can fail any more: the string
/// it looks for cannot be reached, because the `call` dies on the strict
/// address parser first.
#[test]
#[ignore]
fn test_network() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("config").arg("clear");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("config").arg("endpoint").arg("reset");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("config").arg("--global").arg("clear");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("config").arg("--global").arg("endpoint").arg("reset");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("config").arg("--url").arg(&*NETWORK);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let res = cmd
        .arg("call")
        .arg("--abi")
        .arg(GIVER_V2_ABI)
        .arg(GIVER_V2_ADDR)
        .arg("--sign")
        .arg(GIVER_V2_KEY)
        .arg("sendTransaction")
        .arg(format!(r#"{{"dest":"{}","value":10000000,"bounce":false}}"#, GIVER_V2_ADDR))
        .assert();
    let res = res.get_output().stdout.clone();
    let res = String::from_utf8(res);
    if res.is_err() {
        return Err(string_error::into_err("Failed to decode output.".to_string()));
    }

    if res.unwrap().contains("Fetch first block failed: Can not send http request:") {
        return Err(string_error::into_err(
            "Node SE is not running. If it is CI run, just restart it.".to_string(),
        ));
    }
    Ok(())
}
