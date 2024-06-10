use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.pong`\n\n```text\nadnl.pong value:long = adnl.Pong;\n```\n"]
pub struct Pong {
    pub value: crate::ton::long,
}
impl Eq for Pong {}
impl crate::BareSerialize for Pong {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x20747c0e)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Pong { value } = self;
        _ser.write_bare::<crate::ton::long>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Pong {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Pong {
    type Boxed = crate::ton::adnl::Pong;
    fn into_boxed(self) -> crate::ton::adnl::Pong {
        crate::ton::adnl::Pong::Adnl_Pong(self)
    }
}
