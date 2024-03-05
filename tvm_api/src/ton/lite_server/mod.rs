use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.AccountId`\n\n```text\nliteServer.accountId workchain:int id:int256 = liteServer.AccountId;\n```\n"]
pub enum AccountId {
    LiteServer_AccountId(crate::ton::lite_server::accountid::AccountId),
}
impl AccountId {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            AccountId::LiteServer_AccountId(ref x) => &x.id,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            AccountId::LiteServer_AccountId(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::accountid::AccountId {
        match self {
            AccountId::LiteServer_AccountId(x) => x,
        }
    }
}
impl Eq for AccountId {}
impl Default for AccountId {
    fn default() -> Self {
        AccountId::LiteServer_AccountId(crate::ton::lite_server::accountid::AccountId::default())
    }
}
impl crate::BoxedSerialize for AccountId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AccountId::LiteServer_AccountId(x) => (crate::ConstructorNumber(0x75a0e2c5), x),
        }
    }
}
impl crate::BoxedDeserialize for AccountId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x75a0e2c5)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x75a0e2c5) => Ok(AccountId::LiteServer_AccountId(
                _de.read_bare::<crate::ton::lite_server::accountid::AccountId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.AccountState`\n\n```text\nliteServer.accountState id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes proof:bytes state:bytes = liteServer.AccountState;\n```\n"]
