use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validator.Group`\n\n```text\nvalidator.group workchain:int shard:long catchain_seqno:int config_hash:int256 members:(vector validator.groupMember) = validator.Group;\n\nvalidator.groupEx workchain:int shard:long vertical_seqno:int catchain_seqno:int config_hash:int256 members:(vector validator.groupMember) = validator.Group;\n\nvalidator.groupNew workchain:int shard:long vertical_seqno:int last_key_block_seqno:int catchain_seqno:int config_hash:int256 members:(vector validator.groupMember) = validator.Group;\n```\n"]
pub enum Group {
    Validator_Group(crate::ton::validator::group::Group),
    Validator_GroupEx(crate::ton::validator::group::GroupEx),
    Validator_GroupNew(crate::ton::validator::group::GroupNew),
}
impl Group {
    pub fn catchain_seqno(&self) -> &crate::ton::int {
        match self {
            Group::Validator_Group(ref x) => &x.catchain_seqno,
            Group::Validator_GroupEx(ref x) => &x.catchain_seqno,
            Group::Validator_GroupNew(ref x) => &x.catchain_seqno,
        }
    }

    pub fn config_hash(&self) -> &crate::ton::int256 {
        match self {
            Group::Validator_Group(ref x) => &x.config_hash,
            Group::Validator_GroupEx(ref x) => &x.config_hash,
            Group::Validator_GroupNew(ref x) => &x.config_hash,
        }
    }

    pub fn last_key_block_seqno(&self) -> Option<&crate::ton::int> {
        match self {
            Group::Validator_GroupNew(ref x) => Some(&x.last_key_block_seqno),
            _ => None,
        }
    }

    pub fn members(
        &self,
    ) -> &crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::validator::groupmember::GroupMember,
    > {
        match self {
            Group::Validator_Group(ref x) => &x.members,
            Group::Validator_GroupEx(ref x) => &x.members,
            Group::Validator_GroupNew(ref x) => &x.members,
        }
    }

    pub fn shard(&self) -> &crate::ton::long {
        match self {
            Group::Validator_Group(ref x) => &x.shard,
            Group::Validator_GroupEx(ref x) => &x.shard,
            Group::Validator_GroupNew(ref x) => &x.shard,
        }
    }

    pub fn vertical_seqno(&self) -> Option<&crate::ton::int> {
        match self {
            Group::Validator_GroupEx(ref x) => Some(&x.vertical_seqno),
            Group::Validator_GroupNew(ref x) => Some(&x.vertical_seqno),
            _ => None,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int {
        match self {
            Group::Validator_Group(ref x) => &x.workchain,
            Group::Validator_GroupEx(ref x) => &x.workchain,
            Group::Validator_GroupNew(ref x) => &x.workchain,
        }
    }
}
impl Eq for Group {}
impl Default for Group {
    fn default() -> Self {
        Group::Validator_Group(crate::ton::validator::group::Group::default())
    }
}
impl crate::BoxedSerialize for Group {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Group::Validator_Group(x) => (crate::ConstructorNumber(0xf8d87ea1), x),
            Group::Validator_GroupEx(x) => (crate::ConstructorNumber(0x1c924dfe), x),
            Group::Validator_GroupNew(x) => (crate::ConstructorNumber(0x9843a14d), x),
        }
    }
}
impl crate::BoxedDeserialize for Group {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xf8d87ea1),
            crate::ConstructorNumber(0x1c924dfe),
            crate::ConstructorNumber(0x9843a14d),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf8d87ea1) => {
                Ok(Group::Validator_Group(_de.read_bare::<crate::ton::validator::group::Group>()?))
            }
            crate::ConstructorNumber(0x1c924dfe) => Ok(Group::Validator_GroupEx(
                _de.read_bare::<crate::ton::validator::group::GroupEx>()?,
            )),
            crate::ConstructorNumber(0x9843a14d) => Ok(Group::Validator_GroupNew(
                _de.read_bare::<crate::ton::validator::group::GroupNew>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod config;
pub mod group;
