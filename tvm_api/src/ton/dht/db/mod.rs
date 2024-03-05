use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.db.Bucket`\n\n```text\ndht.db.bucket nodes:dht.nodes = dht.db.Bucket;\n```\n"]
pub enum Bucket {
    Dht_Db_Bucket(crate::ton::dht::db::bucket::Bucket),
}
impl Bucket {
    pub fn nodes(&self) -> &crate::ton::dht::nodes::Nodes {
        match self {
            Bucket::Dht_Db_Bucket(ref x) => &x.nodes,
        }
    }

    pub fn only(self) -> crate::ton::dht::db::bucket::Bucket {
        match self {
            Bucket::Dht_Db_Bucket(x) => x,
        }
    }
}
impl Eq for Bucket {}
impl Default for Bucket {
    fn default() -> Self {
        Bucket::Dht_Db_Bucket(crate::ton::dht::db::bucket::Bucket::default())
    }
}
impl crate::BoxedSerialize for Bucket {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Bucket::Dht_Db_Bucket(x) => (crate::ConstructorNumber(0xb39cfa6c), x),
        }
    }
}
impl crate::BoxedDeserialize for Bucket {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb39cfa6c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb39cfa6c) => {
                Ok(Bucket::Dht_Db_Bucket(_de.read_bare::<crate::ton::dht::db::bucket::Bucket>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.db.Key`\n\n```text\ndht.db.key.bucket id:int = dht.db.Key;\n```\n"]
pub enum Key {
    Dht_Db_Key_Bucket(crate::ton::dht::db::key::bucket::Bucket),
}
impl Key {
    pub fn id(&self) -> &crate::ton::int {
        match self {
            Key::Dht_Db_Key_Bucket(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::dht::db::key::bucket::Bucket {
        match self {
            Key::Dht_Db_Key_Bucket(x) => x,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Dht_Db_Key_Bucket(crate::ton::dht::db::key::bucket::Bucket::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Dht_Db_Key_Bucket(x) => (crate::ConstructorNumber(0xa368ae4c), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa368ae4c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa368ae4c) => Ok(Key::Dht_Db_Key_Bucket(
                _de.read_bare::<crate::ton::dht::db::key::bucket::Bucket>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod bucket;
pub mod key;
