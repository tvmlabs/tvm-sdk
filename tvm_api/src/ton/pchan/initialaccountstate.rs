use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.initialAccountState`\n\n```text\npchan.initialAccountState config:pchan.config = InitialAccountState;\n```\n"]
pub struct InitialAccountState {
    pub config: crate::ton::pchan::config::Config,
}
impl Eq for InitialAccountState {}
impl crate::BareSerialize for InitialAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb23e1d44)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let InitialAccountState { config } = self;
        _ser.write_bare::<crate::ton::pchan::config::Config>(config)?;
        Ok(())
    }
}
impl crate::BareDeserialize for InitialAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config = _de.read_bare::<crate::ton::pchan::config::Config>()?;
            Ok(Self { config })
        }
    }
}
impl crate::IntoBoxed for InitialAccountState {
    type Boxed = crate::ton::InitialAccountState;
    fn into_boxed(self) -> crate::ton::InitialAccountState {
        crate::ton::InitialAccountState::Pchan_InitialAccountState(self)
    }
}
