use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.status.value`\n\n```text\ndb.lt.status.value total_shards:int = db.lt.status.Value;\n```\n"]
pub struct Value {
    pub total_shards: crate::ton::int,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfabeed39)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { total_shards } = self;
        _ser.write_bare::<crate::ton::int>(total_shards)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let total_shards = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { total_shards })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::lt::status::Value;
    fn into_boxed(self) -> crate::ton::db::lt::status::Value {
        crate::ton::db::lt::status::Value::Db_Lt_Status_Value(self)
    }
}
