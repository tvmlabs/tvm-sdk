use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.message`\n\n```text\nraw.message source:accountAddress destination:accountAddress value:int64 fwd_fee:int64 ihr_fee:int64 created_lt:int64 body_hash:bytes msg_data:msg.Data = raw.Message;\n```\n"]
pub struct Message {
    pub source: crate::ton::accountaddress::AccountAddress,
    pub destination: crate::ton::accountaddress::AccountAddress,
    pub value: crate::ton::int64,
    pub fwd_fee: crate::ton::int64,
    pub ihr_fee: crate::ton::int64,
    pub created_lt: crate::ton::int64,
    pub body_hash: crate::ton::bytes,
    pub msg_data: crate::ton::msg::Data,
}
impl Eq for Message {}
impl crate::BareSerialize for Message {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x518b724f)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Message {
            source,
            destination,
            value,
            fwd_fee,
            ihr_fee,
            created_lt,
            body_hash,
            msg_data,
        } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(source)?;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(destination)?;
        _ser.write_bare::<crate::ton::int64>(value)?;
        _ser.write_bare::<crate::ton::int64>(fwd_fee)?;
        _ser.write_bare::<crate::ton::int64>(ihr_fee)?;
        _ser.write_bare::<crate::ton::int64>(created_lt)?;
        _ser.write_bare::<crate::ton::bytes>(body_hash)?;
        _ser.write_boxed::<crate::ton::msg::Data>(msg_data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Message {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let source = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let destination = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let value = _de.read_bare::<crate::ton::int64>()?;
            let fwd_fee = _de.read_bare::<crate::ton::int64>()?;
            let ihr_fee = _de.read_bare::<crate::ton::int64>()?;
            let created_lt = _de.read_bare::<crate::ton::int64>()?;
            let body_hash = _de.read_bare::<crate::ton::bytes>()?;
            let msg_data = _de.read_boxed::<crate::ton::msg::Data>()?;
            Ok(Self {
                source,
                destination,
                value,
                fwd_fee,
                ihr_fee,
                created_lt,
                body_hash,
                msg_data,
            })
        }
    }
}
impl crate::IntoBoxed for Message {
    type Boxed = crate::ton::raw::Message;
    fn into_boxed(self) -> crate::ton::raw::Message {
        crate::ton::raw::Message::Raw_Message(self)
    }
}
