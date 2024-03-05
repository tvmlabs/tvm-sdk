use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.certificate`\n\n```text\noverlay.certificate issued_by:PublicKey expire_at:int max_size:int signature:bytes = overlay.Certificate;\n```\n"]
pub struct Certificate {
    pub issued_by: crate::ton::PublicKey,
    pub expire_at: crate::ton::int,
    pub max_size: crate::ton::int,
    pub signature: crate::ton::bytes,
}
impl Eq for Certificate {}
impl crate::Signing for Certificate {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Certificate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe09ed731)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Certificate { issued_by, expire_at, max_size, signature } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(issued_by)?;
        _ser.write_bare::<crate::ton::int>(expire_at)?;
        _ser.write_bare::<crate::ton::int>(max_size)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Certificate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let issued_by = _de.read_boxed::<crate::ton::PublicKey>()?;
            let expire_at = _de.read_bare::<crate::ton::int>()?;
            let max_size = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { issued_by, expire_at, max_size, signature })
        }
    }
}
impl crate::IntoBoxed for Certificate {
    type Boxed = crate::ton::overlay::Certificate;

    fn into_boxed(self) -> crate::ton::overlay::Certificate {
        crate::ton::overlay::Certificate::Overlay_Certificate(self)
    }
}
