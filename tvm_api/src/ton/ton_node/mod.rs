use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.ArchiveInfo`\n\n```text\ntonNode.archiveInfo id:long = tonNode.ArchiveInfo;\n\ntonNode.archiveNotFound = tonNode.ArchiveInfo;\n```\n"]
pub enum ArchiveInfo {
    TonNode_ArchiveInfo(crate::ton::ton_node::archiveinfo::ArchiveInfo),
    TonNode_ArchiveNotFound,
}
impl ArchiveInfo {
    pub fn id(&self) -> Option<&crate::ton::long> {
        match self {
            ArchiveInfo::TonNode_ArchiveInfo(ref x) => Some(&x.id),
            _ => None,
        }
    }
}
impl Eq for ArchiveInfo {}
impl Default for ArchiveInfo {
    fn default() -> Self {
        ArchiveInfo::TonNode_ArchiveInfo(crate::ton::ton_node::archiveinfo::ArchiveInfo::default())
    }
}
impl crate::BoxedSerialize for ArchiveInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ArchiveInfo::TonNode_ArchiveInfo(x) => (crate::ConstructorNumber(0x19efff8c), x),
            ArchiveInfo::TonNode_ArchiveNotFound => (crate::ConstructorNumber(0x99291683), &()),
        }
    }
}
impl crate::BoxedDeserialize for ArchiveInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x19efff8c), crate::ConstructorNumber(0x99291683)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x19efff8c) => Ok(ArchiveInfo::TonNode_ArchiveInfo(
                _de.read_bare::<crate::ton::ton_node::archiveinfo::ArchiveInfo>()?,
            )),
            crate::ConstructorNumber(0x99291683) => Ok(ArchiveInfo::TonNode_ArchiveNotFound),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::ton_node::archiveinfo::ArchiveInfo> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x99291683), &()),
            Some(ref x) => (crate::ConstructorNumber(0x19efff8c), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::ton_node::archiveinfo::ArchiveInfo> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x99291683), crate::ConstructorNumber(0x19efff8c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x99291683) => Ok(None),
            crate::ConstructorNumber(0x19efff8c) => {
                Ok(Some(_de.read_bare::<crate::ton::ton_node::archiveinfo::ArchiveInfo>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.BlockCandidateStatus`\n\n```text\ntonNode.blockCandidateStatus candidate_id:int256 deliveries_signature:bytes approvals_signature:bytes rejections_signature:bytes merges_cnt:int created_timestamp:long = tonNode.BlockCandidateStatus;\n```\n"]
