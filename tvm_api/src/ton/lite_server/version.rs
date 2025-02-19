use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.version`\n\n```text\nliteServer.version mode:# version:int capabilities:long now:int = liteServer.Version;\n```\n"]
pub struct Version {
    pub mode: crate::ton::int,
    pub version: crate::ton::int,
    pub capabilities: crate::ton::long,
    pub now: crate::ton::int,
}
impl Eq for Version {}
impl crate::BareSerialize for Version {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5a0491e5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Version { mode, version, capabilities, now } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::int>(version)?;
        _ser.write_bare::<crate::ton::long>(capabilities)?;
        _ser.write_bare::<crate::ton::int>(now)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Version {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let version = _de.read_bare::<crate::ton::int>()?;
            let capabilities = _de.read_bare::<crate::ton::long>()?;
            let now = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { mode, version, capabilities, now })
        }
    }
}
impl crate::IntoBoxed for Version {
    type Boxed = crate::ton::lite_server::Version;

    fn into_boxed(self) -> crate::ton::lite_server::Version {
        crate::ton::lite_server::Version::LiteServer_Version(self)
    }
}
