use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.accountState`\n\n```text\npchan.accountState config:pchan.config state:pchan.State description:string = AccountState;\n```\n"]
pub struct AccountState {
    pub config: crate::ton::pchan::config::Config,
    pub state: crate::ton::pchan::State,
    pub description: crate::ton::string,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x60226f78)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { config, state, description } = self;
        _ser.write_bare::<crate::ton::pchan::config::Config>(config)?;
        _ser.write_boxed::<crate::ton::pchan::State>(state)?;
        _ser.write_bare::<crate::ton::string>(description)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config = _de.read_bare::<crate::ton::pchan::config::Config>()?;
            let state = _de.read_boxed::<crate::ton::pchan::State>()?;
            let description = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { config, state, description })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::AccountState;

    fn into_boxed(self) -> crate::ton::AccountState {
        crate::ton::AccountState::Pchan_AccountState(self)
    }
}
