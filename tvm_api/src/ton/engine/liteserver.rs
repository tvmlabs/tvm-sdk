use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.liteServer`\n\n```text\nengine.liteServer id:int256 port:int = engine.LiteServer;\n```\n"]
pub struct LiteServer {
    pub id: crate::ton::int256,
    pub port: crate::ton::int,
}
impl Eq for LiteServer {}
impl crate::BareSerialize for LiteServer {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbb708efe)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let LiteServer { id, port } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for LiteServer {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, port })
        }
    }
}
impl crate::IntoBoxed for LiteServer {
    type Boxed = crate::ton::engine::LiteServer;

    fn into_boxed(self) -> crate::ton::engine::LiteServer {
        crate::ton::engine::LiteServer::Engine_LiteServer(self)
    }
}
