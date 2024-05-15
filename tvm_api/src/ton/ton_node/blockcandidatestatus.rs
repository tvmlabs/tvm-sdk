use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.blockCandidateStatus`\n\n```text\ntonNode.blockCandidateStatus candidate_id:int256 deliveries_signature:bytes approvals_signature:bytes rejections_signature:bytes merges_cnt:int created_timestamp:long = tonNode.BlockCandidateStatus;\n```\n"]
pub struct BlockCandidateStatus {
    pub candidate_id: crate::ton::int256,
    pub deliveries_signature: crate::ton::bytes,
    pub approvals_signature: crate::ton::bytes,
    pub rejections_signature: crate::ton::bytes,
    pub merges_cnt: crate::ton::int,
    pub created_timestamp: crate::ton::long,
}
impl Eq for BlockCandidateStatus {}
impl crate::BareSerialize for BlockCandidateStatus {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbc31fb7c)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockCandidateStatus {
            candidate_id,
            deliveries_signature,
            approvals_signature,
            rejections_signature,
            merges_cnt,
            created_timestamp,
        } = self;
        _ser.write_bare::<crate::ton::int256>(candidate_id)?;
        _ser.write_bare::<crate::ton::bytes>(deliveries_signature)?;
        _ser.write_bare::<crate::ton::bytes>(approvals_signature)?;
        _ser.write_bare::<crate::ton::bytes>(rejections_signature)?;
        _ser.write_bare::<crate::ton::int>(merges_cnt)?;
        _ser.write_bare::<crate::ton::long>(created_timestamp)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockCandidateStatus {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let candidate_id = _de.read_bare::<crate::ton::int256>()?;
            let deliveries_signature = _de.read_bare::<crate::ton::bytes>()?;
            let approvals_signature = _de.read_bare::<crate::ton::bytes>()?;
            let rejections_signature = _de.read_bare::<crate::ton::bytes>()?;
            let merges_cnt = _de.read_bare::<crate::ton::int>()?;
            let created_timestamp = _de.read_bare::<crate::ton::long>()?;
            Ok(Self {
                candidate_id,
                deliveries_signature,
                approvals_signature,
                rejections_signature,
                merges_cnt,
                created_timestamp,
            })
        }
    }
}
impl crate::IntoBoxed for BlockCandidateStatus {
    type Boxed = crate::ton::ton_node::BlockCandidateStatus;
    fn into_boxed(self) -> crate::ton::ton_node::BlockCandidateStatus {
        crate::ton::ton_node::BlockCandidateStatus::TonNode_BlockCandidateStatus(self)
    }
}
