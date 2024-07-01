use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.initBlockId`\n\n```text\ndb.state.initBlockId block:tonNode.blockIdExt = db.state.InitBlockId;\n```\n"]
pub struct InitBlockId {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for InitBlockId {}
impl crate::BareSerialize for InitBlockId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x732c9cf5)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let InitBlockId { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for InitBlockId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for InitBlockId {
    type Boxed = crate::ton::db::state::InitBlockId;
    fn into_boxed(self) -> crate::ton::db::state::InitBlockId {
        crate::ton::db::state::InitBlockId::Db_State_InitBlockId(self)
    }
}
