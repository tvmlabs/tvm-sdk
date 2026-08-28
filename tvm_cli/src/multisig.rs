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
extern crate reqwest;

use std::str::FromStr;

use clap::Arg;
use clap::ArgMatches;
use clap::Command;
use serde_json::json;
use tvm_client::abi::Abi;
use tvm_client::abi::AbiContract;
use tvm_client::abi::AbiParam;
use tvm_client::abi::CallSet;
use tvm_client::abi::ParamsOfEncodeMessageBody;
use tvm_client::abi::encode_message_body;

use crate::call;
use crate::config::Config;
use crate::convert;
use crate::crypto::load_keypair;
use crate::deploy::prepare_deploy_message_params;
use crate::helpers::SdkAddress;
use crate::helpers::create_client_local;
use crate::helpers::create_client_verbose;
use crate::helpers::load_file_with_url;
use crate::helpers::now_ms;

const SAFEMULTISIG_LINK: &str = "https://github.com/tonlabs/ton-labs-contracts/blob/master/solidity/safemultisig/SafeMultisigWallet.tvc?raw=true";
const SETCODEMULTISIG_LINK: &str = "https://github.com/tonlabs/ton-labs-contracts/blob/master/solidity/setcodemultisig/SetcodeMultisigWallet.tvc?raw=true";
const SAFEMULTISIG_V2_LINK: &str =
    "https://github.com/EverSurf/contracts/blob/main/multisig2/build/SafeMultisig.tvc?raw=true";
const SETCODEMULTISIG_V2_LINK: &str =
    "https://github.com/EverSurf/contracts/blob/main/multisig2/build/SetcodeMultisig.tvc?raw=true";

