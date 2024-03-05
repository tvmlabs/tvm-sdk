use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.answer`\n\n```text\nrldp.answer query_id:int256 data:bytes = rldp.Message;\n```\n"]
pub struct Answer {
    pub query_id: crate::ton::int256,
    pub data: crate::ton::bytes,
}
impl Eq for Answer {}
impl crate::BareSerialize for Answer {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa3fc5c03)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Answer { query_id, data } = self;
        _ser.write_bare::<crate::ton::int256>(query_id)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Answer {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let query_id = _de.read_bare::<crate::ton::int256>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { query_id, data })
        }
    }
}
impl crate::IntoBoxed for Answer {
    type Boxed = crate::ton::rldp::Message;

    fn into_boxed(self) -> crate::ton::rldp::Message {
        crate::ton::rldp::Message::Rldp_Answer(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.message`\n\n```text\nrldp.message id:int256 data:bytes = rldp.Message;\n```\n"]
pub struct Message {
    pub id: crate::ton::int256,
    pub data: crate::ton::bytes,
}
impl Eq for Message {}
impl crate::BareSerialize for Message {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7d1bcd1e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Message { id, data } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Message {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, data })
        }
    }
}
impl crate::IntoBoxed for Message {
    type Boxed = crate::ton::rldp::Message;

    fn into_boxed(self) -> crate::ton::rldp::Message {
        crate::ton::rldp::Message::Rldp_Message(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.query`\n\n```text\nrldp.query query_id:int256 max_answer_size:long timeout:int data:bytes = rldp.Message;\n```\n"]
pub struct Query {
    pub query_id: crate::ton::int256,
    pub max_answer_size: crate::ton::long,
    pub timeout: crate::ton::int,
    pub data: crate::ton::bytes,
}
impl Eq for Query {}
impl crate::BareSerialize for Query {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8a794d69)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Query { query_id, max_answer_size, timeout, data } = self;
        _ser.write_bare::<crate::ton::int256>(query_id)?;
        _ser.write_bare::<crate::ton::long>(max_answer_size)?;
        _ser.write_bare::<crate::ton::int>(timeout)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Query {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let query_id = _de.read_bare::<crate::ton::int256>()?;
            let max_answer_size = _de.read_bare::<crate::ton::long>()?;
            let timeout = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { query_id, max_answer_size, timeout, data })
        }
    }
}
impl crate::IntoBoxed for Query {
    type Boxed = crate::ton::rldp::Message;

    fn into_boxed(self) -> crate::ton::rldp::Message {
        crate::ton::rldp::Message::Rldp_Query(self)
    }
}
