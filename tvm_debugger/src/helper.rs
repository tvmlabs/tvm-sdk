use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;

use tvm_block::CurrencyCollection;
use tvm_block::MsgAddressInt;
use tvm_block::StateInit;
use tvm_block::UnixTime32;
use tvm_client::crypto::KeyPair;
use tvm_types::Cell;
use tvm_types::SliceData;
use tvm_vm::executor::Engine;
use tvm_vm::executor::EngineTraceInfo;
use tvm_vm::executor::EngineTraceInfoType;

use crate::RunArgs;

const DEFAULT_CAPABILITIES: u64 = 1525038;
const DEFAULT_CONTRACT_BALANCE: u64 = 1_000_000_000_000_000;

pub(crate) fn load_abi_as_string(path: &PathBuf) -> anyhow::Result<String> {
    std::fs::read_to_string(path)
        .map_err(|e| anyhow::format_err!("Failed to load abi from file {path:?}: {e}"))
}

pub(crate) fn read_keys(filename: &PathBuf) -> anyhow::Result<KeyPair> {
    let keys_str = std::fs::read_to_string(filename)
        .map_err(|e| anyhow::format_err!("failed to read the keypair file {filename:?}: {}", e))?;
    let keys: KeyPair = serde_json::from_str(&keys_str)
        .map_err(|e| anyhow::format_err!("failed to load keypair: {}", e))?;
    Ok(keys)
}

pub(crate) fn get_now(_args: &RunArgs) -> UnixTime32 {
    (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32).into()
}

pub(crate) fn get_dest_address(args: &RunArgs) -> anyhow::Result<MsgAddressInt> {
    Ok(match args.address.as_ref() {
        Some(address) => MsgAddressInt::from_str(address)
            .map_err(|e| anyhow::format_err!("Failed to decode contract address: {e}"))?,
        None => MsgAddressInt::default(),
    })
}

pub(crate) fn load_code_and_data_from_state_init(state_init: &StateInit) -> (SliceData, SliceData) {
    let code: SliceData =
        SliceData::load_cell(state_init.code.clone().unwrap_or_default()).unwrap();
    let data = SliceData::load_cell(state_init.data.clone().unwrap_or_default()).unwrap();
    (code, data)
}

pub(crate) fn contract_balance(_args: &RunArgs) -> CurrencyCollection {
    CurrencyCollection::with_grams(DEFAULT_CONTRACT_BALANCE)
}

pub(crate) fn capabilities(_args: &RunArgs) -> u64 {
    DEFAULT_CAPABILITIES
}

pub(crate) fn config_params(_args: &RunArgs) -> Option<Cell> {
    None
}

pub(crate) fn trace_callback(
    _engine: &Engine,
    info: &EngineTraceInfo,
    extended: bool, // , debug_info: &Option<DbgInfo>
) {
    if info.info_type == EngineTraceInfoType::Dump {
        println!("{}", info.cmd_str);
        return;
    }
    println!("{}: {}", info.step, info.cmd_str);
    if extended {
        println!("{} {}", info.cmd_code.remaining_bits(), info.cmd_code.to_hex_string());
    }
    println!("\nGas: {} ({})", info.gas_used, info.gas_cmd);
    // let position = get_position(info, debug_info);
    // if position.is_some() {
    //     println!("Position: {}", position.unwrap());
    // }
    println!("\n--- Stack trace ------------------------");
    for item in info.stack.iter() {
        println!("{item}");
    }
    println!("----------------------------------------\n");
}

fn is_valid_base64(s: &str) -> bool {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s).is_ok()
}

// Read the value from the file if the value is not a valid base64 string
pub(crate) fn get_base64_or_read_from_file(s: Option<&str>) -> anyhow::Result<Option<String>> {
    if let Some(s) = s {
        if is_valid_base64(s) {
            return Ok(Some(s.to_string()));
        }
        let content = std::fs::read_to_string(s)?.trim().to_string();
        if is_valid_base64(&content) {
            return Ok(Some(content));
        }
        anyhow::bail!("Failed to parse provided parameters as base64 string!");
    }
    Ok(None)
}

// Read the value from the file if the value is not a valid json string
pub(crate) fn get_json_value_or_read_file(s: &str) -> anyhow::Result<serde_json::Value> {
    serde_json::from_str(s).or_else(|_| {
        let content = std::fs::read_to_string(s)?;
        serde_json::from_str(content.trim())
            .map_err(|_| anyhow::anyhow!("Failed to parse provided parameters as JSON"))
    })
}
