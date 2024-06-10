use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportedPemKey`\n\n```text\nexportedPemKey pem:secureString = ExportedPemKey;\n```\n"]
pub struct ExportedPemKey {
    pub pem: crate::ton::secureString,
}
impl Eq for ExportedPemKey {}
impl crate::BareSerialize for ExportedPemKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x54f700bd)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportedPemKey { pem } = self;
        _ser.write_bare::<crate::ton::secureString>(pem)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportedPemKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let pem = _de.read_bare::<crate::ton::secureString>()?;
            Ok(Self { pem })
        }
    }
}
impl crate::IntoBoxed for ExportedPemKey {
    type Boxed = crate::ton::ExportedPemKey;
    fn into_boxed(self) -> crate::ton::ExportedPemKey {
        crate::ton::ExportedPemKey::ExportedPemKey(self)
    }
}
