mod account;
mod boc;
mod decode;
mod execute;
mod helper;
mod message;
mod result;
mod state;

use std::path::PathBuf;

use clap::ArgAction;
use clap::Parser;
use serde_json::Value;
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
#[derive(Parser, Debug)]
#[command(long_version = &**LONG_VERSION, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Run contract localy with specified parameters
    Run(RunArgs),
    /// Encodes given parameters in JSON into a BOC
    BocEncode(BocEncodeArgs),
    /// Decodes BOC into JSON as a set of provided parameters
    BocDecode(BocDecodeArgs),
    /// Read BOC string from stdin and print its hash
    BocHash,
    /// Encodes initial contract state from code, data, libraries ans special
    /// options
    StateEncode(StateEncodeArgs),

    /// Decodes initial contract state into code, data, libraries ans special
    /// options
    StateDecode(StateDecodeArgs),

    /// Creates account state BOC
    AccountEncode(AccountEncodeArgs),
}

// Read BOC string fron stdin and encode it as a set of provided parameters in
// JSON BocDecode(BocDecodeArgs),

#[derive(Parser, Debug, Default)]
struct BocEncodeArgs {
    /// Provided parameters specified as a JSON string or file path
    #[arg(short, long)]
    data: String,

    /// JSON encoded ABI params or file path
    #[arg(short, long)]
    params: PathBuf,
}

#[derive(Parser, Debug, Default)]
struct BocDecodeArgs {
    /// Contract code BOC encoded as base64 or file path
    #[arg(short, long)]
    boc: String,

    /// JSON encoded ABI params or file path
    #[arg(short, long)]
    params: PathBuf,
}

#[derive(Parser, Debug, Default)]
struct StateEncodeArgs {
    /// Contract code BOC encoded as base64 or file path
    #[arg(short, long)]
    code: Option<String>,

    /// Contract data BOC encoded as base64 or file path
    #[arg(short, long)]
    data: Option<String>,
}

#[derive(Parser, Debug, Default)]
struct StateDecodeArgs {
    /// Contract state init encoded as base64 or file path
    #[arg(short, long)]
    state_init: String,
}

#[derive(Parser, Debug, Default)]
struct AccountEncodeArgs {
    /// Contract state init encoded as base64 or file path
    #[arg(short, long)]
    state_init: String,

    /// Initial balance.
    #[arg(short, long)]
    balance: Option<u64>,

    /// Initial value for the `last_trans_lt`.
    #[arg(long)]
    last_trans_lt: Option<u64>,

    /// Initial value for the `last_paid`.
    #[arg(long)]
    last_paid: Option<u32>,
}

#[derive(Parser, Debug, Default)]
struct RunArgs {
    /// TVC file with contract state init
    #[arg(short, long, required(true))]
    input_file: PathBuf,

    /// Path to the contract ABI file
    #[arg(short, long)]
    abi_file: Option<PathBuf>,

    /// ABI header
    #[arg(short('r'), long, value_parser = parse_json_object)]
    abi_header: Option<Value>,

    /// Contract function name
    #[arg(short('m'), long)]
    function_name: Option<String>,

    /// Call parameters. Must be specified as a json string
    #[arg(short('p'), long, value_parser = parse_json_object)]
    call_parameters: Option<Value>,

    /// Contract address, that will be used for execution
    #[arg(long, allow_hyphen_values(true))]
    address: Option<String>,

    /// Path to the key pair file to sign the external message
    #[arg(short, long)]
    sign: Option<PathBuf>,

    /// Emulate inbound internal message
    #[clap(long, action=ArgAction::SetTrue, default_value = "false")]
    internal: bool,

    /// Emulate inbound cross dapp message
    #[clap(long, action=ArgAction::SetTrue, default_value = "false")]
    cross_dapp: bool,

    /// Internal message balance
    #[arg(long, requires("internal"))]
    message_value: Option<u128>,

    /// Internal message extra currency collection,
    #[arg(long, requires("internal"))]
    message_ecc: Option<String>,

    /// Internal message source address
    #[clap(long, requires("internal"))]
    message_source: Option<String>,

    /// Internal message source dapp id
    #[clap(long, requires("cross_dapp"))]
    message_source_dapp_id: Option<String>,

    /// Internal message dest dapp id
    #[clap(long, requires("cross_dapp"))]
    message_dest_dapp_id: Option<String>,
    
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

fn parse_json_object(s: &str) -> Result<Value, String> {
    let s = s.trim_matches('"').trim_matches('\'');
    if s.is_empty() {
        Ok(Value::Object(serde_json::Map::new()))
    } else if s.starts_with('{') && s.ends_with('}') {
        Ok(serde_json::from_str::<Value>(s)
            .map_err(|e| format!("Failed to parse json arg: {e}"))?)
    } else {
        Err(format!("Invalid json object: {s}"))
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    let output = match &cli.command {
        Commands::Run(args) => {
            if let Some(new_code) = args.replace_code.clone() {
                replace_code(&args.input_file, new_code).map(|_| "".to_string())
            } else {
                let mut res = ExecutionResult::new(args.json);
                execute(args, &mut res).map(|_| res.output())
            }
        }
        Commands::BocEncode(args) => run_command(|| boc::encode(args)),
        Commands::BocDecode(args) => run_command(|| boc::decode(args)),
        Commands::BocHash => run_command(boc::hash),
        Commands::StateEncode(args) => run_command(|| state::encode(args)),
        Commands::StateDecode(args) => run_command(|| state::decode(args)),
        Commands::AccountEncode(args) => run_command(|| account::encode(args)),
    };

    match output {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn replace_code(input_file: &PathBuf, code: String) -> anyhow::Result<()> {
    let mut contract_state_init = StateInit::construct_from_file(input_file).map_err(|e| {
        anyhow::format_err!("Failed to load state init from input file {:?}: {e}", input_file)
    })?;
    let bytes = base64_decode(&code)
        .map_err(|e| anyhow::format_err!("Failed to decode code as base64: {e}"))?;
    let code_cell = read_single_root_boc(bytes).map_err(|e| {
        anyhow::format_err!("Failed to construct code cell from base64 decoded code cell: {e}",)
    })?;
    contract_state_init.set_code(code_cell);
    contract_state_init
        .write_to_file(input_file)
        .map_err(|e| anyhow::format_err!("Failed to save state init after execution: {e}"))?;
    Ok(())
}

fn run_command<F, T>(f: F) -> anyhow::Result<String>
where
    F: FnOnce() -> anyhow::Result<T>,
    T: serde::Serialize,
{
    f().map(|result| serde_json::to_string(&result).expect("Failed to serialize result"))
}

pub(crate) fn read_file_as_base64(file_path: &str) -> anyhow::Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut buffer)?;
    Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &buffer))
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

    fn default_args(input_file: PathBuf, func: &str) -> RunArgs {
        RunArgs {
            input_file,
            abi_file: Some(PathBuf::from("tests/contract/contract.abi.json")),
            abi_header: None,
            function_name: Some(func.to_string()),
            call_parameters: None,
            address: None,
            sign: None,
            internal: false,
            cross_dapp: false,
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
