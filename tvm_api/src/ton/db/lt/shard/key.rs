use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.shard.key`\n\n```text\ndb.lt.shard.key idx:int = db.lt.Key;\n```\n"]
pub struct Key {
    pub idx: crate::ton::int,
}
impl Eq for Key {}
impl crate::BareSerialize for Key {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x50a6f90f)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Key { idx } = self;
        _ser.write_bare::<crate::ton::int>(idx)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Key {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let idx = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { idx })
        }
    }
}
impl crate::IntoBoxed for Key {
    type Boxed = crate::ton::db::lt::Key;
    fn into_boxed(self) -> crate::ton::db::lt::Key {
        crate::ton::db::lt::Key::Db_Lt_Shard_Key(self)
    }
}
