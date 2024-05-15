use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.shardAccountState`\n\n```text\nraw.shardAccountState shard_account:bytes = raw.ShardAccountState;\n```\n"]
pub struct ShardAccountState {
    pub shard_account: crate::ton::bytes,
}
impl Eq for ShardAccountState {}
impl crate::BareSerialize for ShardAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc5a7834f)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ShardAccountState { shard_account } = self;
        _ser.write_bare::<crate::ton::bytes>(shard_account)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ShardAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let shard_account = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { shard_account })
        }
    }
}
impl crate::IntoBoxed for ShardAccountState {
    type Boxed = crate::ton::raw::ShardAccountState;
    fn into_boxed(self) -> crate::ton::raw::ShardAccountState {
        crate::ton::raw::ShardAccountState::Raw_ShardAccountState(self)
    }
}
