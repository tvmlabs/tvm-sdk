use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.round.Id`\n\n```text\nvalidatorSession.round.id session:int256 height:long prev_block:int256 seqno:int = validatorSession.round.Id;\n```\n"]
pub enum Id {
    ValidatorSession_Round_Id(crate::ton::validator_session::round::id::Id),
}
impl Id {
    pub fn height(&self) -> &crate::ton::long {
        match self {
            Id::ValidatorSession_Round_Id(ref x) => &x.height,
        }
    }
    pub fn prev_block(&self) -> &crate::ton::int256 {
        match self {
            Id::ValidatorSession_Round_Id(ref x) => &x.prev_block,
        }
    }
    pub fn seqno(&self) -> &crate::ton::int {
        match self {
            Id::ValidatorSession_Round_Id(ref x) => &x.seqno,
        }
    }
    pub fn session(&self) -> &crate::ton::int256 {
        match self {
            Id::ValidatorSession_Round_Id(ref x) => &x.session,
        }
    }
    pub fn only(self) -> crate::ton::validator_session::round::id::Id {
        match self {
            Id::ValidatorSession_Round_Id(x) => x,
        }
    }
}
impl Eq for Id {}
impl Default for Id {
    fn default() -> Self {
        Id::ValidatorSession_Round_Id(crate::ton::validator_session::round::id::Id::default())
    }
}
impl crate::BoxedSerialize for Id {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Id::ValidatorSession_Round_Id(x) => (crate::ConstructorNumber(0x0025cfa5), x),
        }
    }
}
impl crate::BoxedDeserialize for Id {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0025cfa5)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0025cfa5) => Ok(Id::ValidatorSession_Round_Id(
                _de.read_bare::<crate::ton::validator_session::round::id::Id>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.round.Message`\n\n```text\nvalidatorSession.message.approvedBlock round:int candidate:int256 signature:bytes = validatorSession.round.Message;\n\nvalidatorSession.message.commit round:int candidate:int256 signature:bytes = validatorSession.round.Message;\n\nvalidatorSession.message.empty round:int attempt:int = validatorSession.round.Message;\n\nvalidatorSession.message.precommit round:int attempt:int candidate:int256 = validatorSession.round.Message;\n\nvalidatorSession.message.rejectedBlock round:int candidate:int256 reason:bytes = validatorSession.round.Message;\n\nvalidatorSession.message.submittedBlock round:int root_hash:int256 file_hash:int256 \n               collated_data_file_hash:int256 = validatorSession.round.Message;\n\nvalidatorSession.message.vote round:int attempt:int candidate:int256 = validatorSession.round.Message;\n\nvalidatorSession.message.voteFor round:int attempt:int candidate:int256 = validatorSession.round.Message;\n```\n"]
