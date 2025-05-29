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
mod callbacks;
mod interfaces;
mod pipechain;
mod processor;
pub mod term_browser;
mod term_encryption_box;
mod term_signing_box;

use callbacks::Callbacks;
use clap::Arg;
use clap::ArgMatches;
use clap::Command;
pub use interfaces::dinterface::SupportedInterfaces;
use pipechain::ApproveKind;
use pipechain::ChainLink;
use pipechain::PipeChain;
use processor::ChainProcessor;
use processor::ProcessorError;
use simplelog::*;
use term_browser::action_input;
use term_browser::input;
use term_browser::run_debot_browser;
use term_browser::terminal_input;

use crate::config::Config;
use crate::helpers::load_ton_address;

pub fn create_debot_command<'b>() -> Command {
    Command::new("debot")
        .about("Debot commands.")
        .allow_hyphen_values(true)
        .trailing_var_arg(true)
        .dont_collapse_args_in_usage(true)
        .arg(Arg::new("DEBUG").long("debug").short('d'))
        .subcommand(
            Command::new("fetch")
                .allow_hyphen_values(true)
                .arg(Arg::new("ADDRESS").required(true).help("DeBot TON address.")),
        )
        .subcommand(
            Command::new("start")
                .allow_hyphen_values(true)
                .arg(Arg::new("ADDRESS").required(true).help("DeBot TON address."))
                .arg(
                    Arg::new("PIPECHAIN")
                        .short('m')
                        .long("pipechain")
                        .num_args(1)
                        .help("Path to the DeBot Manifest."),
                )
                .arg(
                    Arg::new("SIGNKEY")
                        .short('s')
                        .long("signkey")
                        .num_args(1)
                        .help("Define keypair to auto sign transactions."),
                ),
        )
        .subcommand(
            Command::new("invoke")
                .allow_hyphen_values(true)
                .arg(Arg::new("ADDRESS").required(true).help("Debot TON address."))
                .arg(
                    Arg::new("MESSAGE")
                        .required(true)
                        .help("Message to DeBot encoded as base64/base64url."),
                ),
        )
}

pub async fn debot_command(m: &ArgMatches, config: Config) -> Result<(), String> {
    let debug = m.contains_id("DEBUG");
    let log_conf = ConfigBuilder::new()
        .add_filter_ignore_str("executor")
        .add_filter_ignore_str("hyper")
        .add_filter_ignore_str("reqwest")
        .build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![];
    let file = std::fs::File::create("debot_err.log");
    if file.is_ok() {
        loggers.push(WriteLogger::new(LevelFilter::Error, log_conf.clone(), file.unwrap()));
    }

    if debug {
        loggers.push(TermLogger::new(
            LevelFilter::Debug,
            log_conf.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ));
    }
    CombinedLogger::init(loggers).unwrap();

    if let Some(m) = m.subcommand_matches("fetch") {
        return fetch_command(m, config).await;
    }
    if let Some(m) = m.subcommand_matches("start") {
        return fetch_command(m, config).await;
    }
    if let Some(m) = m.subcommand_matches("invoke") {
        return invoke_command(m, config);
    }
    Err("unknown debot command".to_owned())
}

async fn fetch_command(m: &ArgMatches, config: Config) -> Result<(), String> {
    let addr = m.get_one::<String>("ADDRESS");
    let pipechain = m.get_one::<String>("PIPECHAIN");
    let signkey_path =
        m.get_one::<String>("SIGNKEY").map(|x| x.to_owned()).or(config.keys_path.clone());
    let is_json = config.is_json;
    let pipechain = if let Some(filename) = pipechain {
        let manifest_raw = std::fs::read_to_string(filename)
            .map_err(|e| format!("failed to read pipechain: {e}"))?;
        serde_json::from_str(&manifest_raw)
            .map_err(|e| format!("failed to parse pipechain: {e}"))?
    } else {
        PipeChain::new()
    };
    let addr = load_ton_address(addr.unwrap(), &config)?;
    let result = run_debot_browser(addr.as_str(), config, pipechain, signkey_path).await;
    match result {
        Ok(Some(arg)) => {
            if !is_json {
                println!("Returned value:");
            }
            println!("{arg:#}");
            Ok(())
        }
        Err(err) if err.contains("NoMoreChainlinks") => Ok(()),
        result => result.map(|_| ()),
    }
}

fn invoke_command(m: &ArgMatches, config: Config) -> Result<(), String> {
    let addr = m.get_one::<String>("ADDRESS");
    load_ton_address(addr.unwrap(), &config)?;
    let _ = m.get_one::<String>("MESSAGE").unwrap().to_owned();
    Ok(())
}
