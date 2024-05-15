use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `bip39Hints`\n\n```text\nbip39Hints words:vector<string> = Bip39Hints;\n```\n"]
pub struct Bip39Hints {
    pub words: crate::ton::vector<crate::ton::Bare, crate::ton::string>,
}
impl Eq for Bip39Hints {}
impl crate::BareSerialize for Bip39Hints {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3c559c00)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Bip39Hints { words } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::string>>(words)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Bip39Hints {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let words =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::string>>()?;
            Ok(Self { words })
        }
    }
}
impl crate::IntoBoxed for Bip39Hints {
    type Boxed = crate::ton::Bip39Hints;
    fn into_boxed(self) -> crate::ton::Bip39Hints {
        crate::ton::Bip39Hints::Bip39Hints(self)
    }
}
