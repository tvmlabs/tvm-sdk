// Copyright 2018-2021 TON Labs LTD.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

#![allow(dead_code)]

use std::str::FromStr;

use num_bigint::BigInt;
use num_traits::cast::NumCast;
use tvm_block::MsgAddressInt;
use tvm_types::Cell;
use tvm_types::SliceData;

use crate::client;
use crate::crypto::internal::tvm_crc16;
use crate::error::ClientResult;

//------------------------------------------------------------------------------------------------------

pub(crate) fn account_encode(value: &MsgAddressInt) -> String {
    value.to_string()
}

#[derive(Serialize, Deserialize, Debug, ApiType, Clone, PartialEq, Eq)]
pub enum AccountAddressType {
    AccountId,
    Hex,
    Base64,
}

#[derive(Serialize, Deserialize, Debug, ApiType, Default, Clone)]
pub struct Base64AddressParams {
    pub url: bool,
    pub test: bool,
    pub bounce: bool,
}

pub(crate) fn account_encode_ex(
    value: &MsgAddressInt,
    addr_type: AccountAddressType,
    base64_params: Option<Base64AddressParams>,
) -> ClientResult<String> {
    match addr_type {
        AccountAddressType::AccountId => Ok(format!("{:x}", value.get_address())),
        AccountAddressType::Hex => Ok(value.to_string()),
        AccountAddressType::Base64 => {
            let params =
                base64_params.ok_or(client::Error::contracts_address_conversion_failed(
                    "No base64 address parameters provided",
                ))?;
            encode_base64(value, params.bounce, params.test, params.url)
        }
    }
}

/// Parsed extended address with optional dapp namespace.
pub(crate) struct DAppAddress {
    /// 64-char hex dapp identifier, or `None` for legacy addresses.
    pub dapp_id: Option<String>,
    /// 64-char hex account identifier (no workchain prefix).
    pub account_hex: String,
}

fn validate_hex64(s: &str, field: &str, original: &str) -> ClientResult<()> {
    if s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        Err(client::Error::invalid_address(
            format!("{} must be 64 hex characters", field),
            original,
        ))
    } else {
        Ok(())
    }
}

/// Parse an address string that may carry a dapp namespace prefix.
///
/// Accepted formats (in detection order):
/// 1. `dapp_hex64::account_hex64` — separator format
/// 2. 128-char all-hex string — compact format (first 64 = dapp, last 64 = account)
/// 3. Anything else — legacy (`0:account_hex` or bare `account_hex`)
pub(crate) fn parse_dapp_address(input: &str) -> ClientResult<DAppAddress> {
    if let Some((dapp, account)) = input.split_once("::") {
        validate_hex64(dapp, "dapp_id", input)?;
        validate_hex64(account, "account_id", input)?;
        Ok(DAppAddress { dapp_id: Some(dapp.to_string()), account_hex: account.to_string() })
    } else if input.len() == 128 && input.chars().all(|c| c.is_ascii_hexdigit()) {
        let (dapp, account) = input.split_at(64);
        Ok(DAppAddress { dapp_id: Some(dapp.to_string()), account_hex: account.to_string() })
    } else {
        let account = input.strip_prefix("0:").unwrap_or(input);
        Ok(DAppAddress { dapp_id: None, account_hex: account.to_string() })
    }
}

/// Format an extended address string from dapp and account identifiers.
pub fn format_dapp_address(dapp_id: &str, account_hex: &str) -> String {
    format!("{}::{}", dapp_id, account_hex)
}

