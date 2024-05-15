use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.fec.completed`\n\n```text\noverlay.fec.completed hash:int256 = overlay.Broadcast;\n```\n"]
pub struct Completed {
    pub hash: crate::ton::int256,
}
impl Eq for Completed {}
impl crate::BareSerialize for Completed {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x09d76914)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Completed { hash } = self;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Completed {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { hash })
        }
    }
}
impl crate::IntoBoxed for Completed {
    type Boxed = crate::ton::overlay::Broadcast;
    fn into_boxed(self) -> crate::ton::overlay::Broadcast {
        crate::ton::overlay::Broadcast::Overlay_Fec_Completed(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.fec.received`\n\n```text\noverlay.fec.received hash:int256 = overlay.Broadcast;\n```\n"]
pub struct Received {
    pub hash: crate::ton::int256,
}
impl Eq for Received {}
impl crate::BareSerialize for Received {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd55c14ec)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Received { hash } = self;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Received {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { hash })
        }
    }
}
impl crate::IntoBoxed for Received {
    type Boxed = crate::ton::overlay::Broadcast;
    fn into_boxed(self) -> crate::ton::overlay::Broadcast {
        crate::ton::overlay::Broadcast::Overlay_Fec_Received(self)
    }
}
