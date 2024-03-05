use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.lt.shard.value`\n\n```text\ndb.lt.shard.value workchain:int shard:long = db.lt.shard.Value;\n```\n"]
pub struct Value {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3c739a7b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { workchain, shard } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { workchain, shard })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::lt::shard::Value;

    fn into_boxed(self) -> crate::ton::db::lt::shard::Value {
        crate::ton::db::lt::shard::Value::Db_Lt_Shard_Value(self)
    }
}
