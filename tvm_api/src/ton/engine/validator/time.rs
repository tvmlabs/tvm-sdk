use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.time`\n\n```text\nengine.validator.time time:int = engine.validator.Time;\n```\n"]
pub struct Time {
    pub time: crate::ton::int,
}
impl Eq for Time {}
impl crate::BareSerialize for Time {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdf5fa1fe)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Time { time } = self;
        _ser.write_bare::<crate::ton::int>(time)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Time {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let time = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { time })
        }
    }
}
impl crate::IntoBoxed for Time {
    type Boxed = crate::ton::engine::validator::Time;
    fn into_boxed(self) -> crate::ton::engine::validator::Time {
        crate::ton::engine::validator::Time::Engine_Validator_Time(self)
    }
}
