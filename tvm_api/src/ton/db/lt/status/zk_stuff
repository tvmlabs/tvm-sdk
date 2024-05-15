use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.status.Value`\n\n```text\ndb.lt.status.value total_shards:int = db.lt.status.Value;\n```\n"]
pub enum Value {
    Db_Lt_Status_Value(crate::ton::db::lt::status::value::Value),
}
impl Value {
    pub fn total_shards(&self) -> &crate::ton::int {
        match self {
            Value::Db_Lt_Status_Value(ref x) => &x.total_shards,
        }
    }
    pub fn only(self) -> crate::ton::db::lt::status::value::Value {
        match self {
            Value::Db_Lt_Status_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Lt_Status_Value(crate::ton::db::lt::status::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Lt_Status_Value(x) => (crate::ConstructorNumber(0xfabeed39), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfabeed39)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xfabeed39) => Ok(Value::Db_Lt_Status_Value(
                _de.read_bare::<crate::ton::db::lt::status::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
