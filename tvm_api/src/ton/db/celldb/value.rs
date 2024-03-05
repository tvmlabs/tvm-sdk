use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.celldb.value`\n\n```text\ndb.celldb.value block_id:tonNode.blockIdExt prev:int256 next:int256 root_hash:int256 = db.celldb.Value;\n```\n"]
pub struct Value {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub prev: crate::ton::int256,
    pub next: crate::ton::int256,
    pub root_hash: crate::ton::int256,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe6101440)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { block_id, prev, next, root_hash } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        _ser.write_bare::<crate::ton::int256>(prev)?;
        _ser.write_bare::<crate::ton::int256>(next)?;
        _ser.write_bare::<crate::ton::int256>(root_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let prev = _de.read_bare::<crate::ton::int256>()?;
            let next = _de.read_bare::<crate::ton::int256>()?;
            let root_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { block_id, prev, next, root_hash })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::celldb::Value;

    fn into_boxed(self) -> crate::ton::db::celldb::Value {
        crate::ton::db::celldb::Value::Db_Celldb_Value(self)
    }
}
