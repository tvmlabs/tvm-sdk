use std::sync::Arc;

use tvm_client::{
    ClientConfig, ClientContext,
    abi::{
        AbiContract, ParamsOfAbiEncodeBoc, ParamsOfDecodeBoc, ResultOfAbiEncodeBoc,
        ResultOfDecodeBoc, decode_boc, encode_boc,
    },
    boc::{ParamsOfGetBocHash, ResultOfGetBocHash, get_boc_hash},
};

use crate::{BocDecodeArgs, BocEncodeArgs, helper::load_abi_as_string};

pub fn encode(args: &BocEncodeArgs) -> anyhow::Result<ResultOfAbiEncodeBoc> {
    println!("Enter parameters in JSON format:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    encode_inner(&input, args)
}

fn encode_inner(input: &str, args: &BocEncodeArgs) -> anyhow::Result<ResultOfAbiEncodeBoc> {
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    let abi: AbiContract = serde_json::from_str(&load_abi_as_string(&args.abi_file)?)?;
    let data = serde_json::from_str(input)?;

    let params = ParamsOfAbiEncodeBoc { params: abi.fields, data, boc_cache: None };

    Ok(encode_boc(client, params)?)
}

pub fn decode(args: &BocDecodeArgs) -> anyhow::Result<ResultOfDecodeBoc> {
    let mut input = String::new();
    println!("Enter BOC");
    std::io::stdin().read_line(&mut input)?;
    // Remove whitespace/newline characters
    input = input.trim_end().to_string();
    decode_inner(&input, args)
}
fn decode_inner(input: &str, args: &BocDecodeArgs) -> anyhow::Result<ResultOfDecodeBoc> {
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    let abi: AbiContract = serde_json::from_str(&load_abi_as_string(&args.abi_file)?)?;

    let params =
        ParamsOfDecodeBoc { params: abi.fields, boc: input.to_string(), allow_partial: false };

    Ok(decode_boc(client, params)?)
}

pub fn hash() -> anyhow::Result<ResultOfGetBocHash> {
    let mut input = String::new();
    println!("Enter BOC");
    std::io::stdin().read_line(&mut input)?;
    // Remove whitespace/newline characters
    input = input.trim_end().to_string();
    hash_inner(&input)
}
fn hash_inner(input: &str) -> anyhow::Result<ResultOfGetBocHash> {
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);

    let params = ParamsOfGetBocHash { boc: input.to_string() };

    Ok(get_boc_hash(client, params)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_decode() {
        let abi_file = "./tests/contract/everwallet.abi.json";
        let expected_boc =
            "te6ccgEBAQEAKgAAUBBNJAZaaPnf8kV8+nQT9uegjrBVtC+/J9FK0mWWRwg2AAAAAEmWAtI=";

        // Encode
        let input = r#"{
            "_pubkey":"0x104d24065a68f9dff2457cfa7413f6e7a08eb055b42fbf27d14ad26596470836",
            "_timestamp":1234567890
        }"#;
        let args = BocEncodeArgs { abi_file: abi_file.into(), abi_header: None };
        let result = encode_inner(input, &args);
        assert!(result.is_ok());
        let boc = result.unwrap().boc;
        assert_eq!(boc, expected_boc.to_string());

        // Decode
        let args = BocDecodeArgs { abi_file: abi_file.into(), abi_header: None };
        let result = decode_inner(expected_boc, &args);
        assert!(result.is_ok());
        let data = result.unwrap().data.to_string();
        assert_eq!(
            data,
            "{\"_pubkey\":\"0x104d24065a68f9dff2457cfa7413f6e7a08eb055b42fbf27d14ad26596470836\",\"_timestamp\":\"1234567890\"}"
        );

        // Hash
        let result = hash_inner(expected_boc);
        assert!(result.is_ok());
        let hash = result.unwrap().hash;
        assert_eq!(&hash, "6130dc45e6a2c5ea4334f338bd50429a75c0e430c628a01910ef2987cbd62dba");
    }
}
