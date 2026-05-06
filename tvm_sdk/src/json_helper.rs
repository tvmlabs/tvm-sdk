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

use std::fmt;
use std::str::FromStr;

use serde::de::Error;
use tvm_block::AccStatusChange;
use tvm_block::AccountStatus;
use tvm_block::ComputeSkipReason;
use tvm_block::MsgAddressInt;
use tvm_block::TransactionProcessingStatus;
use tvm_types::Cell;
use tvm_types::base64_decode;

use crate::MessageType;

pub struct StringVisitor;

impl<'de> serde::de::Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("String")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok("null".to_owned())
    }

    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_string(StringVisitor)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok("null".to_owned())
    }
}

struct U8Visitor;

impl<'de> serde::de::Visitor<'de> for U8Visitor {
    type Value = u8;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Number")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v as u8)
    }
}

pub mod opt_cell {
    use tvm_types::base64_encode;

    use super::*;

    pub fn deserialize<'de, D>(d: D) -> Result<Option<Cell>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b64 = d.deserialize_option(StringVisitor)?;

        if "null" == b64 {
            Ok(None)
        } else {
            Ok(Some(deserialize_tree_of_cells_from_base64::<D>(&b64)?))
        }
    }

    pub fn serialize<S>(value: &Option<Cell>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(cell) = value {
            let str_value = base64_encode(tvm_types::boc::write_boc(cell).map_err(|err| {
                serde::ser::Error::custom(format!("Cannot serialize BOC: {}", err))
            })?);
            serializer.serialize_some(&str_value)
        } else {
            serializer.serialize_none()
        }
    }
}

pub fn deserialize_tree_of_cells_from_base64<'de, D>(b64: &str) -> Result<Cell, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bytes = base64_decode(b64)
        .map_err(|err| D::Error::custom(format!("error decode base64: {}", err)))?;

    tvm_types::boc::read_single_root_boc(bytes)
        .map_err(|err| D::Error::custom(format!("BOC read error: {}", err)))
}

pub mod address {
    use super::*;

    pub fn deserialize<'de, D>(d: D) -> Result<MsgAddressInt, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = d.deserialize_string(StringVisitor)?;

        MsgAddressInt::from_str(&string)
            .map_err(|err| D::Error::custom(format!("Address parsing error: {}", err)))
    }

    pub fn serialize<S>(value: &MsgAddressInt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }
}

pub mod uint {
    use super::*;

    pub fn deserialize<'de, D>(d: D) -> Result<u64, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = d.deserialize_option(StringVisitor)?;

        if "null" == string {
            return Ok(0);
        }

        if !string.starts_with("0x") {
            return Err(D::Error::custom(format!(
                "Number parsing error: number must be prefixed with 0x ({})",
                string
            )));
        }

        u64::from_str_radix(&string[2..], 16)
            .map_err(|err| D::Error::custom(format!("Error parsing number: {}", err)))
    }

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("0x{:x}", value))
    }
}

pub fn deserialize_tr_state<'de, D>(d: D) -> Result<TransactionProcessingStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match d.deserialize_u8(U8Visitor) {
        Err(_) => Ok(TransactionProcessingStatus::Unknown),
        Ok(0) => Ok(TransactionProcessingStatus::Unknown),
        Ok(1) => Ok(TransactionProcessingStatus::Preliminary),
        Ok(2) => Ok(TransactionProcessingStatus::Proposed),
        Ok(3) => Ok(TransactionProcessingStatus::Finalized),
        Ok(4) => Ok(TransactionProcessingStatus::Refused),
        Ok(num) => Err(D::Error::custom(format!("Invalid transaction state: {}", num))),
    }
}

pub fn transaction_status_to_u8(status: TransactionProcessingStatus) -> u8 {
    match status {
        TransactionProcessingStatus::Unknown => 0,
        TransactionProcessingStatus::Preliminary => 1,
        TransactionProcessingStatus::Proposed => 2,
        TransactionProcessingStatus::Finalized => 3,
        TransactionProcessingStatus::Refused => 4,
    }
}

