use assert_cmd::Command;
mod common;
use common::BIN_NAME;
use common::GIVER_V2_ABI;
use common::GIVER_V2_ADDR;
use common::GIVER_V2_KEY;
use common::NETWORK;

#[test]
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
        .arg(format!(r#"{{"dest":"{GIVER_V2_ADDR}","value":10000000,"bounce":false}}"#))
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
