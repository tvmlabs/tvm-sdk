use serde_json::Value;
use tvm_types::base64_decode;
use tvm_types::base64_encode;

use super::dinterface::DebotInterface;
use super::dinterface::InterfaceResult;
use super::dinterface::decode_answer_id;
use super::dinterface::get_arg;
use crate::abi::Abi;

const ABI: &str = r#"
{
	"ABI version": 2,
	"version": "2.2",
	"header": ["time"],
	"functions": [
		{
			"name": "encode",
			"id": "0x31d9f12c",
			"inputs": [
				{"name":"answerId","type":"uint32"},
				{"name":"data","type":"bytes"}
			],
			"outputs": [
				{"name":"base64","type":"string"}
			]
		},
		{
			"name": "decode",
			"id": "0x5992a05b",
			"inputs": [
				{"name":"answerId","type":"uint32"},
				{"name":"base64","type":"string"}
			],
			"outputs": [
				{"name":"data","type":"bytes"}
			]
		},
		{
			"name": "constructor",
			"inputs": [
			],
			"outputs": [
			]
		}
	],
	"data": [
	],
	"events": [
	],
	"fields": [
		{"name":"_pubkey","type":"uint256"},
		{"name":"_timestamp","type":"uint64"},
		{"name":"_constructorFlag","type":"bool"}
	]
}
"#;

const BASE64_ID: &str = "8913b27b45267aad3ee08437e64029ac38fb59274f19adca0b23c4f957c8cfa1";

pub struct Base64Interface {}

impl Base64Interface {
    pub fn new() -> Self {
        Self {}
    }

    fn encode(&self, args: &Value) -> InterfaceResult {
        let answer_id = decode_answer_id(args)?;
        let data_to_encode = hex::decode(get_arg(args, "data")?).map_err(|e| format!("{}", e))?;
        let encoded = base64_encode(data_to_encode);
        Ok((answer_id, json!({ "base64": encoded })))
    }

    fn decode(&self, args: &Value) -> InterfaceResult {
        let answer_id = decode_answer_id(args)?;
        let str_to_decode = get_arg(args, "base64")?;
        let decoded = base64_decode(str_to_decode).map_err(|e| format!("invalid base64: {}", e))?;
        Ok((answer_id, json!({ "data": hex::encode(decoded) })))
    }
}

#[async_trait::async_trait]
impl DebotInterface for Base64Interface {
    fn get_id(&self) -> String {
        BASE64_ID.to_string()
    }

    fn get_abi(&self) -> Abi {
        Abi::Json(ABI.to_owned())
    }

    async fn call(&self, func: &str, args: &Value) -> InterfaceResult {
        match func {
            "encode" => self.encode(args),
            "decode" => self.decode(args),
            _ => Err(format!("function \"{}\" is not implemented", func)),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::debot::dinterface::DebotInterface;

    #[test]
    fn get_id_and_abi_are_stable() {
        let iface = Base64Interface::new();
        assert_eq!(iface.get_id(), BASE64_ID);

        let abi = iface.get_abi().json_string().unwrap();
        assert!(abi.contains("\"encode\""));
        assert!(abi.contains("\"decode\""));
    }

    #[tokio::test]
    async fn call_encodes_and_decodes_payload() {
        let iface = Base64Interface::new();

        let (answer_id, encoded) = iface
            .call("encode", &json!({ "answerId": "7", "data": hex::encode("hello") }))
            .await
            .unwrap();
        assert_eq!(answer_id, 7);
        assert_eq!(encoded["base64"], "aGVsbG8=");

        let (answer_id, decoded) =
            iface.call("decode", &json!({ "answerId": "8", "base64": "aGVsbG8=" })).await.unwrap();
        assert_eq!(answer_id, 8);
        assert_eq!(decoded["data"], hex::encode("hello"));
    }

    #[tokio::test]
    async fn call_reports_invalid_input_and_unknown_function() {
        let iface = Base64Interface::new();

        let err =
            iface.call("decode", &json!({ "answerId": "1", "base64": "***" })).await.unwrap_err();
        assert!(err.contains("invalid base64"));

        let err = iface.call("missing", &json!({})).await.unwrap_err();
        assert_eq!(err, "function \"missing\" is not implemented");
    }
}
