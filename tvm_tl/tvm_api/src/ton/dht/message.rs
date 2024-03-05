use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.message`\n\n```text\ndht.message node:dht.node = dht.Message;\n```\n"]
pub struct Message {
    pub node: crate::ton::dht::node::Node,
}
impl Eq for Message {}
impl crate::BareSerialize for Message {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbc0cdb8e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Message { node } = self;
        _ser.write_bare::<crate::ton::dht::node::Node>(node)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Message {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let node = _de.read_bare::<crate::ton::dht::node::Node>()?;
            Ok(Self { node })
        }
    }
}
impl crate::IntoBoxed for Message {
    type Boxed = crate::ton::dht::Message;

    fn into_boxed(self) -> crate::ton::dht::Message {
        crate::ton::dht::Message::Dht_Message(self)
    }
}
