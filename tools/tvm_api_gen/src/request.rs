// Copyright 2018-2021 EverX Labs Ltd.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific EVERX DEV software governing permissions and
// limitations under the License.
//

use std::sync::Arc;

use api_info::API;
use base64::Engine;
use regex::Captures;
use regex::Regex;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tvm_client::ContextHandle;
use tvm_client::StringData;
use tvm_client::client::ClientContext;
use tvm_client::error::ClientResult;
use tvm_client::tc_create_context;
use tvm_client::tc_destroy_context;
use tvm_client::tc_destroy_string;
use tvm_client::tc_read_string;
use tvm_client::tc_request_sync;

use crate::errors::CliError;

enum ParseState {
    OptionOrFunctionName,
    OptionValue,
    Parameters,
}

fn reformat_json(value: impl Serialize) -> Result<String, CliError> {
    serde_json::to_string_pretty(&value).map_err(|e| CliError::with_message(e.to_string()))
}

fn resolve_json_path<'a>(value: &'a Value, _path: &str) -> &'a Value {
    value
}

fn include_json(json_ref: &str) -> Result<String, CliError> {
    let ref_parts: Vec<&str> = json_ref.split("+").collect();
    let home = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    let ref_file = ref_parts[0].replace("~", home.as_str());
    let ref_path = if ref_parts.len() > 1 { ref_parts[1] } else { "" };
    if ref_file.ends_with(".json") {
        let ref_string = std::fs::read_to_string(&ref_file)
            .map_err(|e| CliError::with_message(format!("Include [{}] failed: {}", ref_file, e)))?;
        let value: Value = serde_json::from_str(&ref_string)
            .map_err(|e| CliError::with_message(format!("Include [{}] failed: {}", ref_file, e)))?;
        let value = resolve_json_path(&value, ref_path);
        Ok(value.to_string())
    } else {
        let ref_bytes = std::fs::read(&ref_file)
            .map_err(|e| CliError::with_message(format!("Include [{}] failed: {}", ref_file, e)))?;
        Ok(format!("\"{}\"", base64::engine::general_purpose::STANDARD.encode(&ref_bytes)))
    }
}

fn parse_sync_response<R: DeserializeOwned>(response: *const String) -> Result<R, CliError> {
    let response = unsafe {
        let result = tc_read_string(response).to_string();
        tc_destroy_string(response);
        result
    };
    match serde_json::from_str::<Value>(&response) {
        Ok(value) => {
            if value["error"].is_object() {
                Err(CliError::with_message(format!("Function failed: {}", value["error"])))
            } else {
                Ok(serde_json::from_value(value["result"].clone()).unwrap())
            }
        }
        Err(err) => Err(CliError::with_message(format!("Read core response failed: {}", err))),
    }
}

fn get_api() -> ClientResult<API> {
    let context = Arc::new(ClientContext::new(Default::default())?);
    Ok(tvm_client::client::get_api_reference(context)?.api)
}

pub fn command(args: &[String]) -> Result<(), CliError> {
    let mut network = "shellnet.ackinacki.org".to_string();
    let mut state = ParseState::OptionOrFunctionName;
    let mut option = String::new();
    let mut function = String::new();
    let mut parameters = String::new();

    let api = get_api()?;
    for arg in args.iter() {
        match state {
            ParseState::OptionOrFunctionName if arg.starts_with("-") => {
                option = arg[1..].to_string();
                state = ParseState::OptionValue
            }
            ParseState::OptionOrFunctionName if arg.starts_with("--") => {
                option = arg[2..].to_string();
                state = ParseState::OptionValue
            }
            ParseState::OptionOrFunctionName => {
                function = arg.clone();
                state = ParseState::Parameters
            }
            ParseState::OptionValue => {
                match option.as_str() {
                    "n" | "network" => network = arg.clone(),
                    _ => {}
                }
                state = ParseState::OptionOrFunctionName
            }
            ParseState::Parameters => {
                if !parameters.is_empty() {
                    parameters.push(' ');
                }
                parameters.push_str(arg.as_str())
            }
        }
    }
    if !parameters.trim().is_empty() {
        let file_refs = Regex::new(r"@([^\s,]*)")?;
        parameters = file_refs
            .replace_all(&parameters, |caps: &Captures| match include_json(&caps[1]) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e.message);
                    std::process::exit(1)
                }
            })
            .to_string();
        let json: Value = json5::from_str(parameters.as_str())?;
        parameters = json.to_string();
    }

    let mut function_names = Vec::<String>::new();
    for module in &api.modules {
        for function in &module.functions {
            function_names.push(format!("{}.{}", module.name, function.name))
        }
    }
    if function.is_empty() {
        return Err(CliError::with_message(format!(
            "Function doesn't specified. Available functions are:\n{}",
            function_names.join("\n")
        )));
    }

    let mut names = Vec::<String>::new();
    for name in function_names {
        if name == function {
            names.clear();
            names.push(name);
            break;
        }
        if name.contains(function.as_str()) {
            names.push(name);
        }
    }
    if names.is_empty() {
        return Err(CliError::with_message(format!(
            "Unknown function [{}]. Available functions are:\n{}",
            function,
            names.join("\n")
        )));
    }
    if names.len() > 1 {
        return Err(CliError::with_message(format!(
            "Unknown function [{}]. May be you mean one of following:\n{}",
            function,
            names.join("\n")
        )));
    }
    if names[0] != function {
        eprintln!("Unknown function [{}]. [{}] used instead.", function, names[0]);
        function = names[0].clone();
    }

    let config = serde_json::json!({
        "network": {
            "endpoints": [network]
        }
    });
    let context = unsafe {
        parse_sync_response::<ContextHandle>(tc_create_context(StringData::new(
            &config.to_string(),
        )))
    }?;

    let response = unsafe {
        parse_sync_response::<Value>(tc_request_sync(
            context,
            StringData::new(&function),
            StringData::new(&parameters),
        ))
    };
    unsafe { tc_destroy_context(context) };

    match response {
        Ok(value) => {
            println!("{}", reformat_json(value)?);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
