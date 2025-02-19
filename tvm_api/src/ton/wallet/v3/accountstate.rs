use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `wallet.v3.accountState`\n\n```text\nwallet.v3.accountState wallet_id:int64 seqno:int32 = AccountState;\n```\n"]
pub struct AccountState {
    pub wallet_id: crate::ton::int64,
    pub seqno: crate::ton::int32,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9f7aa84a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { wallet_id, seqno } = self;
        _ser.write_bare::<crate::ton::int64>(wallet_id)?;
        _ser.write_bare::<crate::ton::int32>(seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let wallet_id = _de.read_bare::<crate::ton::int64>()?;
            let seqno = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { wallet_id, seqno })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::AccountState;

    fn into_boxed(self) -> crate::ton::AccountState {
        crate::ton::AccountState::Wallet_V3_AccountState(self)
    }
}
