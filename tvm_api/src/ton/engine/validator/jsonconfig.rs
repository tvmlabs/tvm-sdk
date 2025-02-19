use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.jsonConfig`\n\n```text\nengine.validator.jsonConfig data:string = engine.validator.JsonConfig;\n```\n"]
pub struct JsonConfig {
    pub data: crate::ton::string,
}
impl Eq for JsonConfig {}
impl crate::BareSerialize for JsonConfig {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x132d920b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let JsonConfig { data } = self;
        _ser.write_bare::<crate::ton::string>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for JsonConfig {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for JsonConfig {
    type Boxed = crate::ton::engine::validator::JsonConfig;

    fn into_boxed(self) -> crate::ton::engine::validator::JsonConfig {
        crate::ton::engine::validator::JsonConfig::Engine_Validator_JsonConfig(self)
    }
}
