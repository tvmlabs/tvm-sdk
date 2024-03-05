use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.root.dbDescription`\n\n```text\ndb.root.dbDescription version:int first_masterchain_block_id:tonNode.blockIdExt flags:int = db.root.DbDescription;\n```\n"]
pub struct DbDescription {
    pub version: crate::ton::int,
    pub first_masterchain_block_id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub flags: crate::ton::int,
}
impl Eq for DbDescription {}
impl crate::BareSerialize for DbDescription {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb41873f3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DbDescription { version, first_masterchain_block_id, flags } = self;
        _ser.write_bare::<crate::ton::int>(version)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(
            first_masterchain_block_id,
        )?;
        _ser.write_bare::<crate::ton::int>(flags)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DbDescription {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let version = _de.read_bare::<crate::ton::int>()?;
            let first_masterchain_block_id =
                _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let flags = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { version, first_masterchain_block_id, flags })
        }
    }
}
impl crate::IntoBoxed for DbDescription {
    type Boxed = crate::ton::db::root::DbDescription;

    fn into_boxed(self) -> crate::ton::db::root::DbDescription {
        crate::ton::db::root::DbDescription::Db_Root_DbDescription(self)
    }
}
