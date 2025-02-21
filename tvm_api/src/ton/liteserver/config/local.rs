use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteserver.config.local`\n\n```text\nliteserver.config.local id:PrivateKey port:int = liteserver.config.Local;\n```\n"]
pub struct Local {
    pub id: crate::ton::PrivateKey,
    pub port: crate::ton::int,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4673eb8f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { id, port } = self;
        _ser.write_boxed::<crate::ton::PrivateKey>(id)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_boxed::<crate::ton::PrivateKey>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, port })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::liteserver::config::Local;

    fn into_boxed(self) -> crate::ton::liteserver::config::Local {
        crate::ton::liteserver::config::Local::Liteserver_Config_Local(self)
    }
}
