use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.externalMessage`\n\n```text\ntonNode.externalMessage data:bytes = tonNode.ExternalMessage;\n```\n"]
pub struct ExternalMessage {
    pub data: crate::ton::bytes,
}
impl Eq for ExternalMessage {}
impl crate::BareSerialize for ExternalMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdc75a209)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExternalMessage { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExternalMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for ExternalMessage {
    type Boxed = crate::ton::ton_node::ExternalMessage;
    fn into_boxed(self) -> crate::ton::ton_node::ExternalMessage {
        crate::ton::ton_node::ExternalMessage::TonNode_ExternalMessage(self)
    }
}
