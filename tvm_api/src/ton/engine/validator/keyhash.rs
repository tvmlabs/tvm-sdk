use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.keyHash`\n\n```text\nengine.validator.keyHash key_hash:int256 = engine.validator.KeyHash;\n```\n"]
pub struct KeyHash {
    pub key_hash: crate::ton::int256,
}
impl Eq for KeyHash {}
impl crate::BareSerialize for KeyHash {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc2c6a54e)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let KeyHash { key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for KeyHash {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key_hash })
        }
    }
}
impl crate::IntoBoxed for KeyHash {
    type Boxed = crate::ton::engine::validator::KeyHash;
    fn into_boxed(self) -> crate::ton::engine::validator::KeyHash {
        crate::ton::engine::validator::KeyHash::Engine_Validator_KeyHash(self)
    }
}
