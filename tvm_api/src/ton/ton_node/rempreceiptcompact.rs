use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempReceiptCompact`\n\n```text\ntonNode.rempReceiptCompact message_id:int256 receipt:tonNode.RempMessageStatusCompact \n        timestamp:long = tonNode.RempReceiptCompact;\n```\n"]
pub struct RempReceiptCompact {
    pub message_id: crate::ton::int256,
    pub receipt: crate::ton::ton_node::RempMessageStatusCompact,
    pub timestamp: crate::ton::long,
}
impl Eq for RempReceiptCompact {}
impl crate::BareSerialize for RempReceiptCompact {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x411c6a07)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempReceiptCompact { message_id, receipt, timestamp } = self;
        _ser.write_bare::<crate::ton::int256>(message_id)?;
        _ser.write_boxed::<crate::ton::ton_node::RempMessageStatusCompact>(receipt)?;
        _ser.write_bare::<crate::ton::long>(timestamp)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempReceiptCompact {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message_id = _de.read_bare::<crate::ton::int256>()?;
            let receipt = _de.read_boxed::<crate::ton::ton_node::RempMessageStatusCompact>()?;
            let timestamp = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { message_id, receipt, timestamp })
        }
    }
}
impl crate::IntoBoxed for RempReceiptCompact {
    type Boxed = crate::ton::ton_node::RempReceiptCompact;

    fn into_boxed(self) -> crate::ton::ton_node::RempReceiptCompact {
        crate::ton::ton_node::RempReceiptCompact::TonNode_RempReceiptCompact(self)
    }
}
