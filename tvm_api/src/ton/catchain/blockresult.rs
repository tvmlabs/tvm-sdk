use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.blockResult`\n\n```text\ncatchain.blockResult block:catchain.block = catchain.BlockResult;\n```\n"]
pub struct BlockResult {
    pub block: crate::ton::catchain::block::Block,
}
impl Eq for BlockResult {}
impl crate::BareSerialize for BlockResult {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9d2a3047)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockResult { block } = self;
        _ser.write_bare::<crate::ton::catchain::block::Block>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockResult {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::catchain::block::Block>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for BlockResult {
    type Boxed = crate::ton::catchain::BlockResult;
    fn into_boxed(self) -> crate::ton::catchain::BlockResult {
        crate::ton::catchain::BlockResult::Catchain_BlockResult(self)
    }
}
