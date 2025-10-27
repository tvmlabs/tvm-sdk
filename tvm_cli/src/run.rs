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

use clap::ArgMatches;
use serde_json::Map;
use serde_json::Value;
use tvm_block::Account;
use tvm_block::Deserializable;
use tvm_block::Serializable;
use tvm_client::abi::FunctionHeader;
use tvm_client::tvm::ExecutionOptions;
use tvm_client::tvm::ParamsOfRunGet;
use tvm_client::tvm::ParamsOfRunTvm;
use tvm_client::tvm::run_get;
use tvm_client::tvm::run_tvm;
use tvm_types::base64_encode;

use crate::call::print_json_result;
use crate::config::Config;
use crate::config::FullConfig;
use crate::debug::DebugParams;
use crate::debug::debug_error;
use crate::debug::init_debug_logger;
use crate::helpers::AccountSource;
use crate::helpers::TonClient;
use crate::helpers::abi_from_matches_or_config;
use crate::helpers::contract_data_from_matches_or_config_alias;
use crate::helpers::create_client;
use crate::helpers::create_client_local;
use crate::helpers::create_client_verbose;
use crate::helpers::get_blockchain_config;
use crate::helpers::load_abi;
use crate::helpers::load_account;
use crate::helpers::load_params;
use crate::helpers::now;
use crate::helpers::now_ms;
use crate::helpers::unpack_alternative_params;
use crate::message::prepare_message;
use crate::replay::construct_blockchain_config;

pub async fn run_command(
    matches: &ArgMatches,
    full_config: &FullConfig,
    is_alternative: bool,
) -> Result<(), String> {
    let config = &full_config.config;
    let (address, abi_path) = if is_alternative {
        let (address, abi, _) = contract_data_from_matches_or_config_alias(matches, full_config)?;
        (address.unwrap(), abi.unwrap())
    } else {
        (
            matches.value_of("ADDRESS").unwrap().to_string(),
            abi_from_matches_or_config(matches, config)?,
        )
    };
    let account_source = if matches.is_present("TVC") {
        AccountSource::TVC
    } else if matches.is_present("BOC") {
        AccountSource::BOC
    } else {
        AccountSource::NETWORK
    };

    let method = if is_alternative {
        matches
            .value_of("METHOD")
            .or(config.method.as_deref())
            .ok_or("Method is not defined. Supply it in the config file or command line.")?
    } else {
        matches.value_of("METHOD").unwrap()
    };
    let trace_path;
    let tvm_client = if account_source == AccountSource::NETWORK {
        trace_path = format!("run_{}_{}.log", address, method);
        create_client(config)?
    } else {
        trace_path = "trace.log".to_string();
        create_client_local()?
    };

    let (account, account_boc, state_timestamp) =
        load_account(&account_source, &address, Some(tvm_client.clone()), config).await?;

    let address = match account_source {
        AccountSource::NETWORK => address,
        AccountSource::BOC => account.get_addr().unwrap().to_string(),
        AccountSource::TVC => "0".repeat(64),
    };

    run(
        matches,
        config,
        Some(tvm_client),
        &address,
        account_boc,
        state_timestamp,
        abi_path,
        is_alternative,
        trace_path,
    )
    .await
}

