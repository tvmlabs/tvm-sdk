use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.message`\n\n```text\nmsg.message destination:accountAddress public_key:string amount:int64 data:msg.Data = msg.Message;\n```\n"]
pub struct Message {
    pub destination: crate::ton::accountaddress::AccountAddress,
    pub public_key: crate::ton::string,
    pub amount: crate::ton::int64,
    pub data: crate::ton::msg::Data,
}
impl Eq for Message {}
impl crate::BareSerialize for Message {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8233d034)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Message { destination, public_key, amount, data } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(destination)?;
        _ser.write_bare::<crate::ton::string>(public_key)?;
        _ser.write_bare::<crate::ton::int64>(amount)?;
        _ser.write_boxed::<crate::ton::msg::Data>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Message {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let destination = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let public_key = _de.read_bare::<crate::ton::string>()?;
            let amount = _de.read_bare::<crate::ton::int64>()?;
            let data = _de.read_boxed::<crate::ton::msg::Data>()?;
            Ok(Self { destination, public_key, amount, data })
        }
    }
}
impl crate::IntoBoxed for Message {
    type Boxed = crate::ton::msg::Message;

    fn into_boxed(self) -> crate::ton::msg::Message {
        crate::ton::msg::Message::Msg_Message(self)
    }
}
