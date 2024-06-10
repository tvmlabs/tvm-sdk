use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.blockUpdate`\n\n```text\nvalidatorSession.blockUpdate ts:long actions:(vector validatorSession.round.Message) state:int = validatorSession.BlockUpdate;\n```\n"]
pub struct BlockUpdate {
    pub ts: crate::ton::long,
    pub actions:
        crate::ton::vector<crate::ton::Boxed, crate::ton::validator_session::round::Message>,
    pub state: crate::ton::int,
}
impl Eq for BlockUpdate {}
impl crate::BareSerialize for BlockUpdate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9283ce37)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockUpdate { ts, actions, state } = self;
        _ser.write_bare::<crate::ton::long>(ts)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: validator_session :: round :: Message > > (actions) ? ;
        _ser.write_bare::<crate::ton::int>(state)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockUpdate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ts = _de.read_bare::<crate::ton::long>()?;
            let actions = _de.read_bare::<crate::ton::vector<
                crate::ton::Boxed,
                crate::ton::validator_session::round::Message,
            >>()?;
            let state = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { ts, actions, state })
        }
    }
}
impl crate::IntoBoxed for BlockUpdate {
    type Boxed = crate::ton::validator_session::BlockUpdate;
    fn into_boxed(self) -> crate::ton::validator_session::BlockUpdate {
        crate::ton::validator_session::BlockUpdate::ValidatorSession_BlockUpdate(self)
    }
}
