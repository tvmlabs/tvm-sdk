use assert_cmd::Command;
use predicates::prelude::*;
use testdir::testdir;

const BIN_NAME: &str = "tvm-cli";

/// The binary runs from the crate root, where `default_config_name()`
/// resolves to `tvm_cli/tvm-cli.conf.json` -- gitignored, machine-specific and
/// whatever the last local CLI run left behind. A `keys_path` in it is enough
/// to change what several of these tests observe, so point `--config` at a
/// path inside the test's own directory instead. The file is never created;
/// the CLI falls back to its defaults when it does not exist.
fn tvm_cli() -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("--config").arg(testdir!().join("tvm-cli.conf.json"));
    Ok(cmd)
}

/// `deploy_message` is looked up by name while dispatching, so every
/// subcommand listed after it aborts if that name is not a registered
/// subcommand id. clap only asserts this in debug builds, which is why it
/// stays invisible in a release binary.
#[test]
fn subcommands_listed_after_deploy_message_are_dispatchable()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
    cmd.arg("genphrase").assert().success().stdout(predicate::str::contains("Seed phrase"));
    Ok(())
}

#[test]
fn deploy_message_is_reachable_by_its_own_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
    cmd.arg("deploy").arg(TVC).arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

#[test]
fn fee_deploy_does_not_look_up_arguments_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
    cmd.arg("fee").arg("deploy").arg(TVC).arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

#[test]
fn deploy_message_does_not_look_up_arguments_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
    cmd.arg("call").arg(ADDR).arg("sayHello").arg("{}").arg("--abi").arg(ABI);
    cmd.assert().stdout(predicate::str::contains("Input arguments:"));
    Ok(())
}

#[test]
fn fee_call_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
    cmd.arg("account").arg("--help").assert().success().stdout(predicate::str::contains("--boc"));
    Ok(())
}

/// The conflict must survive where it means something: `--boc` and `--tvc`
/// both exist on `run`, `runx` and `debug run`, and passing both must still
/// be refused.
#[test]
fn run_still_refuses_boc_together_with_tvc() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
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
        let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
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

/// `debug call` was built as a renamed clone of `debug run`, which leaves it
/// registered under the id `run`: `subcommand_matches("call")` then names an
/// id clap does not know and aborts every `debug` subcommand dispatched at or
/// after that lookup. `debug run` additionally reaches this handler without
/// `--keys` and `--update`, which only `debug call` defines.
#[test]
fn debug_run_traces_a_getter() -> Result<(), Box<dyn std::error::Error>> {
    let trace = testdir!().join("trace.log");
    let mut cmd = tvm_cli()?;
    cmd.arg("debug")
        .arg("run")
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
        .success()
        .stdout(predicate::str::contains("Execution finished."));
    Ok(())
}

/// The renamed clone also decided which handler ran: with no `call` id
/// registered, `debug call` fell through to the `run` arm and traced as a
/// getter -- unlimited gas, and a message the contract never accepted
/// reported as "Execution finished.". Release builds have no assertion to
/// stop this, so they took that path silently.
#[test]
fn debug_call_does_not_trace_as_a_getter() -> Result<(), Box<dyn std::error::Error>> {
    let trace = testdir!().join("trace.log");
    let mut cmd = tvm_cli()?;
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
    let mut cmd = tvm_cli()?;
    cmd.arg("debug")
        .arg("sequence-diagram")
        .arg(&missing)
        .assert()
        .failure()
        .stdout(predicate::str::contains("Failed to open file"));
    Ok(())
}

/// `debug message` registers `--boc` but no `--tvc`, and the shared `--boc`
/// definition declared the conflict, so clap asserted on a `conflicts_with`
/// target the command does not have -- aborting even `--help`.
#[test]
fn debug_message_does_not_conflict_with_an_argument_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
    cmd.arg("debug")
        .arg("message")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--boc"));
    Ok(())
}

/// `runx` and `callx` share the helper that resolves address, ABI and keys
/// from an alias, but only `callx` defines `--keys`.
#[test]
fn runx_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
    cmd.arg("runx")
        .arg("--addr")
        .arg(HELLO_TVC)
        .arg("--tvc")
        .arg("--abi")
        .arg(HELLO_ABI)
        .arg("-m")
        .arg("sayHello")
        .assert()
        .stdout(predicate::str::contains("Running get-method"));
    Ok(())
}

/// `multisig send` and `multisig deploy` share the argument struct, but the
/// key is `--sign` on one and `--keys` on the other, and the wallet address is
/// `--addr` on `send` alone.
#[test]
fn multisig_send_does_not_look_up_arguments_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
    cmd.arg("multisig")
        .arg("send")
        .arg("--addr")
        .arg(ADDR)
        .arg("--dest")
        .arg(ADDR)
        .arg("--value")
        .arg("1")
        .assert()
        .failure()
        .stdout(predicate::str::contains("sign key is not defined"));
    Ok(())
}

