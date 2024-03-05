use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.keyBlocks`\n\n```text\ntonNode.keyBlocks blocks:(vector tonNode.blockIdExt) incomplete:Bool error:Bool = tonNode.KeyBlocks;\n```\n"]
pub struct KeyBlocks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
    pub incomplete: crate::ton::Bool,
    pub error: crate::ton::Bool,
}
impl Eq for KeyBlocks {}
impl crate::BareSerialize for KeyBlocks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x07664d59)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let KeyBlocks { blocks, incomplete, error } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        _ser.write_boxed::<crate::ton::Bool>(incomplete)?;
        _ser.write_boxed::<crate::ton::Bool>(error)?;
        Ok(())
    }
}
impl crate::BareDeserialize for KeyBlocks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            let incomplete = _de.read_boxed::<crate::ton::Bool>()?;
            let error = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { blocks, incomplete, error })
        }
    }
}
impl crate::IntoBoxed for KeyBlocks {
    type Boxed = crate::ton::ton_node::KeyBlocks;

    fn into_boxed(self) -> crate::ton::ton_node::KeyBlocks {
        crate::ton::ton_node::KeyBlocks::TonNode_KeyBlocks(self)
    }
}
