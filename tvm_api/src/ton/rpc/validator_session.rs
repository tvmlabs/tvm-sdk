use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.downloadCandidate`\n\n```text\nvalidatorSession.downloadCandidate round:int id:validatorSession.candidateId = validatorSession.Candidate;\n```\n"]
pub struct DownloadCandidate {
    pub round: crate::ton::int,
    pub id: crate::ton::validator_session::candidateid::CandidateId,
}
impl Eq for DownloadCandidate {}
impl crate::BareSerialize for DownloadCandidate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe0fd3df5)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadCandidate { round, id } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::validator_session::candidateid::CandidateId>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadCandidate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::validator_session::candidateid::CandidateId>()?;
            Ok(Self { round, id })
        }
    }
}
impl crate::BoxedDeserialize for DownloadCandidate {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe0fd3df5)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xe0fd3df5) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for DownloadCandidate {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xe0fd3df5), self)
    }
}
impl crate::Function for DownloadCandidate {
    type Reply = crate::ton::validator_session::Candidate;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.ping`\n\n```text\nvalidatorSession.ping hash:long = validatorSession.Pong;\n```\n"]
pub struct Ping {
    pub hash: crate::ton::long,
}
impl Eq for Ping {}
impl crate::BareSerialize for Ping {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x680449ad)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Ping { hash } = self;
        _ser.write_bare::<crate::ton::long>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Ping {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { hash })
        }
    }
}
impl crate::BoxedDeserialize for Ping {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x680449ad)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x680449ad) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for Ping {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x680449ad), self)
    }
}
impl crate::Function for Ping {
    type Reply = crate::ton::validator_session::Pong;
}
