use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `ton.blockId`\n\n```text\nton.blockId workchain:int32 shard:int64 seqno:int32 = internal.BlockId;\n```\n"]
pub struct BlockId {
    pub workchain: crate::ton::int32,
    pub shard: crate::ton::int64,
    pub seqno: crate::ton::int32,
}
impl Eq for BlockId {}
impl crate::BareSerialize for BlockId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb9587fa2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockId { workchain, shard, seqno } = self;
        _ser.write_bare::<crate::ton::int32>(workchain)?;
        _ser.write_bare::<crate::ton::int64>(shard)?;
        _ser.write_bare::<crate::ton::int32>(seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int32>()?;
            let shard = _de.read_bare::<crate::ton::int64>()?;
            let seqno = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { workchain, shard, seqno })
        }
    }
}
impl crate::IntoBoxed for BlockId {
    type Boxed = crate::ton::internal::BlockId;

    fn into_boxed(self) -> crate::ton::internal::BlockId {
        crate::ton::internal::BlockId::Ton_BlockId(self)
    }
}
