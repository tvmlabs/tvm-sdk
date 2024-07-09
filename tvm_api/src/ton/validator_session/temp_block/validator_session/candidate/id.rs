use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.candidate.id`\n\n```text\nvalidatorSession.candidate.id round:int256 block_hash:int256 = validatorSession.tempBlock.Id;\n```\n"]
pub struct Id {
    pub round: crate::ton::int256,
    pub block_hash: crate::ton::int256,
}
impl Eq for Id {}
impl crate::BareSerialize for Id {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbcd74139)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Id { round, block_hash } = self;
        _ser.write_bare::<crate::ton::int256>(round)?;
        _ser.write_bare::<crate::ton::int256>(block_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Id {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let round = _de.read_bare::<crate::ton::int256>()?;
            let block_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { round, block_hash })
        }
    }
}
impl crate::IntoBoxed for Id {
    type Boxed = crate::ton::validator_session::temp_block::Id;
    fn into_boxed(self) -> crate::ton::validator_session::temp_block::Id {
        crate::ton::validator_session::temp_block::Id::ValidatorSession_Candidate_Id(self)
    }
}
