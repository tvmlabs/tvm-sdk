use std::sync::Arc;

use tvm_client::{
    ClientConfig, ClientContext,
    abi::{
        AbiContract, AbiParam, ParamsOfAbiEncodeBoc, ParamsOfDecodeBoc, ResultOfAbiEncodeBoc,
        ResultOfDecodeBoc, decode_boc, encode_boc,
    },
    boc::{ParamsOfGetBocHash, ResultOfGetBocHash, get_boc_hash},
};

use crate::{
    BocDecodeArgs, BocEncodeArgs,
    helper::{get_base64_or_read_from_file, get_json_value_or_read_file, load_abi_as_string},
};

pub fn encode(args: &BocEncodeArgs) -> anyhow::Result<ResultOfAbiEncodeBoc> {
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    let params: Vec<AbiParam> = serde_json::from_str(&load_abi_as_string(&args.params)?)?;
    let data = get_json_value_or_read_file(&args.data)?;

    Ok(encode_boc(client, ParamsOfAbiEncodeBoc { params, data, boc_cache: None })?)
}

pub fn decode(args: &BocDecodeArgs) -> anyhow::Result<ResultOfDecodeBoc> {
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    let params: Vec<AbiParam> = serde_json::from_str(&load_abi_as_string(&args.params)?)?;
    let boc = get_base64_or_read_from_file(Some(&args.boc))?
        .ok_or_else(|| anyhow::anyhow!("BOC is required"))?;
    let params = ParamsOfDecodeBoc { params, boc, allow_partial: false };

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
    use serde_json::json;

    use super::*;
    #[test]
    fn test_encode_decode() {
        let params = "./tests/contract/everwallet.params.json";
        let expected_boc =
            "te6ccgEBAQEAKgAAUBBNJAZaaPnf8kV8+nQT9uegjrBVtC+/J9FK0mWWRwg2AAAAAEmWAtI=";

        // Encode
        let data = r#"{
            "_pubkey":"0x104d24065a68f9dff2457cfa7413f6e7a08eb055b42fbf27d14ad26596470836",
            "_timestamp":"1234567890"
        }"#;

        let args = BocEncodeArgs { params: params.into(), data: data.into() };
        let result = encode(&args);
        assert!(result.is_ok());
        let boc = result.unwrap().boc;
        assert_eq!(boc, expected_boc.to_string());

        // Decode
        let args = BocDecodeArgs { params: params.into(), boc };
        let result = decode(&args);
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

    #[test]
    fn test_encode_from_file() {
        let data = "./tests/contract/everwallet.data.json";
        let params = "./tests/contract/everwallet.params.json";
        let args = BocEncodeArgs { params: params.into(), data: data.into() };

        let result = encode(&args);

        assert!(result.is_ok());
        let boc = result.unwrap().boc;
        assert_eq!(
            boc,
            "te6ccgEBAQEAKgAAUBBNJAZaaPnf8kV8+nQT9uegjrBVtC+/J9FK0mWWRwg2AAAAAEmWAtI=".to_string()
        );
    }
    #[test]
    fn test_decode_from_file() {
        let boc = "./tests/contract/everwallet.boc".to_string();
        let params = "./tests/contract/everwallet.params.json";
        let args = BocDecodeArgs { boc, params: params.into() };

        let result = decode(&args);

        assert!(result.is_ok());
        let data = result.unwrap().data;
        assert_eq!(
            data,
            json!({
                "_pubkey":"0x104d24065a68f9dff2457cfa7413f6e7a08eb055b42fbf27d14ad26596470836",
                "_timestamp":"1234567890"
            })
        );
    }
}
