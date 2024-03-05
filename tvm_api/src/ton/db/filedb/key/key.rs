use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.blockFile`\n\n```text\ndb.filedb.key.blockFile block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub struct BlockFile {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for BlockFile {}
impl crate::BareSerialize for BlockFile {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb0eae471)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockFile { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockFile {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::IntoBoxed for BlockFile {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_BlockFile(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.blockInfo`\n\n```text\ndb.filedb.key.blockInfo block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub struct BlockInfo {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for BlockInfo {}
impl crate::BareSerialize for BlockInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc499d4fc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockInfo { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::IntoBoxed for BlockInfo {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_BlockInfo(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.candidate`\n\n```text\ndb.filedb.key.candidate id:db.candidate.id = db.filedb.Key;\n```\n"]
pub struct Candidate {
    pub id: crate::ton::db::candidate::id::Id,
}
impl Eq for Candidate {}
impl crate::BareSerialize for Candidate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe28a0ab9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Candidate { id } = self;
        _ser.write_bare::<crate::ton::db::candidate::id::Id>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Candidate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::db::candidate::id::Id>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Candidate {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_Candidate(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.persistentStateFile`\n\n```text\ndb.filedb.key.persistentStateFile block_id:tonNode.blockIdExt masterchain_block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub struct PersistentStateFile {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub masterchain_block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for PersistentStateFile {}
impl crate::BareSerialize for PersistentStateFile {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xafb6764c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PersistentStateFile { block_id, masterchain_block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(masterchain_block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PersistentStateFile {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let masterchain_block_id =
                _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id, masterchain_block_id })
        }
    }
}
impl crate::IntoBoxed for PersistentStateFile {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_PersistentStateFile(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.proof`\n\n```text\ndb.filedb.key.proof block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub struct Proof {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for Proof {}
impl crate::BareSerialize for Proof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xda954dec)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Proof { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Proof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::IntoBoxed for Proof {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_Proof(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.proofLink`\n\n```text\ndb.filedb.key.proofLink block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub struct ProofLink {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for ProofLink {}
impl crate::BareSerialize for ProofLink {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x98fbc5ce)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProofLink { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProofLink {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::IntoBoxed for ProofLink {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_ProofLink(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.signatures`\n\n```text\ndb.filedb.key.signatures block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub struct Signatures {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for Signatures {}
impl crate::BareSerialize for Signatures {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd7290d0b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Signatures { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Signatures {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::IntoBoxed for Signatures {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_Signatures(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.filedb.key.zeroStateFile`\n\n```text\ndb.filedb.key.zeroStateFile block_id:tonNode.blockIdExt = db.filedb.Key;\n```\n"]
pub struct ZeroStateFile {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for ZeroStateFile {}
impl crate::BareSerialize for ZeroStateFile {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1252863d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ZeroStateFile { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ZeroStateFile {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::IntoBoxed for ZeroStateFile {
    type Boxed = crate::ton::db::filedb::Key;

    fn into_boxed(self) -> crate::ton::db::filedb::Key {
        crate::ton::db::filedb::Key::Db_Filedb_Key_ZeroStateFile(self)
    }
}
