use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportedUnencryptedKey`\n\n```text\nexportedUnencryptedKey data:secureBytes = ExportedUnencryptedKey;\n```\n"]
pub struct ExportedUnencryptedKey {
    pub data: crate::ton::secureBytes,
}
impl Eq for ExportedUnencryptedKey {}
impl crate::BareSerialize for ExportedUnencryptedKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2b839ae8)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportedUnencryptedKey { data } = self;
        _ser.write_bare::<crate::ton::secureBytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportedUnencryptedKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for ExportedUnencryptedKey {
    type Boxed = crate::ton::ExportedUnencryptedKey;
    fn into_boxed(self) -> crate::ton::ExportedUnencryptedKey {
        crate::ton::ExportedUnencryptedKey::ExportedUnencryptedKey(self)
    }
}