pub enum AccountState {
    LiteServer_AccountState(crate::ton::lite_server::accountstate::AccountState),
}
impl AccountState {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            AccountState::LiteServer_AccountState(ref x) => &x.id,
        }
    }

    pub fn proof(&self) -> &crate::ton::bytes {
        match self {
            AccountState::LiteServer_AccountState(ref x) => &x.proof,
        }
    }

    pub fn shard_proof(&self) -> &crate::ton::bytes {
        match self {
            AccountState::LiteServer_AccountState(ref x) => &x.shard_proof,
        }
    }

    pub fn shardblk(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            AccountState::LiteServer_AccountState(ref x) => &x.shardblk,
        }
    }

    pub fn state(&self) -> &crate::ton::bytes {
        match self {
            AccountState::LiteServer_AccountState(ref x) => &x.state,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::accountstate::AccountState {
        match self {
            AccountState::LiteServer_AccountState(x) => x,
        }
    }
}
impl Eq for AccountState {}
impl Default for AccountState {
    fn default() -> Self {
        AccountState::LiteServer_AccountState(
            crate::ton::lite_server::accountstate::AccountState::default(),
        )
    }
}
impl crate::BoxedSerialize for AccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AccountState::LiteServer_AccountState(x) => (crate::ConstructorNumber(0x7079c751), x),
        }
    }
}
impl crate::BoxedDeserialize for AccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7079c751)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7079c751) => Ok(AccountState::LiteServer_AccountState(
                _de.read_bare::<crate::ton::lite_server::accountstate::AccountState>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.AllShardsInfo`\n\n```text\nliteServer.allShardsInfo id:tonNode.blockIdExt proof:bytes data:bytes = liteServer.AllShardsInfo;\n```\n"]
pub enum AllShardsInfo {
    LiteServer_AllShardsInfo(crate::ton::lite_server::allshardsinfo::AllShardsInfo),
}
impl AllShardsInfo {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            AllShardsInfo::LiteServer_AllShardsInfo(ref x) => &x.data,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            AllShardsInfo::LiteServer_AllShardsInfo(ref x) => &x.id,
        }
    }

    pub fn proof(&self) -> &crate::ton::bytes {
        match self {
            AllShardsInfo::LiteServer_AllShardsInfo(ref x) => &x.proof,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::allshardsinfo::AllShardsInfo {
        match self {
            AllShardsInfo::LiteServer_AllShardsInfo(x) => x,
        }
    }
}
impl Eq for AllShardsInfo {}
impl Default for AllShardsInfo {
    fn default() -> Self {
        AllShardsInfo::LiteServer_AllShardsInfo(
            crate::ton::lite_server::allshardsinfo::AllShardsInfo::default(),
        )
    }
}
impl crate::BoxedSerialize for AllShardsInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AllShardsInfo::LiteServer_AllShardsInfo(x) => (crate::ConstructorNumber(0x098fe72d), x),
        }
    }
}
impl crate::BoxedDeserialize for AllShardsInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x098fe72d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x098fe72d) => Ok(AllShardsInfo::LiteServer_AllShardsInfo(
                _de.read_bare::<crate::ton::lite_server::allshardsinfo::AllShardsInfo>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.BlockData`\n\n```text\nliteServer.blockData id:tonNode.blockIdExt data:bytes = liteServer.BlockData;\n```\n"]
pub enum BlockData {
    LiteServer_BlockData(crate::ton::lite_server::blockdata::BlockData),
}
impl BlockData {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            BlockData::LiteServer_BlockData(ref x) => &x.data,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            BlockData::LiteServer_BlockData(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::blockdata::BlockData {
        match self {
            BlockData::LiteServer_BlockData(x) => x,
        }
    }
}
impl Eq for BlockData {}
impl Default for BlockData {
    fn default() -> Self {
        BlockData::LiteServer_BlockData(crate::ton::lite_server::blockdata::BlockData::default())
    }
}
impl crate::BoxedSerialize for BlockData {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockData::LiteServer_BlockData(x) => (crate::ConstructorNumber(0xa574ed6c), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockData {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa574ed6c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa574ed6c) => Ok(BlockData::LiteServer_BlockData(
                _de.read_bare::<crate::ton::lite_server::blockdata::BlockData>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.BlockHeader`\n\n```text\nliteServer.blockHeader id:tonNode.blockIdExt mode:# header_proof:bytes = liteServer.BlockHeader;\n```\n"]
pub enum BlockHeader {
    LiteServer_BlockHeader(crate::ton::lite_server::blockheader::BlockHeader),
}
impl BlockHeader {
    pub fn header_proof(&self) -> &crate::ton::bytes {
        match self {
            BlockHeader::LiteServer_BlockHeader(ref x) => &x.header_proof,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            BlockHeader::LiteServer_BlockHeader(ref x) => &x.id,
        }
    }

    pub fn mode(&self) -> &crate::ton::int {
        match self {
            BlockHeader::LiteServer_BlockHeader(ref x) => &x.mode,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::blockheader::BlockHeader {
        match self {
            BlockHeader::LiteServer_BlockHeader(x) => x,
        }
    }
}
impl Eq for BlockHeader {}
impl Default for BlockHeader {
    fn default() -> Self {
        BlockHeader::LiteServer_BlockHeader(
            crate::ton::lite_server::blockheader::BlockHeader::default(),
        )
    }
}
impl crate::BoxedSerialize for BlockHeader {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockHeader::LiteServer_BlockHeader(x) => (crate::ConstructorNumber(0x752d8219), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockHeader {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x752d8219)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x752d8219) => Ok(BlockHeader::LiteServer_BlockHeader(
                _de.read_bare::<crate::ton::lite_server::blockheader::BlockHeader>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.BlockLink`\n\n```text\nliteServer.blockLinkBack to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes proof:bytes state_proof:bytes = liteServer.BlockLink;\n\nliteServer.blockLinkForward to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes config_proof:bytes signatures:liteServer.SignatureSet = liteServer.BlockLink;\n```\n"]
pub enum BlockLink {
    LiteServer_BlockLinkBack(crate::ton::lite_server::blocklink::BlockLinkBack),
    LiteServer_BlockLinkForward(crate::ton::lite_server::blocklink::BlockLinkForward),
}
impl BlockLink {
    pub fn config_proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            BlockLink::LiteServer_BlockLinkForward(ref x) => Some(&x.config_proof),
            _ => None,
        }
    }

    pub fn dest_proof(&self) -> &crate::ton::bytes {
        match self {
            BlockLink::LiteServer_BlockLinkBack(ref x) => &x.dest_proof,
            BlockLink::LiteServer_BlockLinkForward(ref x) => &x.dest_proof,
        }
    }

    pub fn from(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            BlockLink::LiteServer_BlockLinkBack(ref x) => &x.from,
            BlockLink::LiteServer_BlockLinkForward(ref x) => &x.from,
        }
    }

    pub fn proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            BlockLink::LiteServer_BlockLinkBack(ref x) => Some(&x.proof),
            _ => None,
        }
    }

    pub fn signatures(&self) -> Option<&crate::ton::lite_server::SignatureSet> {
        match self {
            BlockLink::LiteServer_BlockLinkForward(ref x) => Some(&x.signatures),
            _ => None,
        }
    }

    pub fn state_proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            BlockLink::LiteServer_BlockLinkBack(ref x) => Some(&x.state_proof),
            _ => None,
        }
    }

    pub fn to(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            BlockLink::LiteServer_BlockLinkBack(ref x) => &x.to,
            BlockLink::LiteServer_BlockLinkForward(ref x) => &x.to,
        }
    }

    pub fn to_key_block(&self) -> &crate::ton::Bool {
        match self {
            BlockLink::LiteServer_BlockLinkBack(ref x) => &x.to_key_block,
            BlockLink::LiteServer_BlockLinkForward(ref x) => &x.to_key_block,
        }
    }
}
impl Eq for BlockLink {}
impl Default for BlockLink {
    fn default() -> Self {
        BlockLink::LiteServer_BlockLinkBack(
            crate::ton::lite_server::blocklink::BlockLinkBack::default(),
        )
    }
}
impl crate::BoxedSerialize for BlockLink {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockLink::LiteServer_BlockLinkBack(x) => (crate::ConstructorNumber(0xef7e1bef), x),
            BlockLink::LiteServer_BlockLinkForward(x) => (crate::ConstructorNumber(0x520fce1c), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockLink {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xef7e1bef), crate::ConstructorNumber(0x520fce1c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xef7e1bef) => Ok(BlockLink::LiteServer_BlockLinkBack(
                _de.read_bare::<crate::ton::lite_server::blocklink::BlockLinkBack>()?,
            )),
            crate::ConstructorNumber(0x520fce1c) => Ok(BlockLink::LiteServer_BlockLinkForward(
                _de.read_bare::<crate::ton::lite_server::blocklink::BlockLinkForward>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.BlockState`\n\n```text\nliteServer.blockState id:tonNode.blockIdExt root_hash:int256 file_hash:int256 data:bytes = liteServer.BlockState;\n```\n"]
pub enum BlockState {
    LiteServer_BlockState(crate::ton::lite_server::blockstate::BlockState),
}
impl BlockState {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            BlockState::LiteServer_BlockState(ref x) => &x.data,
        }
    }

    pub fn file_hash(&self) -> &crate::ton::int256 {
        match self {
            BlockState::LiteServer_BlockState(ref x) => &x.file_hash,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            BlockState::LiteServer_BlockState(ref x) => &x.id,
        }
    }

    pub fn root_hash(&self) -> &crate::ton::int256 {
        match self {
            BlockState::LiteServer_BlockState(ref x) => &x.root_hash,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::blockstate::BlockState {
        match self {
            BlockState::LiteServer_BlockState(x) => x,
        }
    }
}
impl Eq for BlockState {}
impl Default for BlockState {
    fn default() -> Self {
        BlockState::LiteServer_BlockState(crate::ton::lite_server::blockstate::BlockState::default())
    }
}
impl crate::BoxedSerialize for BlockState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockState::LiteServer_BlockState(x) => (crate::ConstructorNumber(0xabaddc0c), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xabaddc0c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xabaddc0c) => Ok(BlockState::LiteServer_BlockState(
                _de.read_bare::<crate::ton::lite_server::blockstate::BlockState>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.BlockTransactions`\n\n```text\nliteServer.blockTransactions id:tonNode.blockIdExt req_count:# incomplete:Bool ids:(vector liteServer.transactionId) proof:bytes = liteServer.BlockTransactions;\n```\n"]
pub enum BlockTransactions {
    LiteServer_BlockTransactions(crate::ton::lite_server::blocktransactions::BlockTransactions),
}
impl BlockTransactions {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            BlockTransactions::LiteServer_BlockTransactions(ref x) => &x.id,
        }
    }

    pub fn ids(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::lite_server::transactionid::TransactionId>
    {
        match self {
            BlockTransactions::LiteServer_BlockTransactions(ref x) => &x.ids,
        }
    }

    pub fn incomplete(&self) -> &crate::ton::Bool {
        match self {
            BlockTransactions::LiteServer_BlockTransactions(ref x) => &x.incomplete,
        }
    }

    pub fn proof(&self) -> &crate::ton::bytes {
        match self {
            BlockTransactions::LiteServer_BlockTransactions(ref x) => &x.proof,
        }
    }

    pub fn req_count(&self) -> &crate::ton::int {
        match self {
            BlockTransactions::LiteServer_BlockTransactions(ref x) => &x.req_count,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::blocktransactions::BlockTransactions {
        match self {
            BlockTransactions::LiteServer_BlockTransactions(x) => x,
        }
    }
}
impl Eq for BlockTransactions {}
impl Default for BlockTransactions {
    fn default() -> Self {
        BlockTransactions::LiteServer_BlockTransactions(
            crate::ton::lite_server::blocktransactions::BlockTransactions::default(),
        )
    }
}
impl crate::BoxedSerialize for BlockTransactions {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockTransactions::LiteServer_BlockTransactions(x) => {
                (crate::ConstructorNumber(0xbd8cad2b), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for BlockTransactions {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbd8cad2b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbd8cad2b) => {
                Ok(BlockTransactions::LiteServer_BlockTransactions(
                    _de.read_bare::<crate::ton::lite_server::blocktransactions::BlockTransactions>(
                    )?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.ConfigInfo`\n\n```text\nliteServer.configInfo mode:# id:tonNode.blockIdExt state_proof:bytes config_proof:bytes = liteServer.ConfigInfo;\n```\n"]
pub enum ConfigInfo {
    LiteServer_ConfigInfo(crate::ton::lite_server::configinfo::ConfigInfo),
}
impl ConfigInfo {
    pub fn config_proof(&self) -> &crate::ton::bytes {
        match self {
            ConfigInfo::LiteServer_ConfigInfo(ref x) => &x.config_proof,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            ConfigInfo::LiteServer_ConfigInfo(ref x) => &x.id,
        }
    }

    pub fn mode(&self) -> &crate::ton::int {
        match self {
            ConfigInfo::LiteServer_ConfigInfo(ref x) => &x.mode,
        }
    }

    pub fn state_proof(&self) -> &crate::ton::bytes {
        match self {
            ConfigInfo::LiteServer_ConfigInfo(ref x) => &x.state_proof,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::configinfo::ConfigInfo {
        match self {
            ConfigInfo::LiteServer_ConfigInfo(x) => x,
        }
    }
}
impl Eq for ConfigInfo {}
impl Default for ConfigInfo {
    fn default() -> Self {
        ConfigInfo::LiteServer_ConfigInfo(crate::ton::lite_server::configinfo::ConfigInfo::default())
    }
}
impl crate::BoxedSerialize for ConfigInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ConfigInfo::LiteServer_ConfigInfo(x) => (crate::ConstructorNumber(0xae7b272f), x),
        }
    }
}
impl crate::BoxedDeserialize for ConfigInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xae7b272f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xae7b272f) => Ok(ConfigInfo::LiteServer_ConfigInfo(
                _de.read_bare::<crate::ton::lite_server::configinfo::ConfigInfo>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.CurrentTime`\n\n```text\nliteServer.currentTime now:int = liteServer.CurrentTime;\n```\n"]
pub enum CurrentTime {
    LiteServer_CurrentTime(crate::ton::lite_server::currenttime::CurrentTime),
}
impl CurrentTime {
    pub fn now(&self) -> &crate::ton::int {
        match self {
            CurrentTime::LiteServer_CurrentTime(ref x) => &x.now,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::currenttime::CurrentTime {
        match self {
            CurrentTime::LiteServer_CurrentTime(x) => x,
        }
    }
}
impl Eq for CurrentTime {}
impl Default for CurrentTime {
    fn default() -> Self {
        CurrentTime::LiteServer_CurrentTime(
            crate::ton::lite_server::currenttime::CurrentTime::default(),
        )
    }
}
impl crate::BoxedSerialize for CurrentTime {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            CurrentTime::LiteServer_CurrentTime(x) => (crate::ConstructorNumber(0xe953000d), x),
        }
    }
}
impl crate::BoxedDeserialize for CurrentTime {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe953000d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe953000d) => Ok(CurrentTime::LiteServer_CurrentTime(
                _de.read_bare::<crate::ton::lite_server::currenttime::CurrentTime>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.Error`\n\n```text\nliteServer.error code:int message:string = liteServer.Error;\n```\n"]
pub enum Error {
    LiteServer_Error(crate::ton::lite_server::error::Error),
}
impl Error {
    pub fn code(&self) -> &crate::ton::int {
        match self {
            Error::LiteServer_Error(ref x) => &x.code,
        }
    }

    pub fn message(&self) -> &crate::ton::string {
        match self {
            Error::LiteServer_Error(ref x) => &x.message,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::error::Error {
        match self {
            Error::LiteServer_Error(x) => x,
        }
    }
}
impl Eq for Error {}
impl Default for Error {
    fn default() -> Self {
        Error::LiteServer_Error(crate::ton::lite_server::error::Error::default())
    }
}
impl crate::BoxedSerialize for Error {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Error::LiteServer_Error(x) => (crate::ConstructorNumber(0xbba9e148), x),
        }
    }
}
impl crate::BoxedDeserialize for Error {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbba9e148)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbba9e148) => Ok(Error::LiteServer_Error(
                _de.read_bare::<crate::ton::lite_server::error::Error>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.Info`\n\n```text\nliteServer.info now:int53 version:int32 capabilities:int64 = liteServer.Info;\n```\n"]
pub enum Info {
    LiteServer_Info(crate::ton::lite_server::info::Info),
}
impl Info {
    pub fn capabilities(&self) -> &crate::ton::int64 {
        match self {
            Info::LiteServer_Info(ref x) => &x.capabilities,
        }
    }

    pub fn now(&self) -> &crate::ton::int53 {
        match self {
            Info::LiteServer_Info(ref x) => &x.now,
        }
    }

    pub fn version(&self) -> &crate::ton::int32 {
        match self {
            Info::LiteServer_Info(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::info::Info {
        match self {
            Info::LiteServer_Info(x) => x,
        }
    }
}
impl Eq for Info {}
impl Default for Info {
    fn default() -> Self {
        Info::LiteServer_Info(crate::ton::lite_server::info::Info::default())
    }
}
impl crate::BoxedSerialize for Info {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Info::LiteServer_Info(x) => (crate::ConstructorNumber(0xb57bfe73), x),
        }
    }
}
impl crate::BoxedDeserialize for Info {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb57bfe73)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb57bfe73) => {
                Ok(Info::LiteServer_Info(_de.read_bare::<crate::ton::lite_server::info::Info>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.MasterchainInfo`\n\n```text\nliteServer.masterchainInfo last:tonNode.blockIdExt state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfo;\n```\n"]
pub enum MasterchainInfo {
    LiteServer_MasterchainInfo(crate::ton::lite_server::masterchaininfo::MasterchainInfo),
}
impl MasterchainInfo {
    pub fn init(&self) -> &crate::ton::ton_node::zerostateidext::ZeroStateIdExt {
        match self {
            MasterchainInfo::LiteServer_MasterchainInfo(ref x) => &x.init,
        }
    }

    pub fn last(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            MasterchainInfo::LiteServer_MasterchainInfo(ref x) => &x.last,
        }
    }

    pub fn state_root_hash(&self) -> &crate::ton::int256 {
        match self {
            MasterchainInfo::LiteServer_MasterchainInfo(ref x) => &x.state_root_hash,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::masterchaininfo::MasterchainInfo {
        match self {
            MasterchainInfo::LiteServer_MasterchainInfo(x) => x,
        }
    }
}
impl Eq for MasterchainInfo {}
impl Default for MasterchainInfo {
    fn default() -> Self {
        MasterchainInfo::LiteServer_MasterchainInfo(
            crate::ton::lite_server::masterchaininfo::MasterchainInfo::default(),
        )
    }
}
impl crate::BoxedSerialize for MasterchainInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            MasterchainInfo::LiteServer_MasterchainInfo(x) => {
                (crate::ConstructorNumber(0x85832881), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for MasterchainInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x85832881)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x85832881) => {
                Ok(MasterchainInfo::LiteServer_MasterchainInfo(
                    _de.read_bare::<crate::ton::lite_server::masterchaininfo::MasterchainInfo>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.MasterchainInfoExt`\n\n```text\nliteServer.masterchainInfoExt mode:# version:int capabilities:long last:tonNode.blockIdExt last_utime:int now:int state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfoExt;\n```\n"]
pub enum MasterchainInfoExt {
    LiteServer_MasterchainInfoExt(crate::ton::lite_server::masterchaininfoext::MasterchainInfoExt),
}
impl MasterchainInfoExt {
    pub fn capabilities(&self) -> &crate::ton::long {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.capabilities,
        }
    }

    pub fn init(&self) -> &crate::ton::ton_node::zerostateidext::ZeroStateIdExt {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.init,
        }
    }

    pub fn last(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.last,
        }
    }

    pub fn last_utime(&self) -> &crate::ton::int {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.last_utime,
        }
    }

    pub fn mode(&self) -> &crate::ton::int {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.mode,
        }
    }

    pub fn now(&self) -> &crate::ton::int {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.now,
        }
    }

    pub fn state_root_hash(&self) -> &crate::ton::int256 {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.state_root_hash,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::masterchaininfoext::MasterchainInfoExt {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(x) => x,
        }
    }
}
impl Eq for MasterchainInfoExt {}
impl Default for MasterchainInfoExt {
    fn default() -> Self {
        MasterchainInfoExt::LiteServer_MasterchainInfoExt(
            crate::ton::lite_server::masterchaininfoext::MasterchainInfoExt::default(),
        )
    }
}
impl crate::BoxedSerialize for MasterchainInfoExt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            MasterchainInfoExt::LiteServer_MasterchainInfoExt(x) => {
                (crate::ConstructorNumber(0xa8cce0f5), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for MasterchainInfoExt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa8cce0f5)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0xa8cce0f5) => Ok (MasterchainInfoExt :: LiteServer_MasterchainInfoExt (_de . read_bare :: < crate :: ton :: lite_server :: masterchaininfoext :: MasterchainInfoExt > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.PartialBlockProof`\n\n```text\nliteServer.partialBlockProof complete:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt steps:(vector liteServer.BlockLink) = liteServer.PartialBlockProof;\n```\n"]
pub enum PartialBlockProof {
    LiteServer_PartialBlockProof(crate::ton::lite_server::partialblockproof::PartialBlockProof),
}
impl PartialBlockProof {
    pub fn complete(&self) -> &crate::ton::Bool {
        match self {
            PartialBlockProof::LiteServer_PartialBlockProof(ref x) => &x.complete,
        }
    }

    pub fn from(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            PartialBlockProof::LiteServer_PartialBlockProof(ref x) => &x.from,
        }
    }

    pub fn steps(
        &self,
    ) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::lite_server::BlockLink> {
        match self {
            PartialBlockProof::LiteServer_PartialBlockProof(ref x) => &x.steps,
        }
    }

    pub fn to(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            PartialBlockProof::LiteServer_PartialBlockProof(ref x) => &x.to,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::partialblockproof::PartialBlockProof {
        match self {
            PartialBlockProof::LiteServer_PartialBlockProof(x) => x,
        }
    }
}
impl Eq for PartialBlockProof {}
impl Default for PartialBlockProof {
    fn default() -> Self {
        PartialBlockProof::LiteServer_PartialBlockProof(
            crate::ton::lite_server::partialblockproof::PartialBlockProof::default(),
        )
    }
}
impl crate::BoxedSerialize for PartialBlockProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PartialBlockProof::LiteServer_PartialBlockProof(x) => {
                (crate::ConstructorNumber(0x8ed0d2c1), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for PartialBlockProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8ed0d2c1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8ed0d2c1) => {
                Ok(PartialBlockProof::LiteServer_PartialBlockProof(
                    _de.read_bare::<crate::ton::lite_server::partialblockproof::PartialBlockProof>(
                    )?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.RunMethodResult`\n\n```text\nliteServer.runMethodResult mode:# id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:mode.0?bytes proof:mode.0?bytes state_proof:mode.1?bytes init_c7:mode.3?bytes lib_extras:mode.4?bytes exit_code:int result:mode.2?bytes = liteServer.RunMethodResult;\n```\n"]
pub enum RunMethodResult {
    LiteServer_RunMethodResult(crate::ton::lite_server::runmethodresult::RunMethodResult),
}
impl RunMethodResult {
    pub fn exit_code(&self) -> &crate::ton::int {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => &x.exit_code,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => &x.id,
        }
    }

    pub fn init_c7(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => x.init_c7.as_ref(),
        }
    }

    pub fn lib_extras(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => x.lib_extras.as_ref(),
        }
    }

    pub fn mode(&self) -> &crate::ton::int {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => &x.mode,
        }
    }

    pub fn proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => x.proof.as_ref(),
        }
    }

    pub fn result(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => x.result.as_ref(),
        }
    }

    pub fn shard_proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => x.shard_proof.as_ref(),
        }
    }

    pub fn shardblk(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => &x.shardblk,
        }
    }

    pub fn state_proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(ref x) => x.state_proof.as_ref(),
        }
    }

    pub fn only(self) -> crate::ton::lite_server::runmethodresult::RunMethodResult {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(x) => x,
        }
    }
}
impl Eq for RunMethodResult {}
impl Default for RunMethodResult {
    fn default() -> Self {
        RunMethodResult::LiteServer_RunMethodResult(
            crate::ton::lite_server::runmethodresult::RunMethodResult::default(),
        )
    }
}
impl crate::BoxedSerialize for RunMethodResult {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RunMethodResult::LiteServer_RunMethodResult(x) => {
                (crate::ConstructorNumber(0xa39a616b), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RunMethodResult {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa39a616b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa39a616b) => {
                Ok(RunMethodResult::LiteServer_RunMethodResult(
                    _de.read_bare::<crate::ton::lite_server::runmethodresult::RunMethodResult>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.SendMsgStatus`\n\n```text\nliteServer.sendMsgStatus status:int = liteServer.SendMsgStatus;\n```\n"]
pub enum SendMsgStatus {
    LiteServer_SendMsgStatus(crate::ton::lite_server::sendmsgstatus::SendMsgStatus),
}
impl SendMsgStatus {
    pub fn status(&self) -> &crate::ton::int {
        match self {
            SendMsgStatus::LiteServer_SendMsgStatus(ref x) => &x.status,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::sendmsgstatus::SendMsgStatus {
        match self {
            SendMsgStatus::LiteServer_SendMsgStatus(x) => x,
        }
    }
}
impl Eq for SendMsgStatus {}
impl Default for SendMsgStatus {
    fn default() -> Self {
        SendMsgStatus::LiteServer_SendMsgStatus(
            crate::ton::lite_server::sendmsgstatus::SendMsgStatus::default(),
        )
    }
}
impl crate::BoxedSerialize for SendMsgStatus {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            SendMsgStatus::LiteServer_SendMsgStatus(x) => (crate::ConstructorNumber(0x3950e597), x),
        }
    }
}
impl crate::BoxedDeserialize for SendMsgStatus {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3950e597)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3950e597) => Ok(SendMsgStatus::LiteServer_SendMsgStatus(
                _de.read_bare::<crate::ton::lite_server::sendmsgstatus::SendMsgStatus>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.ShardInfo`\n\n```text\nliteServer.shardInfo id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes shard_descr:bytes = liteServer.ShardInfo;\n```\n"]
pub enum ShardInfo {
    LiteServer_ShardInfo(crate::ton::lite_server::shardinfo::ShardInfo),
}
impl ShardInfo {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            ShardInfo::LiteServer_ShardInfo(ref x) => &x.id,
        }
    }

    pub fn shard_descr(&self) -> &crate::ton::bytes {
        match self {
            ShardInfo::LiteServer_ShardInfo(ref x) => &x.shard_descr,
        }
    }

    pub fn shard_proof(&self) -> &crate::ton::bytes {
        match self {
            ShardInfo::LiteServer_ShardInfo(ref x) => &x.shard_proof,
        }
    }

    pub fn shardblk(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            ShardInfo::LiteServer_ShardInfo(ref x) => &x.shardblk,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::shardinfo::ShardInfo {
        match self {
            ShardInfo::LiteServer_ShardInfo(x) => x,
        }
    }
}
impl Eq for ShardInfo {}
impl Default for ShardInfo {
    fn default() -> Self {
        ShardInfo::LiteServer_ShardInfo(crate::ton::lite_server::shardinfo::ShardInfo::default())
    }
}
impl crate::BoxedSerialize for ShardInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ShardInfo::LiteServer_ShardInfo(x) => (crate::ConstructorNumber(0x9fe6cd84), x),
        }
    }
}
impl crate::BoxedDeserialize for ShardInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x9fe6cd84)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x9fe6cd84) => Ok(ShardInfo::LiteServer_ShardInfo(
                _de.read_bare::<crate::ton::lite_server::shardinfo::ShardInfo>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.Signature`\n\n```text\nliteServer.signature node_id_short:int256 signature:bytes = liteServer.Signature;\n```\n"]
pub enum Signature {
    LiteServer_Signature(crate::ton::lite_server::signature::Signature),
}
impl Signature {
    pub fn node_id_short(&self) -> &crate::ton::int256 {
        match self {
            Signature::LiteServer_Signature(ref x) => &x.node_id_short,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Signature::LiteServer_Signature(ref x) => &x.signature,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::signature::Signature {
        match self {
            Signature::LiteServer_Signature(x) => x,
        }
    }
}
impl Eq for Signature {}
impl Default for Signature {
    fn default() -> Self {
        Signature::LiteServer_Signature(crate::ton::lite_server::signature::Signature::default())
    }
}
impl crate::BoxedSerialize for Signature {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Signature::LiteServer_Signature(x) => (crate::ConstructorNumber(0xa3def855), x),
        }
    }
}
impl crate::BoxedDeserialize for Signature {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa3def855)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa3def855) => Ok(Signature::LiteServer_Signature(
                _de.read_bare::<crate::ton::lite_server::signature::Signature>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.SignatureSet`\n\n```text\nliteServer.signatureSet validator_set_hash:int catchain_seqno:int signatures:(vector liteServer.signature) = liteServer.SignatureSet;\n```\n"]
pub enum SignatureSet {
    LiteServer_SignatureSet(crate::ton::lite_server::signatureset::SignatureSet),
}
impl SignatureSet {
    pub fn catchain_seqno(&self) -> &crate::ton::int {
        match self {
            SignatureSet::LiteServer_SignatureSet(ref x) => &x.catchain_seqno,
        }
    }

    pub fn signatures(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::lite_server::signature::Signature> {
        match self {
            SignatureSet::LiteServer_SignatureSet(ref x) => &x.signatures,
        }
    }

    pub fn validator_set_hash(&self) -> &crate::ton::int {
        match self {
            SignatureSet::LiteServer_SignatureSet(ref x) => &x.validator_set_hash,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::signatureset::SignatureSet {
        match self {
            SignatureSet::LiteServer_SignatureSet(x) => x,
        }
    }
}
impl Eq for SignatureSet {}
impl Default for SignatureSet {
    fn default() -> Self {
        SignatureSet::LiteServer_SignatureSet(
            crate::ton::lite_server::signatureset::SignatureSet::default(),
        )
    }
}
impl crate::BoxedSerialize for SignatureSet {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            SignatureSet::LiteServer_SignatureSet(x) => (crate::ConstructorNumber(0xf644a6e6), x),
        }
    }
}
impl crate::BoxedDeserialize for SignatureSet {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf644a6e6)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf644a6e6) => Ok(SignatureSet::LiteServer_SignatureSet(
                _de.read_bare::<crate::ton::lite_server::signatureset::SignatureSet>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.TransactionId`\n\n```text\nliteServer.transactionId mode:# account:mode.0?int256 lt:mode.1?long hash:mode.2?int256 = liteServer.TransactionId;\n```\n"]
pub enum TransactionId {
    LiteServer_TransactionId(crate::ton::lite_server::transactionid::TransactionId),
}
impl TransactionId {
    pub fn account(&self) -> Option<&crate::ton::int256> {
        match self {
            TransactionId::LiteServer_TransactionId(ref x) => x.account.as_ref(),
        }
    }

    pub fn hash(&self) -> Option<&crate::ton::int256> {
        match self {
            TransactionId::LiteServer_TransactionId(ref x) => x.hash.as_ref(),
        }
    }

    pub fn lt(&self) -> Option<&crate::ton::long> {
        match self {
            TransactionId::LiteServer_TransactionId(ref x) => x.lt.as_ref(),
        }
    }

    pub fn mode(&self) -> &crate::ton::int {
        match self {
            TransactionId::LiteServer_TransactionId(ref x) => &x.mode,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::transactionid::TransactionId {
        match self {
            TransactionId::LiteServer_TransactionId(x) => x,
        }
    }
}
impl Eq for TransactionId {}
impl Default for TransactionId {
    fn default() -> Self {
        TransactionId::LiteServer_TransactionId(
            crate::ton::lite_server::transactionid::TransactionId::default(),
        )
    }
}
impl crate::BoxedSerialize for TransactionId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            TransactionId::LiteServer_TransactionId(x) => (crate::ConstructorNumber(0xb12f65af), x),
        }
    }
}
impl crate::BoxedDeserialize for TransactionId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb12f65af)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb12f65af) => Ok(TransactionId::LiteServer_TransactionId(
                _de.read_bare::<crate::ton::lite_server::transactionid::TransactionId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.TransactionId3`\n\n```text\nliteServer.transactionId3 account:int256 lt:long = liteServer.TransactionId3;\n```\n"]
pub enum TransactionId3 {
    LiteServer_TransactionId3(crate::ton::lite_server::transactionid3::TransactionId3),
}
impl TransactionId3 {
    pub fn account(&self) -> &crate::ton::int256 {
        match self {
            TransactionId3::LiteServer_TransactionId3(ref x) => &x.account,
        }
    }

    pub fn lt(&self) -> &crate::ton::long {
        match self {
            TransactionId3::LiteServer_TransactionId3(ref x) => &x.lt,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::transactionid3::TransactionId3 {
        match self {
            TransactionId3::LiteServer_TransactionId3(x) => x,
        }
    }
}
impl Eq for TransactionId3 {}
impl Default for TransactionId3 {
    fn default() -> Self {
        TransactionId3::LiteServer_TransactionId3(
            crate::ton::lite_server::transactionid3::TransactionId3::default(),
        )
    }
}
impl crate::BoxedSerialize for TransactionId3 {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            TransactionId3::LiteServer_TransactionId3(x) => {
                (crate::ConstructorNumber(0x2c81da77), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for TransactionId3 {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2c81da77)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2c81da77) => Ok(TransactionId3::LiteServer_TransactionId3(
                _de.read_bare::<crate::ton::lite_server::transactionid3::TransactionId3>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.TransactionInfo`\n\n```text\nliteServer.transactionInfo id:tonNode.blockIdExt proof:bytes transaction:bytes = liteServer.TransactionInfo;\n```\n"]
pub enum TransactionInfo {
    LiteServer_TransactionInfo(crate::ton::lite_server::transactioninfo::TransactionInfo),
}
impl TransactionInfo {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            TransactionInfo::LiteServer_TransactionInfo(ref x) => &x.id,
        }
    }

    pub fn proof(&self) -> &crate::ton::bytes {
        match self {
            TransactionInfo::LiteServer_TransactionInfo(ref x) => &x.proof,
        }
    }

    pub fn transaction(&self) -> &crate::ton::bytes {
        match self {
            TransactionInfo::LiteServer_TransactionInfo(ref x) => &x.transaction,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::transactioninfo::TransactionInfo {
        match self {
            TransactionInfo::LiteServer_TransactionInfo(x) => x,
        }
    }
}
impl Eq for TransactionInfo {}
impl Default for TransactionInfo {
    fn default() -> Self {
        TransactionInfo::LiteServer_TransactionInfo(
            crate::ton::lite_server::transactioninfo::TransactionInfo::default(),
        )
    }
}
impl crate::BoxedSerialize for TransactionInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            TransactionInfo::LiteServer_TransactionInfo(x) => {
                (crate::ConstructorNumber(0x0edeed47), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for TransactionInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0edeed47)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0edeed47) => {
                Ok(TransactionInfo::LiteServer_TransactionInfo(
                    _de.read_bare::<crate::ton::lite_server::transactioninfo::TransactionInfo>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.TransactionList`\n\n```text\nliteServer.transactionList ids:(vector tonNode.blockIdExt) transactions:bytes = liteServer.TransactionList;\n```\n"]
pub enum TransactionList {
    LiteServer_TransactionList(crate::ton::lite_server::transactionlist::TransactionList),
}
impl TransactionList {
    pub fn ids(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            TransactionList::LiteServer_TransactionList(ref x) => &x.ids,
        }
    }

    pub fn transactions(&self) -> &crate::ton::bytes {
        match self {
            TransactionList::LiteServer_TransactionList(ref x) => &x.transactions,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::transactionlist::TransactionList {
        match self {
            TransactionList::LiteServer_TransactionList(x) => x,
        }
    }
}
impl Eq for TransactionList {}
impl Default for TransactionList {
    fn default() -> Self {
        TransactionList::LiteServer_TransactionList(
            crate::ton::lite_server::transactionlist::TransactionList::default(),
        )
    }
}
impl crate::BoxedSerialize for TransactionList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            TransactionList::LiteServer_TransactionList(x) => {
                (crate::ConstructorNumber(0x6f26c60b), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for TransactionList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6f26c60b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x6f26c60b) => {
                Ok(TransactionList::LiteServer_TransactionList(
                    _de.read_bare::<crate::ton::lite_server::transactionlist::TransactionList>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.ValidatorStats`\n\n```text\nliteServer.validatorStats mode:# id:tonNode.blockIdExt count:int complete:Bool state_proof:bytes data_proof:bytes = liteServer.ValidatorStats;\n```\n"]
pub enum ValidatorStats {
    LiteServer_ValidatorStats(crate::ton::lite_server::validatorstats::ValidatorStats),
}
impl ValidatorStats {
    pub fn complete(&self) -> &crate::ton::Bool {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(ref x) => &x.complete,
        }
    }

    pub fn count(&self) -> &crate::ton::int {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(ref x) => &x.count,
        }
    }

    pub fn data_proof(&self) -> &crate::ton::bytes {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(ref x) => &x.data_proof,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(ref x) => &x.id,
        }
    }

    pub fn mode(&self) -> &crate::ton::int {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(ref x) => &x.mode,
        }
    }

    pub fn state_proof(&self) -> &crate::ton::bytes {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(ref x) => &x.state_proof,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::validatorstats::ValidatorStats {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(x) => x,
        }
    }
}
impl Eq for ValidatorStats {}
impl Default for ValidatorStats {
    fn default() -> Self {
        ValidatorStats::LiteServer_ValidatorStats(
            crate::ton::lite_server::validatorstats::ValidatorStats::default(),
        )
    }
}
impl crate::BoxedSerialize for ValidatorStats {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ValidatorStats::LiteServer_ValidatorStats(x) => {
                (crate::ConstructorNumber(0xb9f796d8), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ValidatorStats {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb9f796d8)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb9f796d8) => Ok(ValidatorStats::LiteServer_ValidatorStats(
                _de.read_bare::<crate::ton::lite_server::validatorstats::ValidatorStats>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.Version`\n\n```text\nliteServer.version mode:# version:int capabilities:long now:int = liteServer.Version;\n```\n"]
pub enum Version {
    LiteServer_Version(crate::ton::lite_server::version::Version),
}
impl Version {
    pub fn capabilities(&self) -> &crate::ton::long {
        match self {
            Version::LiteServer_Version(ref x) => &x.capabilities,
        }
    }

    pub fn mode(&self) -> &crate::ton::int {
        match self {
            Version::LiteServer_Version(ref x) => &x.mode,
        }
    }

    pub fn now(&self) -> &crate::ton::int {
        match self {
            Version::LiteServer_Version(ref x) => &x.now,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            Version::LiteServer_Version(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::version::Version {
        match self {
            Version::LiteServer_Version(x) => x,
        }
    }
}
impl Eq for Version {}
impl Default for Version {
    fn default() -> Self {
        Version::LiteServer_Version(crate::ton::lite_server::version::Version::default())
    }
}
impl crate::BoxedSerialize for Version {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Version::LiteServer_Version(x) => (crate::ConstructorNumber(0x5a0491e5), x),
        }
    }
}
impl crate::BoxedDeserialize for Version {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5a0491e5)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5a0491e5) => Ok(Version::LiteServer_Version(
                _de.read_bare::<crate::ton::lite_server::version::Version>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod accountid;
pub mod accountstate;
pub mod allshardsinfo;
pub mod blockdata;
pub mod blockheader;
pub mod blocklink;
pub mod blockstate;
pub mod blocktransactions;
pub mod configinfo;
pub mod currenttime;
pub mod debug;
pub mod error;
pub mod info;
pub mod masterchaininfo;
pub mod masterchaininfoext;
pub mod partialblockproof;
pub mod runmethodresult;
pub mod sendmsgstatus;
pub mod shardinfo;
pub mod signature;
pub mod signatureset;
pub mod transactionid;
pub mod transactionid3;
pub mod transactioninfo;
pub mod transactionlist;
pub mod validatorstats;
pub mod version;