pub enum Message {
    ValidatorSession_Message_ApprovedBlock(
        crate::ton::validator_session::round::validator_session::message::message::ApprovedBlock,
    ),
    ValidatorSession_Message_Commit(
        crate::ton::validator_session::round::validator_session::message::message::Commit,
    ),
    ValidatorSession_Message_Empty(
        crate::ton::validator_session::round::validator_session::message::message::Empty,
    ),
    ValidatorSession_Message_Precommit(
        crate::ton::validator_session::round::validator_session::message::message::Precommit,
    ),
    ValidatorSession_Message_RejectedBlock(
        crate::ton::validator_session::round::validator_session::message::message::RejectedBlock,
    ),
    ValidatorSession_Message_SubmittedBlock(
        crate::ton::validator_session::round::validator_session::message::message::SubmittedBlock,
    ),
    ValidatorSession_Message_Vote(
        crate::ton::validator_session::round::validator_session::message::message::Vote,
    ),
    ValidatorSession_Message_VoteFor(
        crate::ton::validator_session::round::validator_session::message::message::VoteFor,
    ),
}
impl Message {
    pub fn attempt(&self) -> Option<&crate::ton::int> {
        match self {
            Message::ValidatorSession_Message_Empty(ref x) => Some(&x.attempt),
            Message::ValidatorSession_Message_Precommit(ref x) => Some(&x.attempt),
            Message::ValidatorSession_Message_Vote(ref x) => Some(&x.attempt),
            Message::ValidatorSession_Message_VoteFor(ref x) => Some(&x.attempt),
            _ => None,
        }
    }
    pub fn candidate(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::ValidatorSession_Message_ApprovedBlock(ref x) => Some(&x.candidate),
            Message::ValidatorSession_Message_Commit(ref x) => Some(&x.candidate),
            Message::ValidatorSession_Message_Precommit(ref x) => Some(&x.candidate),
            Message::ValidatorSession_Message_RejectedBlock(ref x) => Some(&x.candidate),
            Message::ValidatorSession_Message_Vote(ref x) => Some(&x.candidate),
            Message::ValidatorSession_Message_VoteFor(ref x) => Some(&x.candidate),
            _ => None,
        }
    }
    pub fn collated_data_file_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::ValidatorSession_Message_SubmittedBlock(ref x) => {
                Some(&x.collated_data_file_hash)
            }
            _ => None,
        }
    }
    pub fn file_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::ValidatorSession_Message_SubmittedBlock(ref x) => Some(&x.file_hash),
            _ => None,
        }
    }
    pub fn reason(&self) -> Option<&crate::ton::bytes> {
        match self {
            Message::ValidatorSession_Message_RejectedBlock(ref x) => Some(&x.reason),
            _ => None,
        }
    }
    pub fn root_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::ValidatorSession_Message_SubmittedBlock(ref x) => Some(&x.root_hash),
            _ => None,
        }
    }
    pub fn round(&self) -> &crate::ton::int {
        match self {
            Message::ValidatorSession_Message_ApprovedBlock(ref x) => &x.round,
            Message::ValidatorSession_Message_Commit(ref x) => &x.round,
            Message::ValidatorSession_Message_Empty(ref x) => &x.round,
            Message::ValidatorSession_Message_Precommit(ref x) => &x.round,
            Message::ValidatorSession_Message_RejectedBlock(ref x) => &x.round,
            Message::ValidatorSession_Message_SubmittedBlock(ref x) => &x.round,
            Message::ValidatorSession_Message_Vote(ref x) => &x.round,
            Message::ValidatorSession_Message_VoteFor(ref x) => &x.round,
        }
    }
    pub fn signature(&self) -> Option<&crate::ton::bytes> {
        match self {
            Message::ValidatorSession_Message_ApprovedBlock(ref x) => Some(&x.signature),
            Message::ValidatorSession_Message_Commit(ref x) => Some(&x.signature),
            _ => None,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message :: ValidatorSession_Message_ApprovedBlock (crate :: ton :: validator_session :: round :: validator_session :: message :: message :: ApprovedBlock :: default ())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::ValidatorSession_Message_ApprovedBlock(x) => {
                (crate::ConstructorNumber(0x04a5b581), x)
            }
            Message::ValidatorSession_Message_Commit(x) => {
                (crate::ConstructorNumber(0xac129ef5), x)
            }
            Message::ValidatorSession_Message_Empty(x) => (crate::ConstructorNumber(0x4a201fa9), x),
            Message::ValidatorSession_Message_Precommit(x) => {
                (crate::ConstructorNumber(0xa854b552), x)
            }
            Message::ValidatorSession_Message_RejectedBlock(x) => {
                (crate::ConstructorNumber(0x95884e6b), x)
            }
            Message::ValidatorSession_Message_SubmittedBlock(x) => {
                (crate::ConstructorNumber(0x127624b6), x)
            }
            Message::ValidatorSession_Message_Vote(x) => (crate::ConstructorNumber(0x9a3251c7), x),
            Message::ValidatorSession_Message_VoteFor(x) => {
                (crate::ConstructorNumber(0x61f0fe2f), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x04a5b581),
            crate::ConstructorNumber(0xac129ef5),
            crate::ConstructorNumber(0x4a201fa9),
            crate::ConstructorNumber(0xa854b552),
            crate::ConstructorNumber(0x95884e6b),
            crate::ConstructorNumber(0x127624b6),
            crate::ConstructorNumber(0x9a3251c7),
            crate::ConstructorNumber(0x61f0fe2f),
        ]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x04a5b581) => Ok (Message :: ValidatorSession_Message_ApprovedBlock (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: ApprovedBlock > () ?)) , crate :: ConstructorNumber (0xac129ef5) => Ok (Message :: ValidatorSession_Message_Commit (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: Commit > () ?)) , crate :: ConstructorNumber (0x4a201fa9) => Ok (Message :: ValidatorSession_Message_Empty (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: Empty > () ?)) , crate :: ConstructorNumber (0xa854b552) => Ok (Message :: ValidatorSession_Message_Precommit (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: Precommit > () ?)) , crate :: ConstructorNumber (0x95884e6b) => Ok (Message :: ValidatorSession_Message_RejectedBlock (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: RejectedBlock > () ?)) , crate :: ConstructorNumber (0x127624b6) => Ok (Message :: ValidatorSession_Message_SubmittedBlock (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: SubmittedBlock > () ?)) , crate :: ConstructorNumber (0x9a3251c7) => Ok (Message :: ValidatorSession_Message_Vote (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: Vote > () ?)) , crate :: ConstructorNumber (0x61f0fe2f) => Ok (Message :: ValidatorSession_Message_VoteFor (_de . read_bare :: < crate :: ton :: validator_session :: round :: validator_session :: message :: message :: VoteFor > () ?)) , id => _invalid_id ! (id) , }
    }
}
pub mod id;
pub mod validator_session;
