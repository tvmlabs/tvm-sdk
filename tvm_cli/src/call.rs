// Copyright 2018-2023 EverX.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.
use std::str::FromStr;
use std::time::Duration;

use serde_json::Value;
use serde_json::json;
use tvm_abi::ParamType;
use tvm_block::Account;
use tvm_block::Serializable;
use tvm_client::abi::Abi;
use tvm_client::abi::ParamsOfDecodeMessage;
use tvm_client::abi::ParamsOfEncodeMessage;
use tvm_client::abi::decode_message;
use tvm_client::abi::encode_message;
use tvm_client::error::ClientError;
use tvm_client::net::ParamsOfQuery;
use tvm_client::processing::ParamsOfProcessMessage;
use tvm_client::processing::ParamsOfSendMessage;
use tvm_client::processing::ProcessingEvent;
use tvm_client::tvm::AccountForExecutor;
use tvm_client::tvm::ParamsOfRunExecutor;
use tvm_client::tvm::run_executor;
use tvm_types::base64_encode;

use crate::config::Config;
use crate::convert;
use crate::debug::init_debug_logger;
use crate::helpers::TonClient;
use crate::helpers::TvmClient;
use crate::helpers::create_client;
use crate::helpers::create_client_verbose;
use crate::helpers::load_abi;
use crate::helpers::load_ton_abi;
use crate::helpers::query_account_field;
use crate::message::EncodedMessage;
use crate::message::prepare_message_params;
use crate::message::print_encoded_message;
use crate::message::unpack_message;

async fn decode_call_parameters(
    ton: TonClient,
    msg: &EncodedMessage,
    abi: Abi,
) -> Result<(String, String), String> {
    let result = decode_message(
        ton,
        ParamsOfDecodeMessage { abi, message: msg.message.clone(), ..Default::default() },
    )
    .map_err(|e| format!("couldn't decode message: {}", e))?;

    Ok((result.name, format!("{:#}", result.value.unwrap_or(json!({})))))
}

fn parse_integer_param(value: &str) -> Result<String, String> {
    let value = value.trim_matches('\"');

    if value.ends_with('T') {
        convert::convert_token(value.trim_end_matches('T'))
    } else {
        Ok(value.to_owned())
    }
}

