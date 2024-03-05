use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.blockHeader`\n\n```text\nliteServer.blockHeader id:tonNode.blockIdExt mode:# header_proof:bytes = liteServer.BlockHeader;\n```\n"]
pub struct BlockHeader {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub mode: crate::ton::int,
    pub header_proof: crate::ton::bytes,
}
impl Eq for BlockHeader {}
impl crate::BareSerialize for BlockHeader {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x752d8219)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockHeader { id, mode, header_proof } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::bytes>(header_proof)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockHeader {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let mode = _de.read_bare::<crate::ton::int>()?;
            let header_proof = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, mode, header_proof })
        }
    }
}
impl crate::IntoBoxed for BlockHeader {
    type Boxed = crate::ton::lite_server::BlockHeader;

    fn into_boxed(self) -> crate::ton::lite_server::BlockHeader {
        crate::ton::lite_server::BlockHeader::LiteServer_BlockHeader(self)
    }
}
