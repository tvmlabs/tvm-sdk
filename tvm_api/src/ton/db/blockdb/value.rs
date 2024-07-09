use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.blockdb.value`\n\n```text\ndb.blockdb.value next:tonNode.blockIdExt data:bytes = db.blockdb.Value;\n```\n"]
pub struct Value {
    pub next: crate::ton::ton_node::blockidext::BlockIdExt,
    pub data: crate::ton::bytes,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb28ec42d)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { next, data } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(next)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let next = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { next, data })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::blockdb::Value;
    fn into_boxed(self) -> crate::ton::db::blockdb::Value {
        crate::ton::db::blockdb::Value::Db_Blockdb_Value(self)
    }
}
