use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.sessionId`\n\n```text\ntonNode.sessionId workchain:int shard:long cc_seqno:int opts_hash:int256 = tonNode.SessionId;\n```\n"]
pub struct SessionId {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub cc_seqno: crate::ton::int,
    pub opts_hash: crate::ton::int256,
}
impl Eq for SessionId {}
impl crate::BareSerialize for SessionId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7a9236ba)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SessionId { workchain, shard, cc_seqno, opts_hash } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_bare::<crate::ton::int>(cc_seqno)?;
        _ser.write_bare::<crate::ton::int256>(opts_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SessionId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let cc_seqno = _de.read_bare::<crate::ton::int>()?;
            let opts_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { workchain, shard, cc_seqno, opts_hash })
        }
    }
}
impl crate::IntoBoxed for SessionId {
    type Boxed = crate::ton::ton_node::SessionId;

    fn into_boxed(self) -> crate::ton::ton_node::SessionId {
        crate::ton::ton_node::SessionId::TonNode_SessionId(self)
    }
}
