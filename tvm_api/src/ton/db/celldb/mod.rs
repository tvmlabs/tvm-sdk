use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.celldb.Value`\n\n```text\ndb.celldb.value block_id:tonNode.blockIdExt prev:int256 next:int256 root_hash:int256 = db.celldb.Value;\n```\n"]
pub enum Value {
    Db_Celldb_Value(crate::ton::db::celldb::value::Value),
}
impl Value {
    pub fn block_id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Value::Db_Celldb_Value(ref x) => &x.block_id,
        }
    }

    pub fn next(&self) -> &crate::ton::int256 {
        match self {
            Value::Db_Celldb_Value(ref x) => &x.next,
        }
    }

    pub fn prev(&self) -> &crate::ton::int256 {
        match self {
            Value::Db_Celldb_Value(ref x) => &x.prev,
        }
    }

    pub fn root_hash(&self) -> &crate::ton::int256 {
        match self {
            Value::Db_Celldb_Value(ref x) => &x.root_hash,
        }
    }

    pub fn only(self) -> crate::ton::db::celldb::value::Value {
        match self {
            Value::Db_Celldb_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Celldb_Value(crate::ton::db::celldb::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Celldb_Value(x) => (crate::ConstructorNumber(0xe6101440), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe6101440)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe6101440) => {
                Ok(Value::Db_Celldb_Value(_de.read_bare::<crate::ton::db::celldb::value::Value>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