pub enum BlockCandidateStatus {
    TonNode_BlockCandidateStatus(crate::ton::ton_node::blockcandidatestatus::BlockCandidateStatus),
}
impl BlockCandidateStatus {
    pub fn approvals_signature(&self) -> &crate::ton::bytes {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(ref x) => &x.approvals_signature,
        }
    }

    pub fn candidate_id(&self) -> &crate::ton::int256 {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(ref x) => &x.candidate_id,
        }
    }

    pub fn created_timestamp(&self) -> &crate::ton::long {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(ref x) => &x.created_timestamp,
        }
    }

    pub fn deliveries_signature(&self) -> &crate::ton::bytes {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(ref x) => &x.deliveries_signature,
        }
    }

    pub fn merges_cnt(&self) -> &crate::ton::int {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(ref x) => &x.merges_cnt,
        }
    }

    pub fn rejections_signature(&self) -> &crate::ton::bytes {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(ref x) => &x.rejections_signature,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::blockcandidatestatus::BlockCandidateStatus {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(x) => x,
        }
    }
}
impl Eq for BlockCandidateStatus {}
impl Default for BlockCandidateStatus {
    fn default() -> Self {
        BlockCandidateStatus::TonNode_BlockCandidateStatus(
            crate::ton::ton_node::blockcandidatestatus::BlockCandidateStatus::default(),
        )
    }
}
impl crate::BoxedSerialize for BlockCandidateStatus {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockCandidateStatus::TonNode_BlockCandidateStatus(x) => {
                (crate::ConstructorNumber(0xbc31fb7c), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for BlockCandidateStatus {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbc31fb7c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0xbc31fb7c) => Ok (BlockCandidateStatus :: TonNode_BlockCandidateStatus (_de . read_bare :: < crate :: ton :: ton_node :: blockcandidatestatus :: BlockCandidateStatus > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.BlockDescription`\n\n```text\ntonNode.blockDescription id:tonNode.blockIdExt = tonNode.BlockDescription;\n\ntonNode.blockDescriptionEmpty = tonNode.BlockDescription;\n```\n"]
pub enum BlockDescription {
    TonNode_BlockDescription(crate::ton::ton_node::blockdescription::BlockDescription),
    TonNode_BlockDescriptionEmpty,
}
impl BlockDescription {
    pub fn id(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            BlockDescription::TonNode_BlockDescription(ref x) => Some(&x.id),
            _ => None,
        }
    }
}
impl Eq for BlockDescription {}
impl Default for BlockDescription {
    fn default() -> Self {
        BlockDescription::TonNode_BlockDescription(
            crate::ton::ton_node::blockdescription::BlockDescription::default(),
        )
    }
}
impl crate::BoxedSerialize for BlockDescription {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockDescription::TonNode_BlockDescription(x) => {
                (crate::ConstructorNumber(0x46a1d088), x)
            }
            BlockDescription::TonNode_BlockDescriptionEmpty => {
                (crate::ConstructorNumber(0x8384ae95), &())
            }
        }
    }
}
impl crate::BoxedDeserialize for BlockDescription {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x46a1d088), crate::ConstructorNumber(0x8384ae95)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x46a1d088) => Ok(BlockDescription::TonNode_BlockDescription(
                _de.read_bare::<crate::ton::ton_node::blockdescription::BlockDescription>()?,
            )),
            crate::ConstructorNumber(0x8384ae95) => {
                Ok(BlockDescription::TonNode_BlockDescriptionEmpty)
            }
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::ton_node::blockdescription::BlockDescription> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x8384ae95), &()),
            Some(ref x) => (crate::ConstructorNumber(0x46a1d088), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::ton_node::blockdescription::BlockDescription> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8384ae95), crate::ConstructorNumber(0x46a1d088)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8384ae95) => Ok(None),
            crate::ConstructorNumber(0x46a1d088) => Ok(Some(
                _de.read_bare::<crate::ton::ton_node::blockdescription::BlockDescription>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.BlockId`\n\n```text\ntonNode.blockId workchain:int shard:long seqno:int = tonNode.BlockId;\n```\n"]
pub enum BlockId {
    TonNode_BlockId(crate::ton::ton_node::blockid::BlockId),
}
impl BlockId {
    pub fn seqno(&self) -> &crate::ton::int {
        match self {
            BlockId::TonNode_BlockId(ref x) => &x.seqno,
        }
    }

    pub fn shard(&self) -> &crate::ton::long {
        match self {
            BlockId::TonNode_BlockId(ref x) => &x.shard,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            BlockId::TonNode_BlockId(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::blockid::BlockId {
        match self {
            BlockId::TonNode_BlockId(x) => x,
        }
    }
}
impl Eq for BlockId {}
impl Default for BlockId {
    fn default() -> Self {
        BlockId::TonNode_BlockId(crate::ton::ton_node::blockid::BlockId::default())
    }
}
impl crate::BoxedSerialize for BlockId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockId::TonNode_BlockId(x) => (crate::ConstructorNumber(0xb7cdb167), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb7cdb167)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb7cdb167) => Ok(BlockId::TonNode_BlockId(
                _de.read_bare::<crate::ton::ton_node::blockid::BlockId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub(crate) type BlockIdExt = tvm_block::BlockIdExt;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.BlockSignature`\n\n```text\ntonNode.blockSignature who:int256 signature:bytes = tonNode.BlockSignature;\n```\n"]
pub enum BlockSignature {
    TonNode_BlockSignature(crate::ton::ton_node::blocksignature::BlockSignature),
}
impl BlockSignature {
    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            BlockSignature::TonNode_BlockSignature(ref x) => &x.signature,
        }
    }

    pub fn who(&self) -> &crate::ton::int256 {
        match self {
            BlockSignature::TonNode_BlockSignature(ref x) => &x.who,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::blocksignature::BlockSignature {
        match self {
            BlockSignature::TonNode_BlockSignature(x) => x,
        }
    }
}
impl Eq for BlockSignature {}
impl Default for BlockSignature {
    fn default() -> Self {
        BlockSignature::TonNode_BlockSignature(
            crate::ton::ton_node::blocksignature::BlockSignature::default(),
        )
    }
}
impl crate::BoxedSerialize for BlockSignature {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockSignature::TonNode_BlockSignature(x) => (crate::ConstructorNumber(0x50f03c33), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockSignature {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x50f03c33)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x50f03c33) => Ok(BlockSignature::TonNode_BlockSignature(
                _de.read_bare::<crate::ton::ton_node::blocksignature::BlockSignature>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.BlocksDescription`\n\n```text\ntonNode.blocksDescription ids:(vector tonNode.blockIdExt) incomplete:Bool = tonNode.BlocksDescription;\n```\n"]
pub enum BlocksDescription {
    TonNode_BlocksDescription(crate::ton::ton_node::blocksdescription::BlocksDescription),
}
impl BlocksDescription {
    pub fn ids(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            BlocksDescription::TonNode_BlocksDescription(ref x) => &x.ids,
        }
    }

    pub fn incomplete(&self) -> &crate::ton::Bool {
        match self {
            BlocksDescription::TonNode_BlocksDescription(ref x) => &x.incomplete,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::blocksdescription::BlocksDescription {
        match self {
            BlocksDescription::TonNode_BlocksDescription(x) => x,
        }
    }
}
impl Eq for BlocksDescription {}
impl Default for BlocksDescription {
    fn default() -> Self {
        BlocksDescription::TonNode_BlocksDescription(
            crate::ton::ton_node::blocksdescription::BlocksDescription::default(),
        )
    }
}
impl crate::BoxedSerialize for BlocksDescription {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlocksDescription::TonNode_BlocksDescription(x) => {
                (crate::ConstructorNumber(0xd62a612c), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for BlocksDescription {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd62a612c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd62a612c) => {
                Ok(BlocksDescription::TonNode_BlocksDescription(
                    _de.read_bare::<crate::ton::ton_node::blocksdescription::BlocksDescription>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.Broadcast`\n\n```text\ntonNode.blockBroadcast id:tonNode.blockIdExt catchain_seqno:int validator_set_hash:int \n              signatures:(vector tonNode.blockSignature) \n              proof:bytes data:bytes = tonNode.Broadcast;\n\ntonNode.blockCandidateBroadcast id:tonNode.blockIdExt data:bytes collated_data:bytes collated_data_file_hash:int256 created_by:int256 created_timestamp:long = tonNode.Broadcast;\n\ntonNode.connectivityCheckBroadcast pub_key:int256 padding:bytes = tonNode.Broadcast;\n\ntonNode.externalMessageBroadcast message:tonNode.externalMessage = tonNode.Broadcast;\n\ntonNode.ihrMessageBroadcast message:tonNode.ihrMessage = tonNode.Broadcast;\n\ntonNode.newShardBlockBroadcast block:tonNode.newShardBlock = tonNode.Broadcast;\n\ntonNode.queueUpdateBroadcast id:tonNode.blockIdExt catchain_seqno:int validator_set_hash:int \n              signatures:(vector tonNode.blockSignature) target_wc:int\n              data:bytes = tonNode.Broadcast;\n```\n"]
pub enum Broadcast {
    TonNode_BlockBroadcast(crate::ton::ton_node::broadcast::BlockBroadcast),
    TonNode_BlockCandidateBroadcast(crate::ton::ton_node::broadcast::BlockCandidateBroadcast),
    TonNode_ConnectivityCheckBroadcast(crate::ton::ton_node::broadcast::ConnectivityCheckBroadcast),
    TonNode_ExternalMessageBroadcast(crate::ton::ton_node::broadcast::ExternalMessageBroadcast),
    TonNode_IhrMessageBroadcast(crate::ton::ton_node::broadcast::IhrMessageBroadcast),
    TonNode_NewShardBlockBroadcast(crate::ton::ton_node::broadcast::NewShardBlockBroadcast),
    TonNode_QueueUpdateBroadcast(crate::ton::ton_node::broadcast::QueueUpdateBroadcast),
}
impl Broadcast {
    pub fn block(&self) -> Option<&crate::ton::ton_node::newshardblock::NewShardBlock> {
        match self {
            Broadcast::TonNode_NewShardBlockBroadcast(ref x) => Some(&x.block),
            _ => None,
        }
    }

    pub fn catchain_seqno(&self) -> Option<&crate::ton::int> {
        match self {
            Broadcast::TonNode_BlockBroadcast(ref x) => Some(&x.catchain_seqno),
            Broadcast::TonNode_QueueUpdateBroadcast(ref x) => Some(&x.catchain_seqno),
            _ => None,
        }
    }

    pub fn collated_data(&self) -> Option<&crate::ton::bytes> {
        match self {
            Broadcast::TonNode_BlockCandidateBroadcast(ref x) => Some(&x.collated_data),
            _ => None,
        }
    }

    pub fn collated_data_file_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Broadcast::TonNode_BlockCandidateBroadcast(ref x) => Some(&x.collated_data_file_hash),
            _ => None,
        }
    }

    pub fn created_by(&self) -> Option<&crate::ton::int256> {
        match self {
            Broadcast::TonNode_BlockCandidateBroadcast(ref x) => Some(&x.created_by),
            _ => None,
        }
    }

    pub fn created_timestamp(&self) -> Option<&crate::ton::long> {
        match self {
            Broadcast::TonNode_BlockCandidateBroadcast(ref x) => Some(&x.created_timestamp),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            Broadcast::TonNode_BlockBroadcast(ref x) => Some(&x.data),
            Broadcast::TonNode_BlockCandidateBroadcast(ref x) => Some(&x.data),
            Broadcast::TonNode_QueueUpdateBroadcast(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Broadcast::TonNode_BlockBroadcast(ref x) => Some(&x.id),
            Broadcast::TonNode_BlockCandidateBroadcast(ref x) => Some(&x.id),
            Broadcast::TonNode_QueueUpdateBroadcast(ref x) => Some(&x.id),
            _ => None,
        }
    }

    pub fn padding(&self) -> Option<&crate::ton::bytes> {
        match self {
            Broadcast::TonNode_ConnectivityCheckBroadcast(ref x) => Some(&x.padding),
            _ => None,
        }
    }

    pub fn proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            Broadcast::TonNode_BlockBroadcast(ref x) => Some(&x.proof),
            _ => None,
        }
    }

    pub fn pub_key(&self) -> Option<&crate::ton::int256> {
        match self {
            Broadcast::TonNode_ConnectivityCheckBroadcast(ref x) => Some(&x.pub_key),
            _ => None,
        }
    }

    pub fn signatures(
        &self,
    ) -> Option<
        &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blocksignature::BlockSignature>,
    > {
        match self {
            Broadcast::TonNode_BlockBroadcast(ref x) => Some(&x.signatures),
            Broadcast::TonNode_QueueUpdateBroadcast(ref x) => Some(&x.signatures),
            _ => None,
        }
    }

    pub fn target_wc(&self) -> Option<&crate::ton::int> {
        match self {
            Broadcast::TonNode_QueueUpdateBroadcast(ref x) => Some(&x.target_wc),
            _ => None,
        }
    }

    pub fn validator_set_hash(&self) -> Option<&crate::ton::int> {
        match self {
            Broadcast::TonNode_BlockBroadcast(ref x) => Some(&x.validator_set_hash),
            Broadcast::TonNode_QueueUpdateBroadcast(ref x) => Some(&x.validator_set_hash),
            _ => None,
        }
    }
}
impl Eq for Broadcast {}
impl Default for Broadcast {
    fn default() -> Self {
        Broadcast::TonNode_BlockBroadcast(crate::ton::ton_node::broadcast::BlockBroadcast::default())
    }
}
impl crate::BoxedSerialize for Broadcast {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Broadcast::TonNode_BlockBroadcast(x) => (crate::ConstructorNumber(0xae2e1105), x),
            Broadcast::TonNode_BlockCandidateBroadcast(x) => {
                (crate::ConstructorNumber(0x2f8cdf1d), x)
            }
            Broadcast::TonNode_ConnectivityCheckBroadcast(x) => {
                (crate::ConstructorNumber(0x336c53d9), x)
            }
            Broadcast::TonNode_ExternalMessageBroadcast(x) => {
                (crate::ConstructorNumber(0x3d1b1867), x)
            }
            Broadcast::TonNode_IhrMessageBroadcast(x) => (crate::ConstructorNumber(0x525da4b3), x),
            Broadcast::TonNode_NewShardBlockBroadcast(x) => {
                (crate::ConstructorNumber(0x0af2fabc), x)
            }
            Broadcast::TonNode_QueueUpdateBroadcast(x) => (crate::ConstructorNumber(0x9b00bad5), x),
        }
    }
}
impl crate::BoxedDeserialize for Broadcast {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xae2e1105),
            crate::ConstructorNumber(0x2f8cdf1d),
            crate::ConstructorNumber(0x336c53d9),
            crate::ConstructorNumber(0x3d1b1867),
            crate::ConstructorNumber(0x525da4b3),
            crate::ConstructorNumber(0x0af2fabc),
            crate::ConstructorNumber(0x9b00bad5),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xae2e1105) => Ok(Broadcast::TonNode_BlockBroadcast(
                _de.read_bare::<crate::ton::ton_node::broadcast::BlockBroadcast>()?,
            )),
            crate::ConstructorNumber(0x2f8cdf1d) => Ok(Broadcast::TonNode_BlockCandidateBroadcast(
                _de.read_bare::<crate::ton::ton_node::broadcast::BlockCandidateBroadcast>()?,
            )),
            crate::ConstructorNumber(0x336c53d9) => {
                Ok(Broadcast::TonNode_ConnectivityCheckBroadcast(
                    _de.read_bare::<crate::ton::ton_node::broadcast::ConnectivityCheckBroadcast>()?,
                ))
            }
            crate::ConstructorNumber(0x3d1b1867) => {
                Ok(Broadcast::TonNode_ExternalMessageBroadcast(
                    _de.read_bare::<crate::ton::ton_node::broadcast::ExternalMessageBroadcast>()?,
                ))
            }
            crate::ConstructorNumber(0x525da4b3) => Ok(Broadcast::TonNode_IhrMessageBroadcast(
                _de.read_bare::<crate::ton::ton_node::broadcast::IhrMessageBroadcast>()?,
            )),
            crate::ConstructorNumber(0x0af2fabc) => Ok(Broadcast::TonNode_NewShardBlockBroadcast(
                _de.read_bare::<crate::ton::ton_node::broadcast::NewShardBlockBroadcast>()?,
            )),
            crate::ConstructorNumber(0x9b00bad5) => Ok(Broadcast::TonNode_QueueUpdateBroadcast(
                _de.read_bare::<crate::ton::ton_node::broadcast::QueueUpdateBroadcast>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.Capabilities`\n\n```text\ntonNode.capabilities version:int capabilities:long = tonNode.Capabilities;\n```\n"]
pub enum Capabilities {
    TonNode_Capabilities(crate::ton::ton_node::capabilities::Capabilities),
}
impl Capabilities {
    pub fn capabilities(&self) -> &crate::ton::long {
        match self {
            Capabilities::TonNode_Capabilities(ref x) => &x.capabilities,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            Capabilities::TonNode_Capabilities(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::capabilities::Capabilities {
        match self {
            Capabilities::TonNode_Capabilities(x) => x,
        }
    }
}
impl Eq for Capabilities {}
impl Default for Capabilities {
    fn default() -> Self {
        Capabilities::TonNode_Capabilities(
            crate::ton::ton_node::capabilities::Capabilities::default(),
        )
    }
}
impl crate::BoxedSerialize for Capabilities {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Capabilities::TonNode_Capabilities(x) => (crate::ConstructorNumber(0xf5bf60c0), x),
        }
    }
}
impl crate::BoxedDeserialize for Capabilities {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf5bf60c0)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf5bf60c0) => Ok(Capabilities::TonNode_Capabilities(
                _de.read_bare::<crate::ton::ton_node::capabilities::Capabilities>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.Data`\n\n```text\ntonNode.data data:bytes = tonNode.Data;\n```\n"]
pub enum Data {
    TonNode_Data(crate::ton::ton_node::data::Data),
}
impl Data {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            Data::TonNode_Data(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::data::Data {
        match self {
            Data::TonNode_Data(x) => x,
        }
    }
}
impl Eq for Data {}
impl Default for Data {
    fn default() -> Self {
        Data::TonNode_Data(crate::ton::ton_node::data::Data::default())
    }
}
impl crate::BoxedSerialize for Data {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Data::TonNode_Data(x) => (crate::ConstructorNumber(0x560a2484), x),
        }
    }
}
impl crate::BoxedDeserialize for Data {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x560a2484)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x560a2484) => {
                Ok(Data::TonNode_Data(_de.read_bare::<crate::ton::ton_node::data::Data>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.DataFull`\n\n```text\ntonNode.dataFull id:tonNode.blockIdExt proof:bytes block:bytes is_link:Bool = tonNode.DataFull;\n\ntonNode.dataFullEmpty = tonNode.DataFull;\n```\n"]
pub enum DataFull {
    TonNode_DataFull(crate::ton::ton_node::datafull::DataFull),
    TonNode_DataFullEmpty,
}
impl DataFull {
    pub fn block(&self) -> Option<&crate::ton::bytes> {
        match self {
            DataFull::TonNode_DataFull(ref x) => Some(&x.block),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            DataFull::TonNode_DataFull(ref x) => Some(&x.id),
            _ => None,
        }
    }

    pub fn is_link(&self) -> Option<&crate::ton::Bool> {
        match self {
            DataFull::TonNode_DataFull(ref x) => Some(&x.is_link),
            _ => None,
        }
    }

    pub fn proof(&self) -> Option<&crate::ton::bytes> {
        match self {
            DataFull::TonNode_DataFull(ref x) => Some(&x.proof),
            _ => None,
        }
    }
}
impl Eq for DataFull {}
impl Default for DataFull {
    fn default() -> Self {
        DataFull::TonNode_DataFull(crate::ton::ton_node::datafull::DataFull::default())
    }
}
impl crate::BoxedSerialize for DataFull {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DataFull::TonNode_DataFull(x) => (crate::ConstructorNumber(0xbe589f93), x),
            DataFull::TonNode_DataFullEmpty => (crate::ConstructorNumber(0x576e85ca), &()),
        }
    }
}
impl crate::BoxedDeserialize for DataFull {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbe589f93), crate::ConstructorNumber(0x576e85ca)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbe589f93) => Ok(DataFull::TonNode_DataFull(
                _de.read_bare::<crate::ton::ton_node::datafull::DataFull>()?,
            )),
            crate::ConstructorNumber(0x576e85ca) => Ok(DataFull::TonNode_DataFullEmpty),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::ton_node::datafull::DataFull> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x576e85ca), &()),
            Some(ref x) => (crate::ConstructorNumber(0xbe589f93), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::ton_node::datafull::DataFull> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x576e85ca), crate::ConstructorNumber(0xbe589f93)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x576e85ca) => Ok(None),
            crate::ConstructorNumber(0xbe589f93) => {
                Ok(Some(_de.read_bare::<crate::ton::ton_node::datafull::DataFull>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.DataList`\n\n```text\ntonNode.dataList data:(vector bytes) = tonNode.DataList;\n```\n"]
pub enum DataList {
    TonNode_DataList(crate::ton::ton_node::datalist::DataList),
}
impl DataList {
    pub fn data(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::bytes> {
        match self {
            DataList::TonNode_DataList(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::datalist::DataList {
        match self {
            DataList::TonNode_DataList(x) => x,
        }
    }
}
impl Eq for DataList {}
impl Default for DataList {
    fn default() -> Self {
        DataList::TonNode_DataList(crate::ton::ton_node::datalist::DataList::default())
    }
}
impl crate::BoxedSerialize for DataList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DataList::TonNode_DataList(x) => (crate::ConstructorNumber(0x14f43313), x),
        }
    }
}
impl crate::BoxedDeserialize for DataList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x14f43313)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x14f43313) => Ok(DataList::TonNode_DataList(
                _de.read_bare::<crate::ton::ton_node::datalist::DataList>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.ExternalMessage`\n\n```text\ntonNode.externalMessage data:bytes = tonNode.ExternalMessage;\n```\n"]
pub enum ExternalMessage {
    TonNode_ExternalMessage(crate::ton::ton_node::externalmessage::ExternalMessage),
}
impl ExternalMessage {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            ExternalMessage::TonNode_ExternalMessage(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::externalmessage::ExternalMessage {
        match self {
            ExternalMessage::TonNode_ExternalMessage(x) => x,
        }
    }
}
impl Eq for ExternalMessage {}
impl Default for ExternalMessage {
    fn default() -> Self {
        ExternalMessage::TonNode_ExternalMessage(
            crate::ton::ton_node::externalmessage::ExternalMessage::default(),
        )
    }
}
impl crate::BoxedSerialize for ExternalMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ExternalMessage::TonNode_ExternalMessage(x) => {
                (crate::ConstructorNumber(0xdc75a209), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ExternalMessage {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdc75a209)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xdc75a209) => Ok(ExternalMessage::TonNode_ExternalMessage(
                _de.read_bare::<crate::ton::ton_node::externalmessage::ExternalMessage>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.IhrMessage`\n\n```text\ntonNode.ihrMessage data:bytes = tonNode.IhrMessage;\n```\n"]
pub enum IhrMessage {
    TonNode_IhrMessage(crate::ton::ton_node::ihrmessage::IhrMessage),
}
impl IhrMessage {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            IhrMessage::TonNode_IhrMessage(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::ihrmessage::IhrMessage {
        match self {
            IhrMessage::TonNode_IhrMessage(x) => x,
        }
    }
}
impl Eq for IhrMessage {}
impl Default for IhrMessage {
    fn default() -> Self {
        IhrMessage::TonNode_IhrMessage(crate::ton::ton_node::ihrmessage::IhrMessage::default())
    }
}
impl crate::BoxedSerialize for IhrMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            IhrMessage::TonNode_IhrMessage(x) => (crate::ConstructorNumber(0x4534c307), x),
        }
    }
}
impl crate::BoxedDeserialize for IhrMessage {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4534c307)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x4534c307) => Ok(IhrMessage::TonNode_IhrMessage(
                _de.read_bare::<crate::ton::ton_node::ihrmessage::IhrMessage>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.KeyBlocks`\n\n```text\ntonNode.keyBlocks blocks:(vector tonNode.blockIdExt) incomplete:Bool error:Bool = tonNode.KeyBlocks;\n```\n"]
pub enum KeyBlocks {
    TonNode_KeyBlocks(crate::ton::ton_node::keyblocks::KeyBlocks),
}
impl KeyBlocks {
    pub fn blocks(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            KeyBlocks::TonNode_KeyBlocks(ref x) => &x.blocks,
        }
    }

    pub fn error(&self) -> &crate::ton::Bool {
        match self {
            KeyBlocks::TonNode_KeyBlocks(ref x) => &x.error,
        }
    }

    pub fn incomplete(&self) -> &crate::ton::Bool {
        match self {
            KeyBlocks::TonNode_KeyBlocks(ref x) => &x.incomplete,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::keyblocks::KeyBlocks {
        match self {
            KeyBlocks::TonNode_KeyBlocks(x) => x,
        }
    }
}
impl Eq for KeyBlocks {}
impl Default for KeyBlocks {
    fn default() -> Self {
        KeyBlocks::TonNode_KeyBlocks(crate::ton::ton_node::keyblocks::KeyBlocks::default())
    }
}
impl crate::BoxedSerialize for KeyBlocks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            KeyBlocks::TonNode_KeyBlocks(x) => (crate::ConstructorNumber(0x07664d59), x),
        }
    }
}
impl crate::BoxedDeserialize for KeyBlocks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x07664d59)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x07664d59) => Ok(KeyBlocks::TonNode_KeyBlocks(
                _de.read_bare::<crate::ton::ton_node::keyblocks::KeyBlocks>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.NewShardBlock`\n\n```text\ntonNode.newShardBlock block:tonNode.blockIdExt cc_seqno:int data:bytes = tonNode.NewShardBlock;\n```\n"]
pub enum NewShardBlock {
    TonNode_NewShardBlock(crate::ton::ton_node::newshardblock::NewShardBlock),
}
impl NewShardBlock {
    pub fn block(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            NewShardBlock::TonNode_NewShardBlock(ref x) => &x.block,
        }
    }

    pub fn cc_seqno(&self) -> &crate::ton::int {
        match self {
            NewShardBlock::TonNode_NewShardBlock(ref x) => &x.cc_seqno,
        }
    }

    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            NewShardBlock::TonNode_NewShardBlock(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::newshardblock::NewShardBlock {
        match self {
            NewShardBlock::TonNode_NewShardBlock(x) => x,
        }
    }
}
impl Eq for NewShardBlock {}
impl Default for NewShardBlock {
    fn default() -> Self {
        NewShardBlock::TonNode_NewShardBlock(
            crate::ton::ton_node::newshardblock::NewShardBlock::default(),
        )
    }
}
impl crate::BoxedSerialize for NewShardBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            NewShardBlock::TonNode_NewShardBlock(x) => (crate::ConstructorNumber(0xa49dc229), x),
        }
    }
}
impl crate::BoxedDeserialize for NewShardBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa49dc229)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa49dc229) => Ok(NewShardBlock::TonNode_NewShardBlock(
                _de.read_bare::<crate::ton::ton_node::newshardblock::NewShardBlock>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `tonNode.Prepared`\n\n```text\ntonNode.notFound = tonNode.Prepared;\n\ntonNode.prepared = tonNode.Prepared;\n```\n"]
pub enum Prepared {
    #[default]
    TonNode_NotFound,
    TonNode_Prepared,
}
impl Eq for Prepared {}
impl crate::BoxedSerialize for Prepared {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Prepared::TonNode_NotFound => (crate::ConstructorNumber(0xe2c33da6), &()),
            Prepared::TonNode_Prepared => (crate::ConstructorNumber(0xeac4bbcd), &()),
        }
    }
}
impl crate::BoxedDeserialize for Prepared {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe2c33da6), crate::ConstructorNumber(0xeac4bbcd)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe2c33da6) => Ok(Prepared::TonNode_NotFound),
            crate::ConstructorNumber(0xeac4bbcd) => Ok(Prepared::TonNode_Prepared),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `tonNode.PreparedProof`\n\n```text\ntonNode.preparedProof = tonNode.PreparedProof;\n\ntonNode.preparedProofEmpty = tonNode.PreparedProof;\n\ntonNode.preparedProofLink = tonNode.PreparedProof;\n```\n"]
pub enum PreparedProof {
    #[default]
    TonNode_PreparedProof,
    TonNode_PreparedProofEmpty,
    TonNode_PreparedProofLink,
}
impl Eq for PreparedProof {}
impl crate::BoxedSerialize for PreparedProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PreparedProof::TonNode_PreparedProof => (crate::ConstructorNumber(0x899f9a4b), &()),
            PreparedProof::TonNode_PreparedProofEmpty => {
                (crate::ConstructorNumber(0xc769c17a), &())
            }
            PreparedProof::TonNode_PreparedProofLink => (crate::ConstructorNumber(0x3dff328d), &()),
        }
    }
}
impl crate::BoxedDeserialize for PreparedProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x899f9a4b),
            crate::ConstructorNumber(0xc769c17a),
            crate::ConstructorNumber(0x3dff328d),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x899f9a4b) => Ok(PreparedProof::TonNode_PreparedProof),
            crate::ConstructorNumber(0xc769c17a) => Ok(PreparedProof::TonNode_PreparedProofEmpty),
            crate::ConstructorNumber(0x3dff328d) => Ok(PreparedProof::TonNode_PreparedProofLink),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `tonNode.PreparedState`\n\n```text\ntonNode.notFoundState = tonNode.PreparedState;\n\ntonNode.preparedState = tonNode.PreparedState;\n```\n"]
pub enum PreparedState {
    #[default]
    TonNode_NotFoundState,
    TonNode_PreparedState,
}
impl Eq for PreparedState {}
impl crate::BoxedSerialize for PreparedState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PreparedState::TonNode_NotFoundState => (crate::ConstructorNumber(0x32390a51), &()),
            PreparedState::TonNode_PreparedState => (crate::ConstructorNumber(0x375bcb6d), &()),
        }
    }
}
impl crate::BoxedDeserialize for PreparedState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x32390a51), crate::ConstructorNumber(0x375bcb6d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x32390a51) => Ok(PreparedState::TonNode_NotFoundState),
            crate::ConstructorNumber(0x375bcb6d) => Ok(PreparedState::TonNode_PreparedState),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCatchainMessageIds`\n\n```text\ntonNode.rempCatchainMessageIds id:int256 uid:int256 = tonNode.rempCatchainMessageIds;\n```\n"]
pub enum RempCatchainMessageIds {
    TonNode_RempCatchainMessageIds(
        crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
    ),
}
impl RempCatchainMessageIds {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            RempCatchainMessageIds::TonNode_RempCatchainMessageIds(ref x) => &x.id,
        }
    }

    pub fn uid(&self) -> &crate::ton::int256 {
        match self {
            RempCatchainMessageIds::TonNode_RempCatchainMessageIds(ref x) => &x.uid,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds {
        match self {
            RempCatchainMessageIds::TonNode_RempCatchainMessageIds(x) => x,
        }
    }
}
impl Eq for RempCatchainMessageIds {}
impl Default for RempCatchainMessageIds {
    fn default() -> Self {
        RempCatchainMessageIds::TonNode_RempCatchainMessageIds(
            crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds::default(),
        )
    }
}
impl crate::BoxedSerialize for RempCatchainMessageIds {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempCatchainMessageIds::TonNode_RempCatchainMessageIds(x) => {
                (crate::ConstructorNumber(0x5509db82), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RempCatchainMessageIds {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5509db82)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x5509db82) => Ok (RempCatchainMessageIds :: TonNode_RempCatchainMessageIds (_de . read_bare :: < crate :: ton :: ton_node :: rempcatchainmessageids :: RempCatchainMessageIds > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCatchainRecord`\n\n```text\ntonNode.rempCatchainMessage message:bytes message_id:int256 source_key_id:int256 source_idx:int masterchain_seqno:int = tonNode.rempCatchainRecord;\n\ntonNode.rempCatchainMessageDigest masterchain_seqno:int messages:(vector tonNode.rempCatchainMessageIds) = tonNode.rempCatchainRecord;\n```\n"]
pub enum RempCatchainRecord {
    TonNode_RempCatchainMessage(crate::ton::ton_node::rempcatchainrecord::RempCatchainMessage),
    TonNode_RempCatchainMessageDigest(
        crate::ton::ton_node::rempcatchainrecord::RempCatchainMessageDigest,
    ),
}
impl RempCatchainRecord {
    pub fn masterchain_seqno(&self) -> &crate::ton::int {
        match self {
            RempCatchainRecord::TonNode_RempCatchainMessage(ref x) => &x.masterchain_seqno,
            RempCatchainRecord::TonNode_RempCatchainMessageDigest(ref x) => &x.masterchain_seqno,
        }
    }

    pub fn message(&self) -> Option<&crate::ton::bytes> {
        match self {
            RempCatchainRecord::TonNode_RempCatchainMessage(ref x) => Some(&x.message),
            _ => None,
        }
    }

    pub fn message_id(&self) -> Option<&crate::ton::int256> {
        match self {
            RempCatchainRecord::TonNode_RempCatchainMessage(ref x) => Some(&x.message_id),
            _ => None,
        }
    }

    pub fn messages(
        &self,
    ) -> Option<
        &crate::ton::vector<
            crate::ton::Bare,
            crate::ton::ton_node::rempcatchainmessageids::RempCatchainMessageIds,
        >,
    > {
        match self {
            RempCatchainRecord::TonNode_RempCatchainMessageDigest(ref x) => Some(&x.messages),
            _ => None,
        }
    }

    pub fn source_idx(&self) -> Option<&crate::ton::int> {
        match self {
            RempCatchainRecord::TonNode_RempCatchainMessage(ref x) => Some(&x.source_idx),
            _ => None,
        }
    }

    pub fn source_key_id(&self) -> Option<&crate::ton::int256> {
        match self {
            RempCatchainRecord::TonNode_RempCatchainMessage(ref x) => Some(&x.source_key_id),
            _ => None,
        }
    }
}
impl Eq for RempCatchainRecord {}
impl Default for RempCatchainRecord {
    fn default() -> Self {
        RempCatchainRecord::TonNode_RempCatchainMessage(
            crate::ton::ton_node::rempcatchainrecord::RempCatchainMessage::default(),
        )
    }
}
impl crate::BoxedSerialize for RempCatchainRecord {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempCatchainRecord::TonNode_RempCatchainMessage(x) => {
                (crate::ConstructorNumber(0xe9d6eb1c), x)
            }
            RempCatchainRecord::TonNode_RempCatchainMessageDigest(x) => {
                (crate::ConstructorNumber(0x384e3f84), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RempCatchainRecord {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe9d6eb1c), crate::ConstructorNumber(0x384e3f84)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0xe9d6eb1c) => Ok (RempCatchainRecord :: TonNode_RempCatchainMessage (_de . read_bare :: < crate :: ton :: ton_node :: rempcatchainrecord :: RempCatchainMessage > () ?)) , crate :: ConstructorNumber (0x384e3f84) => Ok (RempCatchainRecord :: TonNode_RempCatchainMessageDigest (_de . read_bare :: < crate :: ton :: ton_node :: rempcatchainrecord :: RempCatchainMessageDigest > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempCombinedReceipt`\n\n```text\ntonNode.rempCombinedReceipt source_id:int256 ids:(vector tonNode.blockIdExt) \n        receipts:(vector tonNode.RempReceiptCompact) = tonNode.RempCombinedReceipt;\n```\n"]
pub enum RempCombinedReceipt {
    TonNode_RempCombinedReceipt(crate::ton::ton_node::rempcombinedreceipt::RempCombinedReceipt),
}
impl RempCombinedReceipt {
    pub fn ids(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            RempCombinedReceipt::TonNode_RempCombinedReceipt(ref x) => &x.ids,
        }
    }

    pub fn receipts(
        &self,
    ) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::ton_node::RempReceiptCompact> {
        match self {
            RempCombinedReceipt::TonNode_RempCombinedReceipt(ref x) => &x.receipts,
        }
    }

    pub fn source_id(&self) -> &crate::ton::int256 {
        match self {
            RempCombinedReceipt::TonNode_RempCombinedReceipt(ref x) => &x.source_id,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempcombinedreceipt::RempCombinedReceipt {
        match self {
            RempCombinedReceipt::TonNode_RempCombinedReceipt(x) => x,
        }
    }
}
impl Eq for RempCombinedReceipt {}
impl Default for RempCombinedReceipt {
    fn default() -> Self {
        RempCombinedReceipt::TonNode_RempCombinedReceipt(
            crate::ton::ton_node::rempcombinedreceipt::RempCombinedReceipt::default(),
        )
    }
}
impl crate::BoxedSerialize for RempCombinedReceipt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempCombinedReceipt::TonNode_RempCombinedReceipt(x) => {
                (crate::ConstructorNumber(0x98eb4db1), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RempCombinedReceipt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x98eb4db1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x98eb4db1) => Ok (RempCombinedReceipt :: TonNode_RempCombinedReceipt (_de . read_bare :: < crate :: ton :: ton_node :: rempcombinedreceipt :: RempCombinedReceipt > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempMessage`\n\n```text\ntonNode.rempMessage message:bytes id:int256 timestamp:long signature:bytes = tonNode.RempMessage;\n```\n"]
pub enum RempMessage {
    TonNode_RempMessage(crate::ton::ton_node::rempmessage::RempMessage),
}
impl RempMessage {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            RempMessage::TonNode_RempMessage(ref x) => &x.id,
        }
    }

    pub fn message(&self) -> &crate::ton::bytes {
        match self {
            RempMessage::TonNode_RempMessage(ref x) => &x.message,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            RempMessage::TonNode_RempMessage(ref x) => &x.signature,
        }
    }

    pub fn timestamp(&self) -> &crate::ton::long {
        match self {
            RempMessage::TonNode_RempMessage(ref x) => &x.timestamp,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempmessage::RempMessage {
        match self {
            RempMessage::TonNode_RempMessage(x) => x,
        }
    }
}
impl Eq for RempMessage {}
impl Default for RempMessage {
    fn default() -> Self {
        RempMessage::TonNode_RempMessage(crate::ton::ton_node::rempmessage::RempMessage::default())
    }
}
impl crate::BoxedSerialize for RempMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempMessage::TonNode_RempMessage(x) => (crate::ConstructorNumber(0xdd1f6db1), x),
        }
    }
}
impl crate::BoxedDeserialize for RempMessage {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdd1f6db1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xdd1f6db1) => Ok(RempMessage::TonNode_RempMessage(
                _de.read_bare::<crate::ton::ton_node::rempmessage::RempMessage>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `tonNode.RempMessageLevel`\n\n```text\ntonNode.rempCollator = tonNode.RempMessageLevel;\n\ntonNode.rempFullnode = tonNode.RempMessageLevel;\n\ntonNode.rempMasterchain = tonNode.RempMessageLevel;\n\ntonNode.rempQueue = tonNode.RempMessageLevel;\n\ntonNode.rempShardchain = tonNode.RempMessageLevel;\n```\n"]
pub enum RempMessageLevel {
    #[default]
    TonNode_RempCollator,
    TonNode_RempFullnode,
    TonNode_RempMasterchain,
    TonNode_RempQueue,
    TonNode_RempShardchain,
}
impl Eq for RempMessageLevel {}
impl crate::BoxedSerialize for RempMessageLevel {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempMessageLevel::TonNode_RempCollator => (crate::ConstructorNumber(0x44f07de7), &()),
            RempMessageLevel::TonNode_RempFullnode => (crate::ConstructorNumber(0xd7145a5a), &()),
            RempMessageLevel::TonNode_RempMasterchain => {
                (crate::ConstructorNumber(0xbcc0eaa3), &())
            }
            RempMessageLevel::TonNode_RempQueue => (crate::ConstructorNumber(0xebabf815), &()),
            RempMessageLevel::TonNode_RempShardchain => (crate::ConstructorNumber(0x237e2b37), &()),
        }
    }
}
impl crate::BoxedDeserialize for RempMessageLevel {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x44f07de7),
            crate::ConstructorNumber(0xd7145a5a),
            crate::ConstructorNumber(0xbcc0eaa3),
            crate::ConstructorNumber(0xebabf815),
            crate::ConstructorNumber(0x237e2b37),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x44f07de7) => Ok(RempMessageLevel::TonNode_RempCollator),
            crate::ConstructorNumber(0xd7145a5a) => Ok(RempMessageLevel::TonNode_RempFullnode),
            crate::ConstructorNumber(0xbcc0eaa3) => Ok(RempMessageLevel::TonNode_RempMasterchain),
            crate::ConstructorNumber(0xebabf815) => Ok(RempMessageLevel::TonNode_RempQueue),
            crate::ConstructorNumber(0x237e2b37) => Ok(RempMessageLevel::TonNode_RempShardchain),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempMessageStatus`\n\n```text\ntonNode.rempAccepted level:tonNode.RempMessageLevel block_id:tonNode.blockIdExt master_id:tonNode.blockIdExt = tonNode.RempMessageStatus;\n\ntonNode.rempDuplicate block_id:tonNode.blockIdExt = tonNode.RempMessageStatus;\n\ntonNode.rempIgnored level:tonNode.RempMessageLevel block_id:tonNode.blockIdExt = tonNode.RempMessageStatus;\n\ntonNode.rempNew = tonNode.RempMessageStatus;\n\ntonNode.rempRejected level:tonNode.RempMessageLevel block_id:tonNode.blockIdExt error:string = tonNode.RempMessageStatus;\n\ntonNode.rempSentToValidators sent_to:int total_validators:int = tonNode.RempMessageStatus;\n\ntonNode.rempTimeout = tonNode.RempMessageStatus;\n```\n"]
pub enum RempMessageStatus {
    TonNode_RempAccepted(crate::ton::ton_node::rempmessagestatus::RempAccepted),
    TonNode_RempDuplicate(crate::ton::ton_node::rempmessagestatus::RempDuplicate),
    TonNode_RempIgnored(crate::ton::ton_node::rempmessagestatus::RempIgnored),
    TonNode_RempNew,
    TonNode_RempRejected(crate::ton::ton_node::rempmessagestatus::RempRejected),
    TonNode_RempSentToValidators(crate::ton::ton_node::rempmessagestatus::RempSentToValidators),
    TonNode_RempTimeout,
}
impl RempMessageStatus {
    pub fn block_id(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            RempMessageStatus::TonNode_RempAccepted(ref x) => Some(&x.block_id),
            RempMessageStatus::TonNode_RempDuplicate(ref x) => Some(&x.block_id),
            RempMessageStatus::TonNode_RempIgnored(ref x) => Some(&x.block_id),
            RempMessageStatus::TonNode_RempRejected(ref x) => Some(&x.block_id),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&crate::ton::string> {
        match self {
            RempMessageStatus::TonNode_RempRejected(ref x) => Some(&x.error),
            _ => None,
        }
    }

    pub fn level(&self) -> Option<&crate::ton::ton_node::RempMessageLevel> {
        match self {
            RempMessageStatus::TonNode_RempAccepted(ref x) => Some(&x.level),
            RempMessageStatus::TonNode_RempIgnored(ref x) => Some(&x.level),
            RempMessageStatus::TonNode_RempRejected(ref x) => Some(&x.level),
            _ => None,
        }
    }

    pub fn master_id(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            RempMessageStatus::TonNode_RempAccepted(ref x) => Some(&x.master_id),
            _ => None,
        }
    }

    pub fn sent_to(&self) -> Option<&crate::ton::int> {
        match self {
            RempMessageStatus::TonNode_RempSentToValidators(ref x) => Some(&x.sent_to),
            _ => None,
        }
    }

    pub fn total_validators(&self) -> Option<&crate::ton::int> {
        match self {
            RempMessageStatus::TonNode_RempSentToValidators(ref x) => Some(&x.total_validators),
            _ => None,
        }
    }
}
impl Eq for RempMessageStatus {}
impl Default for RempMessageStatus {
    fn default() -> Self {
        RempMessageStatus::TonNode_RempAccepted(
            crate::ton::ton_node::rempmessagestatus::RempAccepted::default(),
        )
    }
}
impl crate::BoxedSerialize for RempMessageStatus {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempMessageStatus::TonNode_RempAccepted(x) => (crate::ConstructorNumber(0x30225e0e), x),
            RempMessageStatus::TonNode_RempDuplicate(x) => {
                (crate::ConstructorNumber(0x9ecd4334), x)
            }
            RempMessageStatus::TonNode_RempIgnored(x) => (crate::ConstructorNumber(0x43bebb8b), x),
            RempMessageStatus::TonNode_RempNew => (crate::ConstructorNumber(0x5bfabd30), &()),
            RempMessageStatus::TonNode_RempRejected(x) => (crate::ConstructorNumber(0xb4e1ee77), x),
            RempMessageStatus::TonNode_RempSentToValidators(x) => {
                (crate::ConstructorNumber(0x2ff6c87b), x)
            }
            RempMessageStatus::TonNode_RempTimeout => (crate::ConstructorNumber(0xb71796af), &()),
        }
    }
}
impl crate::BoxedDeserialize for RempMessageStatus {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x30225e0e),
            crate::ConstructorNumber(0x9ecd4334),
            crate::ConstructorNumber(0x43bebb8b),
            crate::ConstructorNumber(0x5bfabd30),
            crate::ConstructorNumber(0xb4e1ee77),
            crate::ConstructorNumber(0x2ff6c87b),
            crate::ConstructorNumber(0xb71796af),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x30225e0e) => Ok(RempMessageStatus::TonNode_RempAccepted(
                _de.read_bare::<crate::ton::ton_node::rempmessagestatus::RempAccepted>()?,
            )),
            crate::ConstructorNumber(0x9ecd4334) => Ok(RempMessageStatus::TonNode_RempDuplicate(
                _de.read_bare::<crate::ton::ton_node::rempmessagestatus::RempDuplicate>()?,
            )),
            crate::ConstructorNumber(0x43bebb8b) => Ok(RempMessageStatus::TonNode_RempIgnored(
                _de.read_bare::<crate::ton::ton_node::rempmessagestatus::RempIgnored>()?,
            )),
            crate::ConstructorNumber(0x5bfabd30) => Ok(RempMessageStatus::TonNode_RempNew),
            crate::ConstructorNumber(0xb4e1ee77) => Ok(RempMessageStatus::TonNode_RempRejected(
                _de.read_bare::<crate::ton::ton_node::rempmessagestatus::RempRejected>()?,
            )),
            crate::ConstructorNumber(0x2ff6c87b) => {
                Ok(RempMessageStatus::TonNode_RempSentToValidators(
                    _de.read_bare::<crate::ton::ton_node::rempmessagestatus::RempSentToValidators>(
                    )?,
                ))
            }
            crate::ConstructorNumber(0xb71796af) => Ok(RempMessageStatus::TonNode_RempTimeout),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempMessageStatusCompact`\n\n```text\ntonNode.rempAcceptedCompact level:byte block_id_index:byte master_id_index:byte = tonNode.RempMessageStatusCompact;\n\ntonNode.rempDuplicateCompact block_id_index:byte = tonNode.RempMessageStatusCompact;\n\ntonNode.rempIgnoredCompact level:byte block_id_index:byte = tonNode.RempMessageStatusCompact;\n\ntonNode.rempNewCompact = tonNode.RempMessageStatusCompact;\n\ntonNode.rempRejectedCompact level:byte block_id_index:byte error:string = tonNode.RempMessageStatusCompact;\n\ntonNode.rempSentToValidatorsCompact sent_to:byte total_validators:byte = tonNode.RempMessageStatusCompact;\n\ntonNode.rempTimeoutCompact = tonNode.RempMessageStatusCompact;\n```\n"]
pub enum RempMessageStatusCompact {
    TonNode_RempAcceptedCompact(
        crate::ton::ton_node::rempmessagestatuscompact::RempAcceptedCompact,
    ),
    TonNode_RempDuplicateCompact(
        crate::ton::ton_node::rempmessagestatuscompact::RempDuplicateCompact,
    ),
    TonNode_RempIgnoredCompact(crate::ton::ton_node::rempmessagestatuscompact::RempIgnoredCompact),
    TonNode_RempNewCompact,
    TonNode_RempRejectedCompact(
        crate::ton::ton_node::rempmessagestatuscompact::RempRejectedCompact,
    ),
    TonNode_RempSentToValidatorsCompact(
        crate::ton::ton_node::rempmessagestatuscompact::RempSentToValidatorsCompact,
    ),
    TonNode_RempTimeoutCompact,
}
impl RempMessageStatusCompact {
    pub fn block_id_index(&self) -> Option<&crate::ton::byte> {
        match self {
            RempMessageStatusCompact::TonNode_RempAcceptedCompact(ref x) => Some(&x.block_id_index),
            RempMessageStatusCompact::TonNode_RempDuplicateCompact(ref x) => {
                Some(&x.block_id_index)
            }
            RempMessageStatusCompact::TonNode_RempIgnoredCompact(ref x) => Some(&x.block_id_index),
            RempMessageStatusCompact::TonNode_RempRejectedCompact(ref x) => Some(&x.block_id_index),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&crate::ton::string> {
        match self {
            RempMessageStatusCompact::TonNode_RempRejectedCompact(ref x) => Some(&x.error),
            _ => None,
        }
    }

    pub fn level(&self) -> Option<&crate::ton::byte> {
        match self {
            RempMessageStatusCompact::TonNode_RempAcceptedCompact(ref x) => Some(&x.level),
            RempMessageStatusCompact::TonNode_RempIgnoredCompact(ref x) => Some(&x.level),
            RempMessageStatusCompact::TonNode_RempRejectedCompact(ref x) => Some(&x.level),
            _ => None,
        }
    }

    pub fn master_id_index(&self) -> Option<&crate::ton::byte> {
        match self {
            RempMessageStatusCompact::TonNode_RempAcceptedCompact(ref x) => {
                Some(&x.master_id_index)
            }
            _ => None,
        }
    }

    pub fn sent_to(&self) -> Option<&crate::ton::byte> {
        match self {
            RempMessageStatusCompact::TonNode_RempSentToValidatorsCompact(ref x) => {
                Some(&x.sent_to)
            }
            _ => None,
        }
    }

    pub fn total_validators(&self) -> Option<&crate::ton::byte> {
        match self {
            RempMessageStatusCompact::TonNode_RempSentToValidatorsCompact(ref x) => {
                Some(&x.total_validators)
            }
            _ => None,
        }
    }
}
impl Eq for RempMessageStatusCompact {}
impl Default for RempMessageStatusCompact {
    fn default() -> Self {
        RempMessageStatusCompact::TonNode_RempAcceptedCompact(
            crate::ton::ton_node::rempmessagestatuscompact::RempAcceptedCompact::default(),
        )
    }
}
impl crate::BoxedSerialize for RempMessageStatusCompact {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempMessageStatusCompact::TonNode_RempAcceptedCompact(x) => {
                (crate::ConstructorNumber(0x8f092f0f), x)
            }
            RempMessageStatusCompact::TonNode_RempDuplicateCompact(x) => {
                (crate::ConstructorNumber(0x16b33de4), x)
            }
            RempMessageStatusCompact::TonNode_RempIgnoredCompact(x) => {
                (crate::ConstructorNumber(0xbd26c204), x)
            }
            RempMessageStatusCompact::TonNode_RempNewCompact => {
                (crate::ConstructorNumber(0x98c365f0), &())
            }
            RempMessageStatusCompact::TonNode_RempRejectedCompact(x) => {
                (crate::ConstructorNumber(0x15a2b254), x)
            }
            RempMessageStatusCompact::TonNode_RempSentToValidatorsCompact(x) => {
                (crate::ConstructorNumber(0x3f854835), x)
            }
            RempMessageStatusCompact::TonNode_RempTimeoutCompact => {
                (crate::ConstructorNumber(0x8434c5b8), &())
            }
        }
    }
}
impl crate::BoxedDeserialize for RempMessageStatusCompact {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x8f092f0f),
            crate::ConstructorNumber(0x16b33de4),
            crate::ConstructorNumber(0xbd26c204),
            crate::ConstructorNumber(0x98c365f0),
            crate::ConstructorNumber(0x15a2b254),
            crate::ConstructorNumber(0x3f854835),
            crate::ConstructorNumber(0x8434c5b8),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x8f092f0f) => Ok (RempMessageStatusCompact :: TonNode_RempAcceptedCompact (_de . read_bare :: < crate :: ton :: ton_node :: rempmessagestatuscompact :: RempAcceptedCompact > () ?)) , crate :: ConstructorNumber (0x16b33de4) => Ok (RempMessageStatusCompact :: TonNode_RempDuplicateCompact (_de . read_bare :: < crate :: ton :: ton_node :: rempmessagestatuscompact :: RempDuplicateCompact > () ?)) , crate :: ConstructorNumber (0xbd26c204) => Ok (RempMessageStatusCompact :: TonNode_RempIgnoredCompact (_de . read_bare :: < crate :: ton :: ton_node :: rempmessagestatuscompact :: RempIgnoredCompact > () ?)) , crate :: ConstructorNumber (0x98c365f0) => Ok (RempMessageStatusCompact :: TonNode_RempNewCompact) , crate :: ConstructorNumber (0x15a2b254) => Ok (RempMessageStatusCompact :: TonNode_RempRejectedCompact (_de . read_bare :: < crate :: ton :: ton_node :: rempmessagestatuscompact :: RempRejectedCompact > () ?)) , crate :: ConstructorNumber (0x3f854835) => Ok (RempMessageStatusCompact :: TonNode_RempSentToValidatorsCompact (_de . read_bare :: < crate :: ton :: ton_node :: rempmessagestatuscompact :: RempSentToValidatorsCompact > () ?)) , crate :: ConstructorNumber (0x8434c5b8) => Ok (RempMessageStatusCompact :: TonNode_RempTimeoutCompact) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempReceipt`\n\n```text\ntonNode.rempReceipt message_id:int256 status:tonNode.RempMessageStatus timestamp:long source_id:int256 = tonNode.RempReceipt;\n```\n"]
pub enum RempReceipt {
    TonNode_RempReceipt(crate::ton::ton_node::rempreceipt::RempReceipt),
}
impl RempReceipt {
    pub fn message_id(&self) -> &crate::ton::int256 {
        match self {
            RempReceipt::TonNode_RempReceipt(ref x) => &x.message_id,
        }
    }

    pub fn source_id(&self) -> &crate::ton::int256 {
        match self {
            RempReceipt::TonNode_RempReceipt(ref x) => &x.source_id,
        }
    }

    pub fn status(&self) -> &crate::ton::ton_node::RempMessageStatus {
        match self {
            RempReceipt::TonNode_RempReceipt(ref x) => &x.status,
        }
    }

    pub fn timestamp(&self) -> &crate::ton::long {
        match self {
            RempReceipt::TonNode_RempReceipt(ref x) => &x.timestamp,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempreceipt::RempReceipt {
        match self {
            RempReceipt::TonNode_RempReceipt(x) => x,
        }
    }
}
impl Eq for RempReceipt {}
impl Default for RempReceipt {
    fn default() -> Self {
        RempReceipt::TonNode_RempReceipt(crate::ton::ton_node::rempreceipt::RempReceipt::default())
    }
}
impl crate::BoxedSerialize for RempReceipt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempReceipt::TonNode_RempReceipt(x) => (crate::ConstructorNumber(0x3122b7a2), x),
        }
    }
}
impl crate::BoxedDeserialize for RempReceipt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3122b7a2)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3122b7a2) => Ok(RempReceipt::TonNode_RempReceipt(
                _de.read_bare::<crate::ton::ton_node::rempreceipt::RempReceipt>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempReceiptCompact`\n\n```text\ntonNode.rempReceiptCompact message_id:int256 receipt:tonNode.RempMessageStatusCompact \n        timestamp:long = tonNode.RempReceiptCompact;\n```\n"]
pub enum RempReceiptCompact {
    TonNode_RempReceiptCompact(crate::ton::ton_node::rempreceiptcompact::RempReceiptCompact),
}
impl RempReceiptCompact {
    pub fn message_id(&self) -> &crate::ton::int256 {
        match self {
            RempReceiptCompact::TonNode_RempReceiptCompact(ref x) => &x.message_id,
        }
    }

    pub fn receipt(&self) -> &crate::ton::ton_node::RempMessageStatusCompact {
        match self {
            RempReceiptCompact::TonNode_RempReceiptCompact(ref x) => &x.receipt,
        }
    }

    pub fn timestamp(&self) -> &crate::ton::long {
        match self {
            RempReceiptCompact::TonNode_RempReceiptCompact(ref x) => &x.timestamp,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempreceiptcompact::RempReceiptCompact {
        match self {
            RempReceiptCompact::TonNode_RempReceiptCompact(x) => x,
        }
    }
}
impl Eq for RempReceiptCompact {}
impl Default for RempReceiptCompact {
    fn default() -> Self {
        RempReceiptCompact::TonNode_RempReceiptCompact(
            crate::ton::ton_node::rempreceiptcompact::RempReceiptCompact::default(),
        )
    }
}
impl crate::BoxedSerialize for RempReceiptCompact {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempReceiptCompact::TonNode_RempReceiptCompact(x) => {
                (crate::ConstructorNumber(0x411c6a07), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RempReceiptCompact {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x411c6a07)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x411c6a07) => {
                Ok(RempReceiptCompact::TonNode_RempReceiptCompact(
                    _de.read_bare::<crate::ton::ton_node::rempreceiptcompact::RempReceiptCompact>(
                    )?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `tonNode.RempReceived`\n\n```text\ntonNode.rempReceived = tonNode.RempReceived;\n```\n"]
pub enum RempReceived {
    #[default]
    TonNode_RempReceived,
}
impl Eq for RempReceived {}
impl crate::BoxedSerialize for RempReceived {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempReceived::TonNode_RempReceived => (crate::ConstructorNumber(0x32e5a18a), &()),
        }
    }
}
impl crate::BoxedDeserialize for RempReceived {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x32e5a18a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x32e5a18a) => Ok(RempReceived::TonNode_RempReceived),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempSessionInfo`\n\n```text\ntonNode.rempSessionInfo workchain:int shard:long vertical_seqno:int last_key_block_seqno:int \n        catchain_seqno:int config_hash:int256\n        members:(vector validator.groupMember) = tonNode.RempSessionInfo;\n```\n"]
pub enum RempSessionInfo {
    TonNode_RempSessionInfo(crate::ton::ton_node::rempsessioninfo::RempSessionInfo),
}
impl RempSessionInfo {
    pub fn catchain_seqno(&self) -> &crate::ton::int {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(ref x) => &x.catchain_seqno,
        }
    }

    pub fn config_hash(&self) -> &crate::ton::int256 {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(ref x) => &x.config_hash,
        }
    }

    pub fn last_key_block_seqno(&self) -> &crate::ton::int {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(ref x) => &x.last_key_block_seqno,
        }
    }

    pub fn members(
        &self,
    ) -> &crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::validator::groupmember::GroupMember,
    > {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(ref x) => &x.members,
        }
    }

    pub fn shard(&self) -> &crate::ton::long {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(ref x) => &x.shard,
        }
    }

    pub fn vertical_seqno(&self) -> &crate::ton::int {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(ref x) => &x.vertical_seqno,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempsessioninfo::RempSessionInfo {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(x) => x,
        }
    }
}
impl Eq for RempSessionInfo {}
impl Default for RempSessionInfo {
    fn default() -> Self {
        RempSessionInfo::TonNode_RempSessionInfo(
            crate::ton::ton_node::rempsessioninfo::RempSessionInfo::default(),
        )
    }
}
impl crate::BoxedSerialize for RempSessionInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempSessionInfo::TonNode_RempSessionInfo(x) => {
                (crate::ConstructorNumber(0x71c8c164), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RempSessionInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x71c8c164)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x71c8c164) => Ok(RempSessionInfo::TonNode_RempSessionInfo(
                _de.read_bare::<crate::ton::ton_node::rempsessioninfo::RempSessionInfo>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempSignedReceipt`\n\n```text\ntonNode.rempSignedReceipt receipt:bytes signature:int512 = tonNode.RempSignedReceipt;\n```\n"]
pub enum RempSignedReceipt {
    TonNode_RempSignedReceipt(crate::ton::ton_node::rempsignedreceipt::RempSignedReceipt),
}
impl RempSignedReceipt {
    pub fn receipt(&self) -> &crate::ton::bytes {
        match self {
            RempSignedReceipt::TonNode_RempSignedReceipt(ref x) => &x.receipt,
        }
    }

    pub fn signature(&self) -> &crate::ton::int512 {
        match self {
            RempSignedReceipt::TonNode_RempSignedReceipt(ref x) => &x.signature,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempsignedreceipt::RempSignedReceipt {
        match self {
            RempSignedReceipt::TonNode_RempSignedReceipt(x) => x,
        }
    }
}
impl Eq for RempSignedReceipt {}
impl Default for RempSignedReceipt {
    fn default() -> Self {
        RempSignedReceipt::TonNode_RempSignedReceipt(
            crate::ton::ton_node::rempsignedreceipt::RempSignedReceipt::default(),
        )
    }
}
impl crate::BoxedSerialize for RempSignedReceipt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempSignedReceipt::TonNode_RempSignedReceipt(x) => {
                (crate::ConstructorNumber(0xb361c8dd), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RempSignedReceipt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb361c8dd)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb361c8dd) => {
                Ok(RempSignedReceipt::TonNode_RempSignedReceipt(
                    _de.read_bare::<crate::ton::ton_node::rempsignedreceipt::RempSignedReceipt>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.RempSignedReceiptCompact`\n\n```text\ntonNode.rempSignedReceiptCompact message_id:int256 receipt:tonNode.RempMessageStatusCompact \n        timestamp:long signature:int512 = tonNode.RempSignedReceiptCompact;\n```\n"]
pub enum RempSignedReceiptCompact {
    TonNode_RempSignedReceiptCompact(
        crate::ton::ton_node::rempsignedreceiptcompact::RempSignedReceiptCompact,
    ),
}
impl RempSignedReceiptCompact {
    pub fn message_id(&self) -> &crate::ton::int256 {
        match self {
            RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(ref x) => &x.message_id,
        }
    }

    pub fn receipt(&self) -> &crate::ton::ton_node::RempMessageStatusCompact {
        match self {
            RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(ref x) => &x.receipt,
        }
    }

    pub fn signature(&self) -> &crate::ton::int512 {
        match self {
            RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(ref x) => &x.signature,
        }
    }

    pub fn timestamp(&self) -> &crate::ton::long {
        match self {
            RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(ref x) => &x.timestamp,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::rempsignedreceiptcompact::RempSignedReceiptCompact {
        match self {
            RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(x) => x,
        }
    }
}
impl Eq for RempSignedReceiptCompact {}
impl Default for RempSignedReceiptCompact {
    fn default() -> Self {
        RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(
            crate::ton::ton_node::rempsignedreceiptcompact::RempSignedReceiptCompact::default(),
        )
    }
}
impl crate::BoxedSerialize for RempSignedReceiptCompact {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RempSignedReceiptCompact::TonNode_RempSignedReceiptCompact(x) => {
                (crate::ConstructorNumber(0x9a3cabcf), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for RempSignedReceiptCompact {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x9a3cabcf)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x9a3cabcf) => Ok (RempSignedReceiptCompact :: TonNode_RempSignedReceiptCompact (_de . read_bare :: < crate :: ton :: ton_node :: rempsignedreceiptcompact :: RempSignedReceiptCompact > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rmqRecord`\n\n```text\ntonNode.rmqMessage message:bytes message_id:int256 source_key_id:int256 source_idx:int masterchain_seqno:int = tonNode.rmqRecord;\n\ntonNode.rmqMessageDigest masterchain_seqno:int messages:(vector int256) = tonNode.rmqRecord;\n```\n"]
pub enum RmqRecord {
    TonNode_RmqMessage(crate::ton::ton_node::rmqrecord::RmqMessage),
    TonNode_RmqMessageDigest(crate::ton::ton_node::rmqrecord::RmqMessageDigest),
}
impl RmqRecord {
    pub fn masterchain_seqno(&self) -> &crate::ton::int {
        match self {
            RmqRecord::TonNode_RmqMessage(ref x) => &x.masterchain_seqno,
            RmqRecord::TonNode_RmqMessageDigest(ref x) => &x.masterchain_seqno,
        }
    }

    pub fn message(&self) -> Option<&crate::ton::bytes> {
        match self {
            RmqRecord::TonNode_RmqMessage(ref x) => Some(&x.message),
            _ => None,
        }
    }

    pub fn message_id(&self) -> Option<&crate::ton::int256> {
        match self {
            RmqRecord::TonNode_RmqMessage(ref x) => Some(&x.message_id),
            _ => None,
        }
    }

    pub fn messages(&self) -> Option<&crate::ton::vector<crate::ton::Bare, crate::ton::int256>> {
        match self {
            RmqRecord::TonNode_RmqMessageDigest(ref x) => Some(&x.messages),
            _ => None,
        }
    }

    pub fn source_idx(&self) -> Option<&crate::ton::int> {
        match self {
            RmqRecord::TonNode_RmqMessage(ref x) => Some(&x.source_idx),
            _ => None,
        }
    }

    pub fn source_key_id(&self) -> Option<&crate::ton::int256> {
        match self {
            RmqRecord::TonNode_RmqMessage(ref x) => Some(&x.source_key_id),
            _ => None,
        }
    }
}
impl Eq for RmqRecord {}
impl Default for RmqRecord {
    fn default() -> Self {
        RmqRecord::TonNode_RmqMessage(crate::ton::ton_node::rmqrecord::RmqMessage::default())
    }
}
impl crate::BoxedSerialize for RmqRecord {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RmqRecord::TonNode_RmqMessage(x) => (crate::ConstructorNumber(0x0de77432), x),
            RmqRecord::TonNode_RmqMessageDigest(x) => (crate::ConstructorNumber(0x974cd134), x),
        }
    }
}
impl crate::BoxedDeserialize for RmqRecord {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0de77432), crate::ConstructorNumber(0x974cd134)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0de77432) => Ok(RmqRecord::TonNode_RmqMessage(
                _de.read_bare::<crate::ton::ton_node::rmqrecord::RmqMessage>()?,
            )),
            crate::ConstructorNumber(0x974cd134) => Ok(RmqRecord::TonNode_RmqMessageDigest(
                _de.read_bare::<crate::ton::ton_node::rmqrecord::RmqMessageDigest>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.SessionId`\n\n```text\ntonNode.sessionId workchain:int shard:long cc_seqno:int opts_hash:int256 = tonNode.SessionId;\n```\n"]
pub enum SessionId {
    TonNode_SessionId(crate::ton::ton_node::sessionid::SessionId),
}
impl SessionId {
    pub fn cc_seqno(&self) -> &crate::ton::int {
        match self {
            SessionId::TonNode_SessionId(ref x) => &x.cc_seqno,
        }
    }

    pub fn opts_hash(&self) -> &crate::ton::int256 {
        match self {
            SessionId::TonNode_SessionId(ref x) => &x.opts_hash,
        }
    }

    pub fn shard(&self) -> &crate::ton::long {
        match self {
            SessionId::TonNode_SessionId(ref x) => &x.shard,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            SessionId::TonNode_SessionId(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::sessionid::SessionId {
        match self {
            SessionId::TonNode_SessionId(x) => x,
        }
    }
}
impl Eq for SessionId {}
impl Default for SessionId {
    fn default() -> Self {
        SessionId::TonNode_SessionId(crate::ton::ton_node::sessionid::SessionId::default())
    }
}
impl crate::BoxedSerialize for SessionId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            SessionId::TonNode_SessionId(x) => (crate::ConstructorNumber(0x7a9236ba), x),
        }
    }
}
impl crate::BoxedDeserialize for SessionId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7a9236ba)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7a9236ba) => Ok(SessionId::TonNode_SessionId(
                _de.read_bare::<crate::ton::ton_node::sessionid::SessionId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.ShardPublicOverlayId`\n\n```text\ntonNode.shardPublicOverlayId workchain:int shard:long zero_state_file_hash:int256 = tonNode.ShardPublicOverlayId;\n```\n"]
pub enum ShardPublicOverlayId {
    TonNode_ShardPublicOverlayId(crate::ton::ton_node::shardpublicoverlayid::ShardPublicOverlayId),
}
impl ShardPublicOverlayId {
    pub fn shard(&self) -> &crate::ton::long {
        match self {
            ShardPublicOverlayId::TonNode_ShardPublicOverlayId(ref x) => &x.shard,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            ShardPublicOverlayId::TonNode_ShardPublicOverlayId(ref x) => &x.workchain,
        }
    }

    pub fn zero_state_file_hash(&self) -> &crate::ton::int256 {
        match self {
            ShardPublicOverlayId::TonNode_ShardPublicOverlayId(ref x) => &x.zero_state_file_hash,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::shardpublicoverlayid::ShardPublicOverlayId {
        match self {
            ShardPublicOverlayId::TonNode_ShardPublicOverlayId(x) => x,
        }
    }
}
impl Eq for ShardPublicOverlayId {}
impl Default for ShardPublicOverlayId {
    fn default() -> Self {
        ShardPublicOverlayId::TonNode_ShardPublicOverlayId(
            crate::ton::ton_node::shardpublicoverlayid::ShardPublicOverlayId::default(),
        )
    }
}
impl crate::BoxedSerialize for ShardPublicOverlayId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ShardPublicOverlayId::TonNode_ShardPublicOverlayId(x) => {
                (crate::ConstructorNumber(0x4d9ed329), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ShardPublicOverlayId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4d9ed329)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x4d9ed329) => Ok (ShardPublicOverlayId :: TonNode_ShardPublicOverlayId (_de . read_bare :: < crate :: ton :: ton_node :: shardpublicoverlayid :: ShardPublicOverlayId > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `tonNode.Success`\n\n```text\ntonNode.success = tonNode.Success;\n```\n"]
pub enum Success {
    #[default]
    TonNode_Success,
}
impl Eq for Success {}
impl crate::BoxedSerialize for Success {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Success::TonNode_Success => (crate::ConstructorNumber(0xc096244f), &()),
        }
    }
}
impl crate::BoxedDeserialize for Success {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc096244f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc096244f) => Ok(Success::TonNode_Success),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.ZeroStateIdExt`\n\n```text\ntonNode.zeroStateIdExt workchain:int root_hash:int256 file_hash:int256 = tonNode.ZeroStateIdExt;\n```\n"]
pub enum ZeroStateIdExt {
    TonNode_ZeroStateIdExt(crate::ton::ton_node::zerostateidext::ZeroStateIdExt),
}
impl ZeroStateIdExt {
    pub fn file_hash(&self) -> &crate::ton::int256 {
        match self {
            ZeroStateIdExt::TonNode_ZeroStateIdExt(ref x) => &x.file_hash,
        }
    }

    pub fn root_hash(&self) -> &crate::ton::int256 {
        match self {
            ZeroStateIdExt::TonNode_ZeroStateIdExt(ref x) => &x.root_hash,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            ZeroStateIdExt::TonNode_ZeroStateIdExt(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::ton_node::zerostateidext::ZeroStateIdExt {
        match self {
            ZeroStateIdExt::TonNode_ZeroStateIdExt(x) => x,
        }
    }
}
impl Eq for ZeroStateIdExt {}
impl Default for ZeroStateIdExt {
    fn default() -> Self {
        ZeroStateIdExt::TonNode_ZeroStateIdExt(
            crate::ton::ton_node::zerostateidext::ZeroStateIdExt::default(),
        )
    }
}
impl crate::BoxedSerialize for ZeroStateIdExt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ZeroStateIdExt::TonNode_ZeroStateIdExt(x) => (crate::ConstructorNumber(0x1d7235ae), x),
        }
    }
}
impl crate::BoxedDeserialize for ZeroStateIdExt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1d7235ae)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x1d7235ae) => Ok(ZeroStateIdExt::TonNode_ZeroStateIdExt(
                _de.read_bare::<crate::ton::ton_node::zerostateidext::ZeroStateIdExt>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod archiveinfo;
pub mod blockcandidatestatus;
pub mod blockdescription;
pub mod blockid;
pub mod blockidext;
pub mod blocksdescription;
pub mod blocksignature;
pub mod broadcast;
pub mod capabilities;
pub mod data;
pub mod datafull;
pub mod datalist;
pub mod externalmessage;
pub mod ihrmessage;
pub mod keyblocks;
pub mod newshardblock;
pub mod prepared;
pub mod preparedproof;
pub mod preparedstate;
pub mod rempcatchainmessageids;
pub mod rempcatchainrecord;
pub mod rempcombinedreceipt;
pub mod rempmessage;
pub mod rempmessagelevel;
pub mod rempmessagestatus;
pub mod rempmessagestatuscompact;
pub mod rempreceipt;
pub mod rempreceiptcompact;
pub mod rempreceived;
pub mod rempsessioninfo;
pub mod rempsignedreceipt;
pub mod rempsignedreceiptcompact;
pub mod rmqrecord;
pub mod sessionid;
pub mod shardpublicoverlayid;
pub mod success;
pub mod zerostateidext;
