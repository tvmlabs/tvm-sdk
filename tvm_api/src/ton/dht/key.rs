use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.key`\n\n```text\ndht.key id:int256 name:bytes idx:int = dht.Key;\n```\n"]
pub struct Key {
    pub id: crate::ton::int256,
    pub name: crate::ton::bytes,
    pub idx: crate::ton::int,
}
impl Eq for Key {}
impl crate::BareSerialize for Key {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf667de8f)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Key { id, name, idx } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::bytes>(name)?;
        _ser.write_bare::<crate::ton::int>(idx)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Key {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let name = _de.read_bare::<crate::ton::bytes>()?;
            let idx = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, name, idx })
        }
    }
}
impl crate::IntoBoxed for Key {
    type Boxed = crate::ton::dht::Key;
    fn into_boxed(self) -> crate::ton::dht::Key {
        crate::ton::dht::Key::Dht_Key(self)
    }
}
