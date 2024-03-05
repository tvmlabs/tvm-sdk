use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.el.Value`\n\n```text\ndb.lt.el.value id:tonNode.blockIdExt lt:long ts:int = db.lt.el.Value;\n```\n"]
pub enum Value {
    Db_Lt_El_Value(crate::ton::db::lt::el::value::Value),
}
impl Value {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Value::Db_Lt_El_Value(ref x) => &x.id,
        }
    }

    pub fn lt(&self) -> &crate::ton::long {
        match self {
            Value::Db_Lt_El_Value(ref x) => &x.lt,
        }
    }

    pub fn ts(&self) -> &crate::ton::int {
        match self {
            Value::Db_Lt_El_Value(ref x) => &x.ts,
        }
    }

    pub fn only(self) -> crate::ton::db::lt::el::value::Value {
        match self {
            Value::Db_Lt_El_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Lt_El_Value(crate::ton::db::lt::el::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Lt_El_Value(x) => (crate::ConstructorNumber(0x95e65f64), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x95e65f64)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x95e65f64) => {
                Ok(Value::Db_Lt_El_Value(_de.read_bare::<crate::ton::db::lt::el::value::Value>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
