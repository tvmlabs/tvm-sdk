use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.blockdb.key.lru`\n\n```text\ndb.blockdb.key.lru id:tonNode.blockIdExt = db.blockdb.Key;\n```\n"]
pub struct Lru {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for Lru {}
impl crate::BareSerialize for Lru {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x50bc963a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Lru { id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Lru {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Lru {
    type Boxed = crate::ton::db::blockdb::Key;
    fn into_boxed(self) -> crate::ton::db::blockdb::Key {
        crate::ton::db::blockdb::Key::Db_Blockdb_Key_Lru(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.blockdb.key.value`\n\n```text\ndb.blockdb.key.value id:tonNode.blockIdExt = db.blockdb.Key;\n```\n"]
pub struct Value {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7f57d173)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::blockdb::Key;
    fn into_boxed(self) -> crate::ton::db::blockdb::Key {
        crate::ton::db::blockdb::Key::Db_Blockdb_Key_Value(self)
    }
}
