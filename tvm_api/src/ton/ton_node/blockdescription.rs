use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.blockDescription`\n\n```text\ntonNode.blockDescription id:tonNode.blockIdExt = tonNode.BlockDescription;\n```\n"]
pub struct BlockDescription {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for BlockDescription {}
impl crate::BareSerialize for BlockDescription {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x46a1d088)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockDescription { id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockDescription {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for BlockDescription {
    type Boxed = crate::ton::ton_node::BlockDescription;

    fn into_boxed(self) -> crate::ton::ton_node::BlockDescription {
        crate::ton::ton_node::BlockDescription::TonNode_BlockDescription(self)
    }
}
