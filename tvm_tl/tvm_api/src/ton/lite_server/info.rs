use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.info`\n\n```text\nliteServer.info now:int53 version:int32 capabilities:int64 = liteServer.Info;\n```\n"]
pub struct Info {
    pub now: crate::ton::int53,
    pub version: crate::ton::int32,
    pub capabilities: crate::ton::int64,
}
impl Eq for Info {}
impl crate::BareSerialize for Info {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb57bfe73)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Info { now, version, capabilities } = self;
        _ser.write_bare::<crate::ton::int53>(now)?;
        _ser.write_bare::<crate::ton::int32>(version)?;
        _ser.write_bare::<crate::ton::int64>(capabilities)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Info {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let now = _de.read_bare::<crate::ton::int53>()?;
            let version = _de.read_bare::<crate::ton::int32>()?;
            let capabilities = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { now, version, capabilities })
        }
    }
}
impl crate::IntoBoxed for Info {
    type Boxed = crate::ton::lite_server::Info;

    fn into_boxed(self) -> crate::ton::lite_server::Info {
        crate::ton::lite_server::Info::LiteServer_Info(self)
    }
}
