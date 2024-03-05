use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.shardInfo`\n\n```text\nliteServer.shardInfo id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes shard_descr:bytes = liteServer.ShardInfo;\n```\n"]
pub struct ShardInfo {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub shardblk: crate::ton::ton_node::blockidext::BlockIdExt,
    pub shard_proof: crate::ton::bytes,
    pub shard_descr: crate::ton::bytes,
}
impl Eq for ShardInfo {}
impl crate::BareSerialize for ShardInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9fe6cd84)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ShardInfo { id, shardblk, shard_proof, shard_descr } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(shardblk)?;
        _ser.write_bare::<crate::ton::bytes>(shard_proof)?;
        _ser.write_bare::<crate::ton::bytes>(shard_descr)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ShardInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let shardblk = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let shard_proof = _de.read_bare::<crate::ton::bytes>()?;
            let shard_descr = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, shardblk, shard_proof, shard_descr })
        }
    }
}
impl crate::IntoBoxed for ShardInfo {
    type Boxed = crate::ton::lite_server::ShardInfo;

    fn into_boxed(self) -> crate::ton::lite_server::ShardInfo {
        crate::ton::lite_server::ShardInfo::LiteServer_ShardInfo(self)
    }
}
