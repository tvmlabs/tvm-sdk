use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.blocks`\n\n```text\ncatchain.blocks blocks:(vector catchain.block) = catchain.Blocks;\n```\n"]
pub struct Blocks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::catchain::block::Block>,
}
impl Eq for Blocks {}
impl crate::BareSerialize for Blocks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x50ecd1c1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Blocks { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: catchain :: block :: Block > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for Blocks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: catchain :: block :: Block > > () ? ;
            Ok(Self { blocks })
        }
    }
}
impl crate::IntoBoxed for Blocks {
    type Boxed = crate::ton::catchain::Blocks;

    fn into_boxed(self) -> crate::ton::catchain::Blocks {
        crate::ton::catchain::Blocks::Catchain_Blocks(self)
    }
}
