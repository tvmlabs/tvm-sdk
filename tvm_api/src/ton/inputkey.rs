use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `inputKeyRegular`\n\n```text\ninputKeyRegular key:key local_password:secureBytes = InputKey;\n```\n"]
pub struct InputKeyRegular {
    pub key: crate::ton::key::Key,
    pub local_password: crate::ton::secureBytes,
}
impl Eq for InputKeyRegular {}
impl crate::BareSerialize for InputKeyRegular {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdee5469e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let InputKeyRegular { key, local_password } = self;
        _ser.write_bare::<crate::ton::key::Key>(key)?;
        _ser.write_bare::<crate::ton::secureBytes>(local_password)?;
        Ok(())
    }
}
impl crate::BareDeserialize for InputKeyRegular {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::key::Key>()?;
            let local_password = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { key, local_password })
        }
    }
}
impl crate::IntoBoxed for InputKeyRegular {
    type Boxed = crate::ton::InputKey;

    fn into_boxed(self) -> crate::ton::InputKey {
        crate::ton::InputKey::InputKeyRegular(self)
    }
}
