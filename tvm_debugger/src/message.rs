#[cfg(test)]
use std::path::PathBuf;
use std::str::FromStr;

use serde_json::Value;
#[cfg(test)]
use serde_json::json;
use tvm_abi::encode_function_call;
use tvm_block::CurrencyCollection;
use tvm_block::ExternalInboundMessageHeader;
use tvm_block::ExtraCurrencyCollection;
use tvm_block::Grams;
use tvm_block::InternalMessageHeader;
use tvm_block::Message;
use tvm_block::MsgAddressExt;
use tvm_block::MsgAddressInt;
use tvm_block::VarUInteger32;
use tvm_types::SliceData;
use tvm_types::ed25519_create_private_key;

use crate::RunArgs;
use crate::helper::get_dest_address;
use crate::helper::get_now;
use crate::helper::load_abi_as_string;
use crate::helper::read_keys;

pub(crate) fn generate_message(args: &RunArgs) -> anyhow::Result<(Message, SliceData)> {
    let body = generate_message_body(args)?;
    let message = if args.internal {
        generate_internal_message(args, Some(body.clone()))
    } else {
        generate_external_message(args, Some(body.clone()))
    }?;
    Ok((message, body))
}

pub(crate) fn generate_external_message(
    args: &RunArgs,
    body: Option<SliceData>,
) -> anyhow::Result<Message> {
    let dst = get_dest_address(args)?;
    let header = ExternalInboundMessageHeader {
        src: MsgAddressExt::with_extern(SliceData::from_raw(vec![0x55; 8], 64)).unwrap(),
        dst,
        import_fee: 0x1234u64.into(), // Taken from TVM-linker
    };
    let mut msg = Message::with_ext_in_header(header);
    if let Some(body) = body {
        msg.set_body(body);
    }
    Ok(msg)
}

pub(crate) fn generate_internal_message(
    args: &RunArgs,
    body: Option<SliceData>,
) -> anyhow::Result<Message> {
    let dst = get_dest_address(args)?;
    let src = match args.message_source.as_ref() {
        Some(address) => MsgAddressInt::from_str(address)
            .map_err(|e| anyhow::format_err!("Failed to decode message source: {e}"))?,
        None => MsgAddressInt::default(),
    };
    let grams = Grams::new(args.message_value.unwrap_or(0))
        .map_err(|e| anyhow::format_err!("Failed to setup message value: {e}"))?;
    let mut ecc = ExtraCurrencyCollection::new();
    if let Some(value) = args.message_ecc.as_ref() {
        let map_value: Value = serde_json::from_str(value)
            .map_err(|e| anyhow::format_err!("Failed to decode message ecc: {e}"))?;
        let ecc_map =
            map_value.as_object().ok_or(anyhow::format_err!("Failed to decode message ecc"))?;
        for (k, v) in ecc_map {
            let key = k.parse::<u32>()?;
            let value_str = v.as_str().ok_or(anyhow::format_err!("Failed to parse ecc value"))?;
            let value = VarUInteger32::from_str(value_str)
                .map_err(|e| anyhow::format_err!("Failed to decode ecc value: {e}"))?;
            ecc.set(&key, &value).map_err(|e| anyhow::format_err!("Failed to set ecc key: {e}"))?;
        }
    }
    let value = CurrencyCollection { grams, other: ecc };
    let mut header = InternalMessageHeader::with_addresses(src, dst, value);

    // Constants taken from TVM-linker
    header.created_lt = 1;
    header.ihr_disabled = true;
    header.created_at = get_now(args);

    let mut msg = Message::with_int_header(header);
    if let Some(body) = body {
        msg.set_body(body);
    }
    Ok(msg)
}

pub(crate) fn generate_message_body(args: &RunArgs) -> anyhow::Result<SliceData> {
    assert!(args.abi_file.is_some());
    assert!(args.function_name.is_some());
    let abi = load_abi_as_string(args.abi_file.as_ref().unwrap())?;
    let function_name = args.function_name.as_ref().unwrap();
    let header = args
        .abi_header
        .clone()
        .map(|v| serde_json::to_string(&v).unwrap_or("{}".to_string()).to_string());
    let parameters = if let Some(file) = args.call_parameters_file.as_ref() {
        let file_content = std::fs::read_to_string(file)
            .map_err(|e| anyhow::format_err!("Failed to read call parameters file: {e}"))?;
        file_content
    } else {
        args.call_parameters
            .clone()
            .map(|v| serde_json::to_string(&v).unwrap_or("{}".to_string()).to_string())
            .unwrap_or("{}".to_string())
    };
    let key = args.sign.as_ref().map(|path| {
        let keypair = read_keys(path).expect("Failed to read key pair file");
        let secret = hex::decode(&keypair.secret).expect("Failed to decode secret key");
        ed25519_create_private_key(&secret).expect("Failed to load secret key")
    });
    let body = encode_function_call(
        &abi,
        function_name,
        header.as_deref(),
        &parameters,
        args.internal,
        key.as_ref(),
        args.address.as_deref(),
    )
    .map_err(|e| anyhow::format_err!("Failed to encode function call: {e}"))?;

    let body = SliceData::load_builder(body)
        .map_err(|e| anyhow::format_err!("Failed to convert call body to slice data: {e}"))?;

    Ok(body)
}

#[test]
fn test_encode_body() -> anyhow::Result<()> {
    let mut args = RunArgs::default();
    args.abi_file = Some(PathBuf::from_str("./tests/contract/contract.abi.json").unwrap());
    args.function_name = Some("inc".to_string());
    let _body = generate_message_body(&args)?;
    args.function_name = Some("iterate".to_string());
    args.call_parameters = Some(json!({"index":35165}));
    let _body = generate_message_body(&args)?;
    args.internal = true;
    args.address =
        Some("0:1111111111111111111111111111111111111111111111111111111111111111".to_string());
    let _body = generate_message_body(&args)?;
    Ok(())
}

#[test]
fn test_generate_message() -> anyhow::Result<()> {
    let mut args = RunArgs::default();
    args.abi_file = Some(PathBuf::from_str("./tests/contract/contract.abi.json").unwrap());
    args.function_name = Some("inc".to_string());
    let _ext_message = generate_message(&args)?;
    args.function_name = Some("iterate".to_string());
    args.call_parameters = Some(json!({"index":35165}));
    args.internal = true;
    args.address =
        Some("0:1111111111111111111111111111111111111111111111111111111111111111".to_string());
    args.message_source =
        Some("0:1111111111111111111111111111111111111111111111111111111111111122".to_string());
    let _int_message = generate_message(&args)?;
    Ok(())
}
