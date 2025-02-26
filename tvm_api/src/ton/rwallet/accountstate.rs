use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rwallet.accountState`\n\n```text\nrwallet.accountState wallet_id:int64 seqno:int32 unlocked_balance:int64 config:rwallet.config = AccountState;\n```\n"]
pub struct AccountState {
    pub wallet_id: crate::ton::int64,
    pub seqno: crate::ton::int32,
    pub unlocked_balance: crate::ton::int64,
    pub config: crate::ton::rwallet::config::Config,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd3eb83d8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { wallet_id, seqno, unlocked_balance, config } = self;
        _ser.write_bare::<crate::ton::int64>(wallet_id)?;
        _ser.write_bare::<crate::ton::int32>(seqno)?;
        _ser.write_bare::<crate::ton::int64>(unlocked_balance)?;
        _ser.write_bare::<crate::ton::rwallet::config::Config>(config)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let wallet_id = _de.read_bare::<crate::ton::int64>()?;
            let seqno = _de.read_bare::<crate::ton::int32>()?;
            let unlocked_balance = _de.read_bare::<crate::ton::int64>()?;
            let config = _de.read_bare::<crate::ton::rwallet::config::Config>()?;
            Ok(Self { wallet_id, seqno, unlocked_balance, config })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::AccountState;

    fn into_boxed(self) -> crate::ton::AccountState {
        crate::ton::AccountState::Rwallet_AccountState(self)
    }
}
