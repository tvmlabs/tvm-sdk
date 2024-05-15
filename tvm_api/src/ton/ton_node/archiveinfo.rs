use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.archiveInfo`\n\n```text\ntonNode.archiveInfo id:long = tonNode.ArchiveInfo;\n```\n"]
pub struct ArchiveInfo {
    pub id: crate::ton::long,
}
impl Eq for ArchiveInfo {}
impl crate::BareSerialize for ArchiveInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x19efff8c)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ArchiveInfo { id } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ArchiveInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for ArchiveInfo {
    type Boxed = crate::ton::ton_node::ArchiveInfo;
    fn into_boxed(self) -> crate::ton::ton_node::ArchiveInfo {
        crate::ton::ton_node::ArchiveInfo::TonNode_ArchiveInfo(self)
    }
}
