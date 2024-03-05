use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.round.id`\n\n```text\nvalidatorSession.round.id session:int256 height:long prev_block:int256 seqno:int = validatorSession.round.Id;\n```\n"]
pub struct Id {
    pub session: crate::ton::int256,
    pub height: crate::ton::long,
    pub prev_block: crate::ton::int256,
    pub seqno: crate::ton::int,
}
impl Eq for Id {}
impl crate::BareSerialize for Id {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0025cfa5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Id { session, height, prev_block, seqno } = self;
        _ser.write_bare::<crate::ton::int256>(session)?;
        _ser.write_bare::<crate::ton::long>(height)?;
        _ser.write_bare::<crate::ton::int256>(prev_block)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Id {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let session = _de.read_bare::<crate::ton::int256>()?;
            let height = _de.read_bare::<crate::ton::long>()?;
            let prev_block = _de.read_bare::<crate::ton::int256>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { session, height, prev_block, seqno })
        }
    }
}
impl crate::IntoBoxed for Id {
    type Boxed = crate::ton::validator_session::round::Id;

    fn into_boxed(self) -> crate::ton::validator_session::round::Id {
        crate::ton::validator_session::round::Id::ValidatorSession_Round_Id(self)
    }
}
