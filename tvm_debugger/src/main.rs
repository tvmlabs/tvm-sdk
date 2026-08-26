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

#[allow(clippy::large_enum_variant)]
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

    /// Execution block seq no
    #[arg(long)]
    block_seq_no: Option<u32>,
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
            message_value: None,
            message_ecc: None,
            message_source: None,
            decode_out_messages: false,
            json: true,
            trace: false,
            replace_code: None,
            block_seq_no: None,
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

    const EMITTER_ABI: &str = "tests/contract/emitter.abi.json";
    /// The same contract, described by an abi whose event is named after the
    /// function under test and pins the id the contract actually emits.
    const EMITTER_COLLISION_ABI: &str = "tests/contract/emitter-name-collision.abi.json";
    const EMITTER_ADDRESS: &str =
        "0:1010101010101010101010101010101010101010101010101010101010101010";
    const EMITTER_SENDER: &str =
        "0:2020202020202020202020202020202020202020202020202020202020202020";

    fn emitter_args(input_file: PathBuf, func: &str) -> RunArgs {
        RunArgs {
            input_file,
            abi_file: Some(PathBuf::from(EMITTER_ABI)),
            function_name: Some(func.to_string()),
            address: Some(EMITTER_ADDRESS.to_string()),
            json: true,
            ..RunArgs::default()
        }
    }

    fn emitter_internal_args(input_file: PathBuf, func: &str, params: Value) -> RunArgs {
        RunArgs {
            internal: true,
            message_source: Some(EMITTER_SENDER.to_string()),
            call_parameters: Some(params),
            ..emitter_args(input_file, func)
        }
    }

    fn bump_params(value: u128) -> Value {
        json!({ "value": value.to_string() })
    }

    /// An ABI header carrying an explicit logical time, `offset_ms` ahead of
    /// now.
    ///
    /// The contract rejects an external message whose time does not exceed the
    /// one it already stored, so two external calls that land in the same
    /// millisecond would make the second fail with exit code 52. Ordering the
    /// two by hand keeps that out of the tests.
    fn emitter_abi_header(offset_ms: u64) -> Value {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to read the clock")
            .as_millis() as u64;
        json!({ "time": now_ms + offset_ms, "expire": now_ms / 1000 + 600 })
    }

    /// Copies the emitter fixture into an isolated directory and runs its
    /// constructor, so that the resulting state accepts calls to its methods.
    fn deployed_emitter() -> PathBuf {
        let state = testdir::testdir!().join("emitter.tvc");
        fs::copy("tests/contract/emitter.tvc", &state).expect("Failed to copy the emitter state");
        let mut args = emitter_args(state.clone(), "constructor");
        args.abi_header = Some(emitter_abi_header(0));
        let mut res = ExecutionResult::new(args.json);
        execute(&args, &mut res).expect("Failed to run the emitter constructor");
        assert_eq!(res.to_json()["exit_code"], 0i32);
        state
    }

    /// Reads `counter` back out of a stored contract state.
    fn emitter_counter(state_file: &PathBuf) -> String {
        let state_init = StateInit::construct_from_file(state_file)
            .expect("Failed to load the emitter state init");
        let data = tvm_types::SliceData::load_cell(state_init.data.clone().unwrap())
            .expect("Failed to load the emitter data cell");
        let abi = fs::read_to_string(EMITTER_ABI).expect("Failed to read the emitter abi");
        let fields: Value = serde_json::from_str(
            &tvm_abi::decode_storage_fields(&abi, data, true)
                .expect("Failed to decode the emitter storage fields"),
        )
        .expect("Failed to parse the decoded storage fields");
        fields["counter"].as_str().expect("counter is not a string").to_string()
    }

    /// An internal call whose method emits an event must still write the
    /// resulting state back to the input file.
    #[test]
    fn test_internal_call_emitting_an_event_updates_the_state() {
        let state = deployed_emitter();
        assert_eq!(emitter_counter(&state), "0");

        let args = emitter_internal_args(state.clone(), "bump", bump_params(7));
        let mut res = ExecutionResult::new(args.json);
        execute(&args, &mut res).expect("An emitted event must not fail the run");

        assert_eq!(res.to_json()["exit_code"], 0i32);
        assert_eq!(res.to_json()["vm_success"], true);
        assert_eq!(emitter_counter(&state), "7");
    }

    /// The event leaves the contract as an external outbound message, so it
    /// must be reported among the out messages rather than taken for the
    /// response.
    #[test]
    fn test_internal_call_reports_the_emitted_event() {
        let state = deployed_emitter();
        let args = emitter_internal_args(state, "bump", bump_params(7));
        let mut res = ExecutionResult::new(false);
        execute(&args, &mut res).expect("An emitted event must not fail the run");

        let messages = res.to_json()["messages"].as_array().cloned().unwrap_or_default();
        assert_eq!(messages.len(), 1, "the emitted event is missing from the out messages");
        assert_eq!(messages[0]["type"], "external");

        // `ExecutionResult` starts out with the response unset, as the string
        // `{}` rather than an empty object.
        // The source is filled in the way it is for internal sends, so the
        // reported BOC carries the address the event really left from.
        let boc = messages[0]["boc"].as_str().expect("the event carries no boc");
        let event = tvm_block::Message::construct_from_bytes(
            &tvm_types::base64_decode(boc).expect("the boc is not base64"),
        )
        .expect("the boc is not a message");
        assert_eq!(
            event.src_ref().map(ToString::to_string),
            Some(EMITTER_ADDRESS.to_string()),
            "the event was reported with an unfilled source"
        );

        assert_eq!(res.to_json()["response"], json!("{}"), "an event is not the function response");
        assert!(
            res.output().contains("Bumped"),
            "the event is not named in the output: {}",
            res.output()
        );
    }

    /// An external call gets its response in an external outbound message too,
    /// so an event emitted alongside it must not displace the decoded
    /// response.
    #[test]
    fn test_external_call_decodes_the_response_next_to_an_event() {
        let state = deployed_emitter();
        let bump = emitter_internal_args(state.clone(), "bump", bump_params(7));
        let mut res = ExecutionResult::new(bump.json);
        execute(&bump, &mut res).expect("An emitted event must not fail the run");

        let mut read = emitter_args(state, "readAndEmit");
        // Strictly later than the constructor's, whatever the two runs cost.
        read.abi_header = Some(emitter_abi_header(60_000));
        let mut res = ExecutionResult::new(read.json);
        execute(&read, &mut res).expect("An emitted event must not fail the run");

        assert_eq!(res.to_json()["exit_code"], 0i32);
        assert_eq!(res.to_json()["response"], json!({ "total": "7" }));
        assert_eq!(
            res.to_json()["messages"].as_array().map(|m| m.len()),
            Some(2),
            "the event and the response are both out messages"
        );
    }

    /// An event and an internal send in one run: the event is reported next to
    /// the internal message, and the logical time that orders internal messages
    /// is not spent on the event.
    #[test]
    fn test_event_does_not_consume_the_internal_logical_time() {
        let state = deployed_emitter();
        let params = json!({ "to": EMITTER_SENDER, "value": "7" });
        let args = emitter_internal_args(state, "bumpAndForward", params);
        let mut res = ExecutionResult::new(args.json);
        execute(&args, &mut res).expect("An emitted event must not fail the run");

        let messages = res.to_json()["messages"].as_array().cloned().unwrap_or_default();
        let kinds: Vec<_> = messages.iter().map(|m| m["type"].as_str().unwrap()).collect();
        assert_eq!(kinds, vec!["external", "internal"], "the event is emitted before the send");

        let boc = messages[1]["boc"].as_str().expect("the internal message carries no boc");
        let sent = tvm_block::Message::construct_from_bytes(
            &tvm_types::base64_decode(boc).expect("the boc is not base64"),
        )
        .expect("the boc is not a message");
        // 1, not 2: the preceding event did not take a number from the counter.
        assert_eq!(sent.lt(), Some(1));
    }

    /// A method that emits nothing keeps behaving exactly as before.
    #[test]
    fn test_internal_call_without_an_event_updates_the_state() {
        let state = deployed_emitter();
        let args = emitter_internal_args(state.clone(), "bumpQuiet", bump_params(7));
        let mut res = ExecutionResult::new(args.json);
        execute(&args, &mut res).expect("Failed to run a method that emits nothing");

        assert_eq!(res.to_json()["exit_code"], 0i32);
        assert_eq!(res.to_json()["messages"], json!([]));
        assert_eq!(emitter_counter(&state), "7");
    }

    /// A non-zero exit code leaves the input file untouched, event or no event.
    #[test]
    fn test_failing_call_leaves_the_state_untouched() {
        let state = deployed_emitter();
        let before = fs::read(&state).expect("Failed to read the emitter state");

        let args = emitter_internal_args(state.clone(), "boom", bump_params(7));
        let mut res = ExecutionResult::new(args.json);
        execute(&args, &mut res).expect("A failing call is reported, not an error");

        assert_eq!(res.to_json()["exit_code"], 199i32);
        assert_eq!(res.to_json()["vm_success"], false);
        assert_eq!(fs::read(&state).expect("Failed to read the emitter state"), before);
        assert_eq!(emitter_counter(&state), "0");
    }

    /// The source address is passed to the contract as given, so a method that
    /// guards on its sender stays callable.
    #[test]
    fn test_internal_call_keeps_the_given_message_source() {
        let state = deployed_emitter();
        let params = json!({ "expected": EMITTER_SENDER, "value": "7" });
        let args = emitter_internal_args(state.clone(), "bumpFromSender", params);
        let mut res = ExecutionResult::new(args.json);
        execute(&args, &mut res).expect("A sender-guarded method must stay callable");

        assert_eq!(res.to_json()["exit_code"], 0i32);
        assert_eq!(emitter_counter(&state), "7");
    }

    /// Functions and events are separate ABI namespaces, so an event may carry
    /// the called function's name. The response is the message whose id matches
    /// the function's output id, and such an event is not it.
    #[test]
    fn test_event_sharing_the_function_name_is_not_the_response() {
        let state = deployed_emitter();
        let mut args = emitter_internal_args(state.clone(), "bump", bump_params(7));
        args.abi_file = Some(PathBuf::from(EMITTER_COLLISION_ABI));
        let mut res = ExecutionResult::new(false);
        execute(&args, &mut res).expect("An emitted event must not fail the run");

        // `bump` declares no outputs, so its response stays unset.
        assert_eq!(res.to_json()["response"], json!("{}"));
        assert!(
            res.output().contains(r#"Event(bump): {"total":"7","value":"7"}"#),
            "the event was not reported as an event: {}",
            res.output()
        );
        assert_eq!(emitter_counter(&state), "7");
    }

    /// The counterpart of the above: a source the method does not expect is
    /// refused, which is what makes the test above meaningful.
    #[test]
    fn test_internal_call_is_refused_for_another_message_source() {
        let state = deployed_emitter();
        let params = json!({ "expected": EMITTER_ADDRESS, "value": "7" });
        let args = emitter_internal_args(state.clone(), "bumpFromSender", params);
        let mut res = ExecutionResult::new(args.json);
        execute(&args, &mut res).expect("A refused call is reported, not an error");

        assert_eq!(res.to_json()["exit_code"], 200i32);
        assert_eq!(emitter_counter(&state), "0");
    }
}