pub const MSIG_ABI: &str = r#"{
	"ABI version": 2,
	"header": ["pubkey", "time", "expire"],
	"functions": [
		{
			"name": "constructor",
			"inputs": [
				{"name":"owners","type":"uint256[]"},
				{"name":"reqConfirms","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "acceptTransfer",
			"inputs": [
				{"name":"payload","type":"bytes"}
			],
			"outputs": [
			]
		},
		{
			"name": "sendTransaction",
			"inputs": [
				{"name":"dest","type":"address"},
				{"name":"value","type":"uint128"},
				{"name":"bounce","type":"bool"},
				{"name":"flags","type":"uint8"},
				{"name":"payload","type":"cell"}
			],
			"outputs": [
			]
		},
		{
			"name": "submitTransaction",
			"inputs": [
				{"name":"dest","type":"address"},
				{"name":"value","type":"uint128"},
				{"name":"bounce","type":"bool"},
				{"name":"allBalance","type":"bool"},
				{"name":"payload","type":"cell"}
			],
			"outputs": [
				{"name":"transId","type":"uint64"}
			]
		},
		{
			"name": "confirmTransaction",
			"inputs": [
				{"name":"transactionId","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "isConfirmed",
			"inputs": [
				{"name":"mask","type":"uint32"},
				{"name":"index","type":"uint8"}
			],
			"outputs": [
				{"name":"confirmed","type":"bool"}
			]
		},
		{
			"name": "getParameters",
			"inputs": [
			],
			"outputs": [
				{"name":"maxQueuedTransactions","type":"uint8"},
				{"name":"maxCustodianCount","type":"uint8"},
				{"name":"expirationTime","type":"uint64"},
				{"name":"minValue","type":"uint128"},
				{"name":"requiredTxnConfirms","type":"uint8"}
			]
		},
		{
			"name": "getTransaction",
			"inputs": [
				{"name":"transactionId","type":"uint64"}
			],
			"outputs": [
				{"components":[{"name":"id","type":"uint64"},{"name":"confirmationsMask","type":"uint32"},{"name":"signsRequired","type":"uint8"},{"name":"signsReceived","type":"uint8"},{"name":"creator","type":"uint256"},{"name":"index","type":"uint8"},{"name":"dest","type":"address"},{"name":"value","type":"uint128"},{"name":"sendFlags","type":"uint16"},{"name":"payload","type":"cell"},{"name":"bounce","type":"bool"}],"name":"trans","type":"tuple"}
			]
		},
		{
			"name": "getTransactions",
			"inputs": [
			],
			"outputs": [
				{"components":[{"name":"id","type":"uint64"},{"name":"confirmationsMask","type":"uint32"},{"name":"signsRequired","type":"uint8"},{"name":"signsReceived","type":"uint8"},{"name":"creator","type":"uint256"},{"name":"index","type":"uint8"},{"name":"dest","type":"address"},{"name":"value","type":"uint128"},{"name":"sendFlags","type":"uint16"},{"name":"payload","type":"cell"},{"name":"bounce","type":"bool"}],"name":"transactions","type":"tuple[]"}
			]
		},
		{
			"name": "getTransactionIds",
			"inputs": [
			],
			"outputs": [
				{"name":"ids","type":"uint64[]"}
			]
		},
		{
			"name": "getCustodians",
			"inputs": [
			],
			"outputs": [
				{"components":[{"name":"index","type":"uint8"},{"name":"pubkey","type":"uint256"}],"name":"custodians","type":"tuple[]"}
			]
		}
	],
	"data": [
	],
	"events": [
		{
			"name": "TransferAccepted",
			"inputs": [
				{"name":"payload","type":"bytes"}
			],
			"outputs": [
			]
		}
	]
}"#;

pub const TRANSFER_WITH_COMMENT: &str = r#"{
	"ABI version": 1,
	"functions": [
		{
			"name": "transfer",
			"id": "0x00000000",
			"inputs": [{"name":"comment","type":"bytes"}],
			"outputs": []
		}
	],
	"events": [],
	"data": []
}"#;

const LOCAL_GIVER_TRANSFER: &str = r#"{
	"ABI version": 1,
	"functions": [
		{
			"name": "sendGrams",
			"inputs": [
				{"name": "dest", "type": "address"},
				{"name": "amount", "type": "uint64"}
			],
			"outputs": []
		}
	],
	"events": [],
	"data": []
}"#;

const LOCAL_GIVER_ADDR: &str = "0:841288ed3b55d9cdafa806807f02a0ae0c169aa5edfe88a789a6482429756a94";

#[derive(Default)]
pub struct CallArgs {
    params: serde_json::Value,
    func_name: String,
    image: Option<Vec<u8>>,
}

impl CallArgs {
    pub async fn submit(matches: &ArgMatches) -> Result<Self, String> {
        let dest = matches
            .value_of("DEST")
            .map(|s| s.to_owned())
            .ok_or("--dst parameter is not defined".to_string())?;
        let value =
            matches.value_of("VALUE").ok_or("--value parameter is not defined".to_string())?;
        let value = convert::convert_token(value)?;
        let comment = matches.value_of("PURPOSE").map(|s| s.to_owned());
        let body = if let Some(ref txt) = comment {
            encode_transfer_body(txt).await?
        } else {
            "".to_owned()
        };
        Self::submit_with_args(matches, &dest, &value, true, body).await
    }

    pub async fn submit_with_args(
        matches: &ArgMatches,
        dest: &str,
        value: &str,
        bounce: bool,
        payload: String,
    ) -> Result<Self, String> {
        let v2 = matches.is_present("V2");
        if v2 {
            // TODO parse stateinit arg
        }
        let params = json!({
            "dest": dest,
            "value": value,
            "bounce": bounce,
            "allBalance": false,
            "payload": payload,
        });

        Ok(Self { params, func_name: "submitTransaction".to_owned(), ..Default::default() })
    }

    pub async fn deploy(matches: &ArgMatches) -> Result<Self, String> {
        let is_setcode = matches.is_present("SETCODE");
        let v2 = matches.is_present("V2");

        let target = if v2 {
            if is_setcode { SETCODEMULTISIG_V2_LINK } else { SAFEMULTISIG_V2_LINK }
        } else if is_setcode {
            SETCODEMULTISIG_LINK
        } else {
            SAFEMULTISIG_LINK
        };

        let image = load_file_with_url(target, 30000).await?;

        let owners = matches.value_of("OWNERS").map(|owners| {
            owners
                .replace(['[', ']', '\"', '\''], "")
                .replace("0x", "")
                .split(',')
                .map(|o| format!("0x{}", o))
                .collect::<Vec<String>>()
        });

        let mut params = json!({
            "owners": owners,
            "reqConfirms": matches.value_of("CONFIRMS").unwrap_or("1"),
        });

        if v2 {
            // `multisig deploy` registers no `--lifetime`, so this used to read
            // an id clap does not know -- an abort in debug builds and a
            // constant 0 everywhere else. Register the argument here first if
            // the constructor's lifetime is ever meant to be configurable.
            params["lifetime"] = json!(0);
        }

        Ok(Self { params, func_name: "constructor".to_owned(), image: Some(image) })
    }
}

/// Which arguments the running command defines for the wallet it signs with.
/// Asking clap for the other variant's id aborts in debug builds, so the
/// lookup has to follow the command that is actually running.
pub enum WalletArgs {
    /// The wallet address comes from `--addr`/`--wallet` and the key from
    /// `--sign`: `multisig send`, and the `depool` subcommands that send
    /// through the wallet -- which is all of them except `events` and
    /// `answers`, neither of which sends anything.
    AddrAndSign,
    /// The key comes from `--keys`, and there is no wallet argument at all:
    /// `multisig deploy` takes the wallet from the config file. That address
    /// is still required, because its dapp_id is what routes the deploy
    /// message; the address of the wallet being deployed is a separate value,
    /// computed from the deploy message itself.
    Keys,
}

pub struct MultisigArgs {
    addr: String,
    dapp_id: Option<String>,
    abi: Abi,
    call_args: CallArgs,
    keys: String,
}

impl WalletArgs {
    /// The wallet address and the key to sign with, from whichever arguments
    /// the running command defines. Separate from `MultisigArgs::new` so that
    /// `multisig deploy` can find out it has no wallet before downloading the
    /// wallet image.
    pub fn resolve(
        &self,
        matches: &ArgMatches,
        config: &Config,
    ) -> Result<(String, String), String> {
        let address = match self {
            WalletArgs::AddrAndSign => matches.value_of("MSIG").map(|s| s.to_owned()),
            WalletArgs::Keys => None,
        }
        .or_else(|| config.wallet.clone())
        .ok_or("multisig address is not defined".to_string())?;
        let keys = match self {
            WalletArgs::AddrAndSign => matches.value_of("SIGN"),
            WalletArgs::Keys => matches.value_of("KEYS"),
        }
        .map(|s| s.to_owned())
        .or_else(|| config.keys_path.clone())
        .ok_or("sign key is not defined".to_string())?;
        Ok((address, keys))
    }
}

impl MultisigArgs {
    pub fn new(
        matches: &ArgMatches,
        config: &Config,
        call_args: CallArgs,
        wallet_args: WalletArgs,
    ) -> Result<Self, String> {
        let (address, keys) = wallet_args.resolve(matches, config)?;
        Self::with_wallet(matches, address, keys, call_args)
    }

    /// For a caller that resolved the wallet earlier, before doing work it
    /// would rather not do without one.
    pub fn with_wallet(
        matches: &ArgMatches,
        address: String,
        keys: String,
        call_args: CallArgs,
    ) -> Result<Self, String> {
        let v2 = matches.is_present("V2");

        let sdk_addr = SdkAddress::from_str(&address)?;
        // `execute()` forwards this address to call_contract_with_result, which
        // re-parses it via SdkAddress::from_str (prepare_message_params /
        // emulate_locally). Store the full round-trippable dapp_id::account_id
        // form so the strict parser accepts it; the dapp_id is also kept
        // separately for the SDK send layer.
        let dapp_id = sdk_addr.dapp_id.clone();
        let addr = sdk_addr.to_string();
        let mut abi = serde_json::from_str::<AbiContract>(MSIG_ABI).unwrap_or_default();
        if v2 {
            abi.version = Some("2.3".to_owned());
            if let Some(f) = abi.functions.iter_mut().find(|e| &e.name == "submitTransaction") {
                f.inputs.push(AbiParam {
                    name: "stateInit".to_owned(),
                    param_type: "optional(cell)".to_owned(),
                    components: vec![],
                    init: false,
                });
            }
            if let Some(f) = abi.functions.iter_mut().find(|e| &e.name == "constructor") {
                f.inputs.push(AbiParam {
                    name: "lifetime".to_owned(),
                    param_type: "uint32".to_owned(),
                    components: vec![],
                    init: false,
                });
            }
        }

        Ok(Self { addr, dapp_id, call_args, abi: Abi::Contract(abi), keys })
    }

    pub fn address(&self) -> &str {
        &self.addr
    }

    pub fn params(&self) -> &serde_json::Value {
        &self.call_args.params
    }

    pub fn abi(&self) -> &Abi {
        &self.abi
    }

    pub fn abi_string(&self) -> String {
        if let Abi::Contract(ref abi) = self.abi {
            serde_json::to_string(abi).unwrap()
        } else {
            unreachable!();
        }
    }

    pub fn func_name(&self) -> &str {
        &self.call_args.func_name
    }

    pub fn keys(&self) -> &str {
        &self.keys
    }

    pub fn image(&self) -> Option<&[u8]> {
        self.call_args.image.as_deref()
    }

    pub async fn execute(self, config: &Config) -> Result<serde_json::Value, String> {
        call::call_contract_with_result(
            config,
            self.address(),
            &self.abi_string(),
            self.func_name(),
            &self.params().to_string(),
            Some(self.keys.clone()),
            false,
            None,
            self.dapp_id.as_ref().map(|x| x.as_str()),
        )
        .await
    }
}

pub fn create_multisig_command<'b>() -> Command<'b> {
    let v2_arg =
        Arg::new("V2").long("--v2").help("Force to interact with wallet account as multisig v2.");
    let bounce_arg = Arg::new("BOUNCE")
        .long("--bounce")
        .short('b')
        .help("Send bounce message to destination account.");

    let keys_arg = Arg::new("KEYS")
        .long("--keys")
        .short('k')
        .takes_value(true)
        .help("Path to the file with a keypair.");

    Command::new("multisig")
        .about("Multisignature wallet commands.")
        .allow_negative_numbers(true)
        .dont_collapse_args_in_usage(true)
        .subcommand(Command::new("send")
            .allow_hyphen_values(true)
            .about("Transfer funds from the wallet to the recipient.")
            .arg(Arg::new("MSIG")
                .long("--addr")
                .takes_value(true)
                .help("Wallet address. If undefined then config.wallet is used."))
            .arg(Arg::new("DEST")
                .long("--dest")
                .takes_value(true)
                .help("Recipient address."))
            .arg(Arg::new("VALUE")
                .long("--value")
                .takes_value(true)
                .help("Amount of funds to transfer (in evers)."))
            .arg(Arg::new("PURPOSE")
                .long("--purpose")
                .takes_value(true)
                .help("Optional, comment attached to transfer."))
            .arg(Arg::new("SIGN")
                .long("--sign")
                .takes_value(true)
                .help("Seed phrase or path to file with keypair."))
            .arg(bounce_arg)
            .arg(v2_arg.clone()))
        .subcommand(Command::new("deploy")
            .allow_hyphen_values(true)
            .about("Deploys a wallet with a given public key. By default, deploys a SafeMultisig with one custodian, which can be tuned with flags.")
            .arg(keys_arg)
            .arg(Arg::new("SETCODE")
                .long("--setcode")
                .help("Deploy SetcodeMultisig instead of SafeMultisig."))
            .arg(Arg::new("VALUE")
                .long("--local")
                .takes_value(true)
                .short('l')
                .help("Perform a preliminary call of local giver to initialize contract with given value."))
            .arg(Arg::new("OWNERS")
                .long("--owners")
                .takes_value(true)
                .short('o')
                .help("Array of wallet owners public keys. Note: deployer could be not included in this case. If not specified the only owner is contract deployer."))
            .arg(Arg::new("CONFIRMS")
                .long("--confirms")
                .takes_value(true)
                .short('c')
                .help("Number of confirmations required for executing transaction. Default value is 1."))
            .arg(v2_arg))
}

pub async fn multisig_command(m: &ArgMatches, config: &Config) -> Result<(), String> {
    if let Some(m) = m.subcommand_matches("send") {
        return multisig_send_command(m, config).await;
    }
    if let Some(m) = m.subcommand_matches("deploy") {
        return multisig_deploy_command(m, config).await;
    }
    Err("unknown multisig command".to_owned())
}

async fn multisig_send_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    let call_args = CallArgs::submit(matches).await?;
    let common_args = MultisigArgs::new(matches, config, call_args, WalletArgs::AddrAndSign)?;
    send(config, common_args).await
}

