use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.pong`\n\n```text\nvalidatorSession.pong hash:long = validatorSession.Pong;\n```\n"]
pub struct Pong {
    pub hash: crate::ton::long,
}
impl Eq for Pong {}
impl crate::BareSerialize for Pong {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdcc6376d)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Pong { hash } = self;
        _ser.write_bare::<crate::ton::long>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Pong {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { hash })
        }
    }
}
impl crate::IntoBoxed for Pong {
    type Boxed = crate::ton::validator_session::Pong;
    fn into_boxed(self) -> crate::ton::validator_session::Pong {
        crate::ton::validator_session::Pong::ValidatorSession_Pong(self)
    }
}
