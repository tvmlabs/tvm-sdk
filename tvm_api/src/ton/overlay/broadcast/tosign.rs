use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcast.toSign`\n\n```text\noverlay.broadcast.toSign hash:int256 date:int = overlay.broadcast.ToSign;\n```\n"]
pub struct ToSign {
    pub hash: crate::ton::int256,
    pub date: crate::ton::int,
}
impl Eq for ToSign {}
impl crate::BareSerialize for ToSign {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfa374e7c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ToSign { hash, date } = self;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        _ser.write_bare::<crate::ton::int>(date)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ToSign {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::int256>()?;
            let date = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { hash, date })
        }
    }
}
impl crate::IntoBoxed for ToSign {
    type Boxed = crate::ton::overlay::broadcast::ToSign;

    fn into_boxed(self) -> crate::ton::overlay::broadcast::ToSign {
        crate::ton::overlay::broadcast::ToSign::Overlay_Broadcast_ToSign(self)
    }
}
