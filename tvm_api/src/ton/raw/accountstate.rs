use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.accountState`\n\n```text\nraw.accountState code:bytes data:bytes frozen_hash:bytes = AccountState;\n```\n"]
pub struct AccountState {
    pub code: crate::ton::bytes,
    pub data: crate::ton::bytes,
    pub frozen_hash: crate::ton::bytes,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe04b963a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { code, data, frozen_hash } = self;
        _ser.write_bare::<crate::ton::bytes>(code)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::bytes>(frozen_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let code = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let frozen_hash = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { code, data, frozen_hash })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::AccountState;

    fn into_boxed(self) -> crate::ton::AccountState {
        crate::ton::AccountState::Raw_AccountState(self)
    }
}
