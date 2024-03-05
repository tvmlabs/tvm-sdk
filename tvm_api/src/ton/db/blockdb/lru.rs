use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.blockdb.lru`\n\n```text\ndb.blockdb.lru id:tonNode.blockIdExt prev:int256 next:int256 = db.blockdb.Lru;\n```\n"]
pub struct Lru {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub prev: crate::ton::int256,
    pub next: crate::ton::int256,
}
impl Eq for Lru {}
impl crate::BareSerialize for Lru {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc11655b3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Lru { id, prev, next } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int256>(prev)?;
        _ser.write_bare::<crate::ton::int256>(next)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Lru {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let prev = _de.read_bare::<crate::ton::int256>()?;
            let next = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id, prev, next })
        }
    }
}
impl crate::IntoBoxed for Lru {
    type Boxed = crate::ton::db::blockdb::Lru;

    fn into_boxed(self) -> crate::ton::db::blockdb::Lru {
        crate::ton::db::blockdb::Lru::Db_Blockdb_Lru(self)
    }
}
