use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.shardAccountMeta`\n\n```text\nraw.shardAccountMeta shard_account_meta:bytes = raw.ShardAccountMeta;\n```\n"]
pub struct ShardAccountMeta {
    pub shard_account_meta: crate::ton::bytes,
}
impl Eq for ShardAccountMeta {}
impl crate::BareSerialize for ShardAccountMeta {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdc93781a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ShardAccountMeta { shard_account_meta } = self;
        _ser.write_bare::<crate::ton::bytes>(shard_account_meta)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ShardAccountMeta {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let shard_account_meta = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { shard_account_meta })
        }
    }
}
impl crate::IntoBoxed for ShardAccountMeta {
    type Boxed = crate::ton::raw::ShardAccountMeta;
    fn into_boxed(self) -> crate::ton::raw::ShardAccountMeta {
        crate::ton::raw::ShardAccountMeta::Raw_ShardAccountMeta(self)
    }
}
