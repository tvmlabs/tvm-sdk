use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.files.package.FirstBlock`\n\n```text\ndb.files.package.firstBlock workchain:int shard:long seqno:int unixtime:int lt:long = db.files.package.FirstBlock;\n```\n"]
pub enum FirstBlock {
    Db_Files_Package_FirstBlock(crate::ton::db::files::package::firstblock::FirstBlock),
}
impl FirstBlock {
    pub fn lt(&self) -> &crate::ton::long {
        match self {
            FirstBlock::Db_Files_Package_FirstBlock(ref x) => &x.lt,
        }
    }

    pub fn seqno(&self) -> &crate::ton::int {
        match self {
            FirstBlock::Db_Files_Package_FirstBlock(ref x) => &x.seqno,
        }
    }

    pub fn shard(&self) -> &crate::ton::long {
        match self {
            FirstBlock::Db_Files_Package_FirstBlock(ref x) => &x.shard,
        }
    }

    pub fn unixtime(&self) -> &crate::ton::int {
        match self {
            FirstBlock::Db_Files_Package_FirstBlock(ref x) => &x.unixtime,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            FirstBlock::Db_Files_Package_FirstBlock(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::db::files::package::firstblock::FirstBlock {
        match self {
            FirstBlock::Db_Files_Package_FirstBlock(x) => x,
        }
    }
}
impl Eq for FirstBlock {}
impl Default for FirstBlock {
    fn default() -> Self {
        FirstBlock::Db_Files_Package_FirstBlock(
            crate::ton::db::files::package::firstblock::FirstBlock::default(),
        )
    }
}
impl crate::BoxedSerialize for FirstBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            FirstBlock::Db_Files_Package_FirstBlock(x) => (crate::ConstructorNumber(0x701269e7), x),
        }
    }
}
impl crate::BoxedDeserialize for FirstBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x701269e7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x701269e7) => Ok(FirstBlock::Db_Files_Package_FirstBlock(
                _de.read_bare::<crate::ton::db::files::package::firstblock::FirstBlock>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.files.package.Value`\n\n```text\ndb.files.package.value package_id:int key:Bool temp:Bool firstblocks:(vector db.files.package.firstBlock) deleted:Bool \n                   = db.files.package.Value;\n```\n"]
pub enum Value {
    Db_Files_Package_Value(crate::ton::db::files::package::value::Value),
}
impl Value {
    pub fn deleted(&self) -> &crate::ton::Bool {
        match self {
            Value::Db_Files_Package_Value(ref x) => &x.deleted,
        }
    }

    pub fn firstblocks(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::db::files::package::firstblock::FirstBlock>
    {
        match self {
            Value::Db_Files_Package_Value(ref x) => &x.firstblocks,
        }
    }

    pub fn key(&self) -> &crate::ton::Bool {
        match self {
            Value::Db_Files_Package_Value(ref x) => &x.key,
        }
    }

    pub fn package_id(&self) -> &crate::ton::int {
        match self {
            Value::Db_Files_Package_Value(ref x) => &x.package_id,
        }
    }

    pub fn temp(&self) -> &crate::ton::Bool {
        match self {
            Value::Db_Files_Package_Value(ref x) => &x.temp,
        }
    }

    pub fn only(self) -> crate::ton::db::files::package::value::Value {
        match self {
            Value::Db_Files_Package_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Files_Package_Value(crate::ton::db::files::package::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Files_Package_Value(x) => (crate::ConstructorNumber(0xe44cd52b), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe44cd52b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe44cd52b) => Ok(Value::Db_Files_Package_Value(
                _de.read_bare::<crate::ton::db::files::package::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod firstblock;
pub mod key;
pub mod value;
