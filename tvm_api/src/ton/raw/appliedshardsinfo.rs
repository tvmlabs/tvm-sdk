use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.appliedShardsInfo`\n\n```text\nraw.appliedShardsInfo shards:vector<tonNode.blockIdExt> = raw.appliedShardsInfo;\n```\n"]
pub struct AppliedShardsInfo {
    pub shards: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for AppliedShardsInfo {}
impl crate::BareSerialize for AppliedShardsInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x683ae48f)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AppliedShardsInfo { shards } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (shards) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for AppliedShardsInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let shards = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { shards })
        }
    }
}
impl crate::IntoBoxed for AppliedShardsInfo {
    type Boxed = crate::ton::raw::AppliedShardsInfo;
    fn into_boxed(self) -> crate::ton::raw::AppliedShardsInfo {
        crate::ton::raw::AppliedShardsInfo::Raw_AppliedShardsInfo(self)
    }
}
