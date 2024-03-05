use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `key`\n\n```text\nkey public_key:string secret:secureBytes = Key;\n```\n"]
pub struct Key {
    pub public_key: crate::ton::string,
    pub secret: crate::ton::secureBytes,
}
impl Eq for Key {}
impl crate::BareSerialize for Key {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8a1493d5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Key { public_key, secret } = self;
        _ser.write_bare::<crate::ton::string>(public_key)?;
        _ser.write_bare::<crate::ton::secureBytes>(secret)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Key {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let public_key = _de.read_bare::<crate::ton::string>()?;
            let secret = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { public_key, secret })
        }
    }
}
impl crate::IntoBoxed for Key {
    type Boxed = crate::ton::Key;

    fn into_boxed(self) -> crate::ton::Key {
        crate::ton::Key::Key(self)
    }
}
