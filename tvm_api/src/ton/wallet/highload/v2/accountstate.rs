use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `wallet.highload.v2.accountState`\n\n```text\nwallet.highload.v2.accountState wallet_id:int64 = AccountState;\n```\n"]
pub struct AccountState {
    pub wallet_id: crate::ton::int64,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x947d5d4f)
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
        crate::ton::AccountState::Wallet_Highload_V2_AccountState(self)
    }
}
