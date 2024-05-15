use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.Candidate`\n\n```text\ndb.candidate source:PublicKey id:tonNode.blockIdExt data:bytes collated_data:bytes = db.Candidate;\n```\n"]
pub enum Candidate {
    Db_Candidate(crate::ton::db::candidate::Candidate),
}
impl Candidate {
    pub fn collated_data(&self) -> &crate::ton::bytes {
        match self {
            Candidate::Db_Candidate(ref x) => &x.collated_data,
        }
    }
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            Candidate::Db_Candidate(ref x) => &x.data,
        }
    }
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Candidate::Db_Candidate(ref x) => &x.id,
        }
    }
    pub fn source(&self) -> &crate::ton::PublicKey {
        match self {
            Candidate::Db_Candidate(ref x) => &x.source,
        }
    }
    pub fn only(self) -> crate::ton::db::candidate::Candidate {
        match self {
            Candidate::Db_Candidate(x) => x,
        }
    }
}
impl Eq for Candidate {}
impl Default for Candidate {
    fn default() -> Self {
        Candidate::Db_Candidate(crate::ton::db::candidate::Candidate::default())
    }
}
impl crate::BoxedSerialize for Candidate {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Candidate::Db_Candidate(x) => (crate::ConstructorNumber(0x65d96ada), x),
        }
    }
}
impl crate::BoxedDeserialize for Candidate {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x65d96ada)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x65d96ada) => Ok(Candidate::Db_Candidate(
                _de.read_bare::<crate::ton::db::candidate::Candidate>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod block;
pub mod blockdb;
pub mod candidate;
pub mod celldb;
pub mod filedb;
pub mod files;
pub mod lt;
pub mod root;
pub mod state;
