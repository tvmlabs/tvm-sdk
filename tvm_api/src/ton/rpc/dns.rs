use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.resolve`\n\n```text\ndns.resolve account_address:accountAddress name:string category:int32 ttl:int32 = dns.Resolved;\n```\n"]
pub struct Resolve {
    pub account_address: crate::ton::accountaddress::AccountAddress,
    pub name: crate::ton::string,
    pub category: crate::ton::int32,
    pub ttl: crate::ton::int32,
}
impl Eq for Resolve {}
impl crate::BareSerialize for Resolve {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf71acecf)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Resolve { account_address, name, category, ttl } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        _ser.write_bare::<crate::ton::string>(name)?;
        _ser.write_bare::<crate::ton::int32>(category)?;
        _ser.write_bare::<crate::ton::int32>(ttl)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Resolve {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let name = _de.read_bare::<crate::ton::string>()?;
            let category = _de.read_bare::<crate::ton::int32>()?;
            let ttl = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { account_address, name, category, ttl })
        }
    }
}
impl crate::BoxedDeserialize for Resolve {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf71acecf)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf71acecf) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Resolve {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf71acecf), self)
    }
}
impl crate::Function for Resolve {
    type Reply = crate::ton::dns::Resolved;
}
