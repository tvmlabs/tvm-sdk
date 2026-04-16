use std::collections::BTreeMap;
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
use std::sync::Arc;

use serde_json::Value;
use serde_json::json;
use tvm_block::Account;
use tvm_block::AccountStatus;
use tvm_block::Deserializable;
use tvm_block::Serializable;
use tvm_client::account;
use tvm_client::error::ClientError;
use tvm_client::net::ParamsOfQueryCollection;
use tvm_client::net::ParamsOfSubscribeCollection;
use tvm_client::net::ResultOfSubscription;
use tvm_client::net::query_collection;
use tvm_client::utils::ParamsOfCalcStorageFee;
use tvm_client::utils::calc_storage_fee;
use tvm_types::base64_decode;

use crate::config::Config;
use crate::decode::msg_printer::tree_of_cells_into_base64;
use crate::decode::print_account_data;
use crate::helpers::check_dir;
use crate::helpers::create_client_verbose;
use crate::helpers::json_account;
use crate::helpers::print_account;
use crate::helpers::query_account_field;

const DEFAULT_PATH: &str = ".";

async fn query_accounts(
    config: &Config,
    addresses: Vec<String>,
    fields: &str,
) -> Result<Vec<Value>, String> {
    let client = create_client_verbose(config)?;

    if !config.is_json {
        println!("Processing...");
    }

    let fields = fields.to_string();

    let mut res = vec![];
    let mut it = 0;
    loop {
        if it >= addresses.len() {
            break;
        }
        let mut filter = json!({ "id": { "eq": addresses[it] } });
        let mut cnt = 1;
        for address in addresses.iter().skip(it).take(50) {
            cnt += 1;
            filter = json!({ "id": { "eq": address },
                "OR": filter
            });
        }
        it += cnt;
        let mut query_result = query_collection(
            client.clone(),
            ParamsOfQueryCollection {
                collection: "accounts".to_owned(),
                filter: Some(filter),
                result: fields.clone(),
                limit: Some(cnt as u32),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| format!("failed to query account info: {}", e))?;
        res.append(query_result.result.as_mut());
    }
    Ok(res)
}

pub async fn get_account(
    config: &Config,
    addresses: Vec<String>,
    dumptvc: Option<&str>,
    dumpboc: Option<&str>,
    is_boc: bool,
) -> Result<(), String> {
    if is_boc {
        let mut accounts = vec![];
        for path in addresses {
            let account = Account::construct_from_file(&path).map_err(|e| {
                format!(" failed to load account from the boc file {}: {}", path, e)
            })?;
            accounts.push(account);
        }
        if !config.is_json {
            println!("\nSucceeded.\n");
        }
        for account in accounts {
            print_account_data(&account, dumptvc, config, false).await?;
        }
        return Ok(());
    }

    let mut accounts = vec![];
    let client = crate::helpers::create_client(config)?;
    for address in addresses.iter() {
        let params = account::ParamsOfGetAccount { address: address.to_string() };

        let result_of_get_account = account::get_account(client.clone(), params)
            .await
            .map_err(|e| format!("failed to get account: {e}"))?;

        let boc_base64 = result_of_get_account.boc;
        let dapp_id = result_of_get_account.dapp_id;
        let state_timestamp = result_of_get_account.state_timestamp;

        let account = Account::construct_from_base64(&boc_base64)
            .map_err(|e| format!("failed to construct account from boc: {e}"))?;

        accounts.push((account, dapp_id, state_timestamp));
    }

    if !config.is_json {
        println!("Succeeded.");
    }

    let mut found_addresses = vec![];

    if !accounts.is_empty() {
        let mut json_res = json!({});
        for (acc, dapp_id, state_timestamp) in accounts.iter() {
            let address = acc.get_id().unwrap().as_hex_string();
            found_addresses.push(format!("0:{address}"));

            let acc_type = match acc.status() {
                AccountStatus::AccStateUninit => "Uninit".to_owned(),
                AccountStatus::AccStateFrozen => "Frozen".to_owned(),
                AccountStatus::AccStateActive => "Active".to_owned(),
                AccountStatus::AccStateNonexist => "NonExist".to_owned(),
            };

            if acc_type != "NonExist" {
                let bal = acc.balance();
                let balance = if bal.is_some() {
                    let bal = bal.unwrap().grams.clone().to_string();
                    if config.balance_in_vmshells {
                        let bal = u64::from_str_radix(&bal, 10)
                            .map_err(|e| format!("failed to decode balance: {}", e))?;
                        let int_bal = bal as f64 / 1e9;
                        let frac_balance = (bal as f64 / 1e6 + 0.5) as u64 % 1000;
                        let balance_str = format!("{}", int_bal as u64);
                        format!(
                            "{}.{}{}",
                            balance_str
                                .chars()
                                .collect::<Vec<char>>()
                                .rchunks(3)
                                .map(|c| c.iter().collect::<String>())
                                .rev()
                                .collect::<Vec<String>>()
                                .join(" "),
                            frac_balance,
                            if config.is_json { "" } else { " vmshell" }
                        )
                    } else {
                        format!("{}{}", bal, if config.is_json { "" } else { " nanovmshell" })
                    }
                } else {
                    "Undefined".to_owned()
                };

                let last_paid = format!("{}", acc.last_paid());

                let trans_lt =
                    acc.last_tr_time().map_or("Undefined".to_owned(), |v| format!("{:#x}", v));

                let data = tree_of_cells_into_base64(acc.get_data().as_ref());

                let data_boc = if data.is_ok() {
                    hex::encode(
                        base64_decode(data.unwrap())
                            .map_err(|e| format!("Failed to decode base64: {}", e))?,
                    )
                } else {
                    "null".to_owned()
                };

                let code_hash = match acc.get_code_hash() {
                    Some(hash) => hash.as_hex_string(),
                    None => "null".to_owned(),
                };

                if config.is_json {
                    json_res = json_account(
                        Some(acc_type),
                        Some(address.clone()),
                        Some(balance),
                        Some(last_paid),
                        Some(trans_lt),
                        Some(data_boc),
                        Some(code_hash),
                        None,
                        *state_timestamp,
                    );
                } else {
                    print_account(
                        config,
                        Some(acc_type),
                        Some(address.clone()),
                        Some(balance),
                        Some(last_paid),
                        Some(trans_lt),
                        Some(data_boc),
                        Some(code_hash),
                        None,
                        *state_timestamp,
                    );
                }

                let dapp_id = dapp_id.as_deref().unwrap_or("None");

                let ecc_balance = acc
                    .balance()
                    .map(|balance| {
                        let mut mapping = BTreeMap::new();
                        balance
                            .other
                            .iterate_with_keys(|k: u32, v| {
                                mapping.insert(k, v.value().to_string());
                                Ok(true)
                            })
                            .unwrap();
                        json!(mapping)
                    })
                    .unwrap_or(serde_json::Value::Null);
                if config.is_json {
                    json_res["dapp_id"] = json!(dapp_id);
                    json_res["ecc_balance"] = ecc_balance;
                } else {
                    println!("dapp_id:         {}", dapp_id);
                    println!("ecc:             {}", serde_json::to_string(&ecc_balance).unwrap());
                }
            } else if config.is_json {
                json_res = json_account(
                    Some(acc_type),
                    Some(address.clone()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    *state_timestamp,
                );
            } else {
                print_account(
                    config,
                    Some(acc_type),
                    Some(address.clone()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    *state_timestamp,
                );
            }
            if !config.is_json {
                println!();
            }
        }
        for address in addresses.iter() {
            if !found_addresses.contains(address) {
                if config.is_json {
                    json_res = json!({
                       "address": address.clone(),
                       "acc_type": "NonExist"
                    });
                } else {
                    println!("{} not found", address);
                    println!();
                }
            }
        }
        if config.is_json {
            println!("{:#}", json_res);
        }
    } else if config.is_json {
        println!("{{\n}}");
    } else {
        println!("Account not found.");
    }

    if dumptvc.is_some() || dumpboc.is_some() && accounts.len() == 1 {
        let (account, ..) = accounts[0].clone();
        if dumptvc.is_some() {
            if account.state_init().is_some() {
                account.state_init().unwrap().write_to_file(dumptvc.unwrap()).map_err(|e| {
                    format!("failed to write data to the file {}: {}", dumptvc.unwrap(), e)
                })?;
            } else {
                return Err("account doesn't contain state init.".to_owned());
            }
            if !config.is_json {
                println!("Saved contract to file {}", &dumptvc.unwrap());
            }
        }
        if dumpboc.is_some() {
            account.write_to_file(dumpboc.unwrap()).map_err(|e| {
                format!("failed to write data to the file {}: {}", dumpboc.unwrap(), e)
            })?;
            if !config.is_json {
                println!("Saved account to file {}", &dumpboc.unwrap());
            }
        }
    }
    Ok(())
}

pub async fn calc_storage(config: &Config, addr: &str, period: u32) -> Result<(), String> {
    let client = create_client_verbose(config)?;

    if !config.is_json {
        println!("Processing...");
    }

    let boc = query_account_field(client.clone(), addr, "boc").await?;

    let res = calc_storage_fee(
        client.clone(),
        ParamsOfCalcStorageFee { account: boc, period, ..Default::default() },
    )
    .await
    .map_err(|e| format!("failed to calculate storage fee: {}", e))?;

    if !config.is_json {
        println!("Storage fee per {} seconds: {} nanovmshells", period, res.fee);
    } else {
        println!("{{");
        println!("  \"storage_fee\": \"{}\",", res.fee);
        println!("  \"period\": \"{}\"", period);
        println!("}}");
    }
    Ok(())
}

pub async fn dump_accounts(
    config: &Config,
    addresses: Vec<String>,
    path: Option<&str>,
) -> Result<(), String> {
    let accounts = query_accounts(config, addresses.clone(), "id boc").await?;
    let mut addresses = addresses.clone();
    check_dir(path.unwrap_or(""))?;
    for account in accounts.iter() {
        let mut address = account["id"]
            .as_str()
            .ok_or("Failed to parse address in the query result".to_owned())?
            .to_owned();
        match addresses.iter().position(|el| el == &address) {
            Some(index) => addresses.remove(index),
            None => {
                return Err("Query contains an unexpected address.".to_string());
            }
        };

        address.replace_range(..address.find(':').unwrap_or(0) + 1, "");
        let path = format!("{}/{}.boc", path.unwrap_or(DEFAULT_PATH), address);
        let boc =
            account["boc"].as_str().ok_or("Failed to parse boc in the query result".to_owned())?;
        Account::construct_from_base64(boc)
            .map_err(|e| format!("Failed to load account from the boc: {}", e))?
            .write_to_file(path.clone())
            .map_err(|e| format!("Failed to write data to the file {}: {}", path.clone(), e))?;
        if !config.is_json {
            println!("{} successfully dumped.", path);
        }
    }

    if !config.is_json {
        if !addresses.is_empty() {
            for address in addresses.iter() {
                println!("{} was not found.", address);
            }
        }
        println!("Succeeded.");
    } else {
        println!("{{}}");
    }
    Ok(())
}

fn extract_last_trans_lt(v: &serde_json::Value) -> Option<&str> {
    v.as_object()?["last_trans_lt"].as_str()
}

pub async fn wait_for_change(
    config: &Config,
    account_address: &str,
    wait_secs: u64,
) -> Result<(), String> {
    let context = create_client_verbose(config)?;

    let query = tvm_client::net::query_collection(
        context.clone(),
        ParamsOfQueryCollection {
            collection: "accounts".to_owned(),
            filter: Some(serde_json::json!({
                "id": {
                    "eq": account_address
                }
            })),
            limit: None,
            order: None,
            result: "last_trans_lt".to_owned(),
            ..Default::default()
        },
    )
    .await
    .map_err(|e| format!("Failed to query the account: {}", e))?;

    let last_trans_lt = extract_last_trans_lt(&query.result[0])
        .ok_or_else(|| format!("Failed to parse query result: {}", query.result[0]))?;

    let (s, mut r) = tokio::sync::mpsc::channel(1);
    let s = Arc::new(s);

    let ss = s.clone();
    let callback = move |result: Result<ResultOfSubscription, ClientError>| {
        let s = ss.clone();
        async move {
            let res = match result {
                Ok(res) => {
                    if extract_last_trans_lt(&res.result).is_some() {
                        Ok(())
                    } else {
                        Err(format!("Can't parse the result: {}", res.result))
                    }
                }
                Err(e) => Err(format!("Client error: {}", e)),
            };
            s.send(res).await.unwrap()
        }
    };

    let subscription = tvm_client::net::subscribe_collection(
        context.clone(),
        ParamsOfSubscribeCollection {
            collection: "accounts".to_owned(),
            filter: Some(serde_json::json!({
                "id": {
                    "eq": account_address
                },
                "last_trans_lt": {
                    "gt": last_trans_lt
                },
            })),
            result: "last_trans_lt".to_owned(),
            ..Default::default()
        },
        callback,
    )
    .await
    .map_err(|e| format!("Failed to subscribe: {}", e))?;

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(wait_secs)).await;
        s.send(Err("Timeout".to_owned())).await.unwrap()
    });

    let res = r.recv().await.ok_or_else(|| "Sender has dropped".to_owned())?;
    tvm_client::net::unsubscribe(context.clone(), subscription)
        .await
        .map_err(|e| format!("Failed to unsubscribe: {}", e))?;

    if !config.is_json {
        if res.is_ok() {
            println!("Succeeded.");
        }
    } else {
        println!("{{}}");
    }
    res
}
