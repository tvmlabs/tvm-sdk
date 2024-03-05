use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.desc.value`\n\n```text\ndb.lt.desc.value first_idx:int last_idx:int last_seqno:int last_lt:long last_ts:int = db.lt.desc.Value;\n```\n"]
pub struct Value {
    pub first_idx: crate::ton::int,
    pub last_idx: crate::ton::int,
    pub last_seqno: crate::ton::int,
    pub last_lt: crate::ton::long,
    pub last_ts: crate::ton::int,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x71af51b4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { first_idx, last_idx, last_seqno, last_lt, last_ts } = self;
        _ser.write_bare::<crate::ton::int>(first_idx)?;
        _ser.write_bare::<crate::ton::int>(last_idx)?;
        _ser.write_bare::<crate::ton::int>(last_seqno)?;
        _ser.write_bare::<crate::ton::long>(last_lt)?;
        _ser.write_bare::<crate::ton::int>(last_ts)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let first_idx = _de.read_bare::<crate::ton::int>()?;
            let last_idx = _de.read_bare::<crate::ton::int>()?;
            let last_seqno = _de.read_bare::<crate::ton::int>()?;
            let last_lt = _de.read_bare::<crate::ton::long>()?;
            let last_ts = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { first_idx, last_idx, last_seqno, last_lt, last_ts })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::lt::desc::Value;

    fn into_boxed(self) -> crate::ton::db::lt::desc::Value {
        crate::ton::db::lt::desc::Value::Db_Lt_Desc_Value(self)
    }
}