pub async fn encode_transfer_body(text: &str) -> Result<String, String> {
    encode_message_body(
        create_client_local()?,
        ParamsOfEncodeMessageBody {
            abi: Abi::Json(TRANSFER_WITH_COMMENT.to_owned()),
            call_set: CallSet::some_with_function_and_input(
                "transfer",
                json!({
                    "comment": hex::encode(text.as_bytes())
                }),
            )
            .ok_or("failed to create CallSet with specified parameters")?,
            is_internal: true,
            ..Default::default()
        },
    )
    .await
    .map_err(|e| format!("failed to encode transfer body: {}", e))
    .map(|r| r.body)
}

async fn send(config: &Config, args: MultisigArgs) -> Result<(), String> {
    let result = args.execute(config).await?;
    if !config.is_json {
        println!("Succeeded.");
    }
    call::print_json_result(result, config)
}

async fn multisig_deploy_command(matches: &ArgMatches, config: &Config) -> Result<(), String> {
    // Before `CallArgs::deploy`, which downloads the wallet image: a run with
    // no wallet configured, or no `--keys`, is knowable without waiting for
    // that, and this is the lookup that has to name `deploy`'s own arguments.
    let (address, keys) = WalletArgs::Keys.resolve(matches, config)?;
    let call_args = CallArgs::deploy(matches).await?;
    let args = MultisigArgs::with_wallet(matches, address, keys, call_args)?;

    let keys = load_keypair(args.keys())?;
    let mut params = args.params().clone();
    if params["owners"].is_null() {
        params["owners"] = json!(vec![format!("0x{}", &keys.public)]);
    }
    let (msg, address) = prepare_deploy_message_params(
        args.image().unwrap_or_default(),
        args.abi().clone(),
        "constructor".to_string(),
        now_ms(),
        &params.to_string(),
        Some(keys),
        config.wc,
    )
    .await?;

    if !config.is_json {
        println!("Wallet address: {}", address);
    }

    let ton = create_client_verbose(config)?;

    if let Some(value) = matches.value_of("VALUE") {
        let params = format!(r#"{{"dest":"{}","amount":"{}"}}"#, address, value);
        call::call_contract_with_client(
            ton.clone(),
            config,
            LOCAL_GIVER_ADDR,
            LOCAL_GIVER_TRANSFER,
            "sendGrams",
            &params,
            None,
            false,
            None,
            None,
        )
        .await?;
    }

    let res = call::process_message(ton.clone(), msg, config, args.dapp_id.as_deref())
        .await
        .map_err(|e| format!("{:#}", e));

    if res.is_err() {
        if res.clone().err().unwrap().contains("Account does not exist.") {
            if !config.is_json {
                println!(
                    "Your account should have initial balance for deployment. Please transfer some value to your wallet address before deploy."
                );
            } else {
                println!("{{");
                println!(
                    "  \"Error\": \"Your account should have initial balance for deployment. Please transfer some value to your wallet address before deploy.\","
                );
                println!("  \"Address\": \"{}\"", address);
                println!("}}");
            }
            return Ok(());
        }
        return Err(res.err().unwrap());
    }

    if !config.is_json {
        println!("Wallet successfully deployed");
        println!("Wallet address: {}", address);
    } else {
        println!("{{");
        println!("  \"Address\": \"{}\"", address);
        println!("}}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Command;

    use super::CallArgs;
    use super::MultisigArgs;
    use super::WalletArgs;
    use super::create_multisig_command;
    use crate::config::Config;

    const WALLET: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                          ::bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    /// `CallArgs::default()` stands in for `CallArgs::deploy()`, which would
    /// download the wallet image from GitHub. Only the arguments the wallet
    /// itself is resolved from matter here.
    fn deploy_matches() -> clap::ArgMatches {
        let matches = Command::new("tvm-cli")
            .subcommand(create_multisig_command())
            .try_get_matches_from(["tvm-cli", "multisig", "deploy", "--keys", "wallet.keys"])
            .expect("multisig deploy takes --keys");
        matches
            .subcommand_matches("multisig")
            .and_then(|m| m.subcommand_matches("deploy"))
            .expect("deploy is a multisig subcommand")
            .clone()
    }

    /// `multisig deploy` names its key `--keys` and registers neither `--addr`
    /// nor `--sign`. Reading the ids `multisig send` defines aborts, so a
    /// regression here fails by panicking rather than by returning a wrong
    /// key. The call site itself is covered from the CLI.
    #[test]
    fn multisig_deploy_takes_its_key_from_the_argument_it_defines() {
        let matches = deploy_matches();
        let config = Config { wallet: Some(WALLET.to_owned()), ..Config::default() };

        let args = MultisigArgs::new(&matches, &config, CallArgs::default(), WalletArgs::Keys)
            .expect("a configured wallet and a key are all it needs");

        assert_eq!(args.keys(), "wallet.keys");
    }

    /// The wallet address is not optional for `deploy` either: it supplies the
    /// dapp_id that routes the deploy message, and `deploy` has no argument to
    /// give it, so it has to come from the config file.
    #[test]
    fn multisig_deploy_still_needs_a_configured_wallet() {
        let result = MultisigArgs::new(
            &deploy_matches(),
            &Config::default(),
            CallArgs::default(),
            WalletArgs::Keys,
        );

        match result {
            Err(err) => assert_eq!(err, "multisig address is not defined"),
            Ok(_) => panic!("deploy resolved a wallet address out of nowhere"),
        }
    }
}
