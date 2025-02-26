use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validator.group`\n\n```text\nvalidator.group workchain:int shard:long catchain_seqno:int config_hash:int256 members:(vector validator.groupMember) = validator.Group;\n```\n"]
pub struct Group {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub catchain_seqno: crate::ton::int,
    pub config_hash: crate::ton::int256,
    pub members: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::validator::groupmember::GroupMember,
    >,
}
impl Eq for Group {}
impl crate::BareSerialize for Group {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf8d87ea1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Group { workchain, shard, catchain_seqno, config_hash, members } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_bare::<crate::ton::int>(catchain_seqno)?;
        _ser.write_bare::<crate::ton::int256>(config_hash)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validator::validator::groupmember::GroupMember,
        >>(members)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Group {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let catchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let config_hash = _de.read_bare::<crate::ton::int256>()?;
            let members = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validator::validator::groupmember::GroupMember,
            >>()?;
            Ok(Self { workchain, shard, catchain_seqno, config_hash, members })
        }
    }
}
impl crate::IntoBoxed for Group {
    type Boxed = crate::ton::validator::Group;

    fn into_boxed(self) -> crate::ton::validator::Group {
        crate::ton::validator::Group::Validator_Group(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validator.groupEx`\n\n```text\nvalidator.groupEx workchain:int shard:long vertical_seqno:int catchain_seqno:int config_hash:int256 members:(vector validator.groupMember) = validator.Group;\n```\n"]
pub struct GroupEx {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub vertical_seqno: crate::ton::int,
    pub catchain_seqno: crate::ton::int,
    pub config_hash: crate::ton::int256,
    pub members: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::validator::groupmember::GroupMember,
    >,
}
impl Eq for GroupEx {}
impl crate::BareSerialize for GroupEx {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1c924dfe)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GroupEx { workchain, shard, vertical_seqno, catchain_seqno, config_hash, members } =
            self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_bare::<crate::ton::int>(vertical_seqno)?;
        _ser.write_bare::<crate::ton::int>(catchain_seqno)?;
        _ser.write_bare::<crate::ton::int256>(config_hash)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validator::validator::groupmember::GroupMember,
        >>(members)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GroupEx {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let vertical_seqno = _de.read_bare::<crate::ton::int>()?;
            let catchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let config_hash = _de.read_bare::<crate::ton::int256>()?;
            let members = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validator::validator::groupmember::GroupMember,
            >>()?;
            Ok(Self { workchain, shard, vertical_seqno, catchain_seqno, config_hash, members })
        }
    }
}
impl crate::IntoBoxed for GroupEx {
    type Boxed = crate::ton::validator::Group;

    fn into_boxed(self) -> crate::ton::validator::Group {
        crate::ton::validator::Group::Validator_GroupEx(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validator.groupNew`\n\n```text\nvalidator.groupNew workchain:int shard:long vertical_seqno:int last_key_block_seqno:int catchain_seqno:int config_hash:int256 members:(vector validator.groupMember) = validator.Group;\n```\n"]
pub struct GroupNew {
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
impl Eq for GroupNew {}
impl crate::BareSerialize for GroupNew {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9843a14d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GroupNew {
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
impl crate::BareDeserialize for GroupNew {
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
impl crate::IntoBoxed for GroupNew {
    type Boxed = crate::ton::validator::Group;

    fn into_boxed(self) -> crate::ton::validator::Group {
        crate::ton::validator::Group::Validator_GroupNew(self)
    }
}
