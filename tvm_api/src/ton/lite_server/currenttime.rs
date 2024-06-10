use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.currentTime`\n\n```text\nliteServer.currentTime now:int = liteServer.CurrentTime;\n```\n"]
pub struct CurrentTime {
    pub now: crate::ton::int,
}
impl Eq for CurrentTime {}
impl crate::BareSerialize for CurrentTime {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe953000d)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CurrentTime { now } = self;
        _ser.write_bare::<crate::ton::int>(now)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CurrentTime {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let now = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { now })
        }
    }
}
impl crate::IntoBoxed for CurrentTime {
    type Boxed = crate::ton::lite_server::CurrentTime;
    fn into_boxed(self) -> crate::ton::lite_server::CurrentTime {
        crate::ton::lite_server::CurrentTime::LiteServer_CurrentTime(self)
    }
}
