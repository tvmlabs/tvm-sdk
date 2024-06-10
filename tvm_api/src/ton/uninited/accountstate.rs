use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `uninited.accountState`\n\n```text\nuninited.accountState frozen_hash:bytes = AccountState;\n```\n"]
pub struct AccountState {
    pub frozen_hash: crate::ton::bytes,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5abd9708)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { frozen_hash } = self;
        _ser.write_bare::<crate::ton::bytes>(frozen_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let frozen_hash = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { frozen_hash })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::AccountState;
    fn into_boxed(self) -> crate::ton::AccountState {
        crate::ton::AccountState::Uninited_AccountState(self)
    }
}
