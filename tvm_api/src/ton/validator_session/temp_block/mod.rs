use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.tempBlock.Id`\n\n```text\nvalidatorSession.candidate.id round:int256 block_hash:int256 = validatorSession.tempBlock.Id;\n```\n"]
pub enum Id {
    ValidatorSession_Candidate_Id(
        crate::ton::validator_session::temp_block::validator_session::candidate::id::Id,
    ),
}
impl Id {
    pub fn block_hash(&self) -> &crate::ton::int256 {
        match self {
            Id::ValidatorSession_Candidate_Id(ref x) => &x.block_hash,
        }
    }
    pub fn round(&self) -> &crate::ton::int256 {
        match self {
            Id::ValidatorSession_Candidate_Id(ref x) => &x.round,
        }
    }
    pub fn only(
        self,
    ) -> crate::ton::validator_session::temp_block::validator_session::candidate::id::Id {
        match self {
            Id::ValidatorSession_Candidate_Id(x) => x,
        }
    }
}
impl Eq for Id {}
impl Default for Id {
    fn default() -> Self {
        Id :: ValidatorSession_Candidate_Id (crate :: ton :: validator_session :: temp_block :: validator_session :: candidate :: id :: Id :: default ())
    }
}
impl crate::BoxedSerialize for Id {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Id::ValidatorSession_Candidate_Id(x) => (crate::ConstructorNumber(0xbcd74139), x),
        }
    }
}
impl crate::BoxedDeserialize for Id {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbcd74139)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0xbcd74139) => Ok (Id :: ValidatorSession_Candidate_Id (_de . read_bare :: < crate :: ton :: validator_session :: temp_block :: validator_session :: candidate :: id :: Id > () ?)) , id => _invalid_id ! (id) , }
    }
}
pub mod validator_session;
