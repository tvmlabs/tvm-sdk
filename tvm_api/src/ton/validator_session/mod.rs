use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.BlockUpdate`\n\n```text\nvalidatorSession.blockUpdate ts:long actions:(vector validatorSession.round.Message) state:int = validatorSession.BlockUpdate;\n```\n"]
pub enum BlockUpdate {
    ValidatorSession_BlockUpdate(crate::ton::validator_session::blockupdate::BlockUpdate),
}
impl BlockUpdate {
    pub fn actions(
        &self,
    ) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::validator_session::round::Message> {
        match self {
            BlockUpdate::ValidatorSession_BlockUpdate(ref x) => &x.actions,
        }
    }

    pub fn state(&self) -> &crate::ton::int {
        match self {
            BlockUpdate::ValidatorSession_BlockUpdate(ref x) => &x.state,
        }
    }

    pub fn ts(&self) -> &crate::ton::long {
        match self {
            BlockUpdate::ValidatorSession_BlockUpdate(ref x) => &x.ts,
        }
    }

    pub fn only(self) -> crate::ton::validator_session::blockupdate::BlockUpdate {
        match self {
            BlockUpdate::ValidatorSession_BlockUpdate(x) => x,
        }
    }
}
impl Eq for BlockUpdate {}
impl Default for BlockUpdate {
    fn default() -> Self {
        BlockUpdate::ValidatorSession_BlockUpdate(
            crate::ton::validator_session::blockupdate::BlockUpdate::default(),
        )
    }
}
impl crate::BoxedSerialize for BlockUpdate {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockUpdate::ValidatorSession_BlockUpdate(x) => {
                (crate::ConstructorNumber(0x9283ce37), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for BlockUpdate {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x9283ce37)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x9283ce37) => Ok(BlockUpdate::ValidatorSession_BlockUpdate(
                _de.read_bare::<crate::ton::validator_session::blockupdate::BlockUpdate>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.Candidate`\n\n```text\nvalidatorSession.candidate src:int256 round:int root_hash:int256 data:bytes collated_data:bytes = validatorSession.Candidate;\n```\n"]
