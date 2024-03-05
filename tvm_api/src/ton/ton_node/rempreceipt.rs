use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempReceipt`\n\n```text\ntonNode.rempReceipt message_id:int256 status:tonNode.RempMessageStatus timestamp:long source_id:int256 = tonNode.RempReceipt;\n```\n"]
pub struct RempReceipt {
    pub message_id: crate::ton::int256,
    pub status: crate::ton::ton_node::RempMessageStatus,
    pub timestamp: crate::ton::long,
    pub source_id: crate::ton::int256,
}
impl Eq for RempReceipt {}
impl crate::BareSerialize for RempReceipt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3122b7a2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempReceipt { message_id, status, timestamp, source_id } = self;
        _ser.write_bare::<crate::ton::int256>(message_id)?;
        _ser.write_boxed::<crate::ton::ton_node::RempMessageStatus>(status)?;
        _ser.write_bare::<crate::ton::long>(timestamp)?;
        _ser.write_bare::<crate::ton::int256>(source_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempReceipt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message_id = _de.read_bare::<crate::ton::int256>()?;
            let status = _de.read_boxed::<crate::ton::ton_node::RempMessageStatus>()?;
            let timestamp = _de.read_bare::<crate::ton::long>()?;
            let source_id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { message_id, status, timestamp, source_id })
        }
    }
}
impl crate::IntoBoxed for RempReceipt {
    type Boxed = crate::ton::ton_node::RempReceipt;

    fn into_boxed(self) -> crate::ton::ton_node::RempReceipt {
        crate::ton::ton_node::RempReceipt::TonNode_RempReceipt(self)
    }
}
