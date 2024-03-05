use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.Message`\n\n```text\nrldp.answer query_id:int256 data:bytes = rldp.Message;\n\nrldp.message id:int256 data:bytes = rldp.Message;\n\nrldp.query query_id:int256 max_answer_size:long timeout:int data:bytes = rldp.Message;\n```\n"]
pub enum Message {
    Rldp_Answer(crate::ton::rldp::message::Answer),
    Rldp_Message(crate::ton::rldp::message::Message),
    Rldp_Query(crate::ton::rldp::message::Query),
}
impl Message {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            Message::Rldp_Answer(ref x) => &x.data,
            Message::Rldp_Message(ref x) => &x.data,
            Message::Rldp_Query(ref x) => &x.data,
        }
    }

    pub fn id(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::Rldp_Message(ref x) => Some(&x.id),
            _ => None,
        }
    }

    pub fn max_answer_size(&self) -> Option<&crate::ton::long> {
        match self {
            Message::Rldp_Query(ref x) => Some(&x.max_answer_size),
            _ => None,
        }
    }

    pub fn query_id(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::Rldp_Answer(ref x) => Some(&x.query_id),
            Message::Rldp_Query(ref x) => Some(&x.query_id),
            _ => None,
        }
    }

    pub fn timeout(&self) -> Option<&crate::ton::int> {
        match self {
            Message::Rldp_Query(ref x) => Some(&x.timeout),
            _ => None,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message::Rldp_Answer(crate::ton::rldp::message::Answer::default())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::Rldp_Answer(x) => (crate::ConstructorNumber(0xa3fc5c03), x),
            Message::Rldp_Message(x) => (crate::ConstructorNumber(0x7d1bcd1e), x),
            Message::Rldp_Query(x) => (crate::ConstructorNumber(0x8a794d69), x),
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xa3fc5c03),
            crate::ConstructorNumber(0x7d1bcd1e),
            crate::ConstructorNumber(0x8a794d69),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa3fc5c03) => {
                Ok(Message::Rldp_Answer(_de.read_bare::<crate::ton::rldp::message::Answer>()?))
            }
            crate::ConstructorNumber(0x7d1bcd1e) => {
                Ok(Message::Rldp_Message(_de.read_bare::<crate::ton::rldp::message::Message>()?))
            }
            crate::ConstructorNumber(0x8a794d69) => {
                Ok(Message::Rldp_Query(_de.read_bare::<crate::ton::rldp::message::Query>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.MessagePart`\n\n```text\nrldp.complete transfer_id:int256 part:int = rldp.MessagePart;\n\nrldp.confirm transfer_id:int256 part:int seqno:int = rldp.MessagePart;\n\nrldp.messagePart transfer_id:int256 fec_type:fec.Type part:int total_size:long seqno:int data:bytes = rldp.MessagePart;\n```\n"]
pub enum MessagePart {
    Rldp_Complete(crate::ton::rldp::messagepart::Complete),
    Rldp_Confirm(crate::ton::rldp::messagepart::Confirm),
    Rldp_MessagePart(crate::ton::rldp::messagepart::MessagePart),
}
impl MessagePart {
    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            MessagePart::Rldp_MessagePart(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn fec_type(&self) -> Option<&crate::ton::fec::Type> {
        match self {
            MessagePart::Rldp_MessagePart(ref x) => Some(&x.fec_type),
            _ => None,
        }
    }

    pub fn part(&self) -> &crate::ton::int {
        match self {
            MessagePart::Rldp_Complete(ref x) => &x.part,
            MessagePart::Rldp_Confirm(ref x) => &x.part,
            MessagePart::Rldp_MessagePart(ref x) => &x.part,
        }
    }

    pub fn seqno(&self) -> Option<&crate::ton::int> {
        match self {
            MessagePart::Rldp_Confirm(ref x) => Some(&x.seqno),
            MessagePart::Rldp_MessagePart(ref x) => Some(&x.seqno),
            _ => None,
        }
    }

    pub fn total_size(&self) -> Option<&crate::ton::long> {
        match self {
            MessagePart::Rldp_MessagePart(ref x) => Some(&x.total_size),
            _ => None,
        }
    }

    pub fn transfer_id(&self) -> &crate::ton::int256 {
        match self {
            MessagePart::Rldp_Complete(ref x) => &x.transfer_id,
            MessagePart::Rldp_Confirm(ref x) => &x.transfer_id,
            MessagePart::Rldp_MessagePart(ref x) => &x.transfer_id,
        }
    }
}
impl Eq for MessagePart {}
impl Default for MessagePart {
    fn default() -> Self {
        MessagePart::Rldp_Complete(crate::ton::rldp::messagepart::Complete::default())
    }
}
impl crate::BoxedSerialize for MessagePart {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            MessagePart::Rldp_Complete(x) => (crate::ConstructorNumber(0xbc0cb2bf), x),
            MessagePart::Rldp_Confirm(x) => (crate::ConstructorNumber(0xf582dc58), x),
            MessagePart::Rldp_MessagePart(x) => (crate::ConstructorNumber(0x185c22cc), x),
        }
    }
}
impl crate::BoxedDeserialize for MessagePart {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xbc0cb2bf),
            crate::ConstructorNumber(0xf582dc58),
            crate::ConstructorNumber(0x185c22cc),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbc0cb2bf) => Ok(MessagePart::Rldp_Complete(
                _de.read_bare::<crate::ton::rldp::messagepart::Complete>()?,
            )),
            crate::ConstructorNumber(0xf582dc58) => Ok(MessagePart::Rldp_Confirm(
                _de.read_bare::<crate::ton::rldp::messagepart::Confirm>()?,
            )),
            crate::ConstructorNumber(0x185c22cc) => Ok(MessagePart::Rldp_MessagePart(
                _de.read_bare::<crate::ton::rldp::messagepart::MessagePart>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod message;
pub mod messagepart;
