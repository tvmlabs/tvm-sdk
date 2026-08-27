use std::env;

use assert_cmd::Command;
use lazy_static::*;
use predicates::prelude::*;
use serde_json::Map;
use serde_json::Value;

pub const BIN_NAME: &str = "tvm-cli";

/// Returns a command for the tvm-cli binary.
///
/// Release is preferred because clap 3's debug assertions reject some of the
/// legacy underscore subcommands.  The target directory is deliberately not
/// derived from the workspace: Cargo may be configured with a custom target
/// directory.
pub fn cargo_bin_smart() -> Command {
    if let Ok(path) = env::var("CLI_NAME") {
        if !path.is_empty() {
            return Command::new(path);
        }
    }

    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        let target_dir = std::path::PathBuf::from(target_dir);
        for profile in ["release", "debug"] {
            let path = target_dir.join(profile).join(BIN_NAME);
            if path.is_file() {
                return Command::new(path);
            }
        }
    }

    // assert_cmd asks Cargo for the package target directory and therefore
    // also works when CARGO_TARGET_DIR is configured outside this repository.
    Command::cargo_bin(BIN_NAME).expect("unable to locate tvm-cli; build it first")
}

pub const GIVER_ADDR: &str = "0:841288ed3b55d9cdafa806807f02a0ae0c169aa5edfe88a789a6482429756a94";
pub const GIVER_ABI: &str = "tests/samples/giver.abi.json";
pub const GIVER_V2_ADDR: &str =
    "0:ece57bcc6c530283becbbd8a3b24d3c5987cdddc3c8b7b33be6e4a6312490415";
pub const GIVER_V2_ABI: &str = "tests/samples/giver_v2.abi.json";
pub const GIVER_V2_KEY: &str = "tests/samples/giver_v2.key";
pub const GIVER_V3_ADDR: &str =
    "0:1111111111111111111111111111111111111111111111111111111111111111";
pub const GIVER_V3_ABI: &str = "tests/samples/giver_v3.abi.json";
pub const GIVER_V3_KEY: &str = "tests/samples/giver_v3.key";

lazy_static! {
    pub static ref NETWORK: String =
        env::var("TON_NETWORK_ADDRESS").unwrap_or("http://127.0.0.1/".to_string());
}

#[allow(dead_code)]
pub fn get_config() -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    let out = cmd.arg("config").arg("--list").output().expect("Failed to get config.");

    let mut out = String::from_utf8_lossy(&out.stdout).to_string();
    out.replace_range(..out.find('\n').unwrap_or(0), "");
    let parsed: Value = serde_json::from_str(&out)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    Ok(obj)
}

#[allow(dead_code)]
pub fn set_config(
    config: &[&str],
    argument: &[&str],
    config_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    if config_path.is_some() {
        cmd.arg("--config").arg(config_path.unwrap());
    }
    cmd.arg("config");
    for i in 0..config.len() {
        cmd.arg(config[i]).arg(argument[i]);
    }
    cmd.assert().success().stdout(predicate::str::contains("Succeeded"));
    Ok(())
}

#[allow(dead_code)]
pub fn giver(addr: &str) {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("call")
        .arg("--abi")
        .arg(GIVER_ABI)
        .arg(GIVER_ADDR)
        .arg("sendGrams")
        .arg(format!(r#"{{"dest":"{}","amount":1000000000}}"#, addr));
    cmd.assert().success();
}

#[allow(dead_code)]
pub fn giver_v2(addr: &str) {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("call")
        .arg("--abi")
        .arg(GIVER_V2_ABI)
        .arg(GIVER_V2_ADDR)
        .arg("--sign")
        .arg(GIVER_V2_KEY)
        .arg("sendTransaction")
        .arg(format!(r#"{{"dest":"{}","value":100000000000,"bounce":false}}"#, addr));
    cmd.assert().success();
}

#[allow(dead_code)]
pub fn giver_v3(addr: &str) {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("call")
        .arg("--abi")
        .arg(GIVER_V3_ABI)
        .arg(GIVER_V3_ADDR)
        .arg("--sign")
        .arg(GIVER_V3_KEY)
        .arg("sendTransaction")
        .arg(format!(r#"{{"dest":"{}","value":100000000000,"bounce":false}}"#, addr));
    cmd.assert().success();
}

#[allow(dead_code)]
pub fn grep_address(output: &[u8]) -> String {
    let mut addr = String::from_utf8_lossy(output).to_string();
    addr.replace_range(..addr.find("0:").unwrap_or(0), "");
    addr.replace_range(addr.find("testnet").unwrap_or(addr.len()) - 1.., "");
    addr
}

#[allow(dead_code)]
pub fn grep_message_id(output: &[u8]) -> String {
    let mut message_id = String::from_utf8_lossy(output).to_string();
    let index = message_id.find("MessageId: ").map(|i| i + "MessageId: ".len()).unwrap_or(0);
    message_id.replace_range(..index, "");
    if message_id.len() >= 64 {
        message_id.replace_range(64.., "");
    }
    message_id
}

#[allow(dead_code)]
pub fn generate_key_and_address(
    key_path: &str,
    tvc_path: &str,
    abi_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    let out = cmd
        .arg("genaddr")
        .arg("--genkey")
        .arg(key_path)
        .arg(tvc_path)
        .arg("--abi")
        .arg(abi_path)
        .output()
        .expect("Failed to generate address.");

    Ok(grep_address(&out.stdout))
}

#[allow(dead_code)]
pub fn generate_phrase_and_key(key_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    let out = cmd
        .arg("genphrase")
        .arg("--dump")
        .arg(key_path)
        .output()
        .expect("Failed to generate a seed phrase.");
    let mut seed = String::from_utf8_lossy(&out.stdout).to_string();
    seed.replace_range(..seed.find('"').unwrap_or(0) + 1, "");
    seed.replace_range(seed.find("Keypair").unwrap_or(seed.len()) - 2.., "");

    Ok(seed)
}

pub mod mock_server;

pub fn with_mock_server<F>(test: F)
where
    F: FnOnce(String) + Send + 'static,
{
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let server = runtime.block_on(async {
        let server = mock_server::MockGraphQLServer::new().await;
        server.setup_default_fixtures();
        let url = server.url();
        server.run().await;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        url
    });
    test(server);
    // Let the runtime keep running until the test finishes.
    // The server task will be dropped when runtime is dropped.
    drop(runtime);
}
