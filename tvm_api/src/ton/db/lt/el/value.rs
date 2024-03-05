use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.el.value`\n\n```text\ndb.lt.el.value id:tonNode.blockIdExt lt:long ts:int = db.lt.el.Value;\n```\n"]
pub struct Value {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub lt: crate::ton::long,
    pub ts: crate::ton::int,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x95e65f64)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { id, lt, ts } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::long>(lt)?;
        _ser.write_bare::<crate::ton::int>(ts)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let lt = _de.read_bare::<crate::ton::long>()?;
            let ts = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, lt, ts })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::lt::el::Value;

    fn into_boxed(self) -> crate::ton::db::lt::el::Value {
        crate::ton::db::lt::el::Value::Db_Lt_El_Value(self)
    }
}
