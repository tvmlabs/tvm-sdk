use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.candidate`\n\n```text\nvalidatorSession.candidate src:int256 round:int root_hash:int256 data:bytes collated_data:bytes = validatorSession.Candidate;\n```\n"]
pub struct Candidate {
    pub src: crate::ton::int256,
    pub round: crate::ton::int,
    pub root_hash: crate::ton::int256,
    pub data: crate::ton::bytes,
    pub collated_data: crate::ton::bytes,
}
impl Eq for Candidate {}
impl crate::BareSerialize for Candidate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7d337845)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Candidate { src, round, root_hash, data, collated_data } = self;
        _ser.write_bare::<crate::ton::int256>(src)?;
        _ser.write_bare::<crate::ton::int>(round)?;
        _ser.write_bare::<crate::ton::int256>(root_hash)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::bytes>(collated_data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Candidate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_bare::<crate::ton::int256>()?;
            let round = _de.read_bare::<crate::ton::int>()?;
            let root_hash = _de.read_bare::<crate::ton::int256>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let collated_data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { src, round, root_hash, data, collated_data })
        }
    }
}
impl crate::IntoBoxed for Candidate {
    type Boxed = crate::ton::validator_session::Candidate;

    fn into_boxed(self) -> crate::ton::validator_session::Candidate {
        crate::ton::validator_session::Candidate::ValidatorSession_Candidate(self)
    }
}
