use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.approvedBlock`\n\n```text\nvalidatorSession.message.approvedBlock round:int candidate:int256 signature:bytes = validatorSession.round.Message;\n```\n"]
pub struct ApprovedBlock {
    pub round: crate::ton::int,
    pub candidate: crate::ton::int256,
    pub signature: crate::ton::bytes,
}
impl Eq for ApprovedBlock {}
impl crate::Signing for ApprovedBlock {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for ApprovedBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x04a5b581)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ApprovedBlock { round, candidate, signature } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int256>(candidate)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ApprovedBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let candidate = _de.read_bare::<crate::ton::int256>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { round, candidate, signature })
        }
    }
}
impl crate::IntoBoxed for ApprovedBlock {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_ApprovedBlock(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.commit`\n\n```text\nvalidatorSession.message.commit round:int candidate:int256 signature:bytes = validatorSession.round.Message;\n```\n"]
pub struct Commit {
    pub round: crate::ton::int,
    pub candidate: crate::ton::int256,
    pub signature: crate::ton::bytes,
}
impl Eq for Commit {}
impl crate::Signing for Commit {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Commit {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xac129ef5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Commit { round, candidate, signature } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int256>(candidate)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Commit {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let candidate = _de.read_bare::<crate::ton::int256>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { round, candidate, signature })
        }
    }
}
impl crate::IntoBoxed for Commit {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_Commit(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.empty`\n\n```text\nvalidatorSession.message.empty round:int attempt:int = validatorSession.round.Message;\n```\n"]
pub struct Empty {
    pub round: crate::ton::int,
    pub attempt: crate::ton::int,
}
impl Eq for Empty {}
impl crate::BareSerialize for Empty {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4a201fa9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Empty { round, attempt } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int>(attempt)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Empty {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let attempt = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { round, attempt })
        }
    }
}
impl crate::IntoBoxed for Empty {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_Empty(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.precommit`\n\n```text\nvalidatorSession.message.precommit round:int attempt:int candidate:int256 = validatorSession.round.Message;\n```\n"]
pub struct Precommit {
    pub round: crate::ton::int,
    pub attempt: crate::ton::int,
    pub candidate: crate::ton::int256,
}
impl Eq for Precommit {}
impl crate::BareSerialize for Precommit {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa854b552)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Precommit { round, attempt, candidate } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int>(attempt)?;
        _ser.write_bare::<crate::ton::int256>(candidate)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Precommit {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let attempt = _de.read_bare::<crate::ton::int>()?;
            let candidate = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { round, attempt, candidate })
        }
    }
}
impl crate::IntoBoxed for Precommit {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_Precommit(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.rejectedBlock`\n\n```text\nvalidatorSession.message.rejectedBlock round:int candidate:int256 reason:bytes = validatorSession.round.Message;\n```\n"]
pub struct RejectedBlock {
    pub round: crate::ton::int,
    pub candidate: crate::ton::int256,
    pub reason: crate::ton::bytes,
}
impl Eq for RejectedBlock {}
impl crate::BareSerialize for RejectedBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x95884e6b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RejectedBlock { round, candidate, reason } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int256>(candidate)?;
        _ser.write_bare::<crate::ton::bytes>(reason)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RejectedBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let candidate = _de.read_bare::<crate::ton::int256>()?;
            let reason = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { round, candidate, reason })
        }
    }
}
impl crate::IntoBoxed for RejectedBlock {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_RejectedBlock(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.submittedBlock`\n\n```text\nvalidatorSession.message.submittedBlock round:int root_hash:int256 file_hash:int256 \n               collated_data_file_hash:int256 = validatorSession.round.Message;\n```\n"]
pub struct SubmittedBlock {
    pub round: crate::ton::int,
    pub root_hash: crate::ton::int256,
    pub file_hash: crate::ton::int256,
    pub collated_data_file_hash: crate::ton::int256,
}
impl Eq for SubmittedBlock {}
impl crate::BareSerialize for SubmittedBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x127624b6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SubmittedBlock { round, root_hash, file_hash, collated_data_file_hash } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int256>(root_hash)?;
        _ser.write_bare::<crate::ton::int256>(file_hash)?;
        _ser.write_bare::<crate::ton::int256>(collated_data_file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SubmittedBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let root_hash = _de.read_bare::<crate::ton::int256>()?;
            let file_hash = _de.read_bare::<crate::ton::int256>()?;
            let collated_data_file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { round, root_hash, file_hash, collated_data_file_hash })
        }
    }
}
impl crate::IntoBoxed for SubmittedBlock {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_SubmittedBlock(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.vote`\n\n```text\nvalidatorSession.message.vote round:int attempt:int candidate:int256 = validatorSession.round.Message;\n```\n"]
pub struct Vote {
    pub round: crate::ton::int,
    pub attempt: crate::ton::int,
    pub candidate: crate::ton::int256,
}
impl Eq for Vote {}
impl crate::BareSerialize for Vote {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9a3251c7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Vote { round, attempt, candidate } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int>(attempt)?;
        _ser.write_bare::<crate::ton::int256>(candidate)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Vote {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let attempt = _de.read_bare::<crate::ton::int>()?;
            let candidate = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { round, attempt, candidate })
        }
    }
}
impl crate::IntoBoxed for Vote {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_Vote(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.message.voteFor`\n\n```text\nvalidatorSession.message.voteFor round:int attempt:int candidate:int256 = validatorSession.round.Message;\n```\n"]
pub struct VoteFor {
    pub round: crate::ton::int,
    pub attempt: crate::ton::int,
    pub candidate: crate::ton::int256,
}
impl Eq for VoteFor {}
impl crate::BareSerialize for VoteFor {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x61f0fe2f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let VoteFor { round, attempt, candidate } = self;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int>(attempt)?;
        _ser.write_bare::<crate::ton::int256>(candidate)?;
        Ok(())
    }
}
impl crate::BareDeserialize for VoteFor {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int>()?;
            let attempt = _de.read_bare::<crate::ton::int>()?;
            let candidate = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { round, attempt, candidate })
        }
    }
}
impl crate::IntoBoxed for VoteFor {
    type Boxed = crate::ton::validator_session::round::Message;

    fn into_boxed(self) -> crate::ton::validator_session::round::Message {
        crate::ton::validator_session::round::Message::ValidatorSession_Message_VoteFor(self)
    }
}
