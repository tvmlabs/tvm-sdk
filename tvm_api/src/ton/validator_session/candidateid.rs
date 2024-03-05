use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.candidateId`\n\n```text\nvalidatorSession.candidateId src:int256 root_hash:int256 file_hash:int256 collated_data_file_hash:int256 = validatorSession.CandidateId;\n```\n"]
pub struct CandidateId {
    pub src: crate::ton::int256,
    pub root_hash: crate::ton::int256,
    pub file_hash: crate::ton::int256,
    pub collated_data_file_hash: crate::ton::int256,
}
impl Eq for CandidateId {}
impl crate::BareSerialize for CandidateId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x19fee56c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CandidateId { src, root_hash, file_hash, collated_data_file_hash } = self;
        _ser.write_bare::<crate::ton::int256>(src)?;
        _ser.write_bare::<crate::ton::int256>(root_hash)?;
        _ser.write_bare::<crate::ton::int256>(file_hash)?;
        _ser.write_bare::<crate::ton::int256>(collated_data_file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CandidateId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_bare::<crate::ton::int256>()?;
            let root_hash = _de.read_bare::<crate::ton::int256>()?;
            let file_hash = _de.read_bare::<crate::ton::int256>()?;
            let collated_data_file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { src, root_hash, file_hash, collated_data_file_hash })
        }
    }
}
impl crate::IntoBoxed for CandidateId {
    type Boxed = crate::ton::validator_session::CandidateId;

    fn into_boxed(self) -> crate::ton::validator_session::CandidateId {
        crate::ton::validator_session::CandidateId::ValidatorSession_CandidateId(self)
    }
}
