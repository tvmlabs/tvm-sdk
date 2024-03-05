use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `db.files.Key`\n\n```text\ndb.files.index.key = db.files.Key;\n\ndb.files.package.key package_id:int key:Bool temp:Bool = db.files.Key;\n```\n"]
pub enum Key {
    #[default]
    Db_Files_Index_Key,
    Db_Files_Package_Key(crate::ton::db::files::package::key::Key),
}
impl Key {
    pub fn key(&self) -> Option<&crate::ton::Bool> {
        match self {
            Key::Db_Files_Package_Key(ref x) => Some(&x.key),
            _ => None,
        }
    }

    pub fn package_id(&self) -> Option<&crate::ton::int> {
        match self {
            Key::Db_Files_Package_Key(ref x) => Some(&x.package_id),
            _ => None,
        }
    }

    pub fn temp(&self) -> Option<&crate::ton::Bool> {
        match self {
            Key::Db_Files_Package_Key(ref x) => Some(&x.temp),
            _ => None,
        }
    }
}
impl Eq for Key {}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Db_Files_Index_Key => (crate::ConstructorNumber(0x7dc40502), &()),
            Key::Db_Files_Package_Key(x) => (crate::ConstructorNumber(0xa504033e), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7dc40502), crate::ConstructorNumber(0xa504033e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7dc40502) => Ok(Key::Db_Files_Index_Key),
            crate::ConstructorNumber(0xa504033e) => Ok(Key::Db_Files_Package_Key(
                _de.read_bare::<crate::ton::db::files::package::key::Key>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::db::files::package::key::Key> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x7dc40502), &()),
            Some(ref x) => (crate::ConstructorNumber(0xa504033e), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::db::files::package::key::Key> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7dc40502), crate::ConstructorNumber(0xa504033e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7dc40502) => Ok(None),
            crate::ConstructorNumber(0xa504033e) => {
                Ok(Some(_de.read_bare::<crate::ton::db::files::package::key::Key>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod index;
pub mod package;
