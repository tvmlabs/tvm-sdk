use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCatchainMessage`\n\n```text\ntonNode.rempCatchainMessage message:bytes message_id:int256 source_key_id:int256 source_idx:int masterchain_seqno:int = tonNode.rempCatchainRecord;\n```\n"]
pub struct RempCatchainMessage {
    pub message: crate::ton::bytes,
    pub message_id: crate::ton::int256,
    pub source_key_id: crate::ton::int256,
    pub source_idx: crate::ton::int,
    pub masterchain_seqno: crate::ton::int,
}
impl Eq for RempCatchainMessage {}
impl crate::BareSerialize for RempCatchainMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe9d6eb1c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempCatchainMessage {
            message,
            message_id,
            source_key_id,
            source_idx,
            masterchain_seqno,
        } = self;
        _ser.write_bare::<crate::ton::bytes>(message)?;
        _ser.write_bare::<crate::ton::int256>(message_id)?;
        _ser.write_bare::<crate::ton::int256>(source_key_id)?;
        _ser.write_bare::<crate::ton::int>(source_idx)?;
        _ser.write_bare::<crate::ton::int>(masterchain_seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempCatchainMessage {
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
impl crate::IntoBoxed for RempCatchainMessage {
    type Boxed = crate::ton::ton_node::RempCatchainRecord;

    fn into_boxed(self) -> crate::ton::ton_node::RempCatchainRecord {
        crate::ton::ton_node::RempCatchainRecord::TonNode_RempCatchainMessage(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCatchainMessageDigest`\n\n```text\ntonNode.rempCatchainMessageDigest masterchain_seqno:int messages:(vector tonNode.rempCatchainMessageIds) = tonNode.rempCatchainRecord;\n```\n"]
pub struct RempCatchainMessageDigest {
    pub masterchain_seqno: crate::ton::int,
    pub messages: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
    >,
}
impl Eq for RempCatchainMessageDigest {}
impl crate::BareSerialize for RempCatchainMessageDigest {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x384e3f84)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempCatchainMessageDigest { masterchain_seqno, messages } = self;
        _ser.write_bare::<crate::ton::int>(masterchain_seqno)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
        >>(messages)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempCatchainMessageDigest {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let masterchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let messages = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
            >>()?;
            Ok(Self { masterchain_seqno, messages })
        }
    }
}
impl crate::IntoBoxed for RempCatchainMessageDigest {
    type Boxed = crate::ton::ton_node::RempCatchainRecord;

    fn into_boxed(self) -> crate::ton::ton_node::RempCatchainRecord {
        crate::ton::ton_node::RempCatchainRecord::TonNode_RempCatchainMessageDigest(self)
    }
}
