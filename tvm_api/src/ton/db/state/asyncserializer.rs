use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.asyncSerializer`\n\n```text\ndb.state.asyncSerializer block:tonNode.blockIdExt last:tonNode.blockIdExt last_ts:int = db.state.AsyncSerializer;\n```\n"]
pub struct AsyncSerializer {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub last: crate::ton::ton_node::blockidext::BlockIdExt,
    pub last_ts: crate::ton::int,
}
impl Eq for AsyncSerializer {}
impl crate::BareSerialize for AsyncSerializer {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd32f29a1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AsyncSerializer { block, last, last_ts } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(last)?;
        _ser.write_bare::<crate::ton::int>(last_ts)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AsyncSerializer {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let last = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let last_ts = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, last, last_ts })
        }
    }
}
impl crate::IntoBoxed for AsyncSerializer {
    type Boxed = crate::ton::db::state::AsyncSerializer;

    fn into_boxed(self) -> crate::ton::db::state::AsyncSerializer {
        crate::ton::db::state::AsyncSerializer::Db_State_AsyncSerializer(self)
    }
}
