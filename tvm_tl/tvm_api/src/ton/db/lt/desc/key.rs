use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.desc.key`\n\n```text\ndb.lt.desc.key workchain:int shard:long = db.lt.Key;\n```\n"]
pub struct Key {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
}
impl Eq for Key {}
impl crate::BareSerialize for Key {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf1e3e791)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Key { workchain, shard } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Key {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { workchain, shard })
        }
    }
}
impl crate::IntoBoxed for Key {
    type Boxed = crate::ton::db::lt::Key;

    fn into_boxed(self) -> crate::ton::db::lt::Key {
        crate::ton::db::lt::Key::Db_Lt_Desc_Key(self)
    }
}
