use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcastFec.partId`\n\n```text\noverlay.broadcastFec.partId broadcast_hash:int256 data_hash:int256 seqno:int = overlay.broadcastFec.PartId;\n```\n"]
pub struct PartId {
    pub broadcast_hash: crate::ton::int256,
    pub data_hash: crate::ton::int256,
    pub seqno: crate::ton::int,
}
impl Eq for PartId {}
impl crate::BareSerialize for PartId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa46962d0)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PartId { broadcast_hash, data_hash, seqno } = self;
        _ser.write_bare::<crate::ton::int256>(broadcast_hash)?;
        _ser.write_bare::<crate::ton::int256>(data_hash)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PartId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let broadcast_hash = _de.read_bare::<crate::ton::int256>()?;
            let data_hash = _de.read_bare::<crate::ton::int256>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { broadcast_hash, data_hash, seqno })
        }
    }
}
impl crate::IntoBoxed for PartId {
    type Boxed = crate::ton::overlay::broadcast_fec::PartId;

    fn into_boxed(self) -> crate::ton::overlay::broadcast_fec::PartId {
        crate::ton::overlay::broadcast_fec::PartId::Overlay_BroadcastFec_PartId(self)
    }
}
