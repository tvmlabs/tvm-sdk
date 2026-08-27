use assert_cmd::Command;
use predicates::prelude::*;

const BIN_NAME: &str = "tvm-cli";

/// `deploy_message` is looked up by name while dispatching, so every
/// subcommand listed after it aborts if that name is not a registered
/// subcommand id. clap only asserts this in debug builds, which is why it
/// stays invisible in a release binary.
#[test]
fn subcommands_listed_after_deploy_message_are_dispatchable()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("genphrase").assert().success().stdout(predicate::str::contains("Seed phrase"));
    Ok(())
}

#[test]
fn deploy_message_is_reachable_by_its_own_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deploy_message")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("deploy_message"));
    Ok(())
}

const TVC: &str = "tests/decode_fields.tvc";
const ABI: &str = "tests/test_abi_v2.1.abi.json";

/// `deploy`, `deploy_message` and `fee deploy` share one handler, but only
/// `deploy_message` defines `--output` and `--raw`. Asking clap for an argument
/// the running command does not define aborts in debug builds, so neither
/// lookup may happen unconditionally. Reaching `Input arguments:` is what shows
/// the arguments were resolved without such a lookup.
#[test]
fn deploy_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deploy").arg(TVC).arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

#[test]
fn fee_deploy_does_not_look_up_arguments_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("fee").arg("deploy").arg(TVC).arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

#[test]
fn deploy_message_does_not_look_up_arguments_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deploy_message").arg(TVC).arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

/// `deployx` has no counterpart to this test: it sets `allow_hyphen_values`
/// and `trailing_var_arg` for its alternative-syntax constructor params, so
/// an unknown flag like `--dst-dapp-id` is absorbed as a positional (the TVC
/// path) rather than rejected by clap — there is no usage error to assert
/// on. Registration is covered instead by
/// `deploy_help_no_longer_offers_dst_dapp_id`, which checks both `deploy` and
/// `deployx`.
#[test]
fn deploy_rejects_dst_dapp_id() -> Result<(), Box<dyn std::error::Error>> {
    let dapp = "a".repeat(64);
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deploy")
        .arg(TVC)
        .arg("{}")
        .arg("--abi")
        .arg(ABI)
        .arg("--dst-dapp-id")
        .arg(&dapp)
        .assert()
        .failure()
        .stderr(predicate::str::contains("dst-dapp-id"));
    Ok(())
}

#[test]
fn deploy_help_no_longer_offers_dst_dapp_id() -> Result<(), Box<dyn std::error::Error>> {
    for subcommand in ["deploy", "deployx"] {
        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.arg(subcommand)
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("--dst-dapp-id").not());
    }
    Ok(())
}

#[test]
fn send_help_still_offers_dst_dapp_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("send")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--dst-dapp-id"));
    Ok(())
}
