use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportedKey`\n\n```text\nexportedKey word_list:vector<secureString> = ExportedKey;\n```\n"]
pub struct ExportedKey {
    pub word_list: crate::ton::vector<crate::ton::Bare, crate::ton::secureString>,
}
impl Eq for ExportedKey {}
impl crate::BareSerialize for ExportedKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa99e39d7)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportedKey { word_list } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::secureString>>(
            word_list,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportedKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let word_list =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::secureString>>()?;
            Ok(Self { word_list })
        }
    }
}
impl crate::IntoBoxed for ExportedKey {
    type Boxed = crate::ton::ExportedKey;
    fn into_boxed(self) -> crate::ton::ExportedKey {
        crate::ton::ExportedKey::ExportedKey(self)
    }
}
