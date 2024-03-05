use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.root.Config`\n\n```text\ndb.root.config celldb_version:int blockdb_version:int = db.root.Config;\n```\n"]
pub enum Config {
    Db_Root_Config(crate::ton::db::root::config::Config),
}
impl Config {
    pub fn blockdb_version(&self) -> &crate::ton::int {
        match self {
            Config::Db_Root_Config(ref x) => &x.blockdb_version,
        }
    }

    pub fn celldb_version(&self) -> &crate::ton::int {
        match self {
            Config::Db_Root_Config(ref x) => &x.celldb_version,
        }
    }

    pub fn only(self) -> crate::ton::db::root::config::Config {
        match self {
            Config::Db_Root_Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Db_Root_Config(crate::ton::db::root::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Db_Root_Config(x) => (crate::ConstructorNumber(0xd61182a1), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd61182a1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd61182a1) => {
                Ok(Config::Db_Root_Config(_de.read_bare::<crate::ton::db::root::config::Config>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.root.DbDescription`\n\n```text\ndb.root.dbDescription version:int first_masterchain_block_id:tonNode.blockIdExt flags:int = db.root.DbDescription;\n```\n"]
pub enum DbDescription {
    Db_Root_DbDescription(crate::ton::db::root::dbdescription::DbDescription),
}
impl DbDescription {
    pub fn first_masterchain_block_id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            DbDescription::Db_Root_DbDescription(ref x) => &x.first_masterchain_block_id,
        }
    }

    pub fn flags(&self) -> &crate::ton::int {
        match self {
            DbDescription::Db_Root_DbDescription(ref x) => &x.flags,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            DbDescription::Db_Root_DbDescription(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::db::root::dbdescription::DbDescription {
        match self {
            DbDescription::Db_Root_DbDescription(x) => x,
        }
    }
}
impl Eq for DbDescription {}
impl Default for DbDescription {
    fn default() -> Self {
        DbDescription::Db_Root_DbDescription(
            crate::ton::db::root::dbdescription::DbDescription::default(),
        )
    }
}
impl crate::BoxedSerialize for DbDescription {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DbDescription::Db_Root_DbDescription(x) => (crate::ConstructorNumber(0xb41873f3), x),
        }
    }
}
impl crate::BoxedDeserialize for DbDescription {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb41873f3)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb41873f3) => Ok(DbDescription::Db_Root_DbDescription(
                _de.read_bare::<crate::ton::db::root::dbdescription::DbDescription>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.root.Key`\n\n```text\ndb.root.key.blockDb version:int = db.root.Key;\n\ndb.root.key.cellDb version:int = db.root.Key;\n\ndb.root.key.config = db.root.Key;\n```\n"]
pub enum Key {
    Db_Root_Key_BlockDb(crate::ton::db::root::key::key::BlockDb),
    Db_Root_Key_CellDb(crate::ton::db::root::key::key::CellDb),
    Db_Root_Key_Config,
}
impl Key {
    pub fn version(&self) -> Option<&crate::ton::int> {
        match self {
            Key::Db_Root_Key_BlockDb(ref x) => Some(&x.version),
            Key::Db_Root_Key_CellDb(ref x) => Some(&x.version),
            _ => None,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Db_Root_Key_BlockDb(crate::ton::db::root::key::key::BlockDb::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Db_Root_Key_BlockDb(x) => (crate::ConstructorNumber(0x3012bf40), x),
            Key::Db_Root_Key_CellDb(x) => (crate::ConstructorNumber(0x72f9b33e), x),
            Key::Db_Root_Key_Config => (crate::ConstructorNumber(0x13c33284), &()),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x3012bf40),
            crate::ConstructorNumber(0x72f9b33e),
            crate::ConstructorNumber(0x13c33284),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3012bf40) => Ok(Key::Db_Root_Key_BlockDb(
                _de.read_bare::<crate::ton::db::root::key::key::BlockDb>()?,
            )),
            crate::ConstructorNumber(0x72f9b33e) => Ok(Key::Db_Root_Key_CellDb(
                _de.read_bare::<crate::ton::db::root::key::key::CellDb>()?,
            )),
            crate::ConstructorNumber(0x13c33284) => Ok(Key::Db_Root_Key_Config),
            id => _invalid_id!(id),
        }
    }
}
pub mod config;
pub mod dbdescription;
pub mod key;
