use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.proposalVote`\n\n```text\nengine.validator.proposalVote perm_key:int256 to_send:bytes = engine.validator.ProposalVote;\n```\n"]
pub struct ProposalVote {
    pub perm_key: crate::ton::int256,
    pub to_send: crate::ton::bytes,
}
impl Eq for ProposalVote {}
impl crate::BareSerialize for ProposalVote {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7f6626ed)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProposalVote { perm_key, to_send } = self;
        _ser.write_bare::<crate::ton::int256>(perm_key)?;
        _ser.write_bare::<crate::ton::bytes>(to_send)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProposalVote {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let perm_key = _de.read_bare::<crate::ton::int256>()?;
            let to_send = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { perm_key, to_send })
        }
    }
}
impl crate::IntoBoxed for ProposalVote {
    type Boxed = crate::ton::engine::validator::ProposalVote;
    fn into_boxed(self) -> crate::ton::engine::validator::ProposalVote {
        crate::ton::engine::validator::ProposalVote::Engine_Validator_ProposalVote(self)
    }
}