pub fn deserialize_acc_state_change<'de, D>(d: D) -> Result<AccStatusChange, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let num = d.deserialize_u8(U8Visitor)?;

    match num {
        0 => Ok(AccStatusChange::Unchanged),
        1 => Ok(AccStatusChange::Frozen),
        2 => Ok(AccStatusChange::Deleted),
        num => Err(D::Error::custom(format!("Invalid account change state: {}", num))),
    }
}

pub fn deserialize_skipped_reason<'de, D>(d: D) -> Result<Option<ComputeSkipReason>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match d.deserialize_u8(U8Visitor) {
        Err(_) => Ok(None),
        Ok(0) => Ok(Some(ComputeSkipReason::NoState)),
        Ok(1) => Ok(Some(ComputeSkipReason::BadState)),
        Ok(2) => Ok(Some(ComputeSkipReason::NoGas)),
        Ok(num) => Err(D::Error::custom(format!("Invalid skip reason: {}", num))),
    }
}

pub fn deserialize_message_type<'de, D>(d: D) -> Result<MessageType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let num = d.deserialize_u8(U8Visitor)?;

    match num {
        0 => Ok(MessageType::Internal),
        1 => Ok(MessageType::ExternalInbound),
        2 => Ok(MessageType::ExternalOutbound),
        num => Err(D::Error::custom(format!("Invalid message type: {}", num))),
    }
}

pub mod account_status {
    use super::*;

    pub fn deserialize<'de, D>(d: D) -> Result<AccountStatus, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let num = d.deserialize_u8(U8Visitor)?;

        match num {
            0 => Ok(AccountStatus::AccStateUninit),
            1 => Ok(AccountStatus::AccStateActive),
            2 => Ok(AccountStatus::AccStateFrozen),
            3 => Ok(AccountStatus::AccStateNonexist),
            num => Err(D::Error::custom(format!("Invalid account status: {}", num))),
        }
    }

    pub fn serialize<S>(value: &AccountStatus, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match value {
            AccountStatus::AccStateUninit => 0,
            AccountStatus::AccStateActive => 1,
            AccountStatus::AccStateFrozen => 2,
            AccountStatus::AccStateNonexist => 3,
        })
    }
}

pub fn account_status_to_u8(status: AccountStatus) -> u8 {
    match status {
        AccountStatus::AccStateUninit => 0,
        AccountStatus::AccStateActive => 1,
        AccountStatus::AccStateFrozen => 2,
        AccountStatus::AccStateNonexist => 3,
    }
}

pub fn deserialize_shard<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string = d.deserialize_string(StringVisitor)?;

    u64::from_str_radix(&string, 16)
        .map_err(|err| D::Error::custom(format!("Error parsing shard: {}", err)))
}

#[cfg(test)]
mod tests {
    use serde::de::Visitor;
    use serde_json::json;
    use tvm_block::AccStatusChange;
    use tvm_block::AccountStatus;
    use tvm_block::ComputeSkipReason;
    use tvm_block::MsgAddressInt;
    use tvm_block::TransactionProcessingStatus;
    use tvm_types::BuilderData;

    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    struct UintHolder {
        #[serde(with = "uint")]
        value: u64,
    }

    #[test]
    fn uint_deserializes_hex_and_null_and_serializes_hex() {
        let holder: UintHolder = serde_json::from_value(json!({ "value": "0x2a" })).unwrap();
        assert_eq!(holder.value, 42);

        let holder: UintHolder = serde_json::from_value(json!({ "value": null })).unwrap();
        assert_eq!(holder.value, 0);

        assert_eq!(
            serde_json::to_value(UintHolder { value: 255 }).unwrap(),
            json!({
                "value": "0xff"
            })
        );
    }

    #[test]
    fn uint_rejects_missing_hex_prefix_and_bad_digits() {
        let err = serde_json::from_value::<UintHolder>(json!({ "value": "42" })).unwrap_err();
        assert!(err.to_string().contains("number must be prefixed with 0x"));

        let err = serde_json::from_value::<UintHolder>(json!({ "value": "0xzz" })).unwrap_err();
        assert!(err.to_string().contains("Error parsing number"));
    }

