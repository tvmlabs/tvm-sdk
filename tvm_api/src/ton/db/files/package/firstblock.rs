use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.files.package.firstBlock`\n\n```text\ndb.files.package.firstBlock workchain:int shard:long seqno:int unixtime:int lt:long = db.files.package.FirstBlock;\n```\n"]
pub struct FirstBlock {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub seqno: crate::ton::int,
    pub unixtime: crate::ton::int,
    pub lt: crate::ton::long,
}
impl Eq for FirstBlock {}
impl crate::BareSerialize for FirstBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x701269e7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let FirstBlock { workchain, shard, seqno, unixtime, lt } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_bare::<crate::ton::int>(unixtime)?;
        _ser.write_bare::<crate::ton::long>(lt)?;
        Ok(())
    }
}
impl crate::BareDeserialize for FirstBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let unixtime = _de.read_bare::<crate::ton::int>()?;
            let lt = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { workchain, shard, seqno, unixtime, lt })
        }
    }
}
impl crate::IntoBoxed for FirstBlock {
    type Boxed = crate::ton::db::files::package::FirstBlock;

    fn into_boxed(self) -> crate::ton::db::files::package::FirstBlock {
        crate::ton::db::files::package::FirstBlock::Db_Files_Package_FirstBlock(self)
    }
}
