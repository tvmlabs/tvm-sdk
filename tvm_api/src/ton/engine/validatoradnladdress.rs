use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validatorAdnlAddress`\n\n```text\nengine.validatorAdnlAddress id:int256 expire_at:int = engine.ValidatorAdnlAddress;\n```\n"]
pub struct ValidatorAdnlAddress {
    pub id: crate::ton::int256,
    pub expire_at: crate::ton::int,
}
impl Eq for ValidatorAdnlAddress {}
impl crate::BareSerialize for ValidatorAdnlAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd34545be)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatorAdnlAddress { id, expire_at } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int>(expire_at)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatorAdnlAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let expire_at = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, expire_at })
        }
    }
}
impl crate::IntoBoxed for ValidatorAdnlAddress {
    type Boxed = crate::ton::engine::ValidatorAdnlAddress;
    fn into_boxed(self) -> crate::ton::engine::ValidatorAdnlAddress {
        crate::ton::engine::ValidatorAdnlAddress::Engine_ValidatorAdnlAddress(self)
    }
}
