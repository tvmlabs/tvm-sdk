use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.Key`\n\n```text\ndb.lt.desc.key workchain:int shard:long = db.lt.Key;\n\ndb.lt.el.key workchain:int shard:long idx:int = db.lt.Key;\n\ndb.lt.shard.key idx:int = db.lt.Key;\n\ndb.lt.status.key = db.lt.Key;\n```\n"]
pub enum Key {
    Db_Lt_Desc_Key(crate::ton::db::lt::desc::key::Key),
    Db_Lt_El_Key(crate::ton::db::lt::el::key::Key),
    Db_Lt_Shard_Key(crate::ton::db::lt::shard::key::Key),
    Db_Lt_Status_Key,
}
impl Key {
    pub fn idx(&self) -> Option<&crate::ton::int> {
        match self {
            Key::Db_Lt_El_Key(ref x) => Some(&x.idx),
            Key::Db_Lt_Shard_Key(ref x) => Some(&x.idx),
            _ => None,
        }
    }

    pub fn shard(&self) -> Option<&crate::ton::long> {
        match self {
            Key::Db_Lt_Desc_Key(ref x) => Some(&x.shard),
            Key::Db_Lt_El_Key(ref x) => Some(&x.shard),
            _ => None,
        }
    }

    pub fn workchain(&self) -> Option<&crate::ton::int> {
        match self {
            Key::Db_Lt_Desc_Key(ref x) => Some(&x.workchain),
            Key::Db_Lt_El_Key(ref x) => Some(&x.workchain),
            _ => None,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Db_Lt_Desc_Key(crate::ton::db::lt::desc::key::Key::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Db_Lt_Desc_Key(x) => (crate::ConstructorNumber(0xf1e3e791), x),
            Key::Db_Lt_El_Key(x) => (crate::ConstructorNumber(0xa5321ae2), x),
            Key::Db_Lt_Shard_Key(x) => (crate::ConstructorNumber(0x50a6f90f), x),
            Key::Db_Lt_Status_Key => (crate::ConstructorNumber(0x776c6057), &()),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xf1e3e791),
            crate::ConstructorNumber(0xa5321ae2),
            crate::ConstructorNumber(0x50a6f90f),
            crate::ConstructorNumber(0x776c6057),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf1e3e791) => {
                Ok(Key::Db_Lt_Desc_Key(_de.read_bare::<crate::ton::db::lt::desc::key::Key>()?))
            }
            crate::ConstructorNumber(0xa5321ae2) => {
                Ok(Key::Db_Lt_El_Key(_de.read_bare::<crate::ton::db::lt::el::key::Key>()?))
            }
            crate::ConstructorNumber(0x50a6f90f) => {
                Ok(Key::Db_Lt_Shard_Key(_de.read_bare::<crate::ton::db::lt::shard::key::Key>()?))
            }
            crate::ConstructorNumber(0x776c6057) => Ok(Key::Db_Lt_Status_Key),
            id => _invalid_id!(id),
        }
    }
}
pub mod desc;
pub mod el;
pub mod shard;
pub mod status;