async fn run(
    matches: &ArgMatches,
    config: &Config,
    ton_client: Option<TonClient>,
    address: &str,
    account_boc: String,
    state_timestamp: Option<u64>,
    abi_path: String,
    is_alternative: bool,
    trace_path: String,
) -> Result<(), String> {
    let method = if is_alternative {
        matches
            .value_of("METHOD")
            .or(config.method.as_deref())
            .ok_or("Method is not defined. Supply it in the config file or command line.")?
    } else {
        matches.value_of("METHOD").unwrap()
    };
    let bc_config = matches.value_of("BCCONFIG");

    if !config.is_json {
        println!("Running get-method...");
    }
    let ton_client = match ton_client {
        Some(ton_client) => ton_client,
        None => create_client_local()?,
    };

    let abi = load_abi(&abi_path, config).await?;
    let params = if is_alternative {
        unpack_alternative_params(matches, &abi_path, method, config).await?
    } else {
        matches.value_of("PARAMS").unwrap().to_string()
    };

    let params = load_params(&params)?;

    let expire_at = config.lifetime + now();
    let header = FunctionHeader { expire: Some(expire_at), ..Default::default() };

    let msg = prepare_message(
        ton_client.clone(),
        address,
        abi.clone(),
        method,
        &params,
        Some(header),
        None,
        config.is_json,
    )
    .await?;

    let execution_options = prepare_execution_options(bc_config)?;
    let result = run_tvm(
        ton_client.clone(),
        ParamsOfRunTvm {
            message: msg.message.clone(),
            account: account_boc.clone(),
            abi: Some(abi.clone()),
            return_updated_account: Some(true),
            execution_options,
            ..Default::default()
        },
    )
    .await;

    let mut result = match result {
        Ok(result) => result,
        Err(e) => {
            let bc_config = get_blockchain_config(config, bc_config).await?;
            let now = now_ms();
            let debug_params = DebugParams {
                account: &account_boc,
                message: Some(&msg.message),
                time_in_ms: now,
                block_lt: now,
                last_tr_lt: now,
                is_getter: true,
                ..DebugParams::new(config, bc_config)
            };
            init_debug_logger(&trace_path)?;
            debug_error(&e, debug_params).await?;
            return Err(format!("{:#}", e));
        }
    };
    if !config.is_json {
        println!("Succeeded.");
    }

    if !result.out_messages.is_empty() {
        let mut data = match result.decoded.as_mut().and_then(|d| d.output.take()) {
            Some(v) => v,
            None => {
                eprintln!("Failed to decode output messages. Check that ABI matches the contract.");
                eprintln!("Messages in base64:\n{:?}", result.out_messages);
                return Ok(());
            }
        };

        if let Some(ts) = state_timestamp {
            if let Some(obj) = data.as_object_mut() {
                obj.insert("state_timestamp".to_string(), Value::from(ts));
            }
        }

        print_json_result(data, config)?;
    }
    Ok(())
}

fn prepare_execution_options(bc_config: Option<&str>) -> Result<Option<ExecutionOptions>, String> {
    if let Some(config) = bc_config {
        let mut bytes = std::fs::read(config)
            .map_err(|e| format!("Failed to read data from file {config}: {e}"))?;
        let cell = tvm_types::read_single_root_boc(&bytes)
            .map_err(|e| format!("Failed to deserialize {config}: {e}"))?;
        if let Ok(acc) = Account::construct_from_cell(cell.clone()) {
            let config = construct_blockchain_config(&acc)?;
            bytes = config
                .raw_config()
                .write_to_bytes()
                .map_err(|e| format!("Failed to serialize config params: {e}"))?;
        }
        let blockchain_config = Some(base64_encode(bytes));
        let ex_opt = ExecutionOptions { blockchain_config, ..Default::default() };
        return Ok(Some(ex_opt));
    }
    Ok(None)
}

pub async fn run_get_method(
    config: &Config,
    addr: &str,
    method: &str,
    params: Option<String>,
    source_type: AccountSource,
    bc_config: Option<&str>,
) -> Result<(), String> {
    let ton = if source_type == AccountSource::NETWORK {
        create_client_verbose(config)?
    } else {
        create_client_local()?
    };

    let (_, acc_boc, _) = load_account(&source_type, addr, Some(ton.clone()), config).await?;

    let params = params
        .map(|p| serde_json::from_str(&p))
        .transpose()
        .map_err(|e| format!("arguments are not in json format: {}", e))?;

    if !config.is_json {
        println!("Running get-method...");
    }
    let execution_options = prepare_execution_options(bc_config)?;
    let result = run_get(
        ton,
        ParamsOfRunGet {
            account: acc_boc,
            function_name: method.to_owned(),
            input: params,
            execution_options,
            ..Default::default()
        },
    )
    .await
    .map_err(|e| format!("run failed: {}", e))?
    .output;

    if !config.is_json {
        println!("Succeeded.");
        println!("Result: {}", result);
    } else {
        let mut res = Map::new();
        match result {
            Value::Array(array) => {
                let mut i = 0;
                for val in array.iter() {
                    res.insert(format!("value{}", i), val.to_owned());
                    i += 1;
                }
            }
            _ => {
                res.insert("value0".to_owned(), result);
            }
        }
        let res = Value::Object(res);
        println!("{:#}", res);
    }
    Ok(())
}
