use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempSessionInfo`\n\n```text\ntonNode.rempSessionInfo workchain:int shard:long vertical_seqno:int last_key_block_seqno:int \n        catchain_seqno:int config_hash:int256\n        members:(vector validator.groupMember) = tonNode.RempSessionInfo;\n```\n"]
pub struct RempSessionInfo {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub vertical_seqno: crate::ton::int,
    pub last_key_block_seqno: crate::ton::int,
    pub catchain_seqno: crate::ton::int,
    pub config_hash: crate::ton::int256,
    pub members: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::validator::groupmember::GroupMember,
    >,
}
impl Eq for RempSessionInfo {}
impl crate::BareSerialize for RempSessionInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x71c8c164)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempSessionInfo {
            workchain,
            shard,
            vertical_seqno,
            last_key_block_seqno,
            catchain_seqno,
            config_hash,
            members,
        } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_bare::<crate::ton::int>(vertical_seqno)?;
        _ser.write_bare::<crate::ton::int>(last_key_block_seqno)?;
        _ser.write_bare::<crate::ton::int>(catchain_seqno)?;
        _ser.write_bare::<crate::ton::int256>(config_hash)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validator::validator::groupmember::GroupMember,
        >>(members)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempSessionInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let vertical_seqno = _de.read_bare::<crate::ton::int>()?;
            let last_key_block_seqno = _de.read_bare::<crate::ton::int>()?;
            let catchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let config_hash = _de.read_bare::<crate::ton::int256>()?;
            let members = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validator::validator::groupmember::GroupMember,
            >>()?;
            Ok(Self {
                workchain,
                shard,
                vertical_seqno,
                last_key_block_seqno,
                catchain_seqno,
                config_hash,
                members,
            })
        }
    }
}
impl crate::IntoBoxed for RempSessionInfo {
    type Boxed = crate::ton::ton_node::RempSessionInfo;

    fn into_boxed(self) -> crate::ton::ton_node::RempSessionInfo {
        crate::ton::ton_node::RempSessionInfo::TonNode_RempSessionInfo(self)
    }
}