pub(crate) fn account_decode(string: &str) -> ClientResult<MsgAddressInt> {
    // Strip dapp prefix from extended format before parsing TVM address.
    let tvm_part: &str = if let Some((_, account)) = string.split_once("::") {
        account
    } else if string.len() == 128 && string.chars().all(|c| c.is_ascii_hexdigit()) {
        &string[64..]
    } else {
        string
    };
    // Bare 64-char hex needs a workchain prefix for MsgAddressInt::from_str.
    let owned;
    let addr_str: &str =
        if tvm_part.len() == 64 && !tvm_part.contains(':') && tvm_part.chars().all(|c| c.is_ascii_hexdigit()) {
            owned = format!("0:{}", tvm_part);
            &owned
        } else {
            tvm_part
        };
    match MsgAddressInt::from_str(addr_str) {
        Ok(address) => Ok(address),
        Err(_) if tvm_part.len() == 48 => decode_std_base64(tvm_part),
        Err(err) => Err(client::Error::invalid_address(err, string)),
    }
}

pub(crate) fn decode_std_base64(data: &str) -> ClientResult<MsgAddressInt> {
    // conversion from base64url
    let data = data.replace('_', "/").replace('-', "+");

    let vec = base64_decode(&data).map_err(|err| client::Error::invalid_address(err, &data))?;

    // check CRC and address tag
    let crc = tvm_crc16(&vec[..34]).to_be_bytes();

    if crc != vec[34..36] || vec[0] & 0x3f != 0x11 {
        return Err(client::Error::invalid_address("CRC mismatch", &data));
    };

    MsgAddressInt::with_standart(None, vec[1] as i8, SliceData::from_raw(vec[2..34].to_vec(), 256))
        .map_err(|err| client::Error::invalid_address(err, &data))
}

fn encode_base64(
    address: &MsgAddressInt,
    bounceable: bool,
    test: bool,
    as_url: bool,
) -> ClientResult<String> {
    if let MsgAddressInt::AddrStd(address) = address {
        let mut tag = if bounceable { 0x11 } else { 0x51 };
        if test {
            tag |= 0x80
        };
        let mut vec = vec![tag];
        vec.extend_from_slice(&address.workchain_id.to_be_bytes());
        vec.append(&mut address.address.get_bytestring(0));

        let crc = tvm_crc16(&vec);
        vec.extend_from_slice(&crc.to_be_bytes());

        let result = tvm_types::base64_encode(&vec);

        if as_url { Ok(result.replace('/', "_").replace('+', "-")) } else { Ok(result) }
    } else {
        Err(client::Error::invalid_address("Non-std address", &address.to_string()))
    }
}

pub(crate) fn hex_decode(hex: &str) -> ClientResult<Vec<u8>> {
    if hex.starts_with('x') || hex.starts_with('X') {
        hex_decode(&hex[1..])
    } else if hex.starts_with("0x") || hex.starts_with("0X") {
        hex_decode(&hex[2..])
    } else {
        hex::decode(hex).map_err(|err| client::Error::invalid_hex(hex, err))
    }
}

pub(crate) fn base64_decode(base64: &str) -> ClientResult<Vec<u8>> {
    tvm_types::base64_decode(base64).map_err(|err| client::Error::invalid_base64(base64, err))
}

pub(crate) fn long_num_to_json_string(num: u64) -> String {
    format!("0x{:x}", num)
}

pub fn decode_abi_bigint(string: &str) -> ClientResult<BigInt> {
    let result = if string.starts_with("-0x") || string.starts_with("-0X") {
        BigInt::parse_bytes(&string.as_bytes()[3..], 16).map(|number| -number)
    } else if string.starts_with("0x") || string.starts_with("0X") {
        BigInt::parse_bytes(&string.as_bytes()[2..], 16)
    } else {
        BigInt::parse_bytes(string.as_bytes(), 10)
    };

    result.ok_or(client::Error::can_not_parse_number(string))
}

pub fn decode_abi_number<N: NumCast>(string: &str) -> ClientResult<N> {
    let bigint = decode_abi_bigint(string)?;
    NumCast::from(bigint).ok_or(client::Error::can_not_parse_number(string))
}

pub fn slice_from_cell(cell: Cell) -> ClientResult<SliceData> {
    SliceData::load_cell(cell).map_err(client::Error::invalid_data)
}
