mod message;
mod helper;
mod execute;
mod decode;

use std::path::PathBuf;
use clap::ArgAction;
use clap::Parser;

use crate::execute::execute;

lazy_static::lazy_static!(
    static ref LONG_VERSION: String = format!("{}\nBUILD_GIT_BRANCH={}\nBUILD_GIT_COMMIT={}\nBUILD_GIT_DATE={}\nBUILD_TIME={}",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_GIT_BRANCH"),
        env!("BUILD_GIT_COMMIT"),
        env!("BUILD_GIT_DATE"),
        env!("BUILD_TIME"),
    );
);

/// Helper tool, that allows you to run Acki-Nacki virtual machine, get VM trace,
/// output messages and update contract state offchain.
#[derive(Parser, Debug, Default)]
#[command(long_version = &**LONG_VERSION, about, long_about = None)]
struct Args {
    /// TVC file with contract state init
    #[arg(short, long, required(true))]
    input_file: PathBuf,

    /// Path to the contract ABI file
    #[arg(short, long)]
    abi_file: Option<PathBuf>,

    /// ABI header
    #[arg(short('r'), long)]
    abi_header: Option<serde_json::Value>,

    /// Contract function name
    #[arg(short('m'), long)]
    function_name: Option<String>,

    /// Call parameters. Must be specified as a json string
    #[arg(short('p'), long)]
    call_parameters: Option<serde_json::Value>,

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

    /// Trace VM execution
    #[arg(long, action=ArgAction::SetTrue, default_value = "false")]
    trace: bool,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    execute(&args)?;
    Ok(())
}
