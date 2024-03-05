use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcast.id`\n\n```text\noverlay.broadcast.id src:int256 data_hash:int256 flags:int = overlay.broadcast.Id;\n```\n"]
pub struct Id {
    pub src: crate::ton::int256,
    pub data_hash: crate::ton::int256,
    pub flags: crate::ton::int,
}
impl Eq for Id {}
impl crate::BareSerialize for Id {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x51fd789a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Id { src, data_hash, flags } = self;
        _ser.write_bare::<crate::ton::int256>(src)?;
        _ser.write_bare::<crate::ton::int256>(data_hash)?;
        _ser.write_bare::<crate::ton::int>(flags)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Id {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_bare::<crate::ton::int256>()?;
            let data_hash = _de.read_bare::<crate::ton::int256>()?;
            let flags = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { src, data_hash, flags })
        }
    }
}
impl crate::IntoBoxed for Id {
    type Boxed = crate::ton::overlay::broadcast::Id;

    fn into_boxed(self) -> crate::ton::overlay::broadcast::Id {
        crate::ton::overlay::broadcast::Id::Overlay_Broadcast_Id(self)
    }
}
