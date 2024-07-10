use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.valueFound`\n\n```text\ndht.valueFound value:dht.Value = dht.ValueResult;\n```\n"]
pub struct ValueFound {
    pub value: crate::ton::dht::Value,
}
impl Eq for ValueFound {}
impl crate::BareSerialize for ValueFound {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe40cf774)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValueFound { value } = self;
        _ser.write_boxed::<crate::ton::dht::Value>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValueFound {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_boxed::<crate::ton::dht::Value>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for ValueFound {
    type Boxed = crate::ton::dht::ValueResult;
    fn into_boxed(self) -> crate::ton::dht::ValueResult {
        crate::ton::dht::ValueResult::Dht_ValueFound(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.valueNotFound`\n\n```text\ndht.valueNotFound nodes:dht.nodes = dht.ValueResult;\n```\n"]
pub struct ValueNotFound {
    pub nodes: crate::ton::dht::nodes::Nodes,
}
impl Eq for ValueNotFound {}
impl crate::BareSerialize for ValueNotFound {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa2620568)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValueNotFound { nodes } = self;
        _ser.write_bare::<crate::ton::dht::nodes::Nodes>(nodes)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValueNotFound {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nodes = _de.read_bare::<crate::ton::dht::nodes::Nodes>()?;
            Ok(Self { nodes })
        }
    }
}
impl crate::IntoBoxed for ValueNotFound {
    type Boxed = crate::ton::dht::ValueResult;
    fn into_boxed(self) -> crate::ton::dht::ValueResult {
        crate::ton::dht::ValueResult::Dht_ValueNotFound(self)
    }
}
