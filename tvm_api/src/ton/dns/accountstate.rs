use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.accountState`\n\n```text\ndns.accountState wallet_id:int64 = AccountState;\n```\n"]
pub struct AccountState {
    pub wallet_id: crate::ton::int64,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x66fad86a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { wallet_id } = self;
        _ser.write_bare::<crate::ton::int64>(wallet_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let wallet_id = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { wallet_id })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::AccountState;
    fn into_boxed(self) -> crate::ton::AccountState {
        crate::ton::AccountState::Dns_AccountState(self)
    }
}
