use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `options.setConfig`\n\n```text\noptions.setConfig config:config = options.ConfigInfo;\n```\n"]
pub struct SetConfig {
    pub config: crate::ton::config::Config,
}
impl Eq for SetConfig {}
impl crate::BareSerialize for SetConfig {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6f76ebc3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SetConfig { config } = self;
        _ser.write_bare::<crate::ton::config::Config>(config)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SetConfig {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config = _de.read_bare::<crate::ton::config::Config>()?;
            Ok(Self { config })
        }
    }
}
impl crate::BoxedDeserialize for SetConfig {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6f76ebc3)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6f76ebc3) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SetConfig {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6f76ebc3), self)
    }
}
impl crate::Function for SetConfig {
    type Reply = crate::ton::options::ConfigInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `options.validateConfig`\n\n```text\noptions.validateConfig config:config = options.ConfigInfo;\n```\n"]
pub struct ValidateConfig {
    pub config: crate::ton::config::Config,
}
impl Eq for ValidateConfig {}
impl crate::BareSerialize for ValidateConfig {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xeb51ba39)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidateConfig { config } = self;
        _ser.write_bare::<crate::ton::config::Config>(config)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidateConfig {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config = _de.read_bare::<crate::ton::config::Config>()?;
            Ok(Self { config })
        }
    }
}
impl crate::BoxedDeserialize for ValidateConfig {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xeb51ba39)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xeb51ba39) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ValidateConfig {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xeb51ba39), self)
    }
}
impl crate::Function for ValidateConfig {
    type Reply = crate::ton::options::ConfigInfo;
}
