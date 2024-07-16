use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.certificateId`\n\n```text\noverlay.certificateId overlay_id:int256 node:int256 expire_at:int max_size:int = overlay.CertificateId;\n```\n"]
pub struct CertificateId {
    pub overlay_id: crate::ton::int256,
    pub node: crate::ton::int256,
    pub expire_at: crate::ton::int,
    pub max_size: crate::ton::int,
}
impl Eq for CertificateId {}
impl crate::BareSerialize for CertificateId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8fae60b9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CertificateId { overlay_id, node, expire_at, max_size } = self;
        _ser.write_bare::<crate::ton::int256>(overlay_id)?;
        _ser.write_bare::<crate::ton::int256>(node)?;
        _ser.write_bare::<crate::ton::int>(expire_at)?;
        _ser.write_bare::<crate::ton::int>(max_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CertificateId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let overlay_id = _de.read_bare::<crate::ton::int256>()?;
            let node = _de.read_bare::<crate::ton::int256>()?;
            let expire_at = _de.read_bare::<crate::ton::int>()?;
            let max_size = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { overlay_id, node, expire_at, max_size })
        }
    }
}
impl crate::IntoBoxed for CertificateId {
    type Boxed = crate::ton::overlay::CertificateId;

    fn into_boxed(self) -> crate::ton::overlay::CertificateId {
        crate::ton::overlay::CertificateId::Overlay_CertificateId(self)
    }
}
