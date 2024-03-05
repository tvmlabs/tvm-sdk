use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.dbVersion`\n\n```text\ndb.state.dbVersion version:int = db.state.DbVersion;\n```\n"]
pub struct DbVersion {
    pub version: crate::ton::int,
}
impl Eq for DbVersion {}
impl crate::BareSerialize for DbVersion {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd93720f7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DbVersion { version } = self;
        _ser.write_bare::<crate::ton::int>(version)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DbVersion {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let version = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { version })
        }
    }
}
impl crate::IntoBoxed for DbVersion {
    type Boxed = crate::ton::db::state::DbVersion;

    fn into_boxed(self) -> crate::ton::db::state::DbVersion {
        crate::ton::db::state::DbVersion::Db_State_DbVersion(self)
    }
}
