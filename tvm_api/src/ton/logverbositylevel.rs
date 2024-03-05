use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `logVerbosityLevel`\n\n```text\nlogVerbosityLevel verbosity_level:int32 = LogVerbosityLevel;\n```\n"]
pub struct LogVerbosityLevel {
    pub verbosity_level: crate::ton::int32,
}
impl Eq for LogVerbosityLevel {}
impl crate::BareSerialize for LogVerbosityLevel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x676443ea)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let LogVerbosityLevel { verbosity_level } = self;
        _ser.write_bare::<crate::ton::int32>(verbosity_level)?;
        Ok(())
    }
}
impl crate::BareDeserialize for LogVerbosityLevel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let verbosity_level = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { verbosity_level })
        }
    }
}
impl crate::IntoBoxed for LogVerbosityLevel {
    type Boxed = crate::ton::LogVerbosityLevel;

    fn into_boxed(self) -> crate::ton::LogVerbosityLevel {
        crate::ton::LogVerbosityLevel::LogVerbosityLevel(self)
    }
}
