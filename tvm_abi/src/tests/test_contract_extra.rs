use serde::Deserialize;

use crate::contract::AbiVersion;
use crate::contract::SerdeEvent;
use crate::contract::SerdeFunction;
use crate::contract::deserialize_opt_u32_from_string;

#[derive(Debug, Deserialize)]
struct HexIdHolder {
    #[serde(deserialize_with = "deserialize_opt_u32_from_string")]
    value: Option<u32>,
}

#[test]
fn abi_version_parse_covers_success_and_error_paths() {
    assert_eq!(AbiVersion::parse("2.4").unwrap(), AbiVersion::from_parts(2, 4));
    assert_eq!(AbiVersion::parse("1.5.9").unwrap(), AbiVersion::from_parts(1, 5));

    let err = AbiVersion::parse("2").unwrap_err().to_string();
    assert!(err.contains("Invalid version:"));
    assert!(!err.trim().is_empty());

    let err = AbiVersion::parse("x.1").unwrap_err().to_string();
    assert!(err.contains("Invalid version:"));
    assert!(!err.trim().is_empty());

    let err = AbiVersion::parse("2.x").unwrap_err().to_string();
    assert!(err.contains("Invalid version:"));
    assert!(!err.trim().is_empty());
}

#[test]
fn deserialize_opt_u32_from_string_handles_string_and_non_string_inputs() {
    let parsed: HexIdHolder = serde_json::from_str(r#"{ "value": "0x0012ABCD" }"#).unwrap();
    assert_eq!(parsed.value, Some(0x0012ABCD));

    let parsed: HexIdHolder = serde_json::from_str(r#"{ "value": 15 }"#).unwrap();
    assert_eq!(parsed.value, None);

    let err = serde_json::from_str::<HexIdHolder>(r#"{ "value": "1234" }"#).unwrap_err();
    assert!(err.to_string().contains("number must be prefixed with 0x"));
}

#[test]
fn serde_function_and_event_deserialize_explicit_hex_ids() {
    let function: SerdeFunction = serde_json::from_str(
        r#"{
            "name": "f",
            "id": "0x89abcdef",
            "inputs": [],
            "outputs": []
        }"#,
    )
    .unwrap();
    assert_eq!(function.id, Some(0x89abcdef));

    let event: SerdeEvent = serde_json::from_str(
        r#"{
            "name": "e",
            "id": "0x01234567",
            "inputs": []
        }"#,
    )
    .unwrap();
    assert_eq!(event.id, Some(0x01234567));
}
