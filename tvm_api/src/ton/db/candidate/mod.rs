use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.candidate`\n\n```text\ndb.candidate source:PublicKey id:tonNode.blockIdExt data:bytes collated_data:bytes = db.Candidate;\n```\n"]
pub struct Candidate {
    pub source: crate::ton::PublicKey,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub data: crate::ton::bytes,
    pub collated_data: crate::ton::bytes,
}
impl Eq for Candidate {}
impl crate::BareSerialize for Candidate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x65d96ada)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Candidate { source, id, data, collated_data } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(source)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::bytes>(collated_data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Candidate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let source = _de.read_boxed::<crate::ton::PublicKey>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let collated_data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { source, id, data, collated_data })
        }
    }
}
impl crate::IntoBoxed for Candidate {
    type Boxed = crate::ton::db::Candidate;

    fn into_boxed(self) -> crate::ton::db::Candidate {
        crate::ton::db::Candidate::Db_Candidate(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.candidate.Id`\n\n```text\ndb.candidate.id source:PublicKey id:tonNode.blockIdExt collated_data_file_hash:int256 = db.candidate.Id;\n```\n"]
pub enum Id {
    Db_Candidate_Id(crate::ton::db::candidate::id::Id),
}
impl Id {
    pub fn collated_data_file_hash(&self) -> &crate::ton::int256 {
        match self {
            Id::Db_Candidate_Id(ref x) => &x.collated_data_file_hash,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Id::Db_Candidate_Id(ref x) => &x.id,
        }
    }

    pub fn source(&self) -> &crate::ton::PublicKey {
        match self {
            Id::Db_Candidate_Id(ref x) => &x.source,
        }
    }

    pub fn only(self) -> crate::ton::db::candidate::id::Id {
        match self {
            Id::Db_Candidate_Id(x) => x,
        }
    }
}
impl Eq for Id {}
impl Default for Id {
    fn default() -> Self {
        Id::Db_Candidate_Id(crate::ton::db::candidate::id::Id::default())
    }
}
impl crate::BoxedSerialize for Id {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Id::Db_Candidate_Id(x) => (crate::ConstructorNumber(0x37c0b287), x),
        }
    }
}
impl crate::BoxedDeserialize for Id {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x37c0b287)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x37c0b287) => {
                Ok(Id::Db_Candidate_Id(_de.read_bare::<crate::ton::db::candidate::id::Id>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod id;
