use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.findNode`\n\n```text\ndht.findNode key:int256 k:int = dht.Nodes;\n```\n"]
pub struct FindNode {
    pub key: crate::ton::int256,
    pub k: crate::ton::int,
}
impl Eq for FindNode {}
impl crate::BareSerialize for FindNode {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6ce2ce6b)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let FindNode { key, k } = self;
        _ser.write_bare::<crate::ton::int256>(key)?;
        _ser.write_bare::<crate::ton::int>(k)?;
        Ok(())
    }
}
impl crate::BareDeserialize for FindNode {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::int256>()?;
            let k = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key, k })
        }
    }
}
impl crate::BoxedDeserialize for FindNode {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6ce2ce6b)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6ce2ce6b) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for FindNode {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6ce2ce6b), self)
    }
}
impl crate::Function for FindNode {
    type Reply = crate::ton::dht::Nodes;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.findValue`\n\n```text\ndht.findValue key:int256 k:int = dht.ValueResult;\n```\n"]
pub struct FindValue {
    pub key: crate::ton::int256,
    pub k: crate::ton::int,
}
impl Eq for FindValue {}
impl crate::BareSerialize for FindValue {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xae4b6011)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let FindValue { key, k } = self;
        _ser.write_bare::<crate::ton::int256>(key)?;
        _ser.write_bare::<crate::ton::int>(k)?;
        Ok(())
    }
}
impl crate::BareDeserialize for FindValue {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::int256>()?;
            let k = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key, k })
        }
    }
}
impl crate::BoxedDeserialize for FindValue {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xae4b6011)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xae4b6011) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for FindValue {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xae4b6011), self)
    }
}
impl crate::Function for FindValue {
    type Reply = crate::ton::dht::ValueResult;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.getSignedAddressList`\n\n```text\ndht.getSignedAddressList = dht.Node;\n```\n"]
pub struct GetSignedAddressList;
impl Eq for GetSignedAddressList {}
impl crate::BareSerialize for GetSignedAddressList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa97948ed)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetSignedAddressList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetSignedAddressList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa97948ed)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa97948ed) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for GetSignedAddressList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa97948ed), self)
    }
}
impl crate::Function for GetSignedAddressList {
    type Reply = crate::ton::dht::Node;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.ping`\n\n```text\ndht.ping random_id:long = dht.Pong;\n```\n"]
pub struct Ping {
    pub random_id: crate::ton::long,
}
impl Eq for Ping {}
impl crate::BareSerialize for Ping {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcbeb3f18)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Ping { random_id } = self;
        _ser.write_bare::<crate::ton::long>(random_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Ping {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let random_id = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { random_id })
        }
    }
}
impl crate::BoxedDeserialize for Ping {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xcbeb3f18)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xcbeb3f18) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for Ping {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xcbeb3f18), self)
    }
}
impl crate::Function for Ping {
    type Reply = crate::ton::dht::Pong;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.query`\n\n```text\ndht.query node:dht.node = True;\n```\n"]
pub struct Query {
    pub node: crate::ton::dht::node::Node,
}
impl Eq for Query {}
impl crate::BareSerialize for Query {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7d530769)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Query { node } = self;
        _ser.write_bare::<crate::ton::dht::node::Node>(node)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Query {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let node = _de.read_bare::<crate::ton::dht::node::Node>()?;
            Ok(Self { node })
        }
    }
}
impl crate::BoxedDeserialize for Query {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7d530769)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x7d530769) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for Query {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x7d530769), self)
    }
}
impl crate::Function for Query {
    type Reply = crate::ton::True;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.store`\n\n```text\ndht.store value:dht.value = dht.Stored;\n```\n"]
pub struct Store {
    pub value: crate::ton::dht::value::Value,
}
impl Eq for Store {}
impl crate::BareSerialize for Store {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x34934212)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Store { value } = self;
        _ser.write_bare::<crate::ton::dht::value::Value>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Store {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::dht::value::Value>()?;
            Ok(Self { value })
        }
    }
}
impl crate::BoxedDeserialize for Store {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x34934212)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x34934212) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for Store {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x34934212), self)
    }
}
impl crate::Function for Store {
    type Reply = crate::ton::dht::Stored;
}