    #[derive(Debug, Deserialize)]
    struct EnumHolder {
        #[serde(deserialize_with = "deserialize_tr_state")]
        tr_state: TransactionProcessingStatus,
        #[serde(deserialize_with = "deserialize_acc_state_change")]
        acc_change: AccStatusChange,
        #[serde(deserialize_with = "deserialize_skipped_reason")]
        skipped_reason: Option<ComputeSkipReason>,
        #[serde(deserialize_with = "deserialize_message_type")]
        msg_type: MessageType,
        #[serde(with = "account_status")]
        account_status: AccountStatus,
        #[serde(deserialize_with = "deserialize_shard")]
        shard: u64,
    }

    #[test]
    fn enum_deserializers_map_wire_numbers() {
        let holder: EnumHolder = serde_json::from_value(json!({
            "tr_state": 3,
            "acc_change": 2,
            "skipped_reason": 1,
            "msg_type": 2,
            "account_status": 1,
            "shard": "8000000000000000"
        }))
        .unwrap();

        assert_eq!(holder.tr_state, TransactionProcessingStatus::Finalized);
        assert_eq!(holder.acc_change, AccStatusChange::Deleted);
        assert_eq!(holder.skipped_reason, Some(ComputeSkipReason::BadState));
        assert_eq!(holder.msg_type, MessageType::ExternalOutbound);
        assert_eq!(holder.account_status, AccountStatus::AccStateActive);
        assert_eq!(holder.shard, 0x8000_0000_0000_0000);
    }

    #[derive(Debug, Deserialize)]
    struct TransactionStateHolder {
        #[serde(deserialize_with = "deserialize_tr_state")]
        value: TransactionProcessingStatus,
    }

    #[test]
    fn transaction_state_uses_unknown_for_non_numeric_values() {
        let holder: TransactionStateHolder =
            serde_json::from_value(json!({ "value": "bad" })).unwrap();
        assert_eq!(holder.value, TransactionProcessingStatus::Unknown);

        let err =
            serde_json::from_value::<TransactionStateHolder>(json!({ "value": 9 })).unwrap_err();
        assert!(err.to_string().contains("Invalid transaction state: 9"));
    }

    #[derive(Debug, Deserialize)]
    struct SkippedReasonHolder {
        #[serde(deserialize_with = "deserialize_skipped_reason")]
        value: Option<ComputeSkipReason>,
    }

    #[test]
    fn skipped_reason_uses_none_for_non_numeric_values() {
        let holder: SkippedReasonHolder =
            serde_json::from_value(json!({ "value": "bad" })).unwrap();
        assert_eq!(holder.value, None);

        let err = serde_json::from_value::<SkippedReasonHolder>(json!({ "value": 9 })).unwrap_err();
        assert!(err.to_string().contains("Invalid skip reason: 9"));
    }

    #[derive(Debug, Deserialize)]
    struct MessageTypeHolder {
        #[serde(deserialize_with = "deserialize_message_type")]
        #[allow(dead_code)]
        value: MessageType,
    }

    #[derive(Debug, Deserialize)]
    struct AccountStatusHolder {
        #[serde(with = "account_status")]
        #[allow(dead_code)]
        value: AccountStatus,
    }

    #[derive(Debug, Deserialize)]
    struct AccountChangeHolder {
        #[serde(deserialize_with = "deserialize_acc_state_change")]
        #[allow(dead_code)]
        value: AccStatusChange,
    }

