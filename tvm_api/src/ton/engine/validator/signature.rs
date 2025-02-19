use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.signature`\n\n```text\nengine.validator.signature signature:bytes = engine.validator.Signature;\n```\n"]
pub struct Signature {
    pub signature: crate::ton::bytes,
}
impl Eq for Signature {}
impl crate::Signing for Signature {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Signature {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfb6c4328)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Signature { signature } = self;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Signature {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { signature })
        }
    }
}
impl crate::IntoBoxed for Signature {
    type Boxed = crate::ton::engine::validator::Signature;

    fn into_boxed(self) -> crate::ton::engine::validator::Signature {
        crate::ton::engine::validator::Signature::Engine_Validator_Signature(self)
    }
}
