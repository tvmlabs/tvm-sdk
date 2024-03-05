use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `fullAccountState`\n\n```text\nfullAccountState address:accountAddress balance:int64 last_transaction_id:internal.transactionId block_id:tonNode.blockIdExt sync_utime:int53 account_state:AccountState revision:int32 = FullAccountState;\n```\n"]
pub struct FullAccountState {
    pub address: crate::ton::accountaddress::AccountAddress,
    pub balance: crate::ton::int64,
    pub last_transaction_id: crate::ton::internal::transactionid::TransactionId,
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub sync_utime: crate::ton::int53,
    pub account_state: crate::ton::AccountState,
    pub revision: crate::ton::int32,
}
impl Eq for FullAccountState {}
impl crate::BareSerialize for FullAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8419b860)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let FullAccountState {
            address,
            balance,
            last_transaction_id,
            block_id,
            sync_utime,
            account_state,
            revision,
        } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(address)?;
        _ser.write_bare::<crate::ton::int64>(balance)?;
        _ser.write_bare::<crate::ton::internal::transactionid::TransactionId>(last_transaction_id)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        _ser.write_bare::<crate::ton::int53>(sync_utime)?;
        _ser.write_boxed::<crate::ton::AccountState>(account_state)?;
        _ser.write_bare::<crate::ton::int32>(revision)?;
        Ok(())
    }
}
impl crate::BareDeserialize for FullAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let balance = _de.read_bare::<crate::ton::int64>()?;
            let last_transaction_id =
                _de.read_bare::<crate::ton::internal::transactionid::TransactionId>()?;
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let sync_utime = _de.read_bare::<crate::ton::int53>()?;
            let account_state = _de.read_boxed::<crate::ton::AccountState>()?;
            let revision = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self {
                address,
                balance,
                last_transaction_id,
                block_id,
                sync_utime,
                account_state,
                revision,
            })
        }
    }
}
impl crate::IntoBoxed for FullAccountState {
    type Boxed = crate::ton::FullAccountState;

    fn into_boxed(self) -> crate::ton::FullAccountState {
        crate::ton::FullAccountState::FullAccountState(self)
    }
}