/// `test deploy` reached for a `--wc` it never registers.
#[test]
fn test_deploy_does_not_look_up_arguments_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let dir = testdir!();
    deploy_account_boc(&dir)?;
    Ok(())
}

/// `test ticktock` shares the tracing callback with the commands that define
/// `--abi`, and the callback resolved that argument unconditionally. The
/// command then fails on its own account -- it hands a ticktock to the
/// ordinary transaction executor -- so what is asserted here is only that it
/// reaches its own logic instead of aborting on an argument it never defines.
#[test]
fn test_ticktock_does_not_look_up_arguments_it_does_not_define()
-> Result<(), Box<dyn std::error::Error>> {
    let dir = testdir!();
    let account = deploy_account_boc(&dir)?;
    let mut cmd = tvm_cli()?;
    cmd.arg("test")
        .arg("ticktock")
        .arg(&account)
        .arg("--bc_config")
        .arg(BC_CONFIG)
        .arg("-o")
        .arg(dir.join("ticktock.log"))
        .assert()
        .stderr(predicate::str::contains("panicked").not());
    Ok(())
}

/// Deploys `Hello.tvc` locally into `dir` and returns the account BOC that
/// `test deploy` writes next to the copied TVC.
fn deploy_account_boc(
    dir: &std::path::Path,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let tvc = dir.join("Hello.tvc");
    std::fs::copy(HELLO_TVC, &tvc)?;
    let mut cmd = tvm_cli()?;
    cmd.arg("test")
        .arg("deploy")
        .arg(&tvc)
        .arg("--abi")
        .arg(HELLO_ABI)
        .arg("--address")
        .arg("0:1111111111111111111111111111111111111111111111111111111111111111")
        .arg("--initial_balance")
        .arg("1000000000")
        .arg("--bc_config")
        .arg(BC_CONFIG)
        .arg("-o")
        .arg(dir.join("deploy.log"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Account written to"));
    Ok(dir.join("Hello.boc"))
}

/// `body` defines no `--output`, but the handler read one to print it among
/// the input arguments.
#[test]
fn body_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = tvm_cli()?;
    cmd.arg("body")
        .arg("sayHello")
        .arg("{}")
        .arg("--abi")
        .arg(HELLO_ABI)
        .assert()
        .success()
        .stdout(predicate::str::contains("Message body:"));
    Ok(())
}

/// The other half of the `debug call` / `debug run` split: `debug call` must
/// still read the `--keys` it defines. Guarding the lookup by the wrong
/// direction would leave the command signing with whatever the config file
/// names, silently, so a key file that cannot be read has to be an error.
#[test]
fn debug_call_reads_the_keys_it_defines() -> Result<(), Box<dyn std::error::Error>> {
    let dir = testdir!();
    let account = deploy_account_boc(&dir)?;
    let mut cmd = tvm_cli()?;
    cmd.arg("debug")
        .arg("call")
        .arg("--addr")
        .arg(&account)
        .arg("--boc")
        .arg("--abi")
        .arg(HELLO_ABI)
        .arg("-m")
        .arg("sayHello")
        .arg("--keys")
        .arg(dir.join("no-such.keys"))
        .arg("--config")
        .arg(BC_CONFIG)
        .arg("-o")
        .arg(dir.join("call.log"))
        .assert()
        .failure()
        .stdout(predicate::str::contains("failed to read the keypair file"));
    Ok(())
}

/// And the `--update` it defines: skipping the lookup would leave the account
/// state unwritten while the command reported success.
#[test]
fn debug_call_writes_the_account_back_with_update() -> Result<(), Box<dyn std::error::Error>> {
    let dir = testdir!();
    let account = deploy_account_boc(&dir)?;
    let mut cmd = tvm_cli()?;
    cmd.arg("debug")
        .arg("call")
        .arg("--addr")
        .arg(&account)
        .arg("--boc")
        .arg("--abi")
        .arg(HELLO_ABI)
        .arg("-m")
        .arg("sayHello")
        .arg("--update")
        .arg("--config")
        .arg(BC_CONFIG)
        .arg("-o")
        .arg(dir.join("call.log"))
        .assert()
        .stdout(predicate::str::contains("successfully updated"));
    Ok(())
}

/// Every `depool` subcommand that sends through the wallet shares the multisig
/// argument struct with `multisig deploy`, which names its key `--keys`. The
/// depool commands define `--sign` alone.
#[test]
fn depool_does_not_look_up_arguments_it_does_not_define() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = tvm_cli()?;
    cmd.arg("depool")
        .arg("--addr")
        .arg(ADDR)
        .arg("withdraw")
        .arg("on")
        .arg("--wallet")
        .arg(ADDR)
        .assert()
        .failure()
        .stdout(predicate::str::contains("sign key is not defined"));
    Ok(())
}
