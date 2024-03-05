use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.files.package.value`\n\n```text\ndb.files.package.value package_id:int key:Bool temp:Bool firstblocks:(vector db.files.package.firstBlock) deleted:Bool \n                   = db.files.package.Value;\n```\n"]
pub struct Value {
    pub package_id: crate::ton::int,
    pub key: crate::ton::Bool,
    pub temp: crate::ton::Bool,
    pub firstblocks: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::db::files::package::firstblock::FirstBlock,
    >,
    pub deleted: crate::ton::Bool,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe44cd52b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { package_id, key, temp, firstblocks, deleted } = self;
        _ser.write_bare::<crate::ton::int>(package_id)?;
        _ser.write_boxed::<crate::ton::Bool>(key)?;
        _ser.write_boxed::<crate::ton::Bool>(temp)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::db::files::package::firstblock::FirstBlock,
        >>(firstblocks)?;
        _ser.write_boxed::<crate::ton::Bool>(deleted)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let package_id = _de.read_bare::<crate::ton::int>()?;
            let key = _de.read_boxed::<crate::ton::Bool>()?;
            let temp = _de.read_boxed::<crate::ton::Bool>()?;
            let firstblocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::db::files::package::firstblock::FirstBlock,
            >>()?;
            let deleted = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { package_id, key, temp, firstblocks, deleted })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::files::package::Value;

    fn into_boxed(self) -> crate::ton::db::files::package::Value {
        crate::ton::db::files::package::Value::Db_Files_Package_Value(self)
    }
}
