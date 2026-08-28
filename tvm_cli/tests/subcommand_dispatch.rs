use assert_cmd::Command;
use predicates::prelude::*;
use testdir::testdir;

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

const ADDR: &str = "1111111111111111111111111111111111111111111111111111111111111111::2222222222222222222222222222222222222222222222222222222222222222";

/// The same hazard on the other shared handler: `call`, `message` and
/// `fee call` share `call_command`, but only `message` defines `--lifetime`,
/// `--timestamp`, `--output` and `--raw`. `call` and `fee call` reached
/// `Input arguments:` only in release builds; in debug they aborted on the
/// first of those lookups.
#[test]
fn call_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("call").arg(ADDR).arg("sayHello").arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

#[test]
fn fee_call_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("fee").arg("call").arg(ADDR).arg("sayHello").arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

/// `--boc` and `--tvc` are alternatives wherever both exist, but `account`
/// registers only `--boc`. Declaring the conflict on the shared argument made
/// `account` name a `TVC` it never registers, and clap asserts on a
/// `conflicts_with` target that does not exist — so every `account`
/// invocation, `--help` included, aborted in debug builds.
#[test]
fn account_does_not_conflict_with_an_argument_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("account").arg("--help").assert().success().stdout(predicate::str::contains("--boc"));
    Ok(())
}

/// The conflict must survive where it means something: `--boc` and `--tvc`
/// both exist on `run`, `runx` and `debug run`, and passing both must still
/// be refused.
#[test]
fn run_still_refuses_boc_together_with_tvc() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("run")
        .arg("--boc")
        .arg("--tvc")
        .arg("account.boc")
        .arg("sayHello")
        .arg("{}")
        .arg("--abi")
        .arg(ABI)
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
    Ok(())
}

#[test]
fn message_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("message").arg(ADDR).arg("sayHello").arg("{}").arg("--abi").arg(ABI);
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

/// `docs/MIGRATION-3.x.md`'s `deployx` parser-trap section promises this
/// exact error text (down to the absorbed flag naming the missing "file").
/// Assert against that string, not a clap usage error: as
/// `deploy_rejects_dst_dapp_id`'s doc comment explains, `deployx` has no
/// usage error here by construction — `allow_hyphen_values` +
/// `trailing_var_arg` make it absorb the unrecognized `--dst-dapp-id` as
/// the TVC positional instead. This is what the guide's promise actually
/// rests on, so a future reorder of `deployx_cmd`'s positionals that
/// changed which one absorbs the flag would break this test, which is the
/// point: it also recovers, for `deployx`, the rejection coverage
/// `deploy_rejects_dst_dapp_id` provides for `deploy`.
#[test]
fn deployx_absorbs_dst_dapp_id_as_tvc_path() -> Result<(), Box<dyn std::error::Error>> {
    let dapp = "a".repeat(64);
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("deployx")
        .arg("--abi")
        .arg(ABI)
        .arg("--dst-dapp-id")
        .arg(&dapp)
        .arg(TVC)
        .arg("{}")
        .assert()
        .failure()
        .stdout(predicate::str::contains(
            "failed to read smart contract file --dst-dapp-id: No such file or directory",
        ));
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

const HELLO_TVC: &str = "tests/Hello.tvc";
const HELLO_ABI: &str = "tests/Hello.abi.json";
// Pins the blockchain config to a file so the run stays offline: without it
// `get_blockchain_config` queries the network and only then falls back.
const BC_CONFIG: &str = "tests/config_contract.saved";

/// The renamed clone also decided which handler ran: with no `call` id
/// registered, `debug call` fell through to the `run` arm and traced as a
/// getter -- unlimited gas, and a message the contract never accepted
/// reported as "Execution finished.". Release builds have no assertion to
/// stop this, so they took that path silently.
#[test]
fn debug_call_does_not_trace_as_a_getter() -> Result<(), Box<dyn std::error::Error>> {
    let trace = testdir!().join("trace.log");
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("debug")
        .arg("call")
        .arg("--addr")
        .arg(HELLO_TVC)
        .arg("--tvc")
        .arg("--abi")
        .arg(HELLO_ABI)
        .arg("-m")
        .arg("sayHello")
        .arg("--config")
        .arg(BC_CONFIG)
        .arg("--output")
        .arg(&trace)
        .assert()
        .stdout(predicate::str::contains("Contract did not accept message"));
    Ok(())
}

/// `debug sequence-diagram` is the last subcommand the dispatcher looks up, so
/// it only runs once every lookup before it names a registered id.
#[test]
fn debug_subcommands_after_call_are_dispatchable() -> Result<(), Box<dyn std::error::Error>> {
    let missing = testdir!().join("no-such-addresses.txt");
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("debug")
        .arg("sequence-diagram")
        .arg(&missing)
        .assert()
        .failure()
        .stdout(predicate::str::contains("Failed to open file"));
    Ok(())
}
