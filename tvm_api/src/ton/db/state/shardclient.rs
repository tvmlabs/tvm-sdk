use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.shardClient`\n\n```text\ndb.state.shardClient block:tonNode.blockIdExt = db.state.ShardClient;\n```\n"]
pub struct ShardClient {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for ShardClient {}
impl crate::BareSerialize for ShardClient {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0b16a69d)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ShardClient { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ShardClient {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for ShardClient {
    type Boxed = crate::ton::db::state::ShardClient;
    fn into_boxed(self) -> crate::ton::db::state::ShardClient {
        crate::ton::db::state::ShardClient::Db_State_ShardClient(self)
    }
}
