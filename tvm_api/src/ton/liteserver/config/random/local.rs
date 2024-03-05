use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteserver.config.random.local`\n\n```text\nliteserver.config.random.local port:int = liteserver.config.Local;\n```\n"]
pub struct Local {
    pub port: crate::ton::int,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7cc9453b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { port } = self;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { port })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::liteserver::config::Local;

    fn into_boxed(self) -> crate::ton::liteserver::config::Local {
        crate::ton::liteserver::config::Local::Liteserver_Config_Random_Local(self)
    }
}
