use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempMessageBody`\n\n```text\ntonNode.rempMessageBody message:bytes = tonNode.rempMessageBody;\n```\n"]
pub struct RempMessageBody {
    pub message: crate::ton::bytes,
}
impl Eq for RempMessageBody {}
impl crate::BareSerialize for RempMessageBody {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x594d0088)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempMessageBody { message } = self;
        _ser.write_bare::<crate::ton::bytes>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempMessageBody {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { message })
        }
    }
}
impl crate::IntoBoxed for RempMessageBody {
    type Boxed = crate::ton::ton_node::RempMessageBody;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageBody {
        crate::ton::ton_node::RempMessageBody::TonNode_RempMessageBody(self)
    }
}
