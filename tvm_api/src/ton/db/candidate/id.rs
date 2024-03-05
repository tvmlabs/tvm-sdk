use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.candidate.id`\n\n```text\ndb.candidate.id source:PublicKey id:tonNode.blockIdExt collated_data_file_hash:int256 = db.candidate.Id;\n```\n"]
pub struct Id {
    pub source: crate::ton::PublicKey,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub collated_data_file_hash: crate::ton::int256,
}
impl Eq for Id {}
impl crate::BareSerialize for Id {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x37c0b287)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Id { source, id, collated_data_file_hash } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(source)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int256>(collated_data_file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Id {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let source = _de.read_boxed::<crate::ton::PublicKey>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let collated_data_file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { source, id, collated_data_file_hash })
        }
    }
}
impl crate::IntoBoxed for Id {
    type Boxed = crate::ton::db::candidate::Id;

    fn into_boxed(self) -> crate::ton::db::candidate::Id {
        crate::ton::db::candidate::Id::Db_Candidate_Id(self)
    }
}
