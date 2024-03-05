use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.value`\n\n```text\ndb.filedb.value key:db.filedb.Key prev:int256 next:int256 file_hash:int256 = db.filedb.Value;\n```\n"]
pub struct Value {
    pub key: crate::ton::db::filedb::Key,
    pub prev: crate::ton::int256,
    pub next: crate::ton::int256,
    pub file_hash: crate::ton::int256,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf2dd1a2d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { key, prev, next, file_hash } = self;
        _ser.write_boxed::<crate::ton::db::filedb::Key>(key)?;
        _ser.write_bare::<crate::ton::int256>(prev)?;
        _ser.write_bare::<crate::ton::int256>(next)?;
        _ser.write_bare::<crate::ton::int256>(file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_boxed::<crate::ton::db::filedb::Key>()?;
            let prev = _de.read_bare::<crate::ton::int256>()?;
            let next = _de.read_bare::<crate::ton::int256>()?;
            let file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key, prev, next, file_hash })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::filedb::Value;

    fn into_boxed(self) -> crate::ton::db::filedb::Value {
        crate::ton::db::filedb::Value::Db_Filedb_Value(self)
    }
}
