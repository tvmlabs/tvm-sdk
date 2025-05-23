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

#![allow(clippy::from_str_radix_10)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::too_many_arguments)]

mod account;
mod call;
mod config;
mod convert;
mod crypto;
mod debot;
mod debug;
mod decode;
mod deploy;
mod depool;
mod depool_abi;
mod genaddr;
mod getconfig;
mod helpers;
mod message;
mod multisig;
mod replay;
mod run;
mod sendfile;
mod test;
mod voting;

use std::collections::BTreeMap;
use std::env;
use std::process::exit;

use account::calc_storage;
use account::get_account;
use account::wait_for_change;
use call::call_contract;
use call::call_contract_with_msg;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use clap::error::ErrorKind;
use config::Config;
use config::clear_config;
use config::set_config;
use crypto::extract_pubkey;
use crypto::generate_keypair;
use crypto::generate_mnemonic;
use debot::create_debot_command;
use debot::debot_command;
use debug::create_debug_command;
use debug::debug_command;
use decode::create_decode_command;
use decode::decode_command;
use deploy::deploy_contract;
use deploy::generate_deploy_message;
use depool::create_depool_command;
use depool::depool_command;
use genaddr::generate_address;
use getconfig::dump_blockchain_config;
use getconfig::query_global_config;
use helpers::contract_data_from_matches_or_config_alias;
use helpers::create_client_local;
use helpers::load_abi;
use helpers::load_ton_address;
use helpers::query_raw;
use multisig::create_multisig_command;
use multisig::multisig_command;
use replay::fetch_block_command;
use replay::fetch_command;
use replay::replay_command;
use serde_json::Value;
use serde_json::json;
use test::create_test_command;
use test::create_test_sign_command;
use test::test_command;
use test::test_sign_command;
use tvm_client::abi::CallSet;
use tvm_client::abi::ParamsOfEncodeMessageBody;
use voting::create_proposal;
use voting::decode_proposal;
use voting::vote;

use crate::account::dump_accounts;
use crate::config::FullConfig;
use crate::config::resolve_net_name;
use crate::getconfig::gen_update_config_message;
use crate::helpers::AccountSource;
use crate::helpers::abi_from_matches_or_config;
use crate::helpers::default_config_name;
use crate::helpers::global_config_path;
use crate::helpers::load_abi_from_tvc;
use crate::helpers::load_params;
use crate::helpers::parse_lifetime;
use crate::helpers::unpack_alternative_params;
use crate::helpers::wc_from_matches_or_config;
use crate::message::generate_message;
use crate::run::run_command;
use crate::run::run_get_method;

const DEF_MSG_LIFETIME: u32 = 30;
const DEF_STORAGE_PERIOD: u32 = 60 * 60 * 24 * 365;

lazy_static::lazy_static!(
    static ref VERSION: String = format!(
        "{}\nCOMMIT_ID: {}\nBUILD_DATE: {}\nCOMMIT_DATE: {}\nGIT_BRANCH: {}",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_GIT_COMMIT"),
        env!("BUILD_TIME"),
        env!("BUILD_GIT_DATE"),
        env!("BUILD_GIT_BRANCH")
    );
);

enum CallType {
    Call,
    Msg,
    Fee,
}

enum DeployType {
    Full,
    MsgOnly,
    Fee,
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .build()
        .expect("Can't create Engine tokio runtime");
    let result = runtime.block_on(async move { main_internal().await });
    if let Err(err_str) = result {
        if !err_str.is_empty() {
            println!("{err_str}");
        }
        exit(1)
    }
}

