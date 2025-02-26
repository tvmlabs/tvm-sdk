use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.gcBlockId`\n\n```text\ndb.state.gcBlockId block:tonNode.blockIdExt = db.state.GcBlockId;\n```\n"]
pub struct GcBlockId {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for GcBlockId {}
impl crate::BareSerialize for GcBlockId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdf30bd4f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GcBlockId { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GcBlockId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for GcBlockId {
    type Boxed = crate::ton::db::state::GcBlockId;

    fn into_boxed(self) -> crate::ton::db::state::GcBlockId {
        crate::ton::db::state::GcBlockId::Db_State_GcBlockId(self)
    }
}
