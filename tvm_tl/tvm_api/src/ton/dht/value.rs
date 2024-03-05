use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.value`\n\n```text\ndht.value key:dht.keyDescription value:bytes ttl:int signature:bytes = dht.Value;\n```\n"]
pub struct Value {
    pub key: crate::ton::dht::keydescription::KeyDescription,
    pub value: crate::ton::bytes,
    pub ttl: crate::ton::int,
    pub signature: crate::ton::bytes,
}
impl Eq for Value {}
impl crate::Signing for Value {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x90ad27cb)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { key, value, ttl, signature } = self;
        _ser.write_bare::<crate::ton::dht::keydescription::KeyDescription>(key)?;
        _ser.write_bare::<crate::ton::bytes>(value)?;
        _ser.write_bare::<crate::ton::int>(ttl)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::dht::keydescription::KeyDescription>()?;
            let value = _de.read_bare::<crate::ton::bytes>()?;
            let ttl = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { key, value, ttl, signature })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::dht::Value;

    fn into_boxed(self) -> crate::ton::dht::Value {
        crate::ton::dht::Value::Dht_Value(self)
    }
}