async fn main_internal() -> Result<(), String> {
    let version_string = env!("CARGO_PKG_VERSION");

    let abi_arg = Arg::new("ABI")
        .long("--abi")
        .num_args(0..=1)
        .help("Path or link to the contract ABI file or pure json ABI data. Can be specified in the config file.");

    let keys_arg = Arg::new("KEYS")
        .long("--keys")
        .num_args(0..=1)
        .help("Seed phrase or path to the file with keypair used to sign the message. Can be specified in the config file.");

    let sign_arg = Arg::new("SIGN")
        .long("--sign")
        .num_args(0..=1)
        .help("Seed phrase or path to the file with keypair used to sign the message. Can be specified in the config.");

    let thread_arg = Arg::new("THREAD")
        .long("--thread")
        .num_args(0..=1)
        .help("The identifier of the thread in which the message should be processed.");

    let method_opt_arg = Arg::new("METHOD")
        .num_args(1)
        .long("--method")
        .short('m')
        .help("Name of the function being called.");

    let address_opt_arg = Arg::new("ADDRESS")
        .long("--addr")
        .num_args(0..=1)
        .help("Contract address. Can be specified in the config file.");

    let multi_params_arg = Arg::new("PARAMS")
        .help("Function arguments. Must be a list of `--name value` pairs or a json string with all arguments.")
        .action(ArgAction::Append);

    let author = "TVM Labs";

    let callx_cmd = Command::new("callx")
        .about("Sends an external message with encoded function call to the contract (alternative syntax).")
        .version(version_string)
        .author(author)
        .allow_hyphen_values(true)
        .trailing_var_arg(true)
        .dont_collapse_args_in_usage(true)
        .arg(address_opt_arg.clone())
        .arg(abi_arg.clone())
        .arg(keys_arg.clone())
        .arg(method_opt_arg.clone())
        .arg(multi_params_arg.clone())
        .arg(thread_arg.clone());

    let tvc_arg = Arg::new("TVC")
        .num_args(1)
        .required(true)
        .help("Path to the compiled smart contract (tvc file).");

    let wc_arg = Arg::new("WC")
        .num_args(0..=1)
        .long("--wc")
        .allow_hyphen_values(true)
        .help("Workchain id of the smart contract (default value is taken from the config).");

    let alias_arg_long = Arg::new("ALIAS")
        .long("--alias")
        .num_args(0..=1)
        .help("Saves contract address and abi to the aliases list to be able to call this contract with alias instaed of address.");

    let deployx_cmd = Command::new("deployx")
        .about("Deploys a smart contract to the blockchain (alternative syntax).")
        .version(version_string)
        .author(author)
        .allow_hyphen_values(true)
        .trailing_var_arg(true)
        .dont_collapse_args_in_usage(true)
        .arg(abi_arg.clone())
        .arg(keys_arg.clone())
        .arg(wc_arg.clone())
        .arg(tvc_arg.clone())
        .arg(alias_arg_long.clone())
        .arg(multi_params_arg.clone());

    let address_boc_tvc_arg = Arg::new("ADDRESS").num_args(1).help(
        "Contract address or path to the saved account state if --boc or --tvc flag is specified.",
    );

    let method_arg =
        Arg::new("METHOD").required(true).num_args(1).help("Name of the function being called.");

    let boc_flag = Arg::new("BOC")
        .long("--boc")
        .conflicts_with("TVC")
        .help("Flag that changes behavior of the command to work with the saved account state (account BOC).");

    let tvc_flag = Arg::new("TVC")
        .long("--tvc")
        .conflicts_with("BOC")
        .help("Flag that changes behavior of the command to work with the saved contract state (stateInit TVC).");

    let bc_config_arg = Arg::new("BCCONFIG")
        .long("--bc_config")
        .requires("BOC")
        .num_args(0..=1)
        .help("Path to the file with blockchain config.");

    let runx_cmd = Command::new("runx")
        .about("Runs contract function locally (alternative syntax).")
        .version(version_string)
        .author(author)
        .allow_hyphen_values(true)
        .trailing_var_arg(true)
        .dont_collapse_args_in_usage(true)
        .arg(address_boc_tvc_arg.clone().long("--addr"))
        .arg(abi_arg.clone())
        .arg(method_opt_arg.clone())
        .arg(multi_params_arg.clone())
        .arg(boc_flag.clone())
        .arg(tvc_flag.clone())
        .arg(bc_config_arg.clone());

    let runget_cmd = Command::new("runget")
        .about("Runs get-method of a FIFT contract.")
        .allow_hyphen_values(true)
        .trailing_var_arg(true)
        .dont_collapse_args_in_usage(true)
        .arg(Arg::new("ADDRESS")
            .required(true)
            .help("Contract address or path to the saved account state if --boc or --tvc flag is specified."))
        .arg(Arg::new("METHOD")
            .required(true)
            .help("Name of the function being called."))
        .arg(Arg::new("PARAMS")
            .help("Function arguments.")
            .action(ArgAction::Append))
        .arg(boc_flag.clone())
        .arg(tvc_flag.clone())
        .arg(bc_config_arg.clone());

    let version_cmd = Command::new("version").about("Prints build and version info.");

    let genphrase_cmd = Command::new("genphrase")
        .about("Generates a seed phrase for keypair.")
        .version(version_string)
        .author(author)
        .arg(
            Arg::new("DUMP_KEYPAIR")
                .long("--dump")
                .num_args(0..=1)
                .help("Path where to dump keypair generated from the phrase"),
        );

    let genpubkey_cmd = Command::new("genpubkey")
        .about("Generates a public key from the seed phrase.")
        .version(version_string)
        .author(author)
        .arg(
            Arg::new("PHRASE")
                .num_args(1)
                .required(true)
                .help("Seed phrase (12 words). Should be specified in quotes."),
        );

    let getkeypair_cmd = Command::new("getkeypair")
        .about("Generates a keypair from the seed phrase or private key and saves it to the file.")
        .version(version_string)
        .author(author)
        .arg(Arg::new("KEY_FILE")
            .num_args(0..=1)
            .long("--output")
            .short('o')
            .help("Path to the file where to store the keypair."))
        .arg(Arg::new("PHRASE")
            .num_args(0..=1)
            .long("--phrase")
            .short('p')
            .help("Seed phrase (12 words) or secret (private) key. Seed phrase should be specified in quotes, secret key as 64 hex chars."));

    let genaddr_cmd = Command::new("genaddr")
        .allow_negative_numbers(true)
        .about("Calculates smart contract address in different formats. By default, input tvc file isn't modified.")
        .version(version_string)
        .author(author)
        .arg(tvc_arg.clone())
        .arg(abi_arg.clone())
        .arg(wc_arg.clone())
        .arg(Arg::new("GENKEY")
            .num_args(0..=1)
            .long("--genkey")
            .conflicts_with("SETKEY")
            .help("Path to the file, where a new generated keypair for the contract will be saved."))
        .arg(Arg::new("SETKEY")
            .num_args(0..=1)
            .long("--setkey")
            .conflicts_with("GENKEY")
            .help("Seed phrase or path to the file with keypair."))
        .arg(Arg::new("DATA")
            .num_args(0..=1)
            .long("--data")
            .help("Initial data to insert into the contract. Should be specified in json format."))
        .arg(Arg::new("SAVE")
            .long("--save")
            .help("If this flag is specified, modifies the tvc file with the keypair and initial data"));

    let deploy_cmd = Command::new("deploy")
        .allow_negative_numbers(true)
        .allow_hyphen_values(true)
        .about("Deploys a smart contract to the blockchain.")
        .version(version_string)
        .author(author)
        .arg(tvc_arg.clone())
        .arg(Arg::new("PARAMS").required(true).help(
            "Constructor arguments. Can be specified with a filename, which contains json data.",
        ))
        .arg(abi_arg.clone())
        .arg(sign_arg.clone())
        .arg(keys_arg.clone())
        .arg(wc_arg.clone());

    let output_arg = Arg::new("OUTPUT")
        .short('o')
        .long("--output")
        .num_args(0..=1)
        .help("Path to the file where to store the message.");

    let raw_arg = Arg::new("RAW").long("--raw").help("Creates raw message boc.");

    let deploy_message_cmd = deploy_cmd
        .clone()
        .name("deploy_message")
        .about("Generates a signed message to deploy a smart contract to the blockchain.")
        .arg(output_arg.clone())
        .arg(raw_arg.clone());

    let address_arg = Arg::new("ADDRESS").required(true).help("Contract address.");

    let params_arg = Arg::new("PARAMS")
        .required(true)
        .help("Function arguments. Can be specified with a filename, which contains json data.");

    let call_cmd = Command::new("call")
        .allow_hyphen_values(true)
        .about("Sends an external message with encoded function call to the contract.")
        .version(version_string)
        .author(author)
        .arg(address_arg.clone())
        .arg(method_arg.clone())
        .arg(params_arg.clone())
        .arg(abi_arg.clone())
        .arg(keys_arg.clone())
        .arg(sign_arg.clone())
        .arg(thread_arg.clone());

    let send_cmd = Command::new("send")
        .about("Sends a prepared message to the contract.")
        .version(version_string)
        .author(author)
        .arg(
            Arg::new("MESSAGE")
                .required(true)
                .help("Message to send. Message data should be specified in quotes."),
        )
        .arg(abi_arg.clone());

    let message_cmd = Command::new("message")
        .allow_hyphen_values(true)
        .about("Generates a signed message with encoded function call.")
        .version(version_string)
        .author(author)
        .arg(address_arg.clone())
        .arg(method_arg.clone())
        .arg(params_arg.clone())
        .arg(abi_arg.clone())
        .arg(keys_arg.clone())
        .arg(sign_arg.clone())
        .arg(
            Arg::new("LIFETIME")
                .long("--lifetime")
                .num_args(0..=1)
                .help("Period of time in seconds while message is valid."),
        )
        .arg(
            Arg::new("TIMESTAMP")
                .long("--time")
                .num_args(0..=1)
                .help("Message creation time in milliseconds. If not specified, `now` is used."),
        )
        .arg(output_arg.clone())
        .arg(raw_arg.clone());

    let body_cmd = Command::new("body")
        .allow_hyphen_values(true)
        .about("Generates a payload for internal function call.")
        .version(version_string)
        .author(author)
        .arg(method_arg.clone())
        .arg(params_arg.clone())
        .arg(abi_arg.clone());

    let sign_cmd =
        create_test_sign_command().author(author).version(version_string).arg(keys_arg.clone());

    let run_cmd = Command::new("run")
        .allow_hyphen_values(true)
        .about("Runs contract function locally.")
        .version(version_string)
        .author(author)
        .arg(address_boc_tvc_arg.clone().required(true))
        .arg(method_arg.clone())
        .arg(params_arg.clone())
        .arg(abi_arg.clone())
        .arg(boc_flag.clone())
        .arg(tvc_flag.clone())
        .arg(bc_config_arg.clone());

    let config_clear_cmd = Command::new("clear")
        .allow_hyphen_values(true)
        .about("Resets certain default values for options in the config file. Resets all values if used without options.")
        .arg(Arg::new("URL")
            .long("--url")
            .help("Url to connect."))
        .arg(Arg::new("ABI")
            .long("--abi")
            .help("Path or link to the contract ABI file or pure json ABI data."))
        .arg(keys_arg.clone())
        .arg(Arg::new("ADDR")
            .long("--addr")
            .help("Contract address."))
        .arg(Arg::new("METHOD")
            .long("--method")
            .help("Method name that can be saved to be used by some commands (runx, callx)."))
        .arg(Arg::new("PARAMETERS")
            .long("--parameters")
            .help("Function parameters that can be saved to be used by some commands (runx, callx)."))
        .arg(Arg::new("WALLET")
            .long("--wallet")
            .help("Multisig wallet address."))
        .arg(Arg::new("PUBKEY")
            .long("--pubkey")
            .help("User public key. Used by DeBot Browser."))
        .arg(Arg::new("WC")
            .long("--wc")
            .help("Workchain id."))
        .arg(Arg::new("RETRIES")
            .long("--retries")
            .help("Number of attempts to call smart contract function if previous attempt was unsuccessful."))
        .arg(Arg::new("TIMEOUT")
            .long("--timeout")
            .help("Network `wait_for` timeout in ms."))
        .arg(Arg::new("MSG_TIMEOUT")
            .long("--message_processing_timeout")
            .help("Network message processing timeout in ms."))
        .arg(Arg::new("DEPOOL_FEE")
            .long("--depool_fee")
            .help("Value added to the message sent to depool to cover it's fees (change will be returned)."))
        .arg(Arg::new("LIFETIME")
            .long("--lifetime")
            .help("Period of time in seconds while message is valid. Change of this parameter may affect \"out_of_sync\" parameter, because \"lifetime\" should be at least 2 times greater than \"out_of_sync\"."))
        .arg(Arg::new("NO_ANSWER")
            .long("--no-answer")
            .help("Flag whether to wait for depool answer when calling a depool function."))
        .arg(Arg::new("BALANCE_IN_VMSHELLS")
            .long("--balance_in_vmshells")
            .help("Print balance for account command in vmshells. If false balance is printed in nanovmshells."))
        .arg(Arg::new("LOCAL_RUN")
            .long("--local_run")
            .help("Enable preliminary local run before deploy and call commands."))
        .arg(Arg::new("ASYNC_CALL")
            .long("--async_call")
            .help("Disables wait for transaction to appear in the network after call command."))
        .arg(Arg::new("DEBUG_FAIL")
            .long("--debug_fail")
            .help("When enabled tonos-cli executes debug command on fail of run or call command. Can be enabled with values 'full' or 'minimal' which set the trace level for debug run and disabled with value 'none'."))
        .arg(Arg::new("OUT_OF_SYNC")
            .long("--out_of_sync")
            .help("Network connection \"out_of_sync_threshold\" parameter in seconds. Mind that it cant exceed half of the \"lifetime\" parameter."))
        .arg(Arg::new("IS_JSON")
            .long("--is_json")
            .help("Cli prints output in json format."))
        .arg(Arg::new("PROJECT_ID")
            .long("--project_id")
            .help("Project Id in Evercloud (dashboard.evercloud.dev)."))
        .arg(Arg::new("ACCESS_KEY")
            .long("--access_key")
            .help("Project secret or JWT in Evercloud (dashboard.evercloud.dev)."));

    let alias_arg = Arg::new("ALIAS").required(true).help("Alias name.");
    let alias_cmd = Command::new("alias")
        .about("Commands to work with aliases map")
        .subcommand(
            Command::new("add")
                .about("Add alias to the aliases map.")
                .arg(alias_arg.clone())
                .arg(Arg::new("ADDRESS").long("--addr").num_args(0..=1).help("Contract address."))
                .arg(keys_arg.clone())
                .arg(
                    Arg::new("ABI")
                        .long("--abi")
                        .num_args(0..=1)
                        .help("Path or link to the contract ABI file or pure json ABI data."),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove alias from the aliases map.")
                .arg(alias_arg.clone()),
        )
        .subcommand(Command::new("print").about("Print the aliases map."))
        .subcommand(Command::new("reset").about("Clear the aliases map."));

    let url_arg = Arg::new("URL").required(true).help("Url of the endpoints list.");
    let config_endpoint_cmd =
        Command::new("endpoint")
            .about("Commands to work with the endpoints map.")
            .subcommand(Command::new("add").about("Add endpoints list.").arg(url_arg.clone()).arg(
                Arg::new("ENDPOINTS").required(true).help("List of endpoints (comma separated)."),
            ))
            .subcommand(Command::new("remove").about("Remove endpoints list.").arg(url_arg.clone()))
            .subcommand(Command::new("reset").about("Reset the endpoints map."))
            .subcommand(Command::new("print").about("Print current endpoints map."));

    let config_cmd = Command::new("config")
        .allow_hyphen_values(true)
        .about("Allows to tune certain default values for options in the config file.")
        .version(version_string)
        .author(author)
        .arg(Arg::new("GLOBAL")
            .long("--global")
            .short('g')
            .help("Change parameters of the global config which contains default values for ordinary configs."))
        .arg(Arg::new("URL")
            .long("--url")
            .num_args(0..=1)
            .help("Url to connect."))
        .arg(Arg::new("ABI")
            .long("--abi")
            .num_args(0..=1)
            .help("Path or link to the contract ABI file or pure json ABI data."))
        .arg(keys_arg.clone())
        .arg(Arg::new("ADDR")
            .long("--addr")
            .num_args(0..=1)
            .help("Contract address."))
        .arg(Arg::new("METHOD")
            .long("--method")
            .num_args(0..=1)
            .help("Method name that can be saved to be used by some commands (runx, callx)."))
        .arg(Arg::new("PARAMETERS")
            .long("--parameters")
            .num_args(0..=1)
            .help("Function parameters that can be saved to be used by some commands (runx, callx)."))
        .arg(Arg::new("WALLET")
            .long("--wallet")
            .num_args(0..=1)
            .help("Multisig wallet address."))
        .arg(Arg::new("PUBKEY")
            .long("--pubkey")
            .num_args(0..=1)
            .help("User public key. Used by DeBot Browser."))
        .arg(Arg::new("WC")
            .long("--wc")
            .num_args(0..=1)
            .help("Workchain id."))
        .arg(Arg::new("RETRIES")
            .long("--retries")
            .num_args(0..=1)
            .help("Number of attempts to call smart contract function if previous attempt was unsuccessful."))
        .arg(Arg::new("TIMEOUT")
            .long("--timeout")
            .num_args(0..=1)
            .help("Network `wait_for` timeout in ms."))
        .arg(Arg::new("MSG_TIMEOUT")
            .long("--message_processing_timeout")
            .num_args(0..=1)
            .help("Network message processing timeout in ms."))
        .arg(Arg::new("LIST")
            .long("--list")
            .conflicts_with_all(["OUT_OF_SYNC", "NO_ANSWER","DEBUG_FAIL", "ASYNC_CALL", "LOCAL_RUN", "BALANCE_IN_VMSHELLS", "LIFETIME", "DEPOOL_FEE", "PUBKEY", "URL", "ABI", "KEYS", "ADDR", "RETRIES", "TIMEOUT", "WC", "WALLET"])
            .help("Prints all config parameters."))
        .arg(Arg::new("DEPOOL_FEE")
            .long("--depool_fee")
            .num_args(0..=1)
            .help("Value added to the message sent to depool to cover it's fees (change will be returned)."))
        .arg(Arg::new("LIFETIME")
            .long("--lifetime")
            .num_args(0..=1)
            .help("Period of time in seconds while message is valid. Change of this parameter may affect \"out_of_sync\" parameter, because \"lifetime\" should be at least 2 times greater than \"out_of_sync\"."))
        .arg(Arg::new("NO_ANSWER")
            .long("--no-answer")
            .num_args(0..=1)
            .help("Flag whether to wait for depool answer when calling a depool function."))
        .arg(Arg::new("BALANCE_IN_VMSHELLS")
            .long("--balance_in_vmshells")
            .num_args(0..=1)
            .help("Print balance for account command in vmshells. If false balance is printed in nanovmshells."))
        .arg(Arg::new("LOCAL_RUN")
            .long("--local_run")
            .num_args(0..=1)
            .help("Enable preliminary local run before deploy and call commands."))
        .arg(Arg::new("ASYNC_CALL")
            .long("--async_call")
            .num_args(0..=1)
            .help("Disables wait for transaction to appear in the network after call command."))
        .arg(Arg::new("DEBUG_FAIL")
            .long("--debug_fail")
            .num_args(0..=1)
            .help("When enabled tonos-cli executes debug command on fail of run or call command. Can be enabled with values 'full' or 'minimal' which set the trace level for debug run and disabled with value 'none'."))
        .arg(Arg::new("OUT_OF_SYNC")
            .long("--out_of_sync")
            .num_args(0..=1)
            .help("Network connection \"out_of_sync_threshold\" parameter in seconds. Mind that it cant exceed half of the \"lifetime\" parameter."))
        .arg(Arg::new("IS_JSON")
            .long("--is_json")
            .num_args(0..=1)
            .help("Cli prints output in json format."))
        .arg(Arg::new("PROJECT_ID")
            .long("--project_id")
            .num_args(0..=1)
            .help("Project Id in Evercloud (dashboard.evercloud.dev)."))
        .arg(Arg::new("ACCESS_KEY")
            .long("--access_key")
            .num_args(0..=1)
            .help("Project secret or JWT in Evercloud (dashboard.evercloud.dev)."))
        .subcommand(config_clear_cmd)
        .subcommand(config_endpoint_cmd)
        .subcommand(alias_cmd);

    let account_cmd = Command::new("account")
        .allow_hyphen_values(true)
        .about("Obtains and prints account information.")
        .version(version_string)
        .author(author)
        .arg(boc_flag.clone())
        .arg(Arg::new("ADDRESS")
            .required(true)
            .help("List of addresses or file paths (if flag --boc is used).")
            .num_args(1..))
        .arg(Arg::new("DUMPTVC")
            .long("--dumptvc")
            .short('d')
            .num_args(0..=1)
            .conflicts_with("DUMPBOC")
            .help("Dumps account StateInit to the specified tvc file. Works only if one address was given."))
        .arg(Arg::new("DUMPBOC")
            .long("--dumpboc")
            .short('b')
            .num_args(0..=1)
            .conflicts_with("DUMPTVC")
            .conflicts_with("BOC")
            .help("Dumps the whole account state boc to the specified file. Works only if one address was given. Use 'tonos-cli dump account` to dump several accounts."));

    let account_wait_cmd = Command::new("account-wait")
        .allow_hyphen_values(true)
        .about("Waits for account change (based on last_trans_lt).")
        .version(version_string)
        .author(author)
        .arg(address_arg.clone())
        .arg(
            Arg::new("TIMEOUT")
                .long("--timeout")
                .num_args(0..=1)
                .help("Timeout in seconds (default value is 30)."),
        );

    let query_raw = Command::new("query-raw")
        .about("Executes a raw GraphQL query.")
        .version(version_string)
        .author(author)
        .arg(Arg::new("COLLECTION").required(true).help("Collection to query."))
        .arg(Arg::new("RESULT").required(true).help("Result fields to print."))
        .arg(Arg::new("FILTER").long("--filter").num_args(0..=1).help("Query filter parameter."))
        .arg(Arg::new("LIMIT").long("--limit").num_args(0..=1).help("Query limit parameter."))
        .arg(Arg::new("ORDER").long("--order").num_args(0..=1).help("Query order parameter."));

    let fee_cmd = Command::new("fee")
        .about("Calculates fees for executing message or account storage fee.")
        .subcommand(
            Command::new("storage")
                .allow_hyphen_values(true)
                .about("Gets account storage fee for specified period in nanovmshells.")
                .version(version_string)
                .author(author)
                .arg(address_arg.clone())
                .arg(
                    Arg::new("PERIOD")
                        .long("--period")
                        .short('p')
                        .num_args(0..=1)
                        .help("Time period in seconds (default value is 1 year)."),
                ),
        )
        .subcommand(deploy_cmd.clone().about(
            "Executes deploy locally, calculates fees and prints table of fees in nanovmshells.",
        ))
        .subcommand(call_cmd.clone().about(
            "Executes call locally, calculates fees and prints table of all fees in nanovmshells.",
        ));

    let proposal_cmd = Command::new("proposal")
        .override_help("Proposal control commands.")
        .subcommand(
            Command::new("create")
                .about("Submits a proposal transaction in the multisignature wallet with a text comment.")
                .arg(address_arg.clone().help("Address of the multisignature wallet."))
                .arg(Arg::new("DEST")
                    .required(true)
                    .help("Address of the proposal contract."))
                .arg(Arg::new("COMMENT")
                    .required(true)
                    .help("Proposal description (max symbols 382)."))
                .arg(keys_arg.clone())
                .arg(Arg::new("OFFLINE")
                    .short('f')
                    .long("--offline")
                    .help("Prints signed message to terminal instead of sending it."))
                .arg(Arg::new("LIFETIME")
                    .short('l')
                    .long("--lifetime")
                    .num_args(0..=1)
                    .help("Period of time in seconds while message is valid.")))
        .subcommand(
            Command::new("vote")
                .about("Confirms a proposal transaction in the multisignature wallet.")
                .arg(address_arg.clone().help("Address of the multisignature wallet."))
                .arg(Arg::new("ID")
                    .required(true)
                    .help("Proposal transaction id."))
                .arg(keys_arg.clone())
                .arg(Arg::new("OFFLINE")
                    .short('f')
                    .long("--offline")
                    .help("Prints signed message to terminal instead of sending it."))
                .arg(Arg::new("LIFETIME")
                    .short('l')
                    .long("--lifetime")
                    .num_args(0..=1)
                    .help("Period of time in seconds while message is valid.")))
        .subcommand(
            Command::new("decode")
                .about("Prints a comment string from the proposal transaction.")
                .arg(address_arg.clone().help("Address of the multisignature wallet."))
                .arg(Arg::new("ID")
                    .required(true)
                    .help("Proposal transaction id.")));

    let getconfig_cmd =
        Command::new("getconfig")
            .about("Reads the global configuration parameter with defined index.")
            .arg(Arg::new("INDEX").num_args(0..=1).help(
                "Parameter index. If not specified, command will print all config parameters.",
            ));

    let update_config_param_cmd = Command::new("update_config")
        .about("Generates message with update of config params.")
        .arg(abi_arg.clone())
        .arg(Arg::new("SEQNO").num_args(0..=1).help("Current seqno from config contract"))
        .arg(Arg::new("CONFIG_MASTER_KEY_FILE").num_args(0..=1).help("path to config-master files"))
        .arg(Arg::new("NEW_PARAM_FILE").num_args(0..=1).help("New config param value"));

    let bcconfig_cmd = Command::new("dump")
        .about("Commands to dump network entities.")
        .version(version_string)
        .author(author)
        .subcommand(
            Command::new("config")
                .about("Dumps the blockchain config for the last key block.")
                .arg(
                    Arg::new("PATH")
                        .required(true)
                        .help("Path to the file where to save the blockchain config."),
                ),
        )
        .subcommand(
            Command::new("account")
                .about("Dumps state of given accounts.")
                .allow_hyphen_values(true)
                .arg(Arg::new("ADDRESS").required(true).help("List of addresses.").num_args(1..))
                .arg(Arg::new("PATH").num_args(0..=1).long("--path").short('p').help(
                    "Path to folder where to store the dumped accounts. Default value is \".\".",
                )),
        );

    let nodeid_cmd = Command::new("nodeid")
        .about("Calculates node ID from the validator public key")
        .arg(Arg::new("KEY").long("--pubkey").num_args(0..=1).help("Validator public key."))
        .arg(
            Arg::new("KEY_PAIR")
                .long("--keypair")
                .num_args(0..=1)
                .help("Validator seed phrase or path to the file with keypair."),
        );

    let sendfile_cmd = Command::new("sendfile")
        .about("Sends the boc file with an external inbound message to account.")
        .arg(Arg::new("BOC").required(true).help("Message boc file."));

    let fetch_block_cmd = Command::new("fetch-block")
        .about("Fetches a block.")
        .arg(Arg::new("BLOCKID").required(true).help("Block ID."))
        .arg(Arg::new("OUTPUT").required(true).help("Output file name"));

    let fetch_cmd = Command::new("fetch")
        .about("Fetches account's zerostate and transactions.")
        .allow_hyphen_values(true)
        .arg(address_arg.clone().help("Account address to fetch zerostate and txns for."))
        .arg(Arg::new("OUTPUT").required(true).help("Output file name"));

    let replay_cmd = Command::new("replay")
        .about("Replays account's transactions starting from zerostate.")
        .arg(Arg::new("CONFIG_TXNS")
            .long("--config")
            .short('c')
            .num_args(0..=1)
            .help("File containing zerostate and txns of -1:555..5 account.")
            .conflicts_with("DEFAULT_CONFIG"))
        .arg(Arg::new("INPUT_TXNS")
            .required(true)
            .help("File containing zerostate and txns of the account to replay."))
        .arg(Arg::new("TXNID")
            .required(true)
            .help("Dump account state before this transaction ID and stop replaying."))
        .arg(Arg::new("DEFAULT_CONFIG")
            .help("Replay transaction with current network config or default if it is not available.")
            .long("--default_config")
            .short('e')
            .conflicts_with("CONFIG_TXNS"));

    let matches = Command::new("tonos_cli")
        .version(VERSION.as_str())
        .author(author)
        .about("TVMLabs console tool for TVM networks")
        .arg(
            Arg::new("NETWORK")
                .help("Network to connect.")
                .short('u')
                .long("--url")
                .num_args(0..=1),
        )
        .arg(
            Arg::new("CONFIG")
                .help("Path to the tonos-cli configuration file.")
                .short('c')
                .long("--config")
                .num_args(0..=1),
        )
        .arg(Arg::new("JSON").help("Cli prints output in json format.").short('j').long("--json"))
        .subcommand(version_cmd)
        .subcommand(genphrase_cmd)
        .subcommand(genpubkey_cmd)
        .subcommand(getkeypair_cmd)
        .subcommand(genaddr_cmd)
        .subcommand(deploy_cmd.arg(alias_arg_long.clone()))
        .subcommand(deploy_message_cmd)
        .subcommand(call_cmd)
        .subcommand(send_cmd)
        .subcommand(message_cmd)
        .subcommand(body_cmd)
        .subcommand(sign_cmd)
        .subcommand(run_cmd)
        .subcommand(runget_cmd)
        .subcommand(config_cmd)
        .subcommand(account_cmd)
        .subcommand(account_wait_cmd)
        .subcommand(query_raw)
        .subcommand(fee_cmd)
        .subcommand(proposal_cmd)
        .subcommand(create_multisig_command())
        .subcommand(create_depool_command())
        .subcommand(create_decode_command())
        .subcommand(create_debot_command())
        .subcommand(create_debug_command())
        .subcommand(create_test_command())
        .subcommand(getconfig_cmd)
        .subcommand(bcconfig_cmd)
        .subcommand(nodeid_cmd)
        .subcommand(sendfile_cmd)
        .subcommand(fetch_block_cmd)
        .subcommand(fetch_cmd)
        .subcommand(replay_cmd)
        .subcommand(callx_cmd)
        .subcommand(deployx_cmd)
        .subcommand(runx_cmd)
        .subcommand(update_config_param_cmd)
        .subcommand_required(true);

    let matches = matches.try_get_matches().map_err(|e| match e.kind() {
        ErrorKind::DisplayVersion => {
            println!();
            exit(0);
        }
        ErrorKind::DisplayHelp => {
            println!("{e}");
            exit(0);
        }
        _ => {
            eprintln!("{e}");
            format!("{:#}", json!({"Error": e.to_string()}))
        }
    })?;

    let is_json = matches.contains_id("JSON");

    command_parser(&matches, is_json).await.map_err(|e| {
        if e.is_empty() {
            e
        } else if is_json {
            let e = serde_json::from_str(&e).unwrap_or(Value::String(e));
            format!("{:#}", json!({"Error": e}))
        } else {
            format!("Error: {e}")
        }
    })
}

async fn command_parser(matches: &ArgMatches, is_json: bool) -> Result<(), String> {
    let config_file = matches
        .get_one::<String>("CONFIG")
        .map(|v| v.to_string())
        .or(env::var("TONOSCLI_CONFIG").ok())
        .unwrap_or(default_config_name());

    let mut full_config = FullConfig::from_file(&config_file);

    if let Some(m) = matches.subcommand_matches("config") {
        return config_command(m, full_config, is_json);
    }

    full_config.config.is_json |= is_json;
    let config = &mut full_config.config;

    if let Some(url) = matches.get_one::<String>("NETWORK") {
        let resolved_url = resolve_net_name(url).unwrap_or(url.to_owned());
        let empty: Vec<String> = Vec::new();
        config.endpoints = full_config.endpoints_map.get(&resolved_url).unwrap_or(&empty).clone();
        config.url = resolved_url;
    }

    if let Some(m) = matches.subcommand_matches("callx") {
        return callx_command(m, &full_config).await;
    }
    if let Some(m) = matches.subcommand_matches("runx") {
        return run_command(m, &full_config, true).await;
    }
    if let Some(m) = matches.subcommand_matches("deployx") {
        return deployx_command(m, &mut full_config).await;
    }
    if let Some(m) = matches.subcommand_matches("call") {
        return call_command(m, config, CallType::Call).await;
    }
    if let Some(m) = matches.subcommand_matches("run") {
        return run_command(m, &full_config, false).await;
    }
    if let Some(m) = matches.subcommand_matches("runget") {
        return runget_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("body") {
        return body_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("sign") {
        return test_sign_command(m, config);
    }
    if let Some(m) = matches.subcommand_matches("message") {
        return call_command(m, config, CallType::Msg).await;
    }
    if let Some(m) = matches.subcommand_matches("send") {
        return send_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("deploy") {
        return deploy_command(m, &mut full_config, DeployType::Full).await;
    }
    if let Some(m) = matches.subcommand_matches("deploy_message") {
        return deploy_command(m, &mut full_config, DeployType::MsgOnly).await;
    }
    if let Some(m) = matches.subcommand_matches("genaddr") {
        return genaddr_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("getkeypair") {
        return getkeypair_command(m, config);
    }
    if let Some(m) = matches.subcommand_matches("account") {
        return account_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("fee") {
        if let Some(m) = m.subcommand_matches("storage") {
            return storage_command(m, config).await;
        }
        if let Some(m) = m.subcommand_matches("deploy") {
            return deploy_command(m, &mut full_config, DeployType::Fee).await;
        }
        if let Some(m) = m.subcommand_matches("call") {
            return call_command(m, config, CallType::Fee).await;
        }
    }
    if let Some(m) = matches.subcommand_matches("genphrase") {
        return genphrase_command(m, config);
    }
    if let Some(m) = matches.subcommand_matches("genpubkey") {
        return genpubkey_command(m, config);
    }
    if let Some(m) = matches.subcommand_matches("proposal") {
        if let Some(m) = m.subcommand_matches("create") {
            return proposal_create_command(m, config).await;
        }
        if let Some(m) = m.subcommand_matches("vote") {
            return proposal_vote_command(m, config).await;
        }
        if let Some(m) = m.subcommand_matches("decode") {
            return proposal_decode_command(m, config).await;
        }
    }
    if let Some(m) = matches.subcommand_matches("multisig") {
        return multisig_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("depool") {
        return depool_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("getconfig") {
        return getconfig_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("update_config") {
        return update_config_command(m, config).await;
    }
    if let Some(matches) = matches.subcommand_matches("dump") {
        if let Some(m) = matches.subcommand_matches("config") {
            return dump_bc_config_command(m, config).await;
        }
        if let Some(m) = matches.subcommand_matches("account") {
            return dump_accounts_command(m, config).await;
        }
    }
    if let Some(m) = matches.subcommand_matches("account-wait") {
        return account_wait_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("query-raw") {
        return query_raw_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("nodeid") {
        return nodeid_command(m, config);
    }
    if let Some(m) = matches.subcommand_matches("sendfile") {
        return sendfile_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("decode") {
        return decode_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("debug") {
        return debug_command(m, &full_config).await;
    }
    if let Some(m) = matches.subcommand_matches("debot") {
        return debot_command(m, config.to_owned()).await;
    }
    if let Some(m) = matches.subcommand_matches("fetch-block") {
        return fetch_block_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("fetch") {
        return fetch_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("replay") {
        return replay_command(m, config).await;
    }
    if let Some(m) = matches.subcommand_matches("test") {
        return test_command(m, &full_config).await;
    }

    if matches.subcommand_matches("version").is_some() {
        if config.is_json {
            println!("{{");
            println!(r#"  "tonos-cli": "{}","#, env!("CARGO_PKG_VERSION"));
            println!(r#"  "COMMIT_ID": "{}","#, env!("BUILD_GIT_COMMIT"));
            println!(r#"  "BUILD_DATE": "{}","#, env!("BUILD_TIME"));
            println!(r#"  "COMMIT_DATE": "{}","#, env!("BUILD_GIT_DATE"));
            println!(r#"  "GIT_BRANCH": "{}""#, env!("BUILD_GIT_BRANCH"));
            println!("}}");
        } else {
            println!(
                "tonos-cli {}\nCOMMIT_ID: {}\nBUILD_DATE: {}\nCOMMIT_DATE: {}\nGIT_BRANCH: {}",
                env!("CARGO_PKG_VERSION"),
                env!("BUILD_GIT_COMMIT"),
                env!("BUILD_TIME"),
                env!("BUILD_GIT_DATE"),
                env!("BUILD_GIT_BRANCH")
            );
        }
        return Ok(());
    }
    Err("invalid arguments".to_string())
}

fn genphrase_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    generate_mnemonic(matches.get_one::<String>("DUMP_KEYPAIR").map(|s| s.as_str()), config)
}

fn genpubkey_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let mnemonic = matches.get_one::<String>("PHRASE").unwrap();
    extract_pubkey(mnemonic, config.is_json)
}

fn getkeypair_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let key_file = matches.get_one::<String>("KEY_FILE").map(|s| s.as_str());
    let phrase = matches.get_one::<String>("PHRASE").map(|s| s.as_str());
    if !config.is_json {
        print_args!(key_file, phrase);
    }
    generate_keypair(key_file, phrase, config)
}

async fn send_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let message = matches.get_one::<String>("MESSAGE");
    let abi = Some(abi_from_matches_or_config(matches, config)?);

    if !config.is_json {
        print_args!(message, abi);
    }

    call_contract_with_msg(config, message.unwrap().to_owned(), &abi.unwrap()).await
}

async fn body_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let method = matches.get_one::<String>("METHOD");
    let params = matches.get_one::<String>("PARAMS");
    let output = matches.get_one::<String>("OUTPUT");
    let abi = Some(abi_from_matches_or_config(matches, config)?);
    let params = Some(load_params(params.unwrap())?);
    if !config.is_json {
        print_args!(method, params, abi, output);
    }

    let params = serde_json::from_str(&params.unwrap())
        .map_err(|e| format!("arguments are not in json format: {e}"))?;

    let client = create_client_local()?;
    let body = tvm_client::abi::encode_message_body(
        client.clone(),
        ParamsOfEncodeMessageBody {
            abi: load_abi(abi.as_ref().unwrap(), config).await?,
            call_set: CallSet::some_with_function_and_input(method.unwrap(), params)
                .ok_or("failed to create CallSet with specified parameters.")?,
            is_internal: true,
            ..Default::default()
        },
    )
    .await
    .map_err(|e| format!("failed to encode body: {e}"))
    .map(|r| r.body)?;

    if !config.is_json {
        println!("Message body: {body}");
    } else {
        println!("{{");
        println!("  \"Message\": \"{body}\"");
        println!("}}");
    }

    Ok(())
}

async fn call_command(matches: &ArgMatches, config: &Config, call: CallType) -> Result<(), String> {
    let address = matches.get_one::<String>("ADDRESS");
    let method = matches.get_one::<String>("METHOD");
    let params = matches.get_one::<String>("PARAMS");
    let lifetime = matches.get_one::<String>("LIFETIME");
    let raw = matches.contains_id("RAW");
    let output = matches.get_one::<String>("OUTPUT").map(|s| s.as_str());

    let abi = Some(abi_from_matches_or_config(matches, config)?);

    let keys = matches
        .get_one::<String>("KEYS")
        .or(matches.get_one::<String>("SIGN"))
        .map(|s| s.to_string())
        .or(config.keys_path.clone());

    let thread_id = matches.get_one::<String>("THREAD").map(|s| s.as_str());

    let params = Some(load_params(params.unwrap())?);
    if !config.is_json {
        print_args!(address, method, params, abi, keys, lifetime, output);
    }
    let address = load_ton_address(address.unwrap(), config)?;

    match call {
        CallType::Call | CallType::Fee => {
            let is_fee = matches!(call, CallType::Fee);
            call_contract(
                config,
                address.as_str(),
                &abi.unwrap(),
                method.unwrap(),
                &params.unwrap(),
                keys,
                is_fee,
                thread_id,
            )
            .await
        }
        CallType::Msg => {
            let lifetime = lifetime
                .map(|val| {
                    u32::from_str_radix(val, 10)
                        .map_err(|e| format!("Failed to parse lifetime: {e}"))
                })
                .transpose()?
                .unwrap_or(DEF_MSG_LIFETIME);
            let timestamp = matches
                .get_one::<String>("TIMESTAMP")
                .map(|val| {
                    u64::from_str_radix(val, 10)
                        .map_err(|e| format!("Failed to parse timestamp: {e}"))
                })
                .transpose()?;
            generate_message(
                config,
                address.as_str(),
                &abi.unwrap(),
                method.unwrap(),
                &params.unwrap(),
                keys,
                lifetime,
                raw,
                output,
                timestamp,
            )
            .await
        }
    }
}

async fn callx_command(matches: &ArgMatches, full_config: &FullConfig) -> Result<(), String> {
    let config = &full_config.config;
    let method = Some(
        matches
            .get_one::<String>("METHOD")
            .map(|s| s.as_str())
            .or(config.method.as_deref())
            .ok_or("Method is not defined. Supply it in the config file or command line.")?,
    );
    let (address, abi, keys) = contract_data_from_matches_or_config_alias(matches, full_config)?;
    let params =
        unpack_alternative_params(matches, abi.as_ref().unwrap(), method.unwrap(), config).await?;
    let params = Some(load_params(&params)?);
    let thread_id = matches.get_one::<String>("THREAD").map(|s| s.as_str());

    if !config.is_json {
        print_args!(address, method, params, abi, keys);
    }

    let address = load_ton_address(address.unwrap().as_str(), config)?;

    call_contract(
        config,
        address.as_str(),
        &abi.unwrap(),
        method.unwrap(),
        &params.unwrap(),
        keys,
        false,
        thread_id,
    )
    .await
}

async fn runget_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let address = matches.get_one::<String>("ADDRESS");
    let method = matches.get_one::<String>("METHOD");
    let params = matches.get_many::<String>("PARAMS");
    let params = params.map(|values| json!(values.collect::<Vec<_>>()).to_string());
    if !config.is_json {
        print_args!(address, method, params);
    }
    let source_type = if matches.contains_id("TVC") {
        AccountSource::TVC
    } else if matches.contains_id("BOC") {
        AccountSource::BOC
    } else {
        AccountSource::NETWORK
    };
    let address = if source_type != AccountSource::NETWORK {
        address.unwrap().to_string()
    } else {
        load_ton_address(address.unwrap(), config)?
    };
    let bc_config = matches.get_one::<String>("BCCONFIG").map(|s| s.as_str());
    run_get_method(config, &address, method.unwrap(), params, source_type, bc_config).await
}

async fn deploy_command(
    matches: &ArgMatches,
    full_config: &mut FullConfig,
    deploy_type: DeployType,
) -> Result<(), String> {
    let config = &full_config.config;
    let tvc = matches.get_one::<String>("TVC");
    let wc = wc_from_matches_or_config(matches, config)?;
    let raw = matches.contains_id("RAW");
    let output = matches.get_one::<String>("OUTPUT").map(|s| s.as_str());
    let abi = Some(abi_from_matches_or_config(matches, config)?);
    let keys = matches
        .get_one::<String>("KEYS")
        .or(matches.get_one::<String>("SIGN"))
        .map(|s| s.to_string())
        .or(config.keys_path.clone());
    let alias = matches.get_one::<String>("ALIAS").map(|s| s.as_str());
    let params = Some(
        unpack_alternative_params(matches, abi.as_ref().unwrap(), "constructor", config).await?,
    );
    if !config.is_json {
        let opt_wc = Some(format!("{wc}"));
        print_args!(tvc, params, abi, keys, opt_wc, alias);
    }
    match deploy_type {
        DeployType::Full => {
            deploy_contract(
                full_config,
                tvc.unwrap(),
                &abi.unwrap(),
                &params.unwrap(),
                keys,
                wc,
                false,
                alias,
            )
            .await
        }
        DeployType::MsgOnly => {
            generate_deploy_message(
                tvc.unwrap(),
                &abi.unwrap(),
                &params.unwrap(),
                keys,
                wc,
                raw,
                output,
                config,
            )
            .await
        }
        DeployType::Fee => {
            deploy_contract(
                full_config,
                tvc.unwrap(),
                &abi.unwrap(),
                &params.unwrap(),
                keys,
                wc,
                true,
                None,
            )
            .await
        }
    }
}

async fn deployx_command(matches: &ArgMatches, full_config: &mut FullConfig) -> Result<(), String> {
    let config = &full_config.config;
    let tvc = matches.get_one::<String>("TVC");
    let wc = wc_from_matches_or_config(matches, config)?;
    let abi = Some(abi_from_matches_or_config(matches, config)?);
    let params = Some(
        unpack_alternative_params(matches, abi.as_ref().unwrap(), "constructor", config).await?,
    );
    let keys =
        matches.get_one::<String>("KEYS").map(|s| s.to_string()).or(config.keys_path.clone());

    let alias = matches.get_one::<String>("ALIAS").map(|s| s.as_str());
    if !config.is_json {
        let opt_wc = Some(format!("{wc}"));
        print_args!(tvc, params, abi, keys, opt_wc, alias);
    }
    deploy_contract(
        full_config,
        tvc.unwrap(),
        &abi.unwrap(),
        &params.unwrap(),
        keys,
        wc,
        false,
        alias,
    )
    .await
}

fn config_command(
    matches: &ArgMatches,
    mut full_config: FullConfig,
    is_json: bool,
) -> Result<(), String> {
    let mut result = Ok(());
    if matches.contains_id("GLOBAL") {
        full_config = FullConfig::from_file(&global_config_path());
    }
    if !matches.contains_id("LIST") {
        if let Some(clear_matches) = matches.subcommand_matches("clear") {
            result = clear_config(&mut full_config, clear_matches, is_json);
        } else if let Some(endpoint_matches) = matches.subcommand_matches("endpoint") {
            if let Some(endpoint_matches) = endpoint_matches.subcommand_matches("add") {
                let url = endpoint_matches.get_one::<String>("URL").unwrap();
                let endpoints = endpoint_matches.get_one::<String>("ENDPOINTS").unwrap();
                FullConfig::add_endpoint(full_config.path.as_str(), url, endpoints)?;
            } else if let Some(endpoint_matches) = endpoint_matches.subcommand_matches("remove") {
                let url = endpoint_matches.get_one::<String>("URL").unwrap();
                FullConfig::remove_endpoint(full_config.path.as_str(), url)?;
            } else if endpoint_matches.subcommand_matches("reset").is_some() {
                FullConfig::reset_endpoints(full_config.path.as_str())?;
            }
            FullConfig::print_endpoints(full_config.path.as_str());
            return Ok(());
        } else if let Some(alias_matches) = matches.subcommand_matches("alias") {
            if let Some(alias_matches) = alias_matches.subcommand_matches("add") {
                full_config.add_alias(
                    alias_matches.get_one::<String>("ALIAS").map(|s| s.as_str()).unwrap(),
                    alias_matches.get_one::<String>("ADDRESS").map(String::from),
                    alias_matches.get_one::<String>("ABI").map(String::from),
                    alias_matches.get_one::<String>("KEYS").map(String::from),
                )?
            } else if let Some(alias_matches) = alias_matches.subcommand_matches("remove") {
                full_config.remove_alias(
                    alias_matches.get_one::<String>("ALIAS").map(|s| s.as_str()).unwrap(),
                )?
            } else if alias_matches.subcommand_matches("reset").is_some() {
                full_config.aliases = BTreeMap::new();
                full_config.to_file(&full_config.path)?;
            }
            full_config.print_aliases();
            return Ok(());
        } else {
            if !matches.args_present() {
                return Err("At least one option must be specified".to_string());
            }

            result = set_config(&mut full_config, matches, is_json);
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&full_config.config)
            .map_err(|e| format!("failed to print config parameters: {e}"))?
    );
    result
}

async fn genaddr_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let tvc = matches.get_one::<String>("TVC");
    let wc = matches.get_one::<String>("WC").map(|s| s.as_str());
    let keys = matches
        .get_one::<String>("GENKEY")
        .map(|s| s.as_str())
        .or(matches.get_one::<String>("SETKEY").map(|s| s.as_str()));
    let new_keys = matches.contains_id("GENKEY");
    let init_data = matches.get_one::<String>("DATA").map(|s| s.as_str());
    let update_tvc = matches.contains_id("SAVE");
    let abi = match abi_from_matches_or_config(matches, config) {
        Ok(abi) => Some(abi),
        Err(err) => match load_abi_from_tvc(tvc.unwrap()) {
            Some(abi) => Some(abi),
            None => return Err(err),
        },
    };
    let is_update_tvc = if update_tvc { Some("true") } else { None };
    if !config.is_json {
        print_args!(tvc, abi, wc, keys, init_data, is_update_tvc);
    }
    generate_address(config, tvc.unwrap(), &abi.unwrap(), wc, keys, new_keys, init_data, update_tvc)
        .await
}

async fn account_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let addresses_list = matches
        .get_many::<String>("ADDRESS")
        .map(|val| val.collect::<Vec<_>>())
        .or(config.addr.as_ref().map(|addr| vec![addr]))
        .ok_or(
            "Address was not found. It must be specified as option or in the config file."
                .to_string(),
        )?;
    if addresses_list.len() > 1
        && (matches.contains_id("DUMPTVC") || matches.contains_id("DUMPTVC"))
    {
        return Err("`DUMPTVC` and `DUMPBOC` options are not applicable to a list of addresses."
            .to_string());
    }
    let is_boc = matches.contains_id("BOC");
    let mut formatted_list = vec![];
    for address in addresses_list.iter() {
        if !is_boc {
            let formatted = load_ton_address(address, config)?;
            formatted_list.push(formatted);
        } else {
            if !std::path::Path::new(address).exists() {
                return Err(format!("File {address} doesn't exist."));
            }
            formatted_list.push(address.to_string());
        }
    }
    let tvcname = matches.get_one::<String>("DUMPTVC").map(|s| s.as_str());
    let bocname = matches.get_one::<String>("DUMPBOC").map(|s| s.as_str());
    let addresses = Some(formatted_list.join(", "));
    if !config.is_json {
        print_args!(addresses);
    }
    get_account(config, formatted_list, tvcname, bocname, is_boc).await
}

async fn dump_accounts_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let addresses_list = matches.get_many::<String>("ADDRESS").unwrap().collect::<Vec<_>>();
    let mut formatted_list = vec![];
    for address in addresses_list.iter() {
        let formatted = load_ton_address(address, config)?;
        formatted_list.push(formatted);
    }
    let path = matches.get_one::<String>("PATH").map(|s| s.as_str());
    let addresses = Some(formatted_list.join(", "));
    if !config.is_json {
        print_args!(addresses, path);
    }
    dump_accounts(config, formatted_list, path).await
}

async fn account_wait_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let address = matches.get_one::<String>("ADDRESS").unwrap();
    let address = load_ton_address(address, config)?;
    let timeout = matches
        .get_one::<String>("TIMEOUT")
        .map(|s| s.as_str())
        .unwrap_or("30")
        .parse::<u64>()
        .map_err(|e| format!("failed to parse timeout: {e}"))?;
    wait_for_change(config, &address, timeout).await
}

async fn query_raw_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let collection = matches.get_one::<String>("COLLECTION").unwrap();
    let filter = matches.get_one::<String>("FILTER").map(|s| s.as_str());
    let limit = matches.get_one::<String>("LIMIT").map(|s| s.as_str());
    let order = matches.get_one::<String>("ORDER").map(|s| s.as_str());
    let result = matches.get_one::<String>("RESULT").unwrap();
    query_raw(config, collection, filter, limit, order, result).await
}

async fn storage_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let address = matches.get_one::<String>("ADDRESS");
    let period = matches.get_one::<String>("PERIOD");
    if !config.is_json {
        print_args!(address, period);
    }
    let address = load_ton_address(address.unwrap(), config)?;
    let period = period
        .map(|val| u32::from_str_radix(val, 10).map_err(|e| format!("failed to parse period: {e}")))
        .transpose()?
        .unwrap_or(DEF_STORAGE_PERIOD);
    calc_storage(config, address.as_str(), period).await
}

async fn proposal_create_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let address = matches.get_one::<String>("ADDRESS");
    let dest = matches.get_one::<String>("DEST");
    let keys = matches.get_one::<String>("KEYS").map(|s| s.as_str());
    let comment = matches.get_one::<String>("COMMENT");
    let lifetime = matches.get_one::<String>("LIFETIME").map(|s| s.as_str());
    let offline = matches.contains_id("OFFLINE");
    if !config.is_json {
        print_args!(address, comment, keys, lifetime);
    }
    let address = load_ton_address(address.unwrap(), config)?;
    let lifetime = parse_lifetime(lifetime, config)?;

    create_proposal(
        config,
        address.as_str(),
        keys,
        dest.unwrap(),
        comment.unwrap(),
        lifetime,
        offline,
    )
    .await
}

async fn proposal_vote_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let address = matches.get_one::<String>("ADDRESS");
    let keys = matches.get_one::<String>("KEYS").map(|s| s.as_str());
    let id = matches.get_one::<String>("ID");
    let lifetime = matches.get_one::<String>("LIFETIME").map(|s| s.as_str());
    let offline = matches.contains_id("OFFLINE");
    if !config.is_json {
        print_args!(address, id, keys, lifetime);
    }
    let address = load_ton_address(address.unwrap(), config)?;
    let lifetime = parse_lifetime(lifetime, config)?;

    vote(config, address.as_str(), keys, id.unwrap(), lifetime, offline).await?;
    println!("{{}}");
    Ok(())
}

async fn proposal_decode_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let address = matches.get_one::<String>("ADDRESS");
    let id = matches.get_one::<String>("ID");
    if !config.is_json {
        print_args!(address, id);
    }
    let address = load_ton_address(address.unwrap(), config)?;
    decode_proposal(config, address.as_str(), id.unwrap()).await
}

async fn getconfig_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let index = matches.get_one::<String>("INDEX").map(|s| s.as_str());
    if !config.is_json {
        print_args!(index);
    }
    query_global_config(config, index).await
}

async fn update_config_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let abi = matches.get_one::<String>("ABI").map(|s| s.as_str());
    let seqno = matches.get_one::<String>("SEQNO").map(|s| s.as_str());
    let config_master = matches.get_one::<String>("CONFIG_MASTER_KEY_FILE");
    let new_param = matches.get_one::<String>("NEW_PARAM_FILE");
    if !config.is_json {
        print_args!(seqno, config_master, new_param);
    }
    gen_update_config_message(
        abi,
        seqno,
        config_master.unwrap(),
        new_param.unwrap(),
        config.is_json,
    )
    .await
}

async fn dump_bc_config_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let path = matches.get_one::<String>("PATH");
    if !config.is_json {
        print_args!(path);
    }
    dump_blockchain_config(config, path.unwrap()).await
}

fn nodeid_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let key = matches.get_one::<String>("KEY").map(|s| s.as_str());
    let keypair = matches.get_one::<String>("KEY_PAIR").map(|s| s.as_str());
    if !config.is_json {
        print_args!(key, keypair);
    }
    let nodeid = if let Some(key) = key {
        let vec = hex::decode(key).map_err(|e| format!("failed to decode public key: {e}"))?;
        convert::nodeid_from_pubkey(&vec)?
    } else if let Some(pair) = keypair {
        let pair = crypto::load_keypair(pair)?;
        convert::nodeid_from_pubkey(
            &hex::decode(&pair.public).map_err(|e| format!("failed to decode public key: {e}"))?,
        )?
    } else {
        return Err("Either public key or key pair parameter should be provided".to_owned());
    };
    if !config.is_json {
        println!("{nodeid}");
    } else {
        println!("{{");
        println!("  \"nodeid\": \"{nodeid}\"");
        println!("}}");
    }
    Ok(())
}

async fn sendfile_command(m: &ArgMatches, config: &Config) -> Result<(), String> {
    let boc = m.get_one::<String>("BOC");
    if !config.is_json {
        print_args!(boc);
    }
    sendfile::sendfile(config, boc.unwrap()).await
}
