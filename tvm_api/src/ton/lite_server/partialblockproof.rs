use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.partialBlockProof`\n\n```text\nliteServer.partialBlockProof complete:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt steps:(vector liteServer.BlockLink) = liteServer.PartialBlockProof;\n```\n"]
pub struct PartialBlockProof {
    pub complete: crate::ton::Bool,
    pub from: crate::ton::ton_node::blockidext::BlockIdExt,
    pub to: crate::ton::ton_node::blockidext::BlockIdExt,
    pub steps: crate::ton::vector<crate::ton::Boxed, crate::ton::lite_server::BlockLink>,
}
impl Eq for PartialBlockProof {}
impl crate::BareSerialize for PartialBlockProof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8ed0d2c1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PartialBlockProof { complete, from, to, steps } = self;
        _ser.write_boxed::<crate::ton::Bool>(complete)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(from)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(to)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: lite_server :: BlockLink > > (steps) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for PartialBlockProof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let complete = _de.read_boxed::<crate::ton::Bool>()?;
            let from = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let to = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let steps = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: lite_server :: BlockLink > > () ? ;
            Ok(Self { complete, from, to, steps })
        }
    }
}
impl crate::IntoBoxed for PartialBlockProof {
    type Boxed = crate::ton::lite_server::PartialBlockProof;

    fn into_boxed(self) -> crate::ton::lite_server::PartialBlockProof {
        crate::ton::lite_server::PartialBlockProof::LiteServer_PartialBlockProof(self)
    }
}
