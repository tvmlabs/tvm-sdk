use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.dhtServerStatus`\n\n```text\nengine.validator.dhtServerStatus id:int256 status:int = engine.validator.DhtServerStatus;\n```\n"]
pub struct DhtServerStatus {
    pub id: crate::ton::int256,
    pub status: crate::ton::int,
}
impl Eq for DhtServerStatus {}
impl crate::BareSerialize for DhtServerStatus {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb11de75e)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DhtServerStatus { id, status } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int>(status)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DhtServerStatus {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let status = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, status })
        }
    }
}
impl crate::IntoBoxed for DhtServerStatus {
    type Boxed = crate::ton::engine::validator::DhtServerStatus;
    fn into_boxed(self) -> crate::ton::engine::validator::DhtServerStatus {
        crate::ton::engine::validator::DhtServerStatus::Engine_Validator_DhtServerStatus(self)
    }
}
