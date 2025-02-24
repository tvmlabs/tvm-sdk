use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `keyStoreTypeDirectory`\n\n```text\nkeyStoreTypeDirectory directory:string = KeyStoreType;\n```\n"]
pub struct KeyStoreTypeDirectory {
    pub directory: crate::ton::string,
}
impl Eq for KeyStoreTypeDirectory {}
impl crate::BareSerialize for KeyStoreTypeDirectory {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe969122a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let KeyStoreTypeDirectory { directory } = self;
        _ser.write_bare::<crate::ton::string>(directory)?;
        Ok(())
    }
}
impl crate::BareDeserialize for KeyStoreTypeDirectory {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let directory = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { directory })
        }
    }
}
impl crate::IntoBoxed for KeyStoreTypeDirectory {
    type Boxed = crate::ton::KeyStoreType;

    fn into_boxed(self) -> crate::ton::KeyStoreType {
        crate::ton::KeyStoreType::KeyStoreTypeDirectory(self)
    }
}
