use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.message`\n\n```text\noverlay.message overlay:int256 = overlay.Message;\n```\n"]
pub struct Message {
    pub overlay: crate::ton::int256,
}
impl Eq for Message {}
impl crate::BareSerialize for Message {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x75252420)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Message { overlay } = self;
        _ser.write_bare::<crate::ton::int256>(overlay)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Message {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let overlay = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { overlay })
        }
    }
}
impl crate::IntoBoxed for Message {
    type Boxed = crate::ton::overlay::Message;

    fn into_boxed(self) -> crate::ton::overlay::Message {
        crate::ton::overlay::Message::Overlay_Message(self)
    }
}
