use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.blockdb.Key`\n\n```text\ndb.blockdb.key.lru id:tonNode.blockIdExt = db.blockdb.Key;\n\ndb.blockdb.key.value id:tonNode.blockIdExt = db.blockdb.Key;\n```\n"]
pub enum Key {
    Db_Blockdb_Key_Lru(crate::ton::db::blockdb::key::key::Lru),
    Db_Blockdb_Key_Value(crate::ton::db::blockdb::key::key::Value),
}
impl Key {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Key::Db_Blockdb_Key_Lru(ref x) => &x.id,
            Key::Db_Blockdb_Key_Value(ref x) => &x.id,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Db_Blockdb_Key_Lru(crate::ton::db::blockdb::key::key::Lru::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Db_Blockdb_Key_Lru(x) => (crate::ConstructorNumber(0x50bc963a), x),
            Key::Db_Blockdb_Key_Value(x) => (crate::ConstructorNumber(0x7f57d173), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x50bc963a), crate::ConstructorNumber(0x7f57d173)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x50bc963a) => Ok(Key::Db_Blockdb_Key_Lru(
                _de.read_bare::<crate::ton::db::blockdb::key::key::Lru>()?,
            )),
            crate::ConstructorNumber(0x7f57d173) => Ok(Key::Db_Blockdb_Key_Value(
                _de.read_bare::<crate::ton::db::blockdb::key::key::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.blockdb.Lru`\n\n```text\ndb.blockdb.lru id:tonNode.blockIdExt prev:int256 next:int256 = db.blockdb.Lru;\n```\n"]
pub enum Lru {
    Db_Blockdb_Lru(crate::ton::db::blockdb::lru::Lru),
}
impl Lru {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Lru::Db_Blockdb_Lru(ref x) => &x.id,
        }
    }

    pub fn next(&self) -> &crate::ton::int256 {
        match self {
            Lru::Db_Blockdb_Lru(ref x) => &x.next,
        }
    }

    pub fn prev(&self) -> &crate::ton::int256 {
        match self {
            Lru::Db_Blockdb_Lru(ref x) => &x.prev,
        }
    }

    pub fn only(self) -> crate::ton::db::blockdb::lru::Lru {
        match self {
            Lru::Db_Blockdb_Lru(x) => x,
        }
    }
}
impl Eq for Lru {}
impl Default for Lru {
    fn default() -> Self {
        Lru::Db_Blockdb_Lru(crate::ton::db::blockdb::lru::Lru::default())
    }
}
impl crate::BoxedSerialize for Lru {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Lru::Db_Blockdb_Lru(x) => (crate::ConstructorNumber(0xc11655b3), x),
        }
    }
}
impl crate::BoxedDeserialize for Lru {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc11655b3)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc11655b3) => {
                Ok(Lru::Db_Blockdb_Lru(_de.read_bare::<crate::ton::db::blockdb::lru::Lru>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.blockdb.Value`\n\n```text\ndb.blockdb.value next:tonNode.blockIdExt data:bytes = db.blockdb.Value;\n```\n"]
pub enum Value {
    Db_Blockdb_Value(crate::ton::db::blockdb::value::Value),
}
impl Value {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            Value::Db_Blockdb_Value(ref x) => &x.data,
        }
    }

    pub fn next(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Value::Db_Blockdb_Value(ref x) => &x.next,
        }
    }

    pub fn only(self) -> crate::ton::db::blockdb::value::Value {
        match self {
            Value::Db_Blockdb_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Blockdb_Value(crate::ton::db::blockdb::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Blockdb_Value(x) => (crate::ConstructorNumber(0xb28ec42d), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb28ec42d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb28ec42d) => Ok(Value::Db_Blockdb_Value(
                _de.read_bare::<crate::ton::db::blockdb::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod lru;
pub mod value;
