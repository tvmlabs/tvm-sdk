use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportedEncryptedKey`\n\n```text\nexportedEncryptedKey data:secureBytes = ExportedEncryptedKey;\n```\n"]
pub struct ExportedEncryptedKey {
    pub data: crate::ton::secureBytes,
}
impl Eq for ExportedEncryptedKey {}
impl crate::BareSerialize for ExportedEncryptedKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x78a9fe54)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportedEncryptedKey { data } = self;
        _ser.write_bare::<crate::ton::secureBytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportedEncryptedKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for ExportedEncryptedKey {
    type Boxed = crate::ton::ExportedEncryptedKey;

    fn into_boxed(self) -> crate::ton::ExportedEncryptedKey {
        crate::ton::ExportedEncryptedKey::ExportedEncryptedKey(self)
    }
}
