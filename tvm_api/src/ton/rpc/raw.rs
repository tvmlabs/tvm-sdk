use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.createAndSendMessage`\n\n```text\nraw.createAndSendMessage destination:accountAddress initial_account_state:bytes data:bytes = Ok;\n```\n"]
pub struct CreateAndSendMessage {
    pub destination: crate::ton::accountaddress::AccountAddress,
    pub initial_account_state: crate::ton::bytes,
    pub data: crate::ton::bytes,
}
impl Eq for CreateAndSendMessage {}
impl crate::BareSerialize for CreateAndSendMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd1f8c9a5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateAndSendMessage { destination, initial_account_state, data } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(destination)?;
        _ser.write_bare::<crate::ton::bytes>(initial_account_state)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateAndSendMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let destination = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let initial_account_state = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { destination, initial_account_state, data })
        }
    }
}
impl crate::BoxedDeserialize for CreateAndSendMessage {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd1f8c9a5)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd1f8c9a5) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CreateAndSendMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd1f8c9a5), self)
    }
}
impl crate::Function for CreateAndSendMessage {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.createQuery`\n\n```text\nraw.createQuery destination:accountAddress init_code:bytes init_data:bytes body:bytes = query.Info;\n```\n"]
pub struct CreateQuery {
    pub destination: crate::ton::accountaddress::AccountAddress,
    pub init_code: crate::ton::bytes,
    pub init_data: crate::ton::bytes,
    pub body: crate::ton::bytes,
}
impl Eq for CreateQuery {}
impl crate::BareSerialize for CreateQuery {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8d0c8aab)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateQuery { destination, init_code, init_data, body } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(destination)?;
        _ser.write_bare::<crate::ton::bytes>(init_code)?;
        _ser.write_bare::<crate::ton::bytes>(init_data)?;
        _ser.write_bare::<crate::ton::bytes>(body)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateQuery {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let destination = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let init_code = _de.read_bare::<crate::ton::bytes>()?;
            let init_data = _de.read_bare::<crate::ton::bytes>()?;
            let body = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { destination, init_code, init_data, body })
        }
    }
}
impl crate::BoxedDeserialize for CreateQuery {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8d0c8aab)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8d0c8aab) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CreateQuery {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8d0c8aab), self)
    }
}
impl crate::Function for CreateQuery {
    type Reply = crate::ton::query::Info;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.getAccount`\n\n```text\nraw.getAccount account_address:accountAddress workchain:int32 = Data;\n```\n"]
pub struct GetAccount {
    pub account_address: crate::ton::accountaddress::AccountAddress,
    pub workchain: crate::ton::int32,
}
impl Eq for GetAccount {}
impl crate::BareSerialize for GetAccount {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x165742b9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetAccount { account_address, workchain } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        _ser.write_bare::<crate::ton::int32>(workchain)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetAccount {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let workchain = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { account_address, workchain })
        }
    }
}
impl crate::BoxedDeserialize for GetAccount {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x165742b9)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x165742b9) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAccount {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x165742b9), self)
    }
}
impl crate::Function for GetAccount {
    type Reply = crate::ton::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.getAccountByBlock`\n\n```text\nraw.getAccountByBlock block_root_hash:int256 account_id:int256 = raw.ShardAccountState;\n```\n"]
pub struct GetAccountByBlock {
    pub block_root_hash: crate::ton::int256,
    pub account_id: crate::ton::int256,
}
impl Eq for GetAccountByBlock {}
impl crate::BareSerialize for GetAccountByBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x290e6f24)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetAccountByBlock { block_root_hash, account_id } = self;
        _ser.write_bare::<crate::ton::int256>(block_root_hash)?;
        _ser.write_bare::<crate::ton::int256>(account_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetAccountByBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let account_id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { block_root_hash, account_id })
        }
    }
}
impl crate::BoxedDeserialize for GetAccountByBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x290e6f24)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x290e6f24) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAccountByBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x290e6f24), self)
    }
}
impl crate::Function for GetAccountByBlock {
    type Reply = crate::ton::raw::ShardAccountState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.getAccountMetaByBlock`\n\n```text\nraw.getAccountMetaByBlock block_root_hash:int256 account_id:int256 = raw.ShardAccountMeta;\n```\n"]
pub struct GetAccountMetaByBlock {
    pub block_root_hash: crate::ton::int256,
    pub account_id: crate::ton::int256,
}
impl Eq for GetAccountMetaByBlock {}
impl crate::BareSerialize for GetAccountMetaByBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfe31dae0)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetAccountMetaByBlock { block_root_hash, account_id } = self;
        _ser.write_bare::<crate::ton::int256>(block_root_hash)?;
        _ser.write_bare::<crate::ton::int256>(account_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetAccountMetaByBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let account_id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { block_root_hash, account_id })
        }
    }
}
impl crate::BoxedDeserialize for GetAccountMetaByBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfe31dae0)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xfe31dae0) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAccountMetaByBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xfe31dae0), self)
    }
}
impl crate::Function for GetAccountMetaByBlock {
    type Reply = crate::ton::raw::ShardAccountMeta;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.getAppliedShardsInfo`\n\n```text\nraw.getAppliedShardsInfo = raw.AppliedShardsInfo;\n```\n"]
pub struct GetAppliedShardsInfo;
impl Eq for GetAppliedShardsInfo {}
impl crate::BareSerialize for GetAppliedShardsInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfc01cdbb)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetAppliedShardsInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetAppliedShardsInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfc01cdbb)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xfc01cdbb) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAppliedShardsInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xfc01cdbb), self)
    }
}
impl crate::Function for GetAppliedShardsInfo {
    type Reply = crate::ton::raw::AppliedShardsInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.getShardAccountMeta`\n\n```text\nraw.getShardAccountMeta account_address:accountAddress = raw.ShardAccountMeta;\n```\n"]
pub struct GetShardAccountMeta {
    pub account_address: crate::ton::accountaddress::AccountAddress,
}
impl Eq for GetShardAccountMeta {}
impl crate::BareSerialize for GetShardAccountMeta {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5ae44062)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetShardAccountMeta { account_address } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetShardAccountMeta {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            Ok(Self { account_address })
        }
    }
}
impl crate::BoxedDeserialize for GetShardAccountMeta {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5ae44062)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x5ae44062) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetShardAccountMeta {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x5ae44062), self)
    }
}
impl crate::Function for GetShardAccountMeta {
    type Reply = crate::ton::raw::ShardAccountMeta;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.getShardAccountState`\n\n```text\nraw.getShardAccountState account_address:accountAddress = raw.ShardAccountState;\n```\n"]
pub struct GetShardAccountState {
    pub account_address: crate::ton::accountaddress::AccountAddress,
}
impl Eq for GetShardAccountState {}
impl crate::BareSerialize for GetShardAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x34adc00a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetShardAccountState { account_address } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetShardAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            Ok(Self { account_address })
        }
    }
}
impl crate::BoxedDeserialize for GetShardAccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x34adc00a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x34adc00a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetShardAccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x34adc00a), self)
    }
}
impl crate::Function for GetShardAccountState {
    type Reply = crate::ton::raw::ShardAccountState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.getTransactions`\n\n```text\nraw.getTransactions private_key:InputKey account_address:accountAddress from_transaction_id:internal.transactionId = raw.Transactions;\n```\n"]
pub struct GetTransactions {
    pub private_key: crate::ton::InputKey,
    pub account_address: crate::ton::accountaddress::AccountAddress,
    pub from_transaction_id: crate::ton::internal::transactionid::TransactionId,
}
impl Eq for GetTransactions {}
impl crate::BareSerialize for GetTransactions {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3d5ea31d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetTransactions { private_key, account_address, from_transaction_id } = self;
        _ser.write_boxed::<crate::ton::InputKey>(private_key)?;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        _ser.write_bare::<crate::ton::internal::transactionid::TransactionId>(from_transaction_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetTransactions {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let private_key = _de.read_boxed::<crate::ton::InputKey>()?;
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let from_transaction_id =
                _de.read_bare::<crate::ton::internal::transactionid::TransactionId>()?;
            Ok(Self { private_key, account_address, from_transaction_id })
        }
    }
}
impl crate::BoxedDeserialize for GetTransactions {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3d5ea31d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x3d5ea31d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetTransactions {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x3d5ea31d), self)
    }
}
impl crate::Function for GetTransactions {
    type Reply = crate::ton::raw::Transactions;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.sendMessage`\n\n```text\nraw.sendMessage body:bytes  = Ok;\n```\n"]
pub struct SendMessage {
    pub body: crate::ton::bytes,
}
impl Eq for SendMessage {}
impl crate::BareSerialize for SendMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x955780e0)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SendMessage { body } = self;
        _ser.write_bare::<crate::ton::bytes>(body)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SendMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let body = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { body })
        }
    }
}
impl crate::BoxedDeserialize for SendMessage {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x955780e0)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x955780e0) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SendMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x955780e0), self)
    }
}
impl crate::Function for SendMessage {
    type Reply = crate::ton::Ok;
}
