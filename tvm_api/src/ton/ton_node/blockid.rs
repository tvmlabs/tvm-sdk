use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.blockId`\n\n```text\ntonNode.blockId workchain:int shard:long seqno:int = tonNode.BlockId;\n```\n"]
pub struct BlockId {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub seqno: crate::ton::int,
}
impl Eq for BlockId {}
impl crate::BareSerialize for BlockId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb7cdb167)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockId { workchain, shard, seqno } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { workchain, shard, seqno })
        }
    }
}
impl crate::IntoBoxed for BlockId {
    type Boxed = crate::ton::ton_node::BlockId;

    fn into_boxed(self) -> crate::ton::ton_node::BlockId {
        crate::ton::ton_node::BlockId::TonNode_BlockId(self)
    }
}
