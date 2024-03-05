use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.root.config`\n\n```text\ndb.root.config celldb_version:int blockdb_version:int = db.root.Config;\n```\n"]
pub struct Config {
    pub celldb_version: crate::ton::int,
    pub blockdb_version: crate::ton::int,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd61182a1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { celldb_version, blockdb_version } = self;
        _ser.write_bare::<crate::ton::int>(celldb_version)?;
        _ser.write_bare::<crate::ton::int>(blockdb_version)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let celldb_version = _de.read_bare::<crate::ton::int>()?;
            let blockdb_version = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { celldb_version, blockdb_version })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::db::root::Config;

    fn into_boxed(self) -> crate::ton::db::root::Config {
        crate::ton::db::root::Config::Db_Root_Config(self)
    }
}
