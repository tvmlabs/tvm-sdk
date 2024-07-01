use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcastFec.Id`\n\n```text\noverlay.broadcastFec.id src:int256 type:int256 data_hash:int256 size:int flags:int = overlay.broadcastFec.Id;\n```\n"]
pub enum Id {
    Overlay_BroadcastFec_Id(crate::ton::overlay::broadcast_fec::id::Id),
}
impl Id {
    pub fn data_hash(&self) -> &crate::ton::int256 {
        match self {
            Id::Overlay_BroadcastFec_Id(ref x) => &x.data_hash,
        }
    }
    pub fn flags(&self) -> &crate::ton::int {
        match self {
            Id::Overlay_BroadcastFec_Id(ref x) => &x.flags,
        }
    }
    pub fn size(&self) -> &crate::ton::int {
        match self {
            Id::Overlay_BroadcastFec_Id(ref x) => &x.size,
        }
    }
    pub fn src(&self) -> &crate::ton::int256 {
        match self {
            Id::Overlay_BroadcastFec_Id(ref x) => &x.src,
        }
    }
    pub fn type_(&self) -> &crate::ton::int256 {
        match self {
            Id::Overlay_BroadcastFec_Id(ref x) => &x.type_,
        }
    }
    pub fn only(self) -> crate::ton::overlay::broadcast_fec::id::Id {
        match self {
            Id::Overlay_BroadcastFec_Id(x) => x,
        }
    }
}
impl Eq for Id {}
impl Default for Id {
    fn default() -> Self {
        Id::Overlay_BroadcastFec_Id(crate::ton::overlay::broadcast_fec::id::Id::default())
    }
}
impl crate::BoxedSerialize for Id {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Id::Overlay_BroadcastFec_Id(x) => (crate::ConstructorNumber(0xfb3155a6), x),
        }
    }
}
impl crate::BoxedDeserialize for Id {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfb3155a6)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xfb3155a6) => Ok(Id::Overlay_BroadcastFec_Id(
                _de.read_bare::<crate::ton::overlay::broadcast_fec::id::Id>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcastFec.PartId`\n\n```text\noverlay.broadcastFec.partId broadcast_hash:int256 data_hash:int256 seqno:int = overlay.broadcastFec.PartId;\n```\n"]
pub enum PartId {
    Overlay_BroadcastFec_PartId(crate::ton::overlay::broadcast_fec::partid::PartId),
}
impl PartId {
    pub fn broadcast_hash(&self) -> &crate::ton::int256 {
        match self {
            PartId::Overlay_BroadcastFec_PartId(ref x) => &x.broadcast_hash,
        }
    }
    pub fn data_hash(&self) -> &crate::ton::int256 {
        match self {
            PartId::Overlay_BroadcastFec_PartId(ref x) => &x.data_hash,
        }
    }
    pub fn seqno(&self) -> &crate::ton::int {
        match self {
            PartId::Overlay_BroadcastFec_PartId(ref x) => &x.seqno,
        }
    }
    pub fn only(self) -> crate::ton::overlay::broadcast_fec::partid::PartId {
        match self {
            PartId::Overlay_BroadcastFec_PartId(x) => x,
        }
    }
}
impl Eq for PartId {}
impl Default for PartId {
    fn default() -> Self {
        PartId::Overlay_BroadcastFec_PartId(
            crate::ton::overlay::broadcast_fec::partid::PartId::default(),
        )
    }
}
impl crate::BoxedSerialize for PartId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PartId::Overlay_BroadcastFec_PartId(x) => (crate::ConstructorNumber(0xa46962d0), x),
        }
    }
}
impl crate::BoxedDeserialize for PartId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa46962d0)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa46962d0) => Ok(PartId::Overlay_BroadcastFec_PartId(
                _de.read_bare::<crate::ton::overlay::broadcast_fec::partid::PartId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod id;
pub mod partid;
