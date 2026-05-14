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

use tvm_block::CommonMsgInfo;
use tvm_block::GetRepresentationHash;
use tvm_block::Message as TvmMessage;
use tvm_types::Cell;
use tvm_types::Result;
use tvm_types::SliceData;

use crate::json_helper;
use crate::types::StringId;
use crate::types::grams_to_u64;

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub enum MessageType {
    Internal,
    ExternalInbound,
    ExternalOutbound,
    #[default]
    Unknown,
}

pub type MessageId = StringId;

#[derive(Debug, Deserialize, Default)]
pub struct Message {
    pub id: MessageId,
    #[serde(default, with = "json_helper::opt_cell")]
    pub body: Option<Cell>,
    #[serde(deserialize_with = "json_helper::deserialize_message_type")]
    pub msg_type: MessageType,
    #[serde(with = "json_helper::uint")]
    pub value: u64,
}

// The struct represents sent message and allows to access their properties.
#[allow(dead_code)]
impl Message {
    pub fn with_msg(tvm_msg: &TvmMessage) -> Result<Self> {
        let id = tvm_msg.hash()?.as_slice()[..].into();
        let body = tvm_msg.body().map(|slice| slice.into_cell());
        let value = tvm_msg.get_value().map(|cc| grams_to_u64(&cc.grams)).transpose()?.unwrap_or(0);

        let msg_type = match tvm_msg.header() {
            CommonMsgInfo::IntMsgInfo(_) => MessageType::Internal,
            CommonMsgInfo::ExtInMsgInfo(_) => MessageType::ExternalInbound,
            CommonMsgInfo::ExtOutMsgInfo(_) => MessageType::ExternalOutbound,
        };

        Ok(Self { id, body, msg_type, value })
    }

    // Returns message's identifier
    pub fn id(&self) -> MessageId {
        // On client side id is ready allways. It is never be calculated, just returned.
        self.id.clone()
    }

    // Returns message's body (as tree of cells) or None if message doesn't have
    // once
    pub fn body(&self) -> Option<SliceData> {
        self.body.clone().and_then(|cell| SliceData::load_cell(cell).ok())
    }

    // Returns message's type
    pub fn msg_type(&self) -> MessageType {
        self.msg_type.clone()
    }
}

#[cfg(test)]
mod tests {
    use tvm_block::CurrencyCollection;
    use tvm_block::ExtOutMessageHeader;
    use tvm_block::ExternalInboundMessageHeader;
    use tvm_block::GetRepresentationHash;
    use tvm_block::InternalMessageHeader;
    use tvm_block::Message as TvmMessage;
    use tvm_block::MsgAddressExt;
    use tvm_block::MsgAddressInt;
    use tvm_types::AccountId;
    use tvm_types::BuilderData;
    use tvm_types::SliceData;

    use super::*;

    fn std_addr(byte: u8) -> MsgAddressInt {
        MsgAddressInt::with_standart(None, 0, AccountId::from([byte; 32])).unwrap()
    }

    fn ext_addr(byte: u8) -> MsgAddressExt {
        MsgAddressExt::with_extern(SliceData::new(vec![byte])).unwrap()
    }

    #[test]
    fn default_message_has_unknown_type_and_no_body() {
        let message = Message::default();

        assert_eq!(message.id().to_string(), "");
        assert_eq!(message.msg_type(), MessageType::Unknown);
        assert!(message.body().is_none());
    }

    #[test]
    fn message_accessors_return_cloned_id_type_and_body_slice() {
        let cell = BuilderData::with_raw(vec![0xab], 8).unwrap().into_cell().unwrap();
        let message = Message {
            id: "abcd".into(),
            body: Some(cell),
            msg_type: MessageType::ExternalInbound,
            value: 7,
        };

        assert_eq!(message.id().to_string(), "abcd");
        assert_eq!(message.msg_type(), MessageType::ExternalInbound);

        let body = message.body().unwrap();
        assert_eq!(body.remaining_bits(), 8);
        assert_eq!(body.get_bytestring(0), vec![0xab]);
    }

    #[test]
    fn with_msg_maps_internal_message_body_and_value() {
        let mut tvm_message = TvmMessage::with_int_header(InternalMessageHeader::with_addresses(
            std_addr(1),
            std_addr(2),
            CurrencyCollection::with_grams(55),
        ));
        tvm_message.set_body(
            SliceData::load_builder(BuilderData::with_raw(vec![0xde, 0xad], 16).unwrap()).unwrap(),
        );

        let message = Message::with_msg(&tvm_message).unwrap();

        assert_eq!(message.id().to_string(), hex::encode(tvm_message.hash().unwrap().as_slice()));
        assert_eq!(message.msg_type(), MessageType::Internal);
        assert_eq!(message.value, 55);
        assert_eq!(message.body().unwrap().get_bytestring(0), vec![0xde, 0xad]);
    }

    #[test]
    fn with_msg_maps_external_message_types_without_value() {
        let ext_in = TvmMessage::with_ext_in_header(ExternalInboundMessageHeader::new(
            ext_addr(0xaa),
            std_addr(3),
        ));
        let ext_out = TvmMessage::with_ext_out_header(ExtOutMessageHeader::with_addresses(
            std_addr(4),
            ext_addr(0xbb),
        ));

        let inbound = Message::with_msg(&ext_in).unwrap();
        let outbound = Message::with_msg(&ext_out).unwrap();

        assert_eq!(inbound.msg_type(), MessageType::ExternalInbound);
        assert_eq!(inbound.value, 0);
        assert!(inbound.body().is_none());

        assert_eq!(outbound.msg_type(), MessageType::ExternalOutbound);
        assert_eq!(outbound.value, 0);
        assert!(outbound.body().is_none());
    }
}
