use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.fullAccountState`\n\n```text\nraw.fullAccountState balance:int64 code:bytes data:bytes last_transaction_id:internal.transactionId block_id:tonNode.blockIdExt frozen_hash:bytes sync_utime:int53 = raw.FullAccountState;\n```\n"]
pub struct FullAccountState {
    pub balance: crate::ton::int64,
    pub code: crate::ton::bytes,
    pub data: crate::ton::bytes,
    pub last_transaction_id: crate::ton::internal::transactionid::TransactionId,
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub frozen_hash: crate::ton::bytes,
    pub sync_utime: crate::ton::int53,
}
impl Eq for FullAccountState {}
impl crate::BareSerialize for FullAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc265ac17)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let FullAccountState {
            balance,
            code,
            data,
            last_transaction_id,
            block_id,
            frozen_hash,
            sync_utime,
        } = self;
        _ser.write_bare::<crate::ton::int64>(balance)?;
        _ser.write_bare::<crate::ton::bytes>(code)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::internal::transactionid::TransactionId>(last_transaction_id)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        _ser.write_bare::<crate::ton::bytes>(frozen_hash)?;
        _ser.write_bare::<crate::ton::int53>(sync_utime)?;
        Ok(())
    }
}
impl crate::BareDeserialize for FullAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let balance = _de.read_bare::<crate::ton::int64>()?;
            let code = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let last_transaction_id =
                _de.read_bare::<crate::ton::internal::transactionid::TransactionId>()?;
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let frozen_hash = _de.read_bare::<crate::ton::bytes>()?;
            let sync_utime = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { balance, code, data, last_transaction_id, block_id, frozen_hash, sync_utime })
        }
    }
}
impl crate::IntoBoxed for FullAccountState {
    type Boxed = crate::ton::raw::FullAccountState;

    fn into_boxed(self) -> crate::ton::raw::FullAccountState {
        crate::ton::raw::FullAccountState::Raw_FullAccountState(self)
    }
}
