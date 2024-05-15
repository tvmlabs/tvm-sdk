use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.debug.verbosity`\n\n```text\nliteServer.debug.verbosity value:int = liteServer.debug.Verbosity;\n```\n"]
pub struct Verbosity {
    pub value: crate::ton::int,
}
impl Eq for Verbosity {}
impl crate::BareSerialize for Verbosity {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5d404733)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Verbosity { value } = self;
        _ser.write_bare::<crate::ton::int>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Verbosity {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Verbosity {
    type Boxed = crate::ton::lite_server::debug::Verbosity;
    fn into_boxed(self) -> crate::ton::lite_server::debug::Verbosity {
        crate::ton::lite_server::debug::Verbosity::LiteServer_Debug_Verbosity(self)
    }
}
