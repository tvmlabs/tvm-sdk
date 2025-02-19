use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcastFec.id`\n\n```text\noverlay.broadcastFec.id src:int256 type:int256 data_hash:int256 size:int flags:int = overlay.broadcastFec.Id;\n```\n"]
pub struct Id {
    pub src: crate::ton::int256,
    pub type_: crate::ton::int256,
    pub data_hash: crate::ton::int256,
    pub size: crate::ton::int,
    pub flags: crate::ton::int,
}
impl Eq for Id {}
impl crate::BareSerialize for Id {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfb3155a6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Id { src, type_, data_hash, size, flags } = self;
        _ser.write_bare::<crate::ton::int256>(src)?;
        _ser.write_bare::<crate::ton::int256>(type_)?;
        _ser.write_bare::<crate::ton::int256>(data_hash)?;
        _ser.write_bare::<crate::ton::int>(size)?;
        _ser.write_bare::<crate::ton::int>(flags)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Id {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_bare::<crate::ton::int256>()?;
            let type_ = _de.read_bare::<crate::ton::int256>()?;
            let data_hash = _de.read_bare::<crate::ton::int256>()?;
            let size = _de.read_bare::<crate::ton::int>()?;
            let flags = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { src, type_, data_hash, size, flags })
        }
    }
}
impl crate::IntoBoxed for Id {
    type Boxed = crate::ton::overlay::broadcast_fec::Id;

    fn into_boxed(self) -> crate::ton::overlay::broadcast_fec::Id {
        crate::ton::overlay::broadcast_fec::Id::Overlay_BroadcastFec_Id(self)
    }
}
