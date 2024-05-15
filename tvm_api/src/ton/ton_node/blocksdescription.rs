use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.blocksDescription`\n\n```text\ntonNode.blocksDescription ids:(vector tonNode.blockIdExt) incomplete:Bool = tonNode.BlocksDescription;\n```\n"]
pub struct BlocksDescription {
    pub ids: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
    pub incomplete: crate::ton::Bool,
}
impl Eq for BlocksDescription {}
impl crate::BareSerialize for BlocksDescription {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd62a612c)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlocksDescription { ids, incomplete } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (ids) ? ;
        _ser.write_boxed::<crate::ton::Bool>(incomplete)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlocksDescription {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ids = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            let incomplete = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { ids, incomplete })
        }
    }
}
impl crate::IntoBoxed for BlocksDescription {
    type Boxed = crate::ton::ton_node::BlocksDescription;
    fn into_boxed(self) -> crate::ton::ton_node::BlocksDescription {
        crate::ton::ton_node::BlocksDescription::TonNode_BlocksDescription(self)
    }
}
