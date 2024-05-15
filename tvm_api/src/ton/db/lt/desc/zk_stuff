use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.desc.Value`\n\n```text\ndb.lt.desc.value first_idx:int last_idx:int last_seqno:int last_lt:long last_ts:int = db.lt.desc.Value;\n```\n"]
pub enum Value {
    Db_Lt_Desc_Value(crate::ton::db::lt::desc::value::Value),
}
impl Value {
    pub fn first_idx(&self) -> &crate::ton::int {
        match self {
            Value::Db_Lt_Desc_Value(ref x) => &x.first_idx,
        }
    }
    pub fn last_idx(&self) -> &crate::ton::int {
        match self {
            Value::Db_Lt_Desc_Value(ref x) => &x.last_idx,
        }
    }
    pub fn last_lt(&self) -> &crate::ton::long {
        match self {
            Value::Db_Lt_Desc_Value(ref x) => &x.last_lt,
        }
    }
    pub fn last_seqno(&self) -> &crate::ton::int {
        match self {
            Value::Db_Lt_Desc_Value(ref x) => &x.last_seqno,
        }
    }
    pub fn last_ts(&self) -> &crate::ton::int {
        match self {
            Value::Db_Lt_Desc_Value(ref x) => &x.last_ts,
        }
    }
    pub fn only(self) -> crate::ton::db::lt::desc::value::Value {
        match self {
            Value::Db_Lt_Desc_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Lt_Desc_Value(crate::ton::db::lt::desc::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Lt_Desc_Value(x) => (crate::ConstructorNumber(0x71af51b4), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x71af51b4)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x71af51b4) => Ok(Value::Db_Lt_Desc_Value(
                _de.read_bare::<crate::ton::db::lt::desc::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
