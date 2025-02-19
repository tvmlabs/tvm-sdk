use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rmqMessage`\n\n```text\ntonNode.rmqMessage message:bytes message_id:int256 source_key_id:int256 source_idx:int masterchain_seqno:int = tonNode.rmqRecord;\n```\n"]
pub struct RmqMessage {
    pub message: crate::ton::bytes,
    pub message_id: crate::ton::int256,
    pub source_key_id: crate::ton::int256,
    pub source_idx: crate::ton::int,
    pub masterchain_seqno: crate::ton::int,
}
impl Eq for RmqMessage {}
impl crate::BareSerialize for RmqMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0de77432)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RmqMessage { message, message_id, source_key_id, source_idx, masterchain_seqno } = self;
        _ser.write_bare::<crate::ton::bytes>(message)?;
        _ser.write_bare::<crate::ton::int256>(message_id)?;
        _ser.write_bare::<crate::ton::int256>(source_key_id)?;
        _ser.write_bare::<crate::ton::int>(source_idx)?;
        _ser.write_bare::<crate::ton::int>(masterchain_seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RmqMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message = _de.read_bare::<crate::ton::bytes>()?;
            let message_id = _de.read_bare::<crate::ton::int256>()?;
            let source_key_id = _de.read_bare::<crate::ton::int256>()?;
            let source_idx = _de.read_bare::<crate::ton::int>()?;
            let masterchain_seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { message, message_id, source_key_id, source_idx, masterchain_seqno })
        }
    }
}
impl crate::IntoBoxed for RmqMessage {
    type Boxed = crate::ton::ton_node::RmqRecord;

    fn into_boxed(self) -> crate::ton::ton_node::RmqRecord {
        crate::ton::ton_node::RmqRecord::TonNode_RmqMessage(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rmqMessageDigest`\n\n```text\ntonNode.rmqMessageDigest masterchain_seqno:int messages:(vector int256) = tonNode.rmqRecord;\n```\n"]
pub struct RmqMessageDigest {
    pub masterchain_seqno: crate::ton::int,
    pub messages: crate::ton::vector<crate::ton::Bare, crate::ton::int256>,
}
impl Eq for RmqMessageDigest {}
impl crate::BareSerialize for RmqMessageDigest {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x974cd134)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RmqMessageDigest { masterchain_seqno, messages } = self;
        _ser.write_bare::<crate::ton::int>(masterchain_seqno)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>(messages)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RmqMessageDigest {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let masterchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let messages =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>()?;
            Ok(Self { masterchain_seqno, messages })
        }
    }
}
impl crate::IntoBoxed for RmqMessageDigest {
    type Boxed = crate::ton::ton_node::RmqRecord;

    fn into_boxed(self) -> crate::ton::ton_node::RmqRecord {
        crate::ton::ton_node::RmqRecord::TonNode_RmqMessageDigest(self)
    }
}
