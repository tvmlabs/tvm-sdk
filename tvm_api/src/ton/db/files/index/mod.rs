use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.files.index.Value`\n\n```text\ndb.files.index.value packages:(vector int) key_packages:(vector int) temp_packages:(vector int) = db.files.index.Value;\n```\n"]
pub enum Value {
    Db_Files_Index_Value(crate::ton::db::files::index::value::Value),
}
impl Value {
    pub fn key_packages(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int> {
        match self {
            Value::Db_Files_Index_Value(ref x) => &x.key_packages,
        }
    }
    pub fn packages(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int> {
        match self {
            Value::Db_Files_Index_Value(ref x) => &x.packages,
        }
    }
    pub fn temp_packages(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int> {
        match self {
            Value::Db_Files_Index_Value(ref x) => &x.temp_packages,
        }
    }
    pub fn only(self) -> crate::ton::db::files::index::value::Value {
        match self {
            Value::Db_Files_Index_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Files_Index_Value(crate::ton::db::files::index::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Files_Index_Value(x) => (crate::ConstructorNumber(0xa2b1dafc), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa2b1dafc)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa2b1dafc) => Ok(Value::Db_Files_Index_Value(
                _de.read_bare::<crate::ton::db::files::index::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
