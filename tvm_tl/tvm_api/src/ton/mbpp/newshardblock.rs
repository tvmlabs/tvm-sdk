use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `mbpp.newShardBlock`\n\n```text\nmbpp.newShardBlock id:tonNode.blockIdExt cc_seqno:int tbd:bytes block:bytes = mbpp.NewShardBlock;\n```\n"]
pub struct NewShardBlock {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub cc_seqno: crate::ton::int,
    pub tbd: crate::ton::bytes,
    pub block: crate::ton::bytes,
}
impl Eq for NewShardBlock {}
impl crate::BareSerialize for NewShardBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x54d61ee6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let NewShardBlock { id, cc_seqno, tbd, block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(cc_seqno)?;
        _ser.write_bare::<crate::ton::bytes>(tbd)?;
        _ser.write_bare::<crate::ton::bytes>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for NewShardBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let cc_seqno = _de.read_bare::<crate::ton::int>()?;
            let tbd = _de.read_bare::<crate::ton::bytes>()?;
            let block = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, cc_seqno, tbd, block })
        }
    }
}
impl crate::IntoBoxed for NewShardBlock {
    type Boxed = crate::ton::mbpp::NewShardBlock;

    fn into_boxed(self) -> crate::ton::mbpp::NewShardBlock {
        crate::ton::mbpp::NewShardBlock::Mbpp_NewShardBlock(self)
    }
}
