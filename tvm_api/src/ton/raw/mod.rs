use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `raw.appliedShardsInfo`\n\n```text\nraw.appliedShardsInfo shards:vector<tonNode.blockIdExt> = raw.appliedShardsInfo;\n```\n"]
pub enum AppliedShardsInfo {
    Raw_AppliedShardsInfo(crate::ton::raw::appliedshardsinfo::AppliedShardsInfo),
}
impl AppliedShardsInfo {
    pub fn shards(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            AppliedShardsInfo::Raw_AppliedShardsInfo(ref x) => &x.shards,
        }
    }

    pub fn only(self) -> crate::ton::raw::appliedshardsinfo::AppliedShardsInfo {
        match self {
            AppliedShardsInfo::Raw_AppliedShardsInfo(x) => x,
        }
    }
}
impl Eq for AppliedShardsInfo {}
impl Default for AppliedShardsInfo {
    fn default() -> Self {
        AppliedShardsInfo::Raw_AppliedShardsInfo(
            crate::ton::raw::appliedshardsinfo::AppliedShardsInfo::default(),
        )
    }
}
impl crate::BoxedSerialize for AppliedShardsInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AppliedShardsInfo::Raw_AppliedShardsInfo(x) => {
                (crate::ConstructorNumber(0x683ae48f), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for AppliedShardsInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x683ae48f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x683ae48f) => Ok(AppliedShardsInfo::Raw_AppliedShardsInfo(
                _de.read_bare::<crate::ton::raw::appliedshardsinfo::AppliedShardsInfo>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `raw.FullAccountState`\n\n```text\nraw.fullAccountState balance:int64 code:bytes data:bytes last_transaction_id:internal.transactionId block_id:tonNode.blockIdExt frozen_hash:bytes sync_utime:int53 = raw.FullAccountState;\n```\n"]
pub enum FullAccountState {
    Raw_FullAccountState(crate::ton::raw::fullaccountstate::FullAccountState),
}
impl FullAccountState {
    pub fn balance(&self) -> &crate::ton::int64 {
        match self {
            FullAccountState::Raw_FullAccountState(ref x) => &x.balance,
        }
    }

    pub fn block_id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            FullAccountState::Raw_FullAccountState(ref x) => &x.block_id,
        }
    }

    pub fn code(&self) -> &crate::ton::bytes {
        match self {
            FullAccountState::Raw_FullAccountState(ref x) => &x.code,
        }
    }

    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            FullAccountState::Raw_FullAccountState(ref x) => &x.data,
        }
    }

    pub fn frozen_hash(&self) -> &crate::ton::bytes {
        match self {
            FullAccountState::Raw_FullAccountState(ref x) => &x.frozen_hash,
        }
    }

    pub fn last_transaction_id(&self) -> &crate::ton::internal::transactionid::TransactionId {
        match self {
            FullAccountState::Raw_FullAccountState(ref x) => &x.last_transaction_id,
        }
    }

    pub fn sync_utime(&self) -> &crate::ton::int53 {
        match self {
            FullAccountState::Raw_FullAccountState(ref x) => &x.sync_utime,
        }
    }

    pub fn only(self) -> crate::ton::raw::fullaccountstate::FullAccountState {
        match self {
            FullAccountState::Raw_FullAccountState(x) => x,
        }
    }
}
impl Eq for FullAccountState {}
impl Default for FullAccountState {
    fn default() -> Self {
        FullAccountState::Raw_FullAccountState(
            crate::ton::raw::fullaccountstate::FullAccountState::default(),
        )
    }
}
impl crate::BoxedSerialize for FullAccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            FullAccountState::Raw_FullAccountState(x) => (crate::ConstructorNumber(0xc265ac17), x),
        }
    }
}
impl crate::BoxedDeserialize for FullAccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc265ac17)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc265ac17) => Ok(FullAccountState::Raw_FullAccountState(
                _de.read_bare::<crate::ton::raw::fullaccountstate::FullAccountState>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `raw.Message`\n\n```text\nraw.message source:accountAddress destination:accountAddress value:int64 fwd_fee:int64 ihr_fee:int64 created_lt:int64 body_hash:bytes msg_data:msg.Data = raw.Message;\n```\n"]
pub enum Message {
    Raw_Message(crate::ton::raw::message::Message),
}
impl Message {
    pub fn body_hash(&self) -> &crate::ton::bytes {
        match self {
            Message::Raw_Message(ref x) => &x.body_hash,
        }
    }

    pub fn created_lt(&self) -> &crate::ton::int64 {
        match self {
            Message::Raw_Message(ref x) => &x.created_lt,
        }
    }

    pub fn destination(&self) -> &crate::ton::accountaddress::AccountAddress {
        match self {
            Message::Raw_Message(ref x) => &x.destination,
        }
    }

    pub fn fwd_fee(&self) -> &crate::ton::int64 {
        match self {
            Message::Raw_Message(ref x) => &x.fwd_fee,
        }
    }

    pub fn ihr_fee(&self) -> &crate::ton::int64 {
        match self {
            Message::Raw_Message(ref x) => &x.ihr_fee,
        }
    }

    pub fn msg_data(&self) -> &crate::ton::msg::Data {
        match self {
            Message::Raw_Message(ref x) => &x.msg_data,
        }
    }

    pub fn source(&self) -> &crate::ton::accountaddress::AccountAddress {
        match self {
            Message::Raw_Message(ref x) => &x.source,
        }
    }

    pub fn value(&self) -> &crate::ton::int64 {
        match self {
            Message::Raw_Message(ref x) => &x.value,
        }
    }

    pub fn only(self) -> crate::ton::raw::message::Message {
        match self {
            Message::Raw_Message(x) => x,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message::Raw_Message(crate::ton::raw::message::Message::default())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::Raw_Message(x) => (crate::ConstructorNumber(0x518b724f), x),
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x518b724f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x518b724f) => {
                Ok(Message::Raw_Message(_de.read_bare::<crate::ton::raw::message::Message>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `raw.ShardAccountMeta`\n\n```text\nraw.shardAccountMeta shard_account_meta:bytes = raw.ShardAccountMeta;\n\nraw.shardAccountMetaNone = raw.ShardAccountMeta;\n```\n"]
pub enum ShardAccountMeta {
    Raw_ShardAccountMeta(crate::ton::raw::shardaccountmeta::ShardAccountMeta),
    Raw_ShardAccountMetaNone,
}
impl ShardAccountMeta {
    pub fn shard_account_meta(&self) -> Option<&crate::ton::bytes> {
        match self {
            ShardAccountMeta::Raw_ShardAccountMeta(ref x) => Some(&x.shard_account_meta),
            _ => None,
        }
    }
}
impl Eq for ShardAccountMeta {}
impl Default for ShardAccountMeta {
    fn default() -> Self {
        ShardAccountMeta::Raw_ShardAccountMeta(
            crate::ton::raw::shardaccountmeta::ShardAccountMeta::default(),
        )
    }
}
impl crate::BoxedSerialize for ShardAccountMeta {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ShardAccountMeta::Raw_ShardAccountMeta(x) => (crate::ConstructorNumber(0xdc93781a), x),
            ShardAccountMeta::Raw_ShardAccountMetaNone => {
                (crate::ConstructorNumber(0x6b946589), &())
            }
        }
    }
}
impl crate::BoxedDeserialize for ShardAccountMeta {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdc93781a), crate::ConstructorNumber(0x6b946589)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xdc93781a) => Ok(ShardAccountMeta::Raw_ShardAccountMeta(
                _de.read_bare::<crate::ton::raw::shardaccountmeta::ShardAccountMeta>()?,
            )),
            crate::ConstructorNumber(0x6b946589) => Ok(ShardAccountMeta::Raw_ShardAccountMetaNone),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::raw::shardaccountmeta::ShardAccountMeta> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x6b946589), &()),
            Some(ref x) => (crate::ConstructorNumber(0xdc93781a), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::raw::shardaccountmeta::ShardAccountMeta> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6b946589), crate::ConstructorNumber(0xdc93781a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x6b946589) => Ok(None),
            crate::ConstructorNumber(0xdc93781a) => {
                Ok(Some(_de.read_bare::<crate::ton::raw::shardaccountmeta::ShardAccountMeta>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `raw.ShardAccountState`\n\n```text\nraw.shardAccountNone = raw.ShardAccountState;\n\nraw.shardAccountState shard_account:bytes = raw.ShardAccountState;\n```\n"]
pub enum ShardAccountState {
    #[default]
    Raw_ShardAccountNone,
    Raw_ShardAccountState(crate::ton::raw::shardaccountstate::ShardAccountState),
}
impl ShardAccountState {
    pub fn shard_account(&self) -> Option<&crate::ton::bytes> {
        match self {
            ShardAccountState::Raw_ShardAccountState(ref x) => Some(&x.shard_account),
            _ => None,
        }
    }
}
impl Eq for ShardAccountState {}
impl crate::BoxedSerialize for ShardAccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ShardAccountState::Raw_ShardAccountNone => (crate::ConstructorNumber(0x4ee44d79), &()),
            ShardAccountState::Raw_ShardAccountState(x) => {
                (crate::ConstructorNumber(0xc5a7834f), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ShardAccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4ee44d79), crate::ConstructorNumber(0xc5a7834f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x4ee44d79) => Ok(ShardAccountState::Raw_ShardAccountNone),
            crate::ConstructorNumber(0xc5a7834f) => Ok(ShardAccountState::Raw_ShardAccountState(
                _de.read_bare::<crate::ton::raw::shardaccountstate::ShardAccountState>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::raw::shardaccountstate::ShardAccountState> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x4ee44d79), &()),
            Some(ref x) => (crate::ConstructorNumber(0xc5a7834f), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::raw::shardaccountstate::ShardAccountState> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4ee44d79), crate::ConstructorNumber(0xc5a7834f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x4ee44d79) => Ok(None),
            crate::ConstructorNumber(0xc5a7834f) => {
                Ok(Some(_de.read_bare::<crate::ton::raw::shardaccountstate::ShardAccountState>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `raw.Transaction`\n\n```text\nraw.transaction utime:int53 data:bytes transaction_id:internal.transactionId fee:int64 storage_fee:int64 other_fee:int64 in_msg:raw.message out_msgs:vector<raw.message> = raw.Transaction;\n```\n"]
pub enum Transaction {
    Raw_Transaction(crate::ton::raw::transaction::Transaction),
}
impl Transaction {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.data,
        }
    }

    pub fn fee(&self) -> &crate::ton::int64 {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.fee,
        }
    }

    pub fn in_msg(&self) -> &crate::ton::raw::message::Message {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.in_msg,
        }
    }

    pub fn other_fee(&self) -> &crate::ton::int64 {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.other_fee,
        }
    }

    pub fn out_msgs(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::raw::message::Message> {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.out_msgs,
        }
    }

    pub fn storage_fee(&self) -> &crate::ton::int64 {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.storage_fee,
        }
    }

    pub fn transaction_id(&self) -> &crate::ton::internal::transactionid::TransactionId {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.transaction_id,
        }
    }

    pub fn utime(&self) -> &crate::ton::int53 {
        match self {
            Transaction::Raw_Transaction(ref x) => &x.utime,
        }
    }

    pub fn only(self) -> crate::ton::raw::transaction::Transaction {
        match self {
            Transaction::Raw_Transaction(x) => x,
        }
    }
}
impl Eq for Transaction {}
impl Default for Transaction {
    fn default() -> Self {
        Transaction::Raw_Transaction(crate::ton::raw::transaction::Transaction::default())
    }
}
impl crate::BoxedSerialize for Transaction {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Transaction::Raw_Transaction(x) => (crate::ConstructorNumber(0x70828481), x),
        }
    }
}
impl crate::BoxedDeserialize for Transaction {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x70828481)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x70828481) => Ok(Transaction::Raw_Transaction(
                _de.read_bare::<crate::ton::raw::transaction::Transaction>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `raw.Transactions`\n\n```text\nraw.transactions transactions:vector<raw.transaction> previous_transaction_id:internal.transactionId = raw.Transactions;\n```\n"]
pub enum Transactions {
    Raw_Transactions(crate::ton::raw::transactions::Transactions),
}
impl Transactions {
    pub fn previous_transaction_id(&self) -> &crate::ton::internal::transactionid::TransactionId {
        match self {
            Transactions::Raw_Transactions(ref x) => &x.previous_transaction_id,
        }
    }

    pub fn transactions(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::raw::transaction::Transaction> {
        match self {
            Transactions::Raw_Transactions(ref x) => &x.transactions,
        }
    }

    pub fn only(self) -> crate::ton::raw::transactions::Transactions {
        match self {
            Transactions::Raw_Transactions(x) => x,
        }
    }
}
impl Eq for Transactions {}
impl Default for Transactions {
    fn default() -> Self {
        Transactions::Raw_Transactions(crate::ton::raw::transactions::Transactions::default())
    }
}
impl crate::BoxedSerialize for Transactions {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Transactions::Raw_Transactions(x) => (crate::ConstructorNumber(0x84fae8ed), x),
        }
    }
}
impl crate::BoxedDeserialize for Transactions {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x84fae8ed)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x84fae8ed) => Ok(Transactions::Raw_Transactions(
                _de.read_bare::<crate::ton::raw::transactions::Transactions>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod accountstate;
pub mod appliedshardsinfo;
pub mod fullaccountstate;
pub mod initialaccountstate;
pub mod message;
pub mod shardaccountmeta;
pub mod shardaccountstate;
pub mod transaction;
pub mod transactions;
