use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.root.key.blockDb`\n\n```text\ndb.root.key.blockDb version:int = db.root.Key;\n```\n"]
pub struct BlockDb {
    pub version: crate::ton::int,
}
impl Eq for BlockDb {}
impl crate::BareSerialize for BlockDb {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3012bf40)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockDb { version } = self;
        _ser.write_bare::<crate::ton::int>(version)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockDb {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let version = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { version })
        }
    }
}
impl crate::IntoBoxed for BlockDb {
    type Boxed = crate::ton::db::root::Key;
    fn into_boxed(self) -> crate::ton::db::root::Key {
        crate::ton::db::root::Key::Db_Root_Key_BlockDb(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.root.key.cellDb`\n\n```text\ndb.root.key.cellDb version:int = db.root.Key;\n```\n"]
pub struct CellDb {
    pub version: crate::ton::int,
}
impl Eq for CellDb {}
impl crate::BareSerialize for CellDb {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x72f9b33e)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CellDb { version } = self;
        _ser.write_bare::<crate::ton::int>(version)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CellDb {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let version = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { version })
        }
    }
}
impl crate::IntoBoxed for CellDb {
    type Boxed = crate::ton::db::root::Key;
    fn into_boxed(self) -> crate::ton::db::root::Key {
        crate::ton::db::root::Key::Db_Root_Key_CellDb(self)
    }
}
