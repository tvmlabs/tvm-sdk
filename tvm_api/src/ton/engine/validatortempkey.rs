use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validatorTempKey`\n\n```text\nengine.validatorTempKey key:int256 expire_at:int = engine.ValidatorTempKey;\n```\n"]
pub struct ValidatorTempKey {
    pub key: crate::ton::int256,
    pub expire_at: crate::ton::int,
}
impl Eq for ValidatorTempKey {}
impl crate::BareSerialize for ValidatorTempKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5e4ad6de)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatorTempKey { key, expire_at } = self;
        _ser.write_bare::<crate::ton::int256>(key)?;
        _ser.write_bare::<crate::ton::int>(expire_at)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatorTempKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::int256>()?;
            let expire_at = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key, expire_at })
        }
    }
}
impl crate::IntoBoxed for ValidatorTempKey {
    type Boxed = crate::ton::engine::ValidatorTempKey;

    fn into_boxed(self) -> crate::ton::engine::ValidatorTempKey {
        crate::ton::engine::ValidatorTempKey::Engine_ValidatorTempKey(self)
    }
}
