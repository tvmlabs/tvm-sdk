use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getAccountState`\n\n```text\nliteServer.getAccountState id:tonNode.blockIdExt account:liteServer.accountId = liteServer.AccountState;\n```\n"]
pub struct GetAccountState {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub account: crate::ton::lite_server::accountid::AccountId,
}
impl Eq for GetAccountState {}
impl crate::BareSerialize for GetAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6b890e25)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetAccountState { id, account } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::lite_server::accountid::AccountId>(account)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let account = _de.read_bare::<crate::ton::lite_server::accountid::AccountId>()?;
            Ok(Self { id, account })
        }
    }
}
impl crate::BoxedDeserialize for GetAccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6b890e25)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6b890e25) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6b890e25), self)
    }
}
impl crate::Function for GetAccountState {
    type Reply = crate::ton::lite_server::AccountState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getAllShardsInfo`\n\n```text\nliteServer.getAllShardsInfo id:tonNode.blockIdExt = liteServer.AllShardsInfo;\n```\n"]
pub struct GetAllShardsInfo {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for GetAllShardsInfo {}
impl crate::BareSerialize for GetAllShardsInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x74d3fd6b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetAllShardsInfo { id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetAllShardsInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for GetAllShardsInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x74d3fd6b)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x74d3fd6b) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAllShardsInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x74d3fd6b), self)
    }
}
impl crate::Function for GetAllShardsInfo {
    type Reply = crate::ton::lite_server::AllShardsInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getBlock`\n\n```text\nliteServer.getBlock id:tonNode.blockIdExt = liteServer.BlockData;\n```\n"]
pub struct GetBlock {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for GetBlock {}
impl crate::BareSerialize for GetBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6377cf0d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBlock { id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for GetBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6377cf0d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6377cf0d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6377cf0d), self)
    }
}
impl crate::Function for GetBlock {
    type Reply = crate::ton::lite_server::BlockData;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getBlockHeader`\n\n```text\nliteServer.getBlockHeader id:tonNode.blockIdExt mode:# = liteServer.BlockHeader;\n```\n"]
pub struct GetBlockHeader {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub mode: crate::ton::int,
}
impl Eq for GetBlockHeader {}
impl crate::BareSerialize for GetBlockHeader {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x21ec069e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBlockHeader { id, mode } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(mode)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBlockHeader {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let mode = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, mode })
        }
    }
}
impl crate::BoxedDeserialize for GetBlockHeader {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x21ec069e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x21ec069e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBlockHeader {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x21ec069e), self)
    }
}
impl crate::Function for GetBlockHeader {
    type Reply = crate::ton::lite_server::BlockHeader;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getBlockProof`\n\n```text\nliteServer.getBlockProof mode:# known_block:tonNode.blockIdExt target_block:mode.0?tonNode.blockIdExt = liteServer.PartialBlockProof;\n```\n"]
pub struct GetBlockProof {
    pub mode: crate::ton::int,
    pub known_block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub target_block: Option<crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for GetBlockProof {}
impl crate::BareSerialize for GetBlockProof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8aea9c44)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBlockProof { mode, known_block, target_block } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(known_block)?;
        if let Some(inner) = target_block {
            _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for GetBlockProof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let known_block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let target_block = if mode & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?)
            } else {
                None
            };
            Ok(Self { mode, known_block, target_block })
        }
    }
}
impl crate::BoxedDeserialize for GetBlockProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8aea9c44)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8aea9c44) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBlockProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8aea9c44), self)
    }
}
impl crate::Function for GetBlockProof {
    type Reply = crate::ton::lite_server::PartialBlockProof;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getConfigAll`\n\n```text\nliteServer.getConfigAll mode:# id:tonNode.blockIdExt = liteServer.ConfigInfo;\n```\n"]
pub struct GetConfigAll {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for GetConfigAll {}
impl crate::BareSerialize for GetConfigAll {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x911b26b7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetConfigAll { mode, id } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetConfigAll {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { mode, id })
        }
    }
}
impl crate::BoxedDeserialize for GetConfigAll {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x911b26b7)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x911b26b7) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetConfigAll {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x911b26b7), self)
    }
}
impl crate::Function for GetConfigAll {
    type Reply = crate::ton::lite_server::ConfigInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getConfigParams`\n\n```text\nliteServer.getConfigParams mode:# id:tonNode.blockIdExt param_list:(vector int) = liteServer.ConfigInfo;\n```\n"]
pub struct GetConfigParams {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub param_list: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for GetConfigParams {}
impl crate::BareSerialize for GetConfigParams {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2a111c19)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetConfigParams { mode, id, param_list } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(param_list)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetConfigParams {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let param_list =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { mode, id, param_list })
        }
    }
}
impl crate::BoxedDeserialize for GetConfigParams {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2a111c19)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x2a111c19) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetConfigParams {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x2a111c19), self)
    }
}
impl crate::Function for GetConfigParams {
    type Reply = crate::ton::lite_server::ConfigInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getInfo`\n\n```text\nliteServer.getInfo = liteServer.Info;\n```\n"]
pub struct GetInfo;
impl Eq for GetInfo {}
impl crate::BareSerialize for GetInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x558d5bee)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x558d5bee)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x558d5bee) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x558d5bee), self)
    }
}
impl crate::Function for GetInfo {
    type Reply = crate::ton::lite_server::Info;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getMasterchainInfo`\n\n```text\nliteServer.getMasterchainInfo = liteServer.MasterchainInfo;\n```\n"]
pub struct GetMasterchainInfo;
impl Eq for GetMasterchainInfo {}
impl crate::BareSerialize for GetMasterchainInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x89b5e62e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetMasterchainInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetMasterchainInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x89b5e62e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x89b5e62e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetMasterchainInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x89b5e62e), self)
    }
}
impl crate::Function for GetMasterchainInfo {
    type Reply = crate::ton::lite_server::MasterchainInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getMasterchainInfoExt`\n\n```text\nliteServer.getMasterchainInfoExt mode:# = liteServer.MasterchainInfoExt;\n```\n"]
pub struct GetMasterchainInfoExt {
    pub mode: crate::ton::int,
}
impl Eq for GetMasterchainInfoExt {}
impl crate::BareSerialize for GetMasterchainInfoExt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x70a671df)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetMasterchainInfoExt { mode } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetMasterchainInfoExt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { mode })
        }
    }
}
impl crate::BoxedDeserialize for GetMasterchainInfoExt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x70a671df)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x70a671df) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetMasterchainInfoExt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x70a671df), self)
    }
}
impl crate::Function for GetMasterchainInfoExt {
    type Reply = crate::ton::lite_server::MasterchainInfoExt;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getOneTransaction`\n\n```text\nliteServer.getOneTransaction id:tonNode.blockIdExt account:liteServer.accountId lt:long = liteServer.TransactionInfo;\n```\n"]
pub struct GetOneTransaction {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub account: crate::ton::lite_server::accountid::AccountId,
    pub lt: crate::ton::long,
}
impl Eq for GetOneTransaction {}
impl crate::BareSerialize for GetOneTransaction {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd40f24ea)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetOneTransaction { id, account, lt } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::lite_server::accountid::AccountId>(account)?;
        _ser.write_bare::<crate::ton::long>(lt)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetOneTransaction {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let account = _de.read_bare::<crate::ton::lite_server::accountid::AccountId>()?;
            let lt = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { id, account, lt })
        }
    }
}
impl crate::BoxedDeserialize for GetOneTransaction {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd40f24ea)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd40f24ea) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetOneTransaction {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd40f24ea), self)
    }
}
impl crate::Function for GetOneTransaction {
    type Reply = crate::ton::lite_server::TransactionInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getShardInfo`\n\n```text\nliteServer.getShardInfo id:tonNode.blockIdExt workchain:int shard:long exact:Bool = liteServer.ShardInfo;\n```\n"]
pub struct GetShardInfo {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub exact: crate::ton::Bool,
}
impl Eq for GetShardInfo {}
impl crate::BareSerialize for GetShardInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x46a2f425)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetShardInfo { id, workchain, shard, exact } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_boxed::<crate::ton::Bool>(exact)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetShardInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let exact = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { id, workchain, shard, exact })
        }
    }
}
impl crate::BoxedDeserialize for GetShardInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x46a2f425)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x46a2f425) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetShardInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x46a2f425), self)
    }
}
impl crate::Function for GetShardInfo {
    type Reply = crate::ton::lite_server::ShardInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getState`\n\n```text\nliteServer.getState id:tonNode.blockIdExt = liteServer.BlockState;\n```\n"]
pub struct GetState {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for GetState {}
impl crate::BareSerialize for GetState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xba6e2eb6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetState { id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for GetState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xba6e2eb6)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xba6e2eb6) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xba6e2eb6), self)
    }
}
impl crate::Function for GetState {
    type Reply = crate::ton::lite_server::BlockState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getTime`\n\n```text\nliteServer.getTime = liteServer.CurrentTime;\n```\n"]
pub struct GetTime;
impl Eq for GetTime {}
impl crate::BareSerialize for GetTime {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x16ad5a34)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetTime {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetTime {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x16ad5a34)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x16ad5a34) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetTime {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x16ad5a34), self)
    }
}
impl crate::Function for GetTime {
    type Reply = crate::ton::lite_server::CurrentTime;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getTransactions`\n\n```text\nliteServer.getTransactions count:# account:liteServer.accountId lt:long hash:int256 = liteServer.TransactionList;\n```\n"]
pub struct GetTransactions {
    pub count: crate::ton::int,
    pub account: crate::ton::lite_server::accountid::AccountId,
    pub lt: crate::ton::long,
    pub hash: crate::ton::int256,
}
impl Eq for GetTransactions {}
impl crate::BareSerialize for GetTransactions {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1c40e7a1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetTransactions { count, account, lt, hash } = self;
        _ser.write_bare::<crate::ton::int>(count)?;
        _ser.write_bare::<crate::ton::lite_server::accountid::AccountId>(account)?;
        _ser.write_bare::<crate::ton::long>(lt)?;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetTransactions {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let count = _de.read_bare::<crate::ton::int>()?;
            let account = _de.read_bare::<crate::ton::lite_server::accountid::AccountId>()?;
            let lt = _de.read_bare::<crate::ton::long>()?;
            let hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { count, account, lt, hash })
        }
    }
}
impl crate::BoxedDeserialize for GetTransactions {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1c40e7a1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x1c40e7a1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetTransactions {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x1c40e7a1), self)
    }
}
impl crate::Function for GetTransactions {
    type Reply = crate::ton::lite_server::TransactionList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getValidatorStats`\n\n```text\nliteServer.getValidatorStats#091a58bc mode:# id:tonNode.blockIdExt limit:int start_after:mode.0?int256 modified_after:mode.2?int = liteServer.ValidatorStats;\n```\n"]
pub struct GetValidatorStats {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub limit: crate::ton::int,
    pub start_after: Option<crate::ton::int256>,
    pub modified_after: Option<crate::ton::int>,
}
impl Eq for GetValidatorStats {}
impl crate::BareSerialize for GetValidatorStats {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x091a58bc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetValidatorStats { mode, id, limit, start_after, modified_after } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(limit)?;
        if let Some(inner) = start_after {
            _ser.write_bare::<crate::ton::int256>(inner)?;
        }
        if let Some(inner) = modified_after {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for GetValidatorStats {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let limit = _de.read_bare::<crate::ton::int>()?;
            let start_after = if mode & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::int256>()?)
            } else {
                None
            };
            let modified_after = if mode & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            Ok(Self { mode, id, limit, start_after, modified_after })
        }
    }
}
impl crate::BoxedDeserialize for GetValidatorStats {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x091a58bc)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x091a58bc) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetValidatorStats {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x091a58bc), self)
    }
}
impl crate::Function for GetValidatorStats {
    type Reply = crate::ton::lite_server::ValidatorStats;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.getVersion`\n\n```text\nliteServer.getVersion = liteServer.Version;\n```\n"]
pub struct GetVersion;
impl Eq for GetVersion {}
impl crate::BareSerialize for GetVersion {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x232b940b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetVersion {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetVersion {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x232b940b)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x232b940b) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetVersion {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x232b940b), self)
    }
}
impl crate::Function for GetVersion {
    type Reply = crate::ton::lite_server::Version;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.listBlockTransactions`\n\n```text\nliteServer.listBlockTransactions id:tonNode.blockIdExt mode:# count:# after:mode.7?liteServer.transactionId3 reverse_order:mode.6?true want_proof:mode.5?true = liteServer.BlockTransactions;\n```\n"]
pub struct ListBlockTransactions {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub mode: crate::ton::int,
    pub count: crate::ton::int,
    pub after: Option<crate::ton::lite_server::transactionid3::TransactionId3>,
    pub reverse_order: bool,
    pub want_proof: bool,
}
impl Eq for ListBlockTransactions {}
impl crate::BareSerialize for ListBlockTransactions {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xadfcc7da)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ListBlockTransactions { id, mode, count, after, reverse_order, want_proof } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::int>(count)?;
        if let Some(inner) = after {
            _ser.write_bare::<crate::ton::lite_server::transactionid3::TransactionId3>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for ListBlockTransactions {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let mode = _de.read_bare::<crate::ton::int>()?;
            let count = _de.read_bare::<crate::ton::int>()?;
            let after = if mode & (1 << 7u32) != 0 {
                Some(_de.read_bare::<crate::ton::lite_server::transactionid3::TransactionId3>()?)
            } else {
                None
            };
            let reverse_order = mode & (1 << 6u32) != 0;
            let want_proof = mode & (1 << 5u32) != 0;
            Ok(Self { id, mode, count, after, reverse_order, want_proof })
        }
    }
}
impl crate::BoxedDeserialize for ListBlockTransactions {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xadfcc7da)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xadfcc7da) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ListBlockTransactions {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xadfcc7da), self)
    }
}
impl crate::Function for ListBlockTransactions {
    type Reply = crate::ton::lite_server::BlockTransactions;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.lookupBlock`\n\n```text\nliteServer.lookupBlock mode:# id:tonNode.blockId lt:mode.1?long utime:mode.2?int = liteServer.BlockHeader;\n```\n"]
pub struct LookupBlock {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockid::BlockId,
    pub lt: Option<crate::ton::long>,
    pub utime: Option<crate::ton::int>,
}
impl Eq for LookupBlock {}
impl crate::BareSerialize for LookupBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfac8f71e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let LookupBlock { mode, id, lt, utime } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockid::BlockId>(id)?;
        if let Some(inner) = lt {
            _ser.write_bare::<crate::ton::long>(inner)?;
        }
        if let Some(inner) = utime {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for LookupBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockid::BlockId>()?;
            let lt = if mode & (1 << 1u32) != 0 {
                Some(_de.read_bare::<crate::ton::long>()?)
            } else {
                None
            };
            let utime = if mode & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            Ok(Self { mode, id, lt, utime })
        }
    }
}
impl crate::BoxedDeserialize for LookupBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfac8f71e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xfac8f71e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for LookupBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xfac8f71e), self)
    }
}
impl crate::Function for LookupBlock {
    type Reply = crate::ton::lite_server::BlockHeader;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.query`\n\n```text\nliteServer.query data:bytes = Object;\n```\n"]
pub struct Query {
    pub data: crate::ton::bytes,
}
impl Eq for Query {}
impl crate::BareSerialize for Query {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x798c06df)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Query { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Query {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::BoxedDeserialize for Query {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x798c06df)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x798c06df) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Query {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x798c06df), self)
    }
}
impl crate::Function for Query {
    type Reply = crate::ton::Object;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.queryPrefix`\n\n```text\nliteServer.queryPrefix = Object;\n```\n"]
pub struct QueryPrefix;
impl Eq for QueryPrefix {}
impl crate::BareSerialize for QueryPrefix {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x72d3e686)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for QueryPrefix {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for QueryPrefix {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x72d3e686)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x72d3e686) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for QueryPrefix {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x72d3e686), self)
    }
}
impl crate::Function for QueryPrefix {
    type Reply = crate::ton::Object;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.runSmcMethod`\n\n```text\nliteServer.runSmcMethod mode:# id:tonNode.blockIdExt account:liteServer.accountId method_id:long params:bytes = liteServer.RunMethodResult;\n```\n"]
pub struct RunSmcMethod {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub account: crate::ton::lite_server::accountid::AccountId,
    pub method_id: crate::ton::long,
    pub params: crate::ton::bytes,
}
impl Eq for RunSmcMethod {}
impl crate::BareSerialize for RunSmcMethod {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5cc65dd2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunSmcMethod { mode, id, account, method_id, params } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::lite_server::accountid::AccountId>(account)?;
        _ser.write_bare::<crate::ton::long>(method_id)?;
        _ser.write_bare::<crate::ton::bytes>(params)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunSmcMethod {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let account = _de.read_bare::<crate::ton::lite_server::accountid::AccountId>()?;
            let method_id = _de.read_bare::<crate::ton::long>()?;
            let params = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { mode, id, account, method_id, params })
        }
    }
}
impl crate::BoxedDeserialize for RunSmcMethod {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5cc65dd2)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x5cc65dd2) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for RunSmcMethod {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x5cc65dd2), self)
    }
}
impl crate::Function for RunSmcMethod {
    type Reply = crate::ton::lite_server::RunMethodResult;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.sendMessage`\n\n```text\nliteServer.sendMessage body:bytes = liteServer.SendMsgStatus;\n```\n"]
pub struct SendMessage {
    pub body: crate::ton::bytes,
}
impl Eq for SendMessage {}
impl crate::BareSerialize for SendMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x690ad482)
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
        vec![crate::ConstructorNumber(0x690ad482)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x690ad482) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SendMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x690ad482), self)
    }
}
impl crate::Function for SendMessage {
    type Reply = crate::ton::lite_server::SendMsgStatus;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.waitMasterchainSeqno`\n\n```text\nliteServer.waitMasterchainSeqno seqno:int timeout_ms:int = Object;\n```\n"]
pub struct WaitMasterchainSeqno {
    pub seqno: crate::ton::int,
    pub timeout_ms: crate::ton::int,
}
impl Eq for WaitMasterchainSeqno {}
impl crate::BareSerialize for WaitMasterchainSeqno {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbaeab892)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let WaitMasterchainSeqno { seqno, timeout_ms } = self;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_bare::<crate::ton::int>(timeout_ms)?;
        Ok(())
    }
}
impl crate::BareDeserialize for WaitMasterchainSeqno {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let timeout_ms = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { seqno, timeout_ms })
        }
    }
}
impl crate::BoxedDeserialize for WaitMasterchainSeqno {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbaeab892)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xbaeab892) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for WaitMasterchainSeqno {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xbaeab892), self)
    }
}
impl crate::Function for WaitMasterchainSeqno {
    type Reply = crate::ton::Object;
}
