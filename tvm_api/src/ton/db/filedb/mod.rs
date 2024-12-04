use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.Key`\n\n```text\ndb.filedb.key.blockFile block_id:tonNode.blockIdExt = db.filedb.Key;\n\ndb.filedb.key.blockInfo block_id:tonNode.blockIdExt = db.filedb.Key;\n\ndb.filedb.key.candidate id:db.candidate.id = db.filedb.Key;\n\ndb.filedb.key.empty = db.filedb.Key;\n\ndb.filedb.key.persistentStateFile block_id:tonNode.blockIdExt masterchain_block_id:tonNode.blockIdExt = db.filedb.Key;\n\ndb.filedb.key.proof block_id:tonNode.blockIdExt = db.filedb.Key;\n\ndb.filedb.key.proofLink block_id:tonNode.blockIdExt = db.filedb.Key;\n\ndb.filedb.key.signatures block_id:tonNode.blockIdExt = db.filedb.Key;\n\ndb.filedb.key.zeroStateFile block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub enum Key {
    Db_Filedb_Key_BlockFile(crate::ton::db::filedb::key::key::BlockFile),
    Db_Filedb_Key_BlockInfo(crate::ton::db::filedb::key::key::BlockInfo),
    Db_Filedb_Key_Candidate(crate::ton::db::filedb::key::key::Candidate),
    Db_Filedb_Key_Empty,
    Db_Filedb_Key_PersistentStateFile(crate::ton::db::filedb::key::key::PersistentStateFile),
    Db_Filedb_Key_Proof(crate::ton::db::filedb::key::key::Proof),
    Db_Filedb_Key_ProofLink(crate::ton::db::filedb::key::key::ProofLink),
    Db_Filedb_Key_Signatures(crate::ton::db::filedb::key::key::Signatures),
    Db_Filedb_Key_ZeroStateFile(crate::ton::db::filedb::key::key::ZeroStateFile),
}
impl Key {
    pub fn block_id(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Key::Db_Filedb_Key_BlockFile(ref x) => Some(&x.block_id),
            Key::Db_Filedb_Key_BlockInfo(ref x) => Some(&x.block_id),
            Key::Db_Filedb_Key_PersistentStateFile(ref x) => Some(&x.block_id),
            Key::Db_Filedb_Key_Proof(ref x) => Some(&x.block_id),
            Key::Db_Filedb_Key_ProofLink(ref x) => Some(&x.block_id),
            Key::Db_Filedb_Key_Signatures(ref x) => Some(&x.block_id),
            Key::Db_Filedb_Key_ZeroStateFile(ref x) => Some(&x.block_id),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&crate::ton::db::candidate::id::Id> {
        match self {
            Key::Db_Filedb_Key_Candidate(ref x) => Some(&x.id),
            _ => None,
        }
    }

    pub fn masterchain_block_id(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Key::Db_Filedb_Key_PersistentStateFile(ref x) => Some(&x.masterchain_block_id),
            _ => None,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Db_Filedb_Key_BlockFile(crate::ton::db::filedb::key::key::BlockFile::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Db_Filedb_Key_BlockFile(x) => (crate::ConstructorNumber(0xb0eae471), x),
            Key::Db_Filedb_Key_BlockInfo(x) => (crate::ConstructorNumber(0xc499d4fc), x),
            Key::Db_Filedb_Key_Candidate(x) => (crate::ConstructorNumber(0xe28a0ab9), x),
            Key::Db_Filedb_Key_Empty => (crate::ConstructorNumber(0x7bff274b), &()),
            Key::Db_Filedb_Key_PersistentStateFile(x) => (crate::ConstructorNumber(0xafb6764c), x),
            Key::Db_Filedb_Key_Proof(x) => (crate::ConstructorNumber(0xda954dec), x),
            Key::Db_Filedb_Key_ProofLink(x) => (crate::ConstructorNumber(0x98fbc5ce), x),
            Key::Db_Filedb_Key_Signatures(x) => (crate::ConstructorNumber(0xd7290d0b), x),
            Key::Db_Filedb_Key_ZeroStateFile(x) => (crate::ConstructorNumber(0x1252863d), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xb0eae471),
            crate::ConstructorNumber(0xc499d4fc),
            crate::ConstructorNumber(0xe28a0ab9),
            crate::ConstructorNumber(0x7bff274b),
            crate::ConstructorNumber(0xafb6764c),
            crate::ConstructorNumber(0xda954dec),
            crate::ConstructorNumber(0x98fbc5ce),
            crate::ConstructorNumber(0xd7290d0b),
            crate::ConstructorNumber(0x1252863d),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb0eae471) => Ok(Key::Db_Filedb_Key_BlockFile(
                _de.read_bare::<crate::ton::db::filedb::key::key::BlockFile>()?,
            )),
            crate::ConstructorNumber(0xc499d4fc) => Ok(Key::Db_Filedb_Key_BlockInfo(
                _de.read_bare::<crate::ton::db::filedb::key::key::BlockInfo>()?,
            )),
            crate::ConstructorNumber(0xe28a0ab9) => Ok(Key::Db_Filedb_Key_Candidate(
                _de.read_bare::<crate::ton::db::filedb::key::key::Candidate>()?,
            )),
            crate::ConstructorNumber(0x7bff274b) => Ok(Key::Db_Filedb_Key_Empty),
            crate::ConstructorNumber(0xafb6764c) => Ok(Key::Db_Filedb_Key_PersistentStateFile(
                _de.read_bare::<crate::ton::db::filedb::key::key::PersistentStateFile>()?,
            )),
            crate::ConstructorNumber(0xda954dec) => Ok(Key::Db_Filedb_Key_Proof(
                _de.read_bare::<crate::ton::db::filedb::key::key::Proof>()?,
            )),
            crate::ConstructorNumber(0x98fbc5ce) => Ok(Key::Db_Filedb_Key_ProofLink(
                _de.read_bare::<crate::ton::db::filedb::key::key::ProofLink>()?,
            )),
            crate::ConstructorNumber(0xd7290d0b) => Ok(Key::Db_Filedb_Key_Signatures(
                _de.read_bare::<crate::ton::db::filedb::key::key::Signatures>()?,
            )),
            crate::ConstructorNumber(0x1252863d) => Ok(Key::Db_Filedb_Key_ZeroStateFile(
                _de.read_bare::<crate::ton::db::filedb::key::key::ZeroStateFile>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.Value`\n\n```text\ndb.filedb.value key:db.filedb.Key prev:int256 next:int256 file_hash:int256 = db.filedb.Value;\n```\n"]
pub enum Value {
    Db_Filedb_Value(crate::ton::db::filedb::value::Value),
}
impl Value {
    pub fn file_hash(&self) -> &crate::ton::int256 {
        match self {
            Value::Db_Filedb_Value(ref x) => &x.file_hash,
        }
    }

    pub fn key(&self) -> &crate::ton::db::filedb::Key {
        match self {
            Value::Db_Filedb_Value(ref x) => &x.key,
        }
    }

    pub fn next(&self) -> &crate::ton::int256 {
        match self {
            Value::Db_Filedb_Value(ref x) => &x.next,
        }
    }

    pub fn prev(&self) -> &crate::ton::int256 {
        match self {
            Value::Db_Filedb_Value(ref x) => &x.prev,
        }
    }

    pub fn only(self) -> crate::ton::db::filedb::value::Value {
        match self {
            Value::Db_Filedb_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Db_Filedb_Value(crate::ton::db::filedb::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Db_Filedb_Value(x) => (crate::ConstructorNumber(0xf2dd1a2d), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf2dd1a2d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf2dd1a2d) => {
                Ok(Value::Db_Filedb_Value(_de.read_bare::<crate::ton::db::filedb::value::Value>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
