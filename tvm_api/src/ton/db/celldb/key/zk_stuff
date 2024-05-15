use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.celldb.key.Value`\n\n```text\ndb.celldb.key.value hash:int256 = db.celldb.key.Value;\n```\n"]
pub enum Value {
    Db_Celldb_Key_Value(crate::ton::db::celldb::key::value::Value),
}
impl Value {
    pub fn hash(&self) -> &crate::ton::int256 {
        match self {
            Value::Db_Celldb_Key_Value(ref x) => &x.hash,
        }
    }
    pub fn only(self) -> crate::ton::db::celldb::key::value::Value {
        match self {
            Value::Db_Celldb_Key_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Celldb_Key_Value(crate::ton::db::celldb::key::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Celldb_Key_Value(x) => (crate::ConstructorNumber(0x5bb13923), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5bb13923)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5bb13923) => Ok(Value::Db_Celldb_Key_Value(
                _de.read_bare::<crate::ton::db::celldb::key::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod value;