async fn build_json_from_params(
    params_vec: Vec<&str>,
    abi_path: &str,
    method: &str,
    config: &Config,
) -> Result<String, String> {
    let abi_obj = load_ton_abi(abi_path, config).await?;
    let functions = abi_obj.functions();

    let func_obj = functions.get(method).ok_or("failed to load function from abi")?;
    let inputs = func_obj.input_params();

    let mut params_json = json!({});
    for input in inputs {
        let mut iter = params_vec.iter();
        let _param = iter
            .find(|x| x.starts_with('-') && (x.trim_start_matches('-') == input.name))
            .ok_or(format!(r#"argument "{}" of type "{}" not found"#, input.name, input.kind))?;

        let value = iter
            .next()
            .ok_or(format!(r#"argument "{}" of type "{}" has no value"#, input.name, input.kind))?
            .to_string();

        let value = match input.kind {
            ParamType::Uint(_) | ParamType::Int(_) => {
                json!(parse_integer_param(&value)?)
            }
            ParamType::Array(ref _x) => {
                let mut result_vec: Vec<String> = vec![];
                for i in value.split([',', '[', ']']) {
                    if !i.is_empty() {
                        result_vec.push(parse_integer_param(i)?)
                    }
                }
                json!(result_vec)
            }
            _ => {
                json!(value)
            }
        };
        params_json[input.name.clone()] = value;
    }

    serde_json::to_string(&params_json).map_err(|e| format!("{}", e))
}

pub async fn emulate_locally(
    ton: TonClient,
    addr: &str,
    msg: String,
    is_fee: bool,
) -> Result<(), String> {
    let state: String;
    let state_boc = query_account_field(ton.clone(), addr, "boc").await;
    if state_boc.is_err() {
        if is_fee {
            let addr = tvm_block::MsgAddressInt::from_str(addr)
                .map_err(|e| format!("couldn't decode address: {}", e))?;
            state = base64_encode(
                &tvm_types::write_boc(&Account::with_address(addr).serialize().map_err(|e| {
                    format!("couldn't create dummy account for deploy emulation: {}", e)
                })?)
                .map_err(|e| format!("failed to serialize account cell: {}", e))?,
            );
        } else {
            return Err(state_boc.err().unwrap());
        }
    } else {
        state = state_boc.unwrap();
    }
    let res = run_executor(
        ton.clone(),
        ParamsOfRunExecutor {
            message: msg.clone(),
            account: AccountForExecutor::Account {
                boc: state,
                unlimited_balance: if is_fee { Some(true) } else { None },
            },
            ..Default::default()
        },
    )
    .await;

    if res.is_err() {
        return Err(format!("{:#}", res.err().unwrap()));
    }
    if is_fee {
        let fees = res.unwrap().fees;
        println!("{{");
        println!("  \"in_msg_fwd_fee\": \"{}\",", fees.in_msg_fwd_fee);
        println!("  \"storage_fee\": \"{}\",", fees.storage_fee);
        println!("  \"gas_fee\": \"{}\",", fees.gas_fee);
        println!("  \"out_msgs_fwd_fee\": \"{}\",", fees.out_msgs_fwd_fee);
        println!("  \"total_account_fees\": \"{}\",", fees.total_account_fees);
        println!("  \"total_output\": \"{}\"", fees.total_output);
        println!("}}");
    } else {
        println!("Local run succeeded. Executing onchain."); // TODO: check
        // is_json
    }
    Ok(())
}

pub async fn send_message_and_wait(
    tvm_client: TvmClient,
    abi: Option<Abi>,
    msg: String,
    config: &Config,
    dst_dapp_id: Option<&str>,
) -> Result<Value, String> {
    if !config.is_json {
        println!("Processing... ");
    }

    let callback = |_| async move {};
    let result = tvm_client::processing::send_message(
        tvm_client.clone(),
        ParamsOfSendMessage {
            message: msg.clone(),
            abi: abi.clone(),
            dapp_id: dst_dapp_id.map(String::from).unwrap_or_default(),
            ..Default::default()
        },
        callback,
    )
    .await
    .map_err(|e| format!("{:#}", e))?;

    require_transaction_hash(result.tx_hash.as_deref(), result.message_hash.as_deref())?;
    wait_for_transaction_hash(
        tvm_client,
        result.tx_hash.as_deref().unwrap_or_default(),
        result.message_hash.as_deref(),
    )
    .await?;

    let value = serde_json::to_value(result).map_err(|e| format!("{e:#}"))?;
    Ok(value)
}

pub async fn send_message(
    context: TvmClient,
    msg: String,
    abi: Option<Abi>,
    config: &Config,
    thread_id: Option<&str>,
    dst_dapp_id: Option<&str>,
) -> Result<Value, String> {
    if !config.is_json {
        println!("Processing... ");
    }

    let callback = |_| async move {};
    let result = tvm_client::processing::send_message(
        context.clone(),
        ParamsOfSendMessage {
            message: msg.clone(),
            abi: abi.clone(),
            thread_id: thread_id.map(ToString::to_string),
            dapp_id: dst_dapp_id.map(String::from).unwrap_or_default(),
            ..Default::default()
        },
        callback,
    )
    .await
    .map_err(|e| format!("{:#}", e))?;

    require_transaction_hash(result.tx_hash.as_deref(), result.message_hash.as_deref())?;
    wait_for_transaction_hash(
        context,
        result.tx_hash.as_deref().unwrap_or_default(),
        result.message_hash.as_deref(),
    )
    .await?;

    serde_json::to_value(result).map_err(|e| format!("{e:#}"))
}

fn require_transaction_hash(
    tx_hash: Option<&str>,
    message_hash: Option<&str>,
) -> Result<(), String> {
    if tx_hash.is_some() {
        Ok(())
    } else {
        Err(format!(
            "Message {} was not processed into a transaction",
            message_hash.unwrap_or("<unknown>")
        ))
    }
}

async fn wait_for_transaction_hash(
    context: TvmClient,
    tx_hash: &str,
    message_hash: Option<&str>,
) -> Result<(), String> {
    let start = tokio::time::Instant::now();
    loop {
        let response = tvm_client::net::query(
            context.clone(),
            ParamsOfQuery {
                query: "query transaction($hash:String!){blockchain{transaction(hash:$hash){id}}}"
                    .to_string(),
                variables: Some(json!({ "hash": tx_hash })),
            },
        )
        .await
        .map_err(|e| format!("{e:#}"))?;
        if transaction_is_present(&response.result) {
            return Ok(());
        }
        if start.elapsed() >= Duration::from_secs(15) {
            return Err(format!(
                "Message {} was not processed into a transaction",
                message_hash.unwrap_or("<unknown>")
            ));
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

fn transaction_is_present(result: &Value) -> bool {
    result.pointer("/data/blockchain/transaction").is_some_and(|transaction| !transaction.is_null())
}

#[cfg(test)]
mod transaction_result_tests {
    use serde_json::json;

    use super::require_transaction_hash;
    use super::transaction_is_present;

    #[test]
    fn rejects_message_without_transaction_hash() {
        let error = require_transaction_hash(None, Some("message-id")).unwrap_err();
        assert!(error.contains("message-id"));
        assert!(error.contains("not processed into a transaction"));
    }

    #[test]
    fn accepts_message_with_transaction_hash() {
        assert!(require_transaction_hash(Some("transaction-id"), Some("message-id")).is_ok());
    }

    #[test]
    fn detects_transaction_presence_in_graphql_response() {
        assert!(transaction_is_present(&json!({
            "data": { "blockchain": { "transaction": { "id": "tx" } } }
        })));
        assert!(!transaction_is_present(&json!({
            "data": { "blockchain": { "transaction": null } }
        })));
    }
}

pub async fn process_message(
    ton: TonClient,
    msg: ParamsOfEncodeMessage,
    config: &Config,
    dst_dapp_id: Option<&str>,
) -> Result<Value, ClientError> {
    let callback = |event| async move {
        if let ProcessingEvent::DidSend {
            shard_block_id: _,
            message_id,
            message_dst: _,
            message: _,
        } = event
        {
            println!("MessageId: {}", message_id)
        }
    };
    let dapp_id = dst_dapp_id.map(String::from).unwrap_or_default();
    let res = if !config.is_json {
        tvm_client::processing::process_message(
            ton.clone(),
            ParamsOfProcessMessage {
                message_encode_params: msg.clone(),
                dapp_id: dapp_id.clone(),
                ..Default::default()
            },
            callback,
        )
        .await
    } else {
        tvm_client::processing::process_message(
            ton.clone(),
            ParamsOfProcessMessage {
                message_encode_params: msg.clone(),
                send_events: true,
                dapp_id,
            },
            |_| async move {},
        )
        .await
    }?;

    Ok(res.decoded.and_then(|d| d.output).unwrap_or(json!({})))
}

pub async fn call_contract_with_result(
    config: &Config,
    addr: &str,
    abi_path: &str,
    method: &str,
    params: &str,
    keys: Option<String>,
    is_fee: bool,
    thread_id: Option<&str>,
    dst_dapp_id: Option<&str>,
) -> Result<Value, String> {
    let tvm_client = if config.debug_fail != *"None" {
        init_debug_logger(&format!("call_{}_{}.log", addr, method))?;
        create_client(config)?
    } else {
        create_client_verbose(config)?
    };
    call_contract_with_client(
        tvm_client,
        config,
        addr,
        abi_path,
        method,
        params,
        keys,
        is_fee,
        thread_id,
        dst_dapp_id,
    )
    .await
}

pub async fn call_contract_with_client(
    tvm_client: TvmClient,
    config: &Config,
    addr: &str,
    abi_path: &str,
    method: &str,
    params: &str,
    keys: Option<String>,
    is_fee: bool,
    thread_id: Option<&str>,
    dst_dapp_id: Option<&str>,
) -> Result<Value, String> {
    let abi = load_abi(abi_path, config).await?;

    let msg_params = prepare_message_params(addr, abi.clone(), method, params, None, keys.clone())?;

    let encoded_message = encode_message(tvm_client.clone(), msg_params.clone())
        .await
        .map_err(|e| format!("failed to create inbound message: {}", e))?;

    if !config.is_json {
        println!("MessageId: {}", encoded_message.message_id);
    }

    let msg = encoded_message.message;
    if config.local_run || is_fee {
        emulate_locally(tvm_client.clone(), addr, msg.clone(), is_fee).await?;
        if is_fee {
            return Ok(Value::Null);
        }
    }

    send_message(tvm_client.clone(), msg, Some(abi), config, thread_id, dst_dapp_id).await
}

pub fn print_json_result(result: Value, config: &Config) -> Result<(), String> {
    if !result.is_null() {
        if !config.is_json {
            println!("Result: {:#}", result);
        } else {
            println!("{:#}", result);
        }
    }
    Ok(())
}

pub async fn call_contract(
    config: &Config,
    addr: &str,
    abi_path: &str,
    method: &str,
    params: &str,
    keys: Option<String>,
    is_fee: bool,
    thread_id: Option<&str>,
    dst_dapp_id: Option<&str>,
) -> Result<(), String> {
    let result = call_contract_with_result(
        config,
        addr,
        abi_path,
        method,
        params,
        keys,
        is_fee,
        thread_id,
        dst_dapp_id,
    )
    .await?;
    if !config.is_json {
        println!("Succeeded.");
    }
    print_json_result(result, config)?;
    Ok(())
}

pub async fn call_contract_with_msg(
    config: &Config,
    str_msg: String,
    abi_path: &str,
    dst_dapp_id: Option<&str>,
) -> Result<(), String> {
    let ton = create_client_verbose(config)?;
    let abi = load_abi(abi_path, config).await?;

    let (msg, _) = unpack_message(&str_msg)?;
    if config.is_json {
        println!("{{");
    }
    print_encoded_message(&msg, config.is_json);

    let params = decode_call_parameters(ton.clone(), &msg, abi.clone()).await?;

    if !config.is_json {
        println!("Calling method {} with parameters:", params.0);
        println!("{}", params.1);
        println!("Processing... ");
    } else {
        println!("  \"Method\": \"{}\",", params.0);
        println!("  \"Parameters\": {},", params.1);
        println!("}}");
    }
    let result = send_message_and_wait(ton, Some(abi), msg.message, config, dst_dapp_id).await?;

    if !config.is_json {
        println!("Succeeded.");
        if !result.is_null() {
            println!("Result: {:#}", result);
        }
    }
    Ok(())
}

pub async fn parse_params(
    params_vec: Vec<&str>,
    abi_path: &str,
    method: &str,
    config: &Config,
) -> Result<String, String> {
    if params_vec.len() == 1 {
        // if there is only 1 parameter it must be a json string with arguments
        Ok(params_vec[0].to_owned())
    } else {
        build_json_from_params(params_vec, abi_path, method, config).await
    }
}
