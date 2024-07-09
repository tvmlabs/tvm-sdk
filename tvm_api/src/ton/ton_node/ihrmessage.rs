use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.ihrMessage`\n\n```text\ntonNode.ihrMessage data:bytes = tonNode.IhrMessage;\n```\n"]
pub struct IhrMessage {
    pub data: crate::ton::bytes,
}
impl Eq for IhrMessage {}
impl crate::BareSerialize for IhrMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4534c307)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let IhrMessage { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for IhrMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for IhrMessage {
    type Boxed = crate::ton::ton_node::IhrMessage;
    fn into_boxed(self) -> crate::ton::ton_node::IhrMessage {
        crate::ton::ton_node::IhrMessage::TonNode_IhrMessage(self)
    }
}
