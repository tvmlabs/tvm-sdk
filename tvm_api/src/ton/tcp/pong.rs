use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tcp.pong`\n\n```text\ntcp.pong random_id:long = tcp.Pong;\n```\n"]
pub struct Pong {
    pub random_id: crate::ton::long,
}
impl Eq for Pong {}
impl crate::BareSerialize for Pong {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdc69fb03)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Pong { random_id } = self;
        _ser.write_bare::<crate::ton::long>(random_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Pong {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let random_id = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { random_id })
        }
    }
}
impl crate::IntoBoxed for Pong {
    type Boxed = crate::ton::tcp::Pong;

    fn into_boxed(self) -> crate::ton::tcp::Pong {
        crate::ton::tcp::Pong::Tcp_Pong(self)
    }
}
