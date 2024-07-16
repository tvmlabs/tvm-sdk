use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.files.package.key`\n\n```text\ndb.files.package.key package_id:int key:Bool temp:Bool = db.files.Key;\n```\n"]
pub struct Key {
    pub package_id: crate::ton::int,
    pub key: crate::ton::Bool,
    pub temp: crate::ton::Bool,
}
impl Eq for Key {}
impl crate::BareSerialize for Key {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa504033e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Key { package_id, key, temp } = self;
        _ser.write_bare::<crate::ton::int>(package_id)?;
        _ser.write_boxed::<crate::ton::Bool>(key)?;
        _ser.write_boxed::<crate::ton::Bool>(temp)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Key {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let package_id = _de.read_bare::<crate::ton::int>()?;
            let key = _de.read_boxed::<crate::ton::Bool>()?;
            let temp = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { package_id, key, temp })
        }
    }
}
impl crate::IntoBoxed for Key {
    type Boxed = crate::ton::db::files::Key;

    fn into_boxed(self) -> crate::ton::db::files::Key {
        crate::ton::db::files::Key::Db_Files_Package_Key(self)
    }
}