    #[test]
    fn enum_deserializers_reject_unknown_numeric_values() {
        let err = serde_json::from_value::<MessageTypeHolder>(json!({ "value": 9 })).unwrap_err();
        assert!(err.to_string().contains("Invalid message type: 9"));

        let err = serde_json::from_value::<AccountStatusHolder>(json!({ "value": 9 })).unwrap_err();
        assert!(err.to_string().contains("Invalid account status: 9"));

        let err = serde_json::from_value::<AccountChangeHolder>(json!({ "value": 9 })).unwrap_err();
        assert!(err.to_string().contains("Invalid account change state: 9"));
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct OptionalCellHolder {
        #[serde(default, with = "opt_cell")]
        body: Option<tvm_types::Cell>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct AddressHolder {
        #[serde(with = "address")]
        value: MsgAddressInt,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct StatusSerdeHolder {
        #[serde(with = "account_status")]
        status: AccountStatus,
    }

    #[test]
    fn opt_cell_roundtrips_some_and_none() {
        let cell = BuilderData::with_raw(vec![0xab], 8).unwrap().into_cell().unwrap();
        let value = serde_json::to_value(OptionalCellHolder { body: Some(cell.clone()) }).unwrap();
        let decoded: OptionalCellHolder = serde_json::from_value(value).unwrap();
        assert_eq!(decoded.body.unwrap(), cell);

        assert_eq!(
            serde_json::to_value(OptionalCellHolder { body: None }).unwrap(),
            json!({
                "body": null
            })
        );
        let decoded: OptionalCellHolder = serde_json::from_value(json!({ "body": null })).unwrap();
        assert!(decoded.body.is_none());
    }

    #[test]
    fn opt_cell_rejects_invalid_base64() {
        let err = serde_json::from_value::<OptionalCellHolder>(json!({ "body": "not-base64" }))
            .unwrap_err();
        assert!(err.to_string().contains("error decode base64"));
    }

    #[test]
    fn opt_cell_rejects_valid_base64_with_invalid_boc() {
        let invalid_boc = tvm_types::base64_encode("not a boc");
        let err = serde_json::from_value::<OptionalCellHolder>(json!({ "body": invalid_boc }))
            .unwrap_err();
        assert!(err.to_string().contains("BOC read error"));
    }

    #[test]
    fn address_and_account_status_roundtrip_through_serde() {
        let address = MsgAddressInt::with_standart(None, 0, [0x11; 32].into()).unwrap();
        let serialized = serde_json::to_value(AddressHolder { value: address.clone() }).unwrap();
        assert_eq!(serialized, json!({ "value": address.to_string() }));

        let decoded: AddressHolder = serde_json::from_value(serialized).unwrap();
        assert_eq!(decoded.value, address);

        let status = StatusSerdeHolder { status: AccountStatus::AccStateFrozen };
        assert_eq!(serde_json::to_value(&status).unwrap(), json!({ "status": 2 }));
    }

    #[test]
    fn address_and_shard_deserializers_report_errors() {
        let err = serde_json::from_value::<AddressHolder>(json!({ "value": "not-an-address" }))
            .unwrap_err();
        assert!(err.to_string().contains("Address parsing error"));

        #[derive(Debug, Deserialize)]
        struct ShardHolder {
            #[serde(deserialize_with = "deserialize_shard")]
            #[allow(dead_code)]
            value: u64,
        }

        let err = serde_json::from_value::<ShardHolder>(json!({ "value": "zzz" })).unwrap_err();
        assert!(err.to_string().contains("Error parsing shard"));
    }

    #[test]
    fn string_visitor_handles_some_and_unit() {
        let from_unit = StringVisitor.visit_unit::<serde::de::value::Error>().unwrap();
        assert_eq!(from_unit, "null");

        let deserializer =
            serde::de::value::StrDeserializer::<serde::de::value::Error>::new("hello");
        let from_some = StringVisitor.visit_some(deserializer).unwrap();
        assert_eq!(from_some, "hello");
    }

    #[test]
    fn transaction_and_account_status_to_u8_cover_all_variants() {
        assert_eq!(transaction_status_to_u8(TransactionProcessingStatus::Unknown), 0);
        assert_eq!(transaction_status_to_u8(TransactionProcessingStatus::Preliminary), 1);
        assert_eq!(transaction_status_to_u8(TransactionProcessingStatus::Proposed), 2);
        assert_eq!(transaction_status_to_u8(TransactionProcessingStatus::Finalized), 3);
        assert_eq!(transaction_status_to_u8(TransactionProcessingStatus::Refused), 4);

        assert_eq!(account_status_to_u8(AccountStatus::AccStateUninit), 0);
        assert_eq!(account_status_to_u8(AccountStatus::AccStateActive), 1);
        assert_eq!(account_status_to_u8(AccountStatus::AccStateFrozen), 2);
        assert_eq!(account_status_to_u8(AccountStatus::AccStateNonexist), 3);
    }
}
