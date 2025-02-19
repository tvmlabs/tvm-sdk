use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempMessage`\n\n```text\ntonNode.rempMessage message:bytes id:int256 timestamp:long signature:bytes = tonNode.RempMessage;\n```\n"]
pub struct RempMessage {
    pub message: crate::ton::bytes,
    pub id: crate::ton::int256,
    pub timestamp: crate::ton::long,
    pub signature: crate::ton::bytes,
}
impl Eq for RempMessage {}
impl crate::Signing for RempMessage {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for RempMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdd1f6db1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempMessage { message, id, timestamp, signature } = self;
        _ser.write_bare::<crate::ton::bytes>(message)?;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::long>(timestamp)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message = _de.read_bare::<crate::ton::bytes>()?;
            let id = _de.read_bare::<crate::ton::int256>()?;
            let timestamp = _de.read_bare::<crate::ton::long>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { message, id, timestamp, signature })
        }
    }
}
impl crate::IntoBoxed for RempMessage {
    type Boxed = crate::ton::ton_node::RempMessage;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessage {
        crate::ton::ton_node::RempMessage::TonNode_RempMessage(self)
    }
}
