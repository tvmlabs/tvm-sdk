use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.blockUpdate`\n\n```text\ncatchain.blockUpdate block:catchain.block = catchain.Update;\n```\n"]
pub struct BlockUpdate {
    pub block: crate::ton::catchain::block::Block,
}
impl Eq for BlockUpdate {}
impl crate::BareSerialize for BlockUpdate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x236758c4)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockUpdate { block } = self;
        _ser.write_bare::<crate::ton::catchain::block::Block>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockUpdate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::catchain::block::Block>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for BlockUpdate {
    type Boxed = crate::ton::catchain::Update;
    fn into_boxed(self) -> crate::ton::catchain::Update {
        crate::ton::catchain::Update::Catchain_BlockUpdate(self)
    }
}
