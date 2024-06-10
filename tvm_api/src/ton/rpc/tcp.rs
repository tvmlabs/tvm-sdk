use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tcp.ping`\n\n```text\ntcp.ping random_id:long = tcp.Pong;\n```\n"]
pub struct Ping {
    pub random_id: crate::ton::long,
}
impl Eq for Ping {}
impl crate::BareSerialize for Ping {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4d082b9a)
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
        vec![crate::ConstructorNumber(0x4d082b9a)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x4d082b9a) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for Ping {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x4d082b9a), self)
    }
}
impl crate::Function for Ping {
    type Reply = crate::ton::tcp::Pong;
}
