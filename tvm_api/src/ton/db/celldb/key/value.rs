use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.celldb.key.value`\n\n```text\ndb.celldb.key.value hash:int256 = db.celldb.key.Value;\n```\n"]
pub struct Value {
    pub hash: crate::ton::int256,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5bb13923)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { hash } = self;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { hash })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::celldb::key::Value;

    fn into_boxed(self) -> crate::ton::db::celldb::key::Value {
        crate::ton::db::celldb::key::Value::Db_Celldb_Key_Value(self)
    }
}
