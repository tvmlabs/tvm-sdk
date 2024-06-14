use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.blockData`\n\n```text\nliteServer.blockData id:tonNode.blockIdExt data:bytes = liteServer.BlockData;\n```\n"]
pub struct BlockData {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub data: crate::ton::bytes,
}
impl Eq for BlockData {}
impl crate::BareSerialize for BlockData {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa574ed6c)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockData { id, data } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockData {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, data })
        }
    }
}
impl crate::IntoBoxed for BlockData {
    type Boxed = crate::ton::lite_server::BlockData;
    fn into_boxed(self) -> crate::ton::lite_server::BlockData {
        crate::ton::lite_server::BlockData::LiteServer_BlockData(self)
    }
}
