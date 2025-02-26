use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.blockLinkBack`\n\n```text\nliteServer.blockLinkBack to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes proof:bytes state_proof:bytes = liteServer.BlockLink;\n```\n"]
pub struct BlockLinkBack {
    pub to_key_block: crate::ton::Bool,
    pub from: crate::ton::ton_node::blockidext::BlockIdExt,
    pub to: crate::ton::ton_node::blockidext::BlockIdExt,
    pub dest_proof: crate::ton::bytes,
    pub proof: crate::ton::bytes,
    pub state_proof: crate::ton::bytes,
}
impl Eq for BlockLinkBack {}
impl crate::BareSerialize for BlockLinkBack {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xef7e1bef)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockLinkBack { to_key_block, from, to, dest_proof, proof, state_proof } = self;
        _ser.write_boxed::<crate::ton::Bool>(to_key_block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(from)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(to)?;
        _ser.write_bare::<crate::ton::bytes>(dest_proof)?;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_bare::<crate::ton::bytes>(state_proof)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockLinkBack {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let to_key_block = _de.read_boxed::<crate::ton::Bool>()?;
            let from = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let to = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let dest_proof = _de.read_bare::<crate::ton::bytes>()?;
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let state_proof = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { to_key_block, from, to, dest_proof, proof, state_proof })
        }
    }
}
impl crate::IntoBoxed for BlockLinkBack {
    type Boxed = crate::ton::lite_server::BlockLink;

    fn into_boxed(self) -> crate::ton::lite_server::BlockLink {
        crate::ton::lite_server::BlockLink::LiteServer_BlockLinkBack(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.blockLinkForward`\n\n```text\nliteServer.blockLinkForward to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes config_proof:bytes signatures:liteServer.SignatureSet = liteServer.BlockLink;\n```\n"]
pub struct BlockLinkForward {
    pub to_key_block: crate::ton::Bool,
    pub from: crate::ton::ton_node::blockidext::BlockIdExt,
    pub to: crate::ton::ton_node::blockidext::BlockIdExt,
    pub dest_proof: crate::ton::bytes,
    pub config_proof: crate::ton::bytes,
    pub signatures: crate::ton::lite_server::SignatureSet,
}
impl Eq for BlockLinkForward {}
impl crate::BareSerialize for BlockLinkForward {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x520fce1c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockLinkForward { to_key_block, from, to, dest_proof, config_proof, signatures } =
            self;
        _ser.write_boxed::<crate::ton::Bool>(to_key_block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(from)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(to)?;
        _ser.write_bare::<crate::ton::bytes>(dest_proof)?;
        _ser.write_bare::<crate::ton::bytes>(config_proof)?;
        _ser.write_boxed::<crate::ton::lite_server::SignatureSet>(signatures)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockLinkForward {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let to_key_block = _de.read_boxed::<crate::ton::Bool>()?;
            let from = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let to = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let dest_proof = _de.read_bare::<crate::ton::bytes>()?;
            let config_proof = _de.read_bare::<crate::ton::bytes>()?;
            let signatures = _de.read_boxed::<crate::ton::lite_server::SignatureSet>()?;
            Ok(Self { to_key_block, from, to, dest_proof, config_proof, signatures })
        }
    }
}
impl crate::IntoBoxed for BlockLinkForward {
    type Boxed = crate::ton::lite_server::BlockLink;

    fn into_boxed(self) -> crate::ton::lite_server::BlockLink {
        crate::ton::lite_server::BlockLink::LiteServer_BlockLinkForward(self)
    }
}
