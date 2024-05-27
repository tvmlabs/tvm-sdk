use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.accountState`\n\n```text\nliteServer.accountState id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes proof:bytes state:bytes = liteServer.AccountState;\n```\n"]
pub struct AccountState {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub shardblk: crate::ton::ton_node::blockidext::BlockIdExt,
    pub shard_proof: crate::ton::bytes,
    pub proof: crate::ton::bytes,
    pub state: crate::ton::bytes,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7079c751)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { id, shardblk, shard_proof, proof, state } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(shardblk)?;
        _ser.write_bare::<crate::ton::bytes>(shard_proof)?;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_bare::<crate::ton::bytes>(state)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let shardblk = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let shard_proof = _de.read_bare::<crate::ton::bytes>()?;
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let state = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, shardblk, shard_proof, proof, state })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::lite_server::AccountState;

    fn into_boxed(self) -> crate::ton::lite_server::AccountState {
        crate::ton::lite_server::AccountState::LiteServer_AccountState(self)
    }
}
