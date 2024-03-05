use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteserver.desc`\n\n```text\nliteserver.desc id:PublicKey ip:int port:int = liteserver.Desc;\n```\n"]
pub struct Desc {
    pub id: crate::ton::PublicKey,
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
}
impl Eq for Desc {}
impl crate::BareSerialize for Desc {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc449a474)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Desc { id, ip, port } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(id)?;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Desc {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_boxed::<crate::ton::PublicKey>()?;
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, ip, port })
        }
    }
}
impl crate::IntoBoxed for Desc {
    type Boxed = crate::ton::liteserver::Desc;

    fn into_boxed(self) -> crate::ton::liteserver::Desc {
        crate::ton::liteserver::Desc::Liteserver_Desc(self)
    }
}
