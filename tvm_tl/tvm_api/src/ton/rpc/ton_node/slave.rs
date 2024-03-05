use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.slave.sendExtMessage`\n\n```text\ntonNode.slave.sendExtMessage message:tonNode.externalMessage = tonNode.Success;\n```\n"]
pub struct SendExtMessage {
    pub message: crate::ton::ton_node::externalmessage::ExternalMessage,
}
impl Eq for SendExtMessage {}
impl crate::BareSerialize for SendExtMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0376f2a9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SendExtMessage { message } = self;
        _ser.write_bare::<crate::ton::ton_node::externalmessage::ExternalMessage>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SendExtMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message =
                _de.read_bare::<crate::ton::ton_node::externalmessage::ExternalMessage>()?;
            Ok(Self { message })
        }
    }
}
impl crate::BoxedDeserialize for SendExtMessage {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0376f2a9)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x0376f2a9) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SendExtMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x0376f2a9), self)
    }
}
impl crate::Function for SendExtMessage {
    type Reply = crate::ton::ton_node::Success;
}
