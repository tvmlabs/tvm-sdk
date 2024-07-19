use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCatchainMessageDigestV2`\n\n```text\ntonNode.rempCatchainMessageDigestV2 masterchain_seqno:int messages:(vector tonNode.rempCatchainMessageIds) = tonNode.rempCatchainRecordV2;\n```\n"]
pub struct RempCatchainMessageDigestV2 {
    pub masterchain_seqno: crate::ton::int,
    pub messages:
        crate::ton::vector<crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds>,
}
impl Eq for RempCatchainMessageDigestV2 {}
impl crate::BareSerialize for RempCatchainMessageDigestV2 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x99f6a754)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempCatchainMessageDigestV2 { masterchain_seqno, messages } = self;
        _ser.write_bare::<crate::ton::int>(masterchain_seqno)?;
        (messages
            as &dyn crate::ton::VectoredBare<
                crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
            >)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempCatchainMessageDigestV2 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let masterchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let messages = <Vec<
                crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
            > as crate::ton::VectoredBare<
                crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
            >>::deserialize(_de)?;
            Ok(Self { masterchain_seqno, messages })
        }
    }
}
impl crate::IntoBoxed for RempCatchainMessageDigestV2 {
    type Boxed = crate::ton::ton_node::RempCatchainRecordV2;

    fn into_boxed(self) -> crate::ton::ton_node::RempCatchainRecordV2 {
        crate::ton::ton_node::RempCatchainRecordV2::TonNode_RempCatchainMessageDigestV2(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCatchainMessageHeaderV2`\n\n```text\ntonNode.rempCatchainMessageHeaderV2 message_id:int256 message_uid:int256 source_key_id:int256 source_idx:int masterchain_seqno:int = tonNode.rempCatchainRecordV2;\n```\n"]
pub struct RempCatchainMessageHeaderV2 {
    pub message_id: crate::ton::int256,
    pub message_uid: crate::ton::int256,
    pub source_key_id: crate::ton::int256,
    pub source_idx: crate::ton::int,
    pub masterchain_seqno: crate::ton::int,
}
impl Eq for RempCatchainMessageHeaderV2 {}
impl crate::BareSerialize for RempCatchainMessageHeaderV2 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8b8f9248)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempCatchainMessageHeaderV2 {
            message_id,
            message_uid,
            source_key_id,
            source_idx,
            masterchain_seqno,
        } = self;
        _ser.write_bare::<crate::ton::int256>(message_id)?;
        _ser.write_bare::<crate::ton::int256>(message_uid)?;
        _ser.write_bare::<crate::ton::int256>(source_key_id)?;
        _ser.write_bare::<crate::ton::int>(source_idx)?;
        _ser.write_bare::<crate::ton::int>(masterchain_seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempCatchainMessageHeaderV2 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message_id = _de.read_bare::<crate::ton::int256>()?;
            let message_uid = _de.read_bare::<crate::ton::int256>()?;
            let source_key_id = _de.read_bare::<crate::ton::int256>()?;
            let source_idx = _de.read_bare::<crate::ton::int>()?;
            let masterchain_seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { message_id, message_uid, source_key_id, source_idx, masterchain_seqno })
        }
    }
}
impl crate::IntoBoxed for RempCatchainMessageHeaderV2 {
    type Boxed = crate::ton::ton_node::RempCatchainRecordV2;

    fn into_boxed(self) -> crate::ton::ton_node::RempCatchainRecordV2 {
        crate::ton::ton_node::RempCatchainRecordV2::TonNode_RempCatchainMessageHeaderV2(self)
    }
}
