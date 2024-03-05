use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.newShardBlock`\n\n```text\ntonNode.newShardBlock block:tonNode.blockIdExt cc_seqno:int data:bytes = tonNode.NewShardBlock;\n```\n"]
pub struct NewShardBlock {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub cc_seqno: crate::ton::int,
    pub data: crate::ton::bytes,
}
impl Eq for NewShardBlock {}
impl crate::BareSerialize for NewShardBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa49dc229)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let NewShardBlock { block, cc_seqno, data } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::int>(cc_seqno)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for NewShardBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let cc_seqno = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { block, cc_seqno, data })
        }
    }
}
impl crate::IntoBoxed for NewShardBlock {
    type Boxed = crate::ton::ton_node::NewShardBlock;

    fn into_boxed(self) -> crate::ton::ton_node::NewShardBlock {
        crate::ton::ton_node::NewShardBlock::TonNode_NewShardBlock(self)
    }
}
