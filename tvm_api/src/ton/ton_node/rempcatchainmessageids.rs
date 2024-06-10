use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCatchainMessageIds`\n\n```text\ntonNode.rempCatchainMessageIds id:int256 uid:int256 = tonNode.rempCatchainMessageIds;\n```\n"]
pub struct RempCatchainMessageIds {
    pub id: crate::ton::int256,
    pub uid: crate::ton::int256,
}
impl Eq for RempCatchainMessageIds {}
impl crate::BareSerialize for RempCatchainMessageIds {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5509db82)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempCatchainMessageIds { id, uid } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int256>(uid)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempCatchainMessageIds {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let uid = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id, uid })
        }
    }
}
impl crate::IntoBoxed for RempCatchainMessageIds {
    type Boxed = crate::ton::ton_node::RempCatchainMessageIds;
    fn into_boxed(self) -> crate::ton::ton_node::RempCatchainMessageIds {
        crate::ton::ton_node::RempCatchainMessageIds::TonNode_RempCatchainMessageIds(self)
    }
}
