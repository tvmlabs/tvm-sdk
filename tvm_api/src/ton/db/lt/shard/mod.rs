use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.shard.Value`\n\n```text\ndb.lt.shard.value workchain:int shard:long = db.lt.shard.Value;\n```\n"]
pub enum Value {
    Db_Lt_Shard_Value(crate::ton::db::lt::shard::value::Value),
}
impl Value {
    pub fn shard(&self) -> &crate::ton::long {
        match self {
            Value::Db_Lt_Shard_Value(ref x) => &x.shard,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            Value::Db_Lt_Shard_Value(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::db::lt::shard::value::Value {
        match self {
            Value::Db_Lt_Shard_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Lt_Shard_Value(crate::ton::db::lt::shard::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Lt_Shard_Value(x) => (crate::ConstructorNumber(0x3c739a7b), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3c739a7b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3c739a7b) => Ok(Value::Db_Lt_Shard_Value(
                _de.read_bare::<crate::ton::db::lt::shard::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
