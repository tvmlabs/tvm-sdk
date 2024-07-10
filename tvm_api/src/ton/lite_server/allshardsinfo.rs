use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.allShardsInfo`\n\n```text\nliteServer.allShardsInfo id:tonNode.blockIdExt proof:bytes data:bytes = liteServer.AllShardsInfo;\n```\n"]
pub struct AllShardsInfo {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub proof: crate::ton::bytes,
    pub data: crate::ton::bytes,
}
impl Eq for AllShardsInfo {}
impl crate::BareSerialize for AllShardsInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x098fe72d)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AllShardsInfo { id, proof, data } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AllShardsInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, proof, data })
        }
    }
}
impl crate::IntoBoxed for AllShardsInfo {
    type Boxed = crate::ton::lite_server::AllShardsInfo;
    fn into_boxed(self) -> crate::ton::lite_server::AllShardsInfo {
        crate::ton::lite_server::AllShardsInfo::LiteServer_AllShardsInfo(self)
    }
}
