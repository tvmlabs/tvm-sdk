use api_info::ApiModule;
use tvm_types::base64_decode;
use tvm_types::base64_encode;

use super::*;
use crate::error::ClientResult;
use crate::json_interface::modules::UtilsModule;
use crate::json_interface::utils::ParamsOfCompressZstd;
use crate::json_interface::utils::ParamsOfDecompressZstd;
use crate::json_interface::utils::ResultOfCompressZstd;
use crate::json_interface::utils::ResultOfDecompressZstd;
use crate::tests::TestClient;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_utils() {
    TestClient::init_log();
    let client = TestClient::new();
    let convert_address =
        client.wrap(convert_address, UtilsModule::api(), super::conversion::convert_address_api());

    let account_id = "fcb91a3a3816d0f7b8c2c76108b8a9bc5a6b7a55bd79f8ab101c52db29232260";
    let hex = "-1:fcb91a3a3816d0f7b8c2c76108b8a9bc5a6b7a55bd79f8ab101c52db29232260";
    let hex_workchain0 = "0:fcb91a3a3816d0f7b8c2c76108b8a9bc5a6b7a55bd79f8ab101c52db29232260";
    let base64 = "Uf/8uRo6OBbQ97jCx2EIuKm8Wmt6Vb15+KsQHFLbKSMiYG+9";
    let base64url = "kf_8uRo6OBbQ97jCx2EIuKm8Wmt6Vb15-KsQHFLbKSMiYIny";

    let converted = convert_address
        .call(ParamsOfConvertAddress {
            address: account_id.into(),
            output_format: AddressStringFormat::Hex {},
        })
        .unwrap();
    assert_eq!(converted.address, hex_workchain0);

    let converted = convert_address
        .call(ParamsOfConvertAddress {
            address: account_id.into(),
            output_format: AddressStringFormat::AccountId {},
        })
        .unwrap();
    assert_eq!(converted.address, account_id);

    let converted = convert_address
        .call(ParamsOfConvertAddress {
            address: hex.into(),
            output_format: AddressStringFormat::Base64 { bounce: false, test: false, url: false },
        })
        .unwrap();
    assert_eq!(converted.address, base64);

    let converted = convert_address
        .call(ParamsOfConvertAddress {
            address: base64.into(),
            output_format: AddressStringFormat::Base64 { bounce: true, test: true, url: true },
        })
        .unwrap();
    assert_eq!(converted.address, base64url);

    let converted = convert_address
        .call(ParamsOfConvertAddress {
            address: base64url.into(),
            output_format: AddressStringFormat::Hex {},
        })
        .unwrap();
    assert_eq!(converted.address, hex);
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_calc_storage_fee() {
    let client = TestClient::new();

    let result: ResultOfCalcStorageFee = client
        .request_async(
            "utils.calc_storage_fee",
            ParamsOfCalcStorageFee {
                account: base64_encode(include_bytes!("../boc/test_data/account.boc")),
                period: 1000,
            },
        )
        .await
        .unwrap();

    assert_eq!(result.fee, "3489");
}

#[test]
fn test_compression() {
    let client = TestClient::new();
    let uncompressed =
        b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
        incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
        exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure \
        dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit \
        anim id est laborum.";

    let compressed: ResultOfCompressZstd = client
        .request(
            "utils.compress_zstd",
            ParamsOfCompressZstd { uncompressed: base64_encode(uncompressed), level: Some(21) },
        )
        .unwrap();

    assert_ne!(base64_decode(&compressed.compressed).unwrap(), uncompressed);

    let decompressed: ResultOfDecompressZstd = client
        .request(
            "utils.decompress_zstd",
            ParamsOfDecompressZstd { compressed: compressed.compressed },
        )
        .unwrap();

    let decompressed = base64_decode(decompressed.decompressed).unwrap();

    assert_eq!(decompressed, uncompressed);
}

#[test]
fn test_decompression() {
    let client = TestClient::new();
    let compressed = "KLUv/QCAdQgAJhc5GJCnsA2AIm2tVzjno88mHb3Ttx9b8fXHHDAAMgAyAMUsVo6Pi3rPTDF2WDl510aHTwt44hrUxb\
        n5oF6iUfiUiRbQhYo/PSM2WvKYt/hMIOQmuOaY/bmJQoRky46EF+cEd+Thsep5Hloo9DLCSwe1vFwcqIHycEKlMqBSo\
        +szAiIBhkukH5kSIVlFukEWNF2SkIv6HBdPjFAjoUliCPjzKB/4jK91X95rTAKoASkPNqwUEw2Gkscdb3lR8YRYOR+P\
        0sULCqzPQ8mQFJWnBSyP25mWIY2bFEUSJiGsWD+9NBqLhIAGDggQkLMbt5Y1aDR4uLKqwJXmQFPg/XTXIL7LCgspIF1\
        YYplND4Uo";

    let uncompressed =
        b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
        incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
        exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure \
        dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit \
        anim id est laborum.";

    let decompressed: ResultOfDecompressZstd = client
        .request(
            "utils.decompress_zstd",
            ParamsOfDecompressZstd { compressed: compressed.to_string() },
        )
        .unwrap();

    let decompressed = base64_decode(decompressed.decompressed).unwrap();

    assert_eq!(decompressed, uncompressed);
}

#[test]
fn test_get_address_type() {
    let client = TestClient::new();

    assert!(get_address_type(&client, "").is_err());
    assert!(get_address_type(&client, "                                  ").is_err());
    assert!(get_address_type(&client, "123456").is_err());
    assert!(get_address_type(&client, "abcdef").is_err());

    assert_eq!(
        get_address_type(
            &client,
            "-1:7777777777777777777777777777777777777777777777777777777777777777",
        )
        .unwrap(),
        AccountAddressType::Hex,
    );

    assert_eq!(
        get_address_type(
            &client,
            "0:919db8e740d50bf349df2eea03fa30c385d846b991ff5542e67098ee833fc7f7",
        )
        .unwrap(),
        AccountAddressType::Hex,
    );

    assert_eq!(
        get_address_type(
            &client,
            "7777777777777777777777777777777777777777777777777777777777777777",
        )
        .unwrap(),
        AccountAddressType::AccountId,
    );

    assert_eq!(
        get_address_type(
            &client,
            "919db8e740d50bf349df2eea03fa30c385d846b991ff5542e67098ee833fc7f7",
        )
        .unwrap(),
        AccountAddressType::AccountId,
    );

    assert_eq!(
        get_address_type(&client, "EQCRnbjnQNUL80nfLuoD+jDDhdhGuZH/VULmcJjugz/H9wam",).unwrap(),
        AccountAddressType::Base64,
    );

    assert_eq!(
        get_address_type(&client, "EQCRnbjnQNUL80nfLuoD-jDDhdhGuZH_VULmcJjugz_H9wam",).unwrap(),
        AccountAddressType::Base64,
    );

    assert_eq!(
        get_address_type(&client, "UQCRnbjnQNUL80nfLuoD+jDDhdhGuZH/VULmcJjugz/H91tj",).unwrap(),
        AccountAddressType::Base64,
    );

    assert_eq!(
        get_address_type(&client, "UQCRnbjnQNUL80nfLuoD-jDDhdhGuZH_VULmcJjugz_H91tj",).unwrap(),
        AccountAddressType::Base64,
    );
}

fn get_address_type(
    client: &TestClient,
    address: &'static str,
) -> ClientResult<AccountAddressType> {
    client
        .request::<_, ResultOfGetAddressType>(
            "utils.get_address_type",
            ParamsOfGetAddressType { address: address.to_string() },
        )
        .map(|result| result.address_type)
}

#[test]
fn json_helper_extracts_typed_fields() {
    use super::json::JsonHelper;

    let value = json!({
        "u64": 42,
        "i64": -7,
        "str": "hello",
        "array": [1, 2, 3]
    });

    assert_eq!(value.get_u64("u64").unwrap(), 42);
    assert_eq!(value.get_u32("u64").unwrap(), 42);
    assert_eq!(value.get_i64("i64").unwrap(), -7);
    assert_eq!(value.get_i32("i64").unwrap(), -7);
    assert_eq!(value.get_str("str").unwrap(), "hello");
    assert_eq!(value.get_array("array").unwrap().len(), 3);
}

#[test]
fn json_helper_reports_type_errors() {
    use super::json::JsonHelper;

    let value = json!({
        "u64": "42",
        "i64": "7",
        "str": 10,
        "array": {}
    });

    assert_eq!(
        value.get_u64("u64").unwrap_err().to_string(),
        "`u64` field must be an unsigned integer"
    );
    assert_eq!(value.get_i64("i64").unwrap_err().to_string(), "`i64` field must be an integer");
    assert_eq!(value.get_str("str").unwrap_err().to_string(), "`str` field must be a string");
    assert_eq!(value.get_array("array").unwrap_err().to_string(), "`array` field must be an array");
    assert_eq!(
        value.get_u64("missing").unwrap_err().to_string(),
        "`missing` field must be an unsigned integer"
    );
}

#[test]
fn json_helper_take_string_consumes_only_strings() {
    use super::json::JsonHelper;

    let mut value = json!("owned");
    assert_eq!(value.take_string(), Some("owned".to_owned()));
    assert_eq!(value, serde_json::Value::Null);

    let mut value = json!(42);
    assert_eq!(value.take_string(), None);
    assert_eq!(value, serde_json::Value::Null);
}

#[test]
fn utility_errors_preserve_code_and_context() {
    let compression = Error::compression_error("bad stream");
    assert_eq!(compression.code(), ErrorCode::CompressionError as u32);
    assert_eq!(compression.message(), "Compression error: bad stream");
    assert_eq!(
        compression.data()["core_version"],
        serde_json::Value::String(crate::client::core_version())
    );

    let decompression = Error::decompression_error("bad frame");
    assert_eq!(decompression.code(), ErrorCode::CompressionError as u32);
    assert_eq!(decompression.message(), "Decompression error: bad frame");
}
