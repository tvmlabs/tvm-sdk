use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.message.answer`\n\n```text\nadnl.message.answer query_id:int256 answer:bytes = adnl.Message;\n```\n"]
pub struct Answer {
    pub query_id: crate::ton::int256,
    pub answer: crate::ton::bytes,
}
impl Eq for Answer {}
impl crate::BareSerialize for Answer {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0fac8416)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Answer { query_id, answer } = self;
        _ser.write_bare::<crate::ton::int256>(query_id)?;
        _ser.write_bare::<crate::ton::bytes>(answer)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Answer {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let query_id = _de.read_bare::<crate::ton::int256>()?;
            let answer = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { query_id, answer })
        }
    }
}
impl crate::IntoBoxed for Answer {
    type Boxed = crate::ton::adnl::Message;

    fn into_boxed(self) -> crate::ton::adnl::Message {
        crate::ton::adnl::Message::Adnl_Message_Answer(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.message.confirmChannel`\n\n```text\nadnl.message.confirmChannel key:int256 peer_key:int256 date:int = adnl.Message;\n```\n"]
pub struct ConfirmChannel {
    pub key: crate::ton::int256,
    pub peer_key: crate::ton::int256,
    pub date: crate::ton::int,
}
impl Eq for ConfirmChannel {}
impl crate::BareSerialize for ConfirmChannel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x60dd1d69)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ConfirmChannel { key, peer_key, date } = self;
        _ser.write_bare::<crate::ton::int256>(key)?;
        _ser.write_bare::<crate::ton::int256>(peer_key)?;
        _ser.write_bare::<crate::ton::int>(date)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ConfirmChannel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::int256>()?;
            let peer_key = _de.read_bare::<crate::ton::int256>()?;
            let date = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key, peer_key, date })
        }
    }
}
impl crate::IntoBoxed for ConfirmChannel {
    type Boxed = crate::ton::adnl::Message;

    fn into_boxed(self) -> crate::ton::adnl::Message {
        crate::ton::adnl::Message::Adnl_Message_ConfirmChannel(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.message.createChannel`\n\n```text\nadnl.message.createChannel key:int256 date:int = adnl.Message;\n```\n"]
pub struct CreateChannel {
    pub key: crate::ton::int256,
    pub date: crate::ton::int,
}
impl Eq for CreateChannel {}
impl crate::BareSerialize for CreateChannel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe673c3bb)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateChannel { key, date } = self;
        _ser.write_bare::<crate::ton::int256>(key)?;
        _ser.write_bare::<crate::ton::int>(date)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateChannel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::int256>()?;
            let date = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key, date })
        }
    }
}
impl crate::IntoBoxed for CreateChannel {
    type Boxed = crate::ton::adnl::Message;

    fn into_boxed(self) -> crate::ton::adnl::Message {
        crate::ton::adnl::Message::Adnl_Message_CreateChannel(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.message.custom`\n\n```text\nadnl.message.custom data:bytes = adnl.Message;\n```\n"]
pub struct Custom {
    pub data: crate::ton::bytes,
}
impl Eq for Custom {}
impl crate::BareSerialize for Custom {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x204818f5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Custom { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Custom {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for Custom {
    type Boxed = crate::ton::adnl::Message;

    fn into_boxed(self) -> crate::ton::adnl::Message {
        crate::ton::adnl::Message::Adnl_Message_Custom(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.message.part`\n\n```text\nadnl.message.part hash:int256 total_size:int offset:int data:bytes = adnl.Message;\n```\n"]
pub struct Part {
    pub hash: crate::ton::int256,
    pub total_size: crate::ton::int,
    pub offset: crate::ton::int,
    pub data: crate::ton::bytes,
}
impl Eq for Part {}
impl crate::BareSerialize for Part {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfd452d39)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Part { hash, total_size, offset, data } = self;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        _ser.write_bare::<crate::ton::int>(total_size)?;
        _ser.write_bare::<crate::ton::int>(offset)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Part {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::int256>()?;
            let total_size = _de.read_bare::<crate::ton::int>()?;
            let offset = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { hash, total_size, offset, data })
        }
    }
}
impl crate::IntoBoxed for Part {
    type Boxed = crate::ton::adnl::Message;

    fn into_boxed(self) -> crate::ton::adnl::Message {
        crate::ton::adnl::Message::Adnl_Message_Part(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.message.query`\n\n```text\nadnl.message.query query_id:int256 query:bytes = adnl.Message;\n```\n"]
pub struct Query {
    pub query_id: crate::ton::int256,
    pub query: crate::ton::bytes,
}
impl Eq for Query {}
impl crate::BareSerialize for Query {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb48bf97a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Query { query_id, query } = self;
        _ser.write_bare::<crate::ton::int256>(query_id)?;
        _ser.write_bare::<crate::ton::bytes>(query)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Query {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let query_id = _de.read_bare::<crate::ton::int256>()?;
            let query = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { query_id, query })
        }
    }
}
impl crate::IntoBoxed for Query {
    type Boxed = crate::ton::adnl::Message;

    fn into_boxed(self) -> crate::ton::adnl::Message {
        crate::ton::adnl::Message::Adnl_Message_Query(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.message.reinit`\n\n```text\nadnl.message.reinit date:int = adnl.Message;\n```\n"]
pub struct Reinit {
    pub date: crate::ton::int,
}
impl Eq for Reinit {}
impl crate::BareSerialize for Reinit {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x10c20520)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Reinit { date } = self;
        _ser.write_bare::<crate::ton::int>(date)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Reinit {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let date = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { date })
        }
    }
}
impl crate::IntoBoxed for Reinit {
    type Boxed = crate::ton::adnl::Message;

    fn into_boxed(self) -> crate::ton::adnl::Message {
        crate::ton::adnl::Message::Adnl_Message_Reinit(self)
    }
}