pub enum Candidate {
    ValidatorSession_Candidate(crate::ton::validator_session::candidate::Candidate),
}
impl Candidate {
    pub fn collated_data(&self) -> &crate::ton::bytes {
        match self {
            Candidate::ValidatorSession_Candidate(ref x) => &x.collated_data,
        }
    }

    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            Candidate::ValidatorSession_Candidate(ref x) => &x.data,
        }
    }

    pub fn root_hash(&self) -> &crate::ton::int256 {
        match self {
            Candidate::ValidatorSession_Candidate(ref x) => &x.root_hash,
        }
    }

    pub fn round(&self) -> &crate::ton::int {
        match self {
            Candidate::ValidatorSession_Candidate(ref x) => &x.round,
        }
    }

    pub fn src(&self) -> &crate::ton::int256 {
        match self {
            Candidate::ValidatorSession_Candidate(ref x) => &x.src,
        }
    }

    pub fn only(self) -> crate::ton::validator_session::candidate::Candidate {
        match self {
            Candidate::ValidatorSession_Candidate(x) => x,
        }
    }
}
impl Eq for Candidate {}
impl Default for Candidate {
    fn default() -> Self {
        Candidate::ValidatorSession_Candidate(
            crate::ton::validator_session::candidate::Candidate::default(),
        )
    }
}
impl crate::BoxedSerialize for Candidate {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Candidate::ValidatorSession_Candidate(x) => (crate::ConstructorNumber(0x7d337845), x),
        }
    }
}
impl crate::BoxedDeserialize for Candidate {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7d337845)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7d337845) => Ok(Candidate::ValidatorSession_Candidate(
                _de.read_bare::<crate::ton::validator_session::candidate::Candidate>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.CandidateId`\n\n```text\nvalidatorSession.candidateId src:int256 root_hash:int256 file_hash:int256 collated_data_file_hash:int256 = validatorSession.CandidateId;\n```\n"]
pub enum CandidateId {
    ValidatorSession_CandidateId(crate::ton::validator_session::candidateid::CandidateId),
}
impl CandidateId {
    pub fn collated_data_file_hash(&self) -> &crate::ton::int256 {
        match self {
            CandidateId::ValidatorSession_CandidateId(ref x) => &x.collated_data_file_hash,
        }
    }

    pub fn file_hash(&self) -> &crate::ton::int256 {
        match self {
            CandidateId::ValidatorSession_CandidateId(ref x) => &x.file_hash,
        }
    }

    pub fn root_hash(&self) -> &crate::ton::int256 {
        match self {
            CandidateId::ValidatorSession_CandidateId(ref x) => &x.root_hash,
        }
    }

    pub fn src(&self) -> &crate::ton::int256 {
        match self {
            CandidateId::ValidatorSession_CandidateId(ref x) => &x.src,
        }
    }

    pub fn only(self) -> crate::ton::validator_session::candidateid::CandidateId {
        match self {
            CandidateId::ValidatorSession_CandidateId(x) => x,
        }
    }
}
impl Eq for CandidateId {}
impl Default for CandidateId {
    fn default() -> Self {
        CandidateId::ValidatorSession_CandidateId(
            crate::ton::validator_session::candidateid::CandidateId::default(),
        )
    }
}
impl crate::BoxedSerialize for CandidateId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            CandidateId::ValidatorSession_CandidateId(x) => {
                (crate::ConstructorNumber(0x19fee56c), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for CandidateId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x19fee56c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x19fee56c) => Ok(CandidateId::ValidatorSession_CandidateId(
                _de.read_bare::<crate::ton::validator_session::candidateid::CandidateId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.Config`\n\n```text\nvalidatorSession.config catchain_idle_timeout:double catchain_max_deps:int round_candidates:int next_candidate_delay:double round_attempt_duration:int\n        max_round_attempts:int max_block_size:int max_collated_data_size:int = validatorSession.Config;\n\nvalidatorSession.configNew catchain_idle_timeout:double catchain_max_deps:int round_candidates:int next_candidate_delay:double round_attempt_duration:int\n        max_round_attempts:int max_block_size:int max_collated_data_size:int new_catchain_ids:Bool = validatorSession.Config;\n```\n"]
pub enum Config {
    ValidatorSession_Config(crate::ton::validator_session::config::Config),
    ValidatorSession_ConfigNew(crate::ton::validator_session::config::ConfigNew),
}
impl Config {
    pub fn catchain_idle_timeout(&self) -> &crate::ton::double {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.catchain_idle_timeout,
            Config::ValidatorSession_ConfigNew(ref x) => &x.catchain_idle_timeout,
        }
    }

    pub fn catchain_max_deps(&self) -> &crate::ton::int {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.catchain_max_deps,
            Config::ValidatorSession_ConfigNew(ref x) => &x.catchain_max_deps,
        }
    }

    pub fn max_block_size(&self) -> &crate::ton::int {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.max_block_size,
            Config::ValidatorSession_ConfigNew(ref x) => &x.max_block_size,
        }
    }

    pub fn max_collated_data_size(&self) -> &crate::ton::int {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.max_collated_data_size,
            Config::ValidatorSession_ConfigNew(ref x) => &x.max_collated_data_size,
        }
    }

    pub fn max_round_attempts(&self) -> &crate::ton::int {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.max_round_attempts,
            Config::ValidatorSession_ConfigNew(ref x) => &x.max_round_attempts,
        }
    }

    pub fn new_catchain_ids(&self) -> Option<&crate::ton::Bool> {
        match self {
            Config::ValidatorSession_ConfigNew(ref x) => Some(&x.new_catchain_ids),
            _ => None,
        }
    }

    pub fn next_candidate_delay(&self) -> &crate::ton::double {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.next_candidate_delay,
            Config::ValidatorSession_ConfigNew(ref x) => &x.next_candidate_delay,
        }
    }

    pub fn round_attempt_duration(&self) -> &crate::ton::int {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.round_attempt_duration,
            Config::ValidatorSession_ConfigNew(ref x) => &x.round_attempt_duration,
        }
    }

    pub fn round_candidates(&self) -> &crate::ton::int {
        match self {
            Config::ValidatorSession_Config(ref x) => &x.round_candidates,
            Config::ValidatorSession_ConfigNew(ref x) => &x.round_candidates,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::ValidatorSession_Config(crate::ton::validator_session::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::ValidatorSession_Config(x) => (crate::ConstructorNumber(0xb661fdc3), x),
            Config::ValidatorSession_ConfigNew(x) => (crate::ConstructorNumber(0xf7afa99c), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb661fdc3), crate::ConstructorNumber(0xf7afa99c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb661fdc3) => Ok(Config::ValidatorSession_Config(
                _de.read_bare::<crate::ton::validator_session::config::Config>()?,
            )),
            crate::ConstructorNumber(0xf7afa99c) => Ok(Config::ValidatorSession_ConfigNew(
                _de.read_bare::<crate::ton::validator_session::config::ConfigNew>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `validatorSession.Message`\n\n```text\nvalidatorSession.message.finishSession = validatorSession.Message;\n\nvalidatorSession.message.startSession = validatorSession.Message;\n```\n"]
pub enum Message {
    #[default]
    ValidatorSession_Message_FinishSession,
    ValidatorSession_Message_StartSession,
}
impl Eq for Message {}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::ValidatorSession_Message_FinishSession => {
                (crate::ConstructorNumber(0xcb9b22e3), &())
            }
            Message::ValidatorSession_Message_StartSession => {
                (crate::ConstructorNumber(0x96a166d1), &())
            }
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xcb9b22e3), crate::ConstructorNumber(0x96a166d1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xcb9b22e3) => {
                Ok(Message::ValidatorSession_Message_FinishSession)
            }
            crate::ConstructorNumber(0x96a166d1) => {
                Ok(Message::ValidatorSession_Message_StartSession)
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.Pong`\n\n```text\nvalidatorSession.pong hash:long = validatorSession.Pong;\n```\n"]
pub enum Pong {
    ValidatorSession_Pong(crate::ton::validator_session::pong::Pong),
}
impl Pong {
    pub fn hash(&self) -> &crate::ton::long {
        match self {
            Pong::ValidatorSession_Pong(ref x) => &x.hash,
        }
    }

    pub fn only(self) -> crate::ton::validator_session::pong::Pong {
        match self {
            Pong::ValidatorSession_Pong(x) => x,
        }
    }
}
impl Eq for Pong {}
impl Default for Pong {
    fn default() -> Self {
        Pong::ValidatorSession_Pong(crate::ton::validator_session::pong::Pong::default())
    }
}
impl crate::BoxedSerialize for Pong {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Pong::ValidatorSession_Pong(x) => (crate::ConstructorNumber(0xdcc6376d), x),
        }
    }
}
impl crate::BoxedDeserialize for Pong {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdcc6376d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xdcc6376d) => Ok(Pong::ValidatorSession_Pong(
                _de.read_bare::<crate::ton::validator_session::pong::Pong>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod blockupdate;
pub mod candidate;
pub mod candidateid;
pub mod config;
pub mod message;
pub mod pong;
pub mod round;
pub mod temp_block;
