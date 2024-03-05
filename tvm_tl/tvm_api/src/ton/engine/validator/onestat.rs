use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.oneStat`\n\n```text\nengine.validator.oneStat key:string value:string = engine.validator.OneStat;\n```\n"]
pub struct OneStat {
    pub key: crate::ton::string,
    pub value: crate::ton::string,
}
impl Eq for OneStat {}
impl crate::BareSerialize for OneStat {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa4983aed)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let OneStat { key, value } = self;
        _ser.write_bare::<crate::ton::string>(key)?;
        _ser.write_bare::<crate::ton::string>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for OneStat {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::string>()?;
            let value = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { key, value })
        }
    }
}
impl crate::IntoBoxed for OneStat {
    type Boxed = crate::ton::engine::validator::OneStat;

    fn into_boxed(self) -> crate::ton::engine::validator::OneStat {
        crate::ton::engine::validator::OneStat::Engine_Validator_OneStat(self)
    }
}
