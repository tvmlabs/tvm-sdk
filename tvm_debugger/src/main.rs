mod decode;
mod execute;
mod helper;
mod message;
mod result;

use std::path::PathBuf;

use clap::ArgAction;
use clap::Parser;
use tvm_block::Deserializable;
use tvm_block::Serializable;
use tvm_block::StateInit;
use tvm_types::base64_decode;
use tvm_types::read_single_root_boc;

use crate::execute::execute;
use crate::result::ExecutionResult;

lazy_static::lazy_static!(
    static ref LONG_VERSION: String = format!("{}\nBUILD_GIT_BRANCH={}\nBUILD_GIT_COMMIT={}\nBUILD_GIT_DATE={}\nBUILD_TIME={}",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_GIT_BRANCH"),
        env!("BUILD_GIT_COMMIT"),
        env!("BUILD_GIT_DATE"),
        env!("BUILD_TIME"),
    );
);

/// Helper tool, that allows you to run Acki-Nacki virtual machine, get VM
/// trace, output messages and update contract state offchain.
#[derive(Parser, Debug, Default)]
#[command(long_version = &**LONG_VERSION, about, long_about = None)]
struct Args {
    /// TVC file with contract state init
    #[arg(short, long, required(true))]
    input_file: PathBuf,

    /// Path to the contract ABI file
    #[arg(short, long)]
    abi_file: Option<PathBuf>,

    /// ABI header
    #[arg(short('r'), long)]
    abi_header: Option<serde_json::Value>,

    /// Contract function name
    #[arg(short('m'), long)]
    function_name: Option<String>,

    /// Call parameters. Must be specified as a json string
    #[arg(short('p'), long)]
    call_parameters: Option<serde_json::Value>,

    /// Contract address, that will be used for execution
    #[arg(long, allow_hyphen_values(true))]
    address: Option<String>,

    /// Path to the key pair file to sign the external message
    #[arg(short, long)]
    sign: Option<PathBuf>,

    /// Emulate inbound internal message
    #[clap(long, action=ArgAction::SetTrue, default_value = "false")]
    internal: bool,

    /// Internal message balance
    #[arg(long, requires("internal"))]
    message_value: Option<u128>,

    /// Internal message extra currency collection,
    #[arg(long, requires("internal"))]
    message_ecc: Option<String>,

    /// Internal message source address
    #[clap(long, requires("internal"))]
    message_source: Option<String>,

    /// Decode out messages
    #[clap(long, action=ArgAction::SetTrue, default_value = "false")]
    decode_out_messages: bool,

    /// Prints output in json format
    #[arg(short, long, action=ArgAction::SetTrue, default_value = "false", conflicts_with = "trace")]
    json: bool,

    /// Trace VM execution
    #[arg(long, action=ArgAction::SetTrue, default_value = "false")]
    trace: bool,

    /// Update code in tvc without executing anything
    #[arg(long)]
    replace_code: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    if let Some(new_code) = args.replace_code {
        replace_code(args.input_file, new_code)?;
        return Ok(());
    }
    let mut res: ExecutionResult = ExecutionResult::new(args.json);
    execute(&args, &mut res)?;
    println!("{}", res.output());
    Ok(())
}

fn replace_code(input_file: PathBuf, code: String) -> anyhow::Result<()> {
    let mut contract_state_init = StateInit::construct_from_file(&input_file).map_err(|e| {
        anyhow::format_err!("Failed to load state init from input file {:?}: {e}", input_file)
    })?;
    let bytes = base64_decode(&code)
        .map_err(|e| anyhow::format_err!("Failed to decode code as base64: {e}"))?;
    let code_cell = read_single_root_boc(bytes).map_err(|e| {
        anyhow::format_err!("Failed to construct code cell from base64 decoded code cell: {e}",)
    })?;
    contract_state_init.set_code(code_cell);
    contract_state_init
        .write_to_file(&input_file)
        .map_err(|e| anyhow::format_err!("Failed to save state init after execution: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use serde_json::json;

    use super::*;

    fn create_temp_contract_file() -> PathBuf {
        let temp_path = PathBuf::from("tests/temp_contract.tvc");
        fs::copy("tests/contract/contract.tvc", &temp_path).expect("Failed to copy contract file");
        temp_path
    }

    fn cleanup_temp_contract_file(temp_path: &PathBuf) {
        fs::remove_file(temp_path).expect("Failed to delete temporary contract file");
    }

    fn default_args(input_file: PathBuf, func: &str) -> Args {
        Args {
            input_file,
            abi_file: Some(PathBuf::from("tests/contract/contract.abi.json")),
            abi_header: None,
            function_name: Some(func.to_string()),
            call_parameters: None,
            address: None,
            sign: None,
            internal: false,
            message_value: None,
            message_ecc: None,
            message_source: None,
            decode_out_messages: false,
            json: true,
            trace: false,
            replace_code: None,
        }
    }

    #[test]
    fn test_valid_input() {
        let temp = create_temp_contract_file();
        let args = &default_args(temp.clone(), "counter");
        let mut res: ExecutionResult = ExecutionResult::new(args.json);
        let result = execute(args, &mut res);
        assert!(result.is_ok());
        let actual = res.to_json();
        let response = json!({
            "counter": "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        });
        assert_eq!(actual["exit_code"], 0i32);
        assert_eq!(actual["vm_success"], true);
        assert_eq!(actual["gas_used"], 4065i64);
        assert_eq!(actual["response"], response);
        cleanup_temp_contract_file(&temp);
    }
}
