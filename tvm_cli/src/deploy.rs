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
use tvm_client::abi::Abi;
use tvm_client::abi::CallSet;
use tvm_client::abi::DeploySet;
use tvm_client::abi::FunctionHeader;
use tvm_client::abi::ParamsOfEncodeMessage;
use tvm_client::abi::Signer;
use tvm_client::abi::encode_message;
use tvm_client::crypto::KeyPair;
use tvm_types::base64_encode;

use crate::call::send_message;
use crate::Config;
use crate::call::emulate_locally;
use crate::call::print_json_result;
use crate::config::FullConfig;
use crate::crypto::load_keypair;
use crate::helpers::create_client_local;
use crate::helpers::create_client_verbose;
use crate::helpers::load_abi;
use crate::helpers::now_ms;
use crate::message::EncodedMessage;
use crate::message::display_generated_message;

pub async fn deploy_contract(
    full_config: &mut FullConfig,
    tvc: &str,
    abi: &str,
    params: &str,
    keys_file: Option<String>,
    wc: i32,
    is_fee: bool,
    alias: Option<&str>,
) -> Result<(), String> {
    let config = &full_config.config;
    let tvm_client = create_client_verbose(config)?;

    if !is_fee && !config.is_json {
        println!("Deploying...");
    }

    let (msg_params, addr) =
        prepare_deploy_message(tvc, abi, params, keys_file.clone(), wc, &full_config.config)
            .await?;

    let encoded_msg = encode_message(tvm_client.clone(), msg_params.clone())
        .await
        .map_err(|e| format!("failed to create inbound message: {}", e))?;

    let msg = encoded_msg.message;

    if config.local_run || is_fee {
        emulate_locally(tvm_client.clone(), addr.as_str(), msg.clone(), is_fee).await?;
        if is_fee {
            return Ok(());
        }
    }

    let result = send_message(tvm_client.clone(), msg, config, None).await?;

    let mut map: serde_json::Map<String, serde_json::Value> = serde_json::from_value(result.clone())
        .map_err(|e| format!("failed to convert result: {e}"))?;

    if !config.is_json {
        if !config.async_call {
            println!("Transaction succeeded.");
        }
        map.iter().for_each(|(k, v)|
            if let Some(str) = v.as_str() {
                println!("{k}: {str}");
            } else {
                println!("{k}: {v}");
            }
        );
        println!("Contract deployed at address: {}", addr);
    } else {
        map.insert("deployed_at".to_string(), serde_json::Value::String(addr.clone()));
        print_json_result(serde_json::Value::Object(map), config)?;
    }
    if let Some(alias) = alias {
        full_config.add_alias(alias, Some(addr), Some(abi.to_string()), keys_file)?;
    }
    Ok(())
}

pub async fn generate_deploy_message(
    tvc: &str,
    abi: &str,
    params: &str,
    keys_file: Option<String>,
    wc: i32,
    is_raw: bool,
    output: Option<&str>,
    config: &Config,
) -> Result<(), String> {
    let ton = create_client_local()?;

    let (msg, addr) = prepare_deploy_message(tvc, abi, params, keys_file, wc, config).await?;
    let msg = encode_message(ton, msg)
        .await
        .map_err(|e| format!("failed to create inbound message: {}", e))?;

    let msg = EncodedMessage {
        message: msg.message,
        message_id: msg.message_id,
        expire: None,
        address: addr.to_owned(),
    };
    display_generated_message(&msg, "constructor", is_raw, output, config.is_json)?;
    if !config.is_json {
        println!("Contract's address: {}", addr);
        println!("Succeeded.");
    }
    Ok(())
}

pub async fn prepare_deploy_message(
    tvc: &str,
    abi: &str,
    params: &str,
    keys_file: Option<String>,
    wc: i32,
    config: &Config,
) -> Result<(ParamsOfEncodeMessage, String), String> {
    let abi = load_abi(abi, config).await?;

    let keys = keys_file.map(|k| load_keypair(&k)).transpose()?;

    let tvc_bytes =
        std::fs::read(tvc).map_err(|e| format!("failed to read smart contract file {tvc}: {e}"))?;

    prepare_deploy_message_params(
        &tvc_bytes,
        abi,
        "constructor".to_string(),
        now_ms(),
        params,
        keys,
        wc,
    )
    .await
}

pub async fn prepare_deploy_message_params(
    tvc_bytes: &[u8],
    abi: Abi,
    function_name: String,
    time: u64,
    params: &str,
    keys: Option<KeyPair>,
    wc: i32,
) -> Result<(ParamsOfEncodeMessage, String), String> {
    let tvc = base64_encode(tvc_bytes);

    let data_map_supported = abi.abi().unwrap().data_map_supported();
    let address = if data_map_supported {
        crate::helpers::calc_acc_address(
            tvc_bytes,
            wc,
            keys.as_ref().map(|k| k.public.clone()),
            None,
            abi.clone(),
        )
        .await?
    } else {
        let tvc_cell = tvm_types::boc::read_single_root_boc(tvc_bytes).unwrap();
        let tvc_hash = tvc_cell.repr_hash();
        format!("{}:{}", wc, tvc_hash.as_hex_string())
    };

    let header = Some(FunctionHeader { time: Some(time), ..Default::default() });
    let deploy_set =
        Some(DeploySet { tvc: Some(tvc), workchain_id: Some(wc), ..Default::default() });
    let params = serde_json::from_str(params)
        .map_err(|e| format!("function arguments is not a json: {}", e))?;
    let call_set =
        Some(CallSet { function_name, input: Some(params), header, ..Default::default() });
    let signer = if let Some(keys) = keys { Signer::Keys { keys } } else { Signer::None };
    Ok((
        ParamsOfEncodeMessage {
            abi,
            address: Some(address.clone()),
            deploy_set,
            call_set,
            signer,
            ..Default::default()
        },
        address,
    ))
}
