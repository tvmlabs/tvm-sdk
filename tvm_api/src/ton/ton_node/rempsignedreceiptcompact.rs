use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempSignedReceiptCompact`\n\n```text\ntonNode.rempSignedReceiptCompact message_id:int256 receipt:tonNode.RempMessageStatusCompact \n        timestamp:long signature:int512 = tonNode.RempSignedReceiptCompact;\n```\n"]
pub struct RempSignedReceiptCompact {
    pub message_id: crate::ton::int256,
    pub receipt: crate::ton::ton_node::RempMessageStatusCompact,
    pub timestamp: crate::ton::long,
    pub signature: crate::ton::int512,
}
impl Eq for RempSignedReceiptCompact {}
impl crate::BareSerialize for RempSignedReceiptCompact {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9a3cabcf)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempSignedReceiptCompact { message_id, receipt, timestamp, signature } = self;
        _ser.write_bare::<crate::ton::int256>(message_id)?;
        _ser.write_boxed::<crate::ton::ton_node::RempMessageStatusCompact>(receipt)?;
        _ser.write_bare::<crate::ton::long>(timestamp)?;
        _ser.write_bare::<crate::ton::int512>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempSignedReceiptCompact {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message_id = _de.read_bare::<crate::ton::int256>()?;
            let receipt = _de.read_boxed::<crate::ton::ton_node::RempMessageStatusCompact>()?;
            let timestamp = _de.read_bare::<crate::ton::long>()?;
            let signature = _de.read_bare::<crate::ton::int512>()?;
            Ok(Self { message_id, receipt, timestamp, signature })
        }
    }
}
impl crate::IntoBoxed for RempSignedReceiptCompact {
    type Boxed = crate::ton::ton_node::RempSignedReceiptCompact;

    fn into_boxed(self) -> crate::ton::ton_node::RempSignedReceiptCompact {
        crate::ton::ton_node::RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(self)
    }
}
