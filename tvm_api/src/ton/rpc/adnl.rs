use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.ping`\n\n```text\nadnl.ping value:long = adnl.Pong;\n```\n"]
pub struct Ping {
    pub value: crate::ton::long,
}
impl Eq for Ping {}
impl crate::BareSerialize for Ping {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1faaa1bf)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Ping { value } = self;
        _ser.write_bare::<crate::ton::long>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Ping {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { value })
        }
    }
}
impl crate::BoxedDeserialize for Ping {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1faaa1bf)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x1faaa1bf) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Ping {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x1faaa1bf), self)
    }
}
impl crate::Function for Ping {
    type Reply = crate::ton::adnl::Pong;
}
