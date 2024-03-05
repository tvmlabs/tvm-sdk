use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.block.Info`\n\n```text\ndb.block.archivedInfo id:tonNode.blockIdExt flags:# next:flags.0?tonNode.blockIdExt = db.block.Info;\n\ndb.block.info#4ac6e727 id:tonNode.blockIdExt flags:# prev_left:flags.1?tonNode.blockIdExt\n                                            prev_right:flags.2?tonNode.blockIdExt\n                                            next_left:flags.3?tonNode.blockIdExt\n                                            next_right:flags.4?tonNode.blockIdExt\n                                            lt:flags.13?long \n                                            ts:flags.14?int\n                                            state:flags.17?int256 \n                                            masterchain_ref_seqno:flags.23?int = db.block.Info;\n\ndb.block.packedInfo id:tonNode.blockIdExt unixtime:int offset:long = db.block.Info;\n```\n"]
pub enum Info {
    Db_Block_ArchivedInfo(crate::ton::db::block::info::ArchivedInfo),
    Db_Block_Info(crate::ton::db::block::info::Info),
    Db_Block_PackedInfo(crate::ton::db::block::info::PackedInfo),
}
impl Info {
    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Info::Db_Block_ArchivedInfo(ref x) => &x.id,
            Info::Db_Block_Info(ref x) => &x.id,
            Info::Db_Block_PackedInfo(ref x) => &x.id,
        }
    }

    pub fn lt(&self) -> Option<&crate::ton::long> {
        match self {
            Info::Db_Block_Info(ref x) => x.lt.as_ref(),
            _ => None,
        }
    }

    pub fn masterchain_ref_seqno(&self) -> Option<&crate::ton::int> {
        match self {
            Info::Db_Block_Info(ref x) => x.masterchain_ref_seqno.as_ref(),
            _ => None,
        }
    }

    pub fn next(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Info::Db_Block_ArchivedInfo(ref x) => x.next.as_ref(),
            _ => None,
        }
    }

    pub fn next_left(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Info::Db_Block_Info(ref x) => x.next_left.as_ref(),
            _ => None,
        }
    }

    pub fn next_right(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Info::Db_Block_Info(ref x) => x.next_right.as_ref(),
            _ => None,
        }
    }

    pub fn offset(&self) -> Option<&crate::ton::long> {
        match self {
            Info::Db_Block_PackedInfo(ref x) => Some(&x.offset),
            _ => None,
        }
    }

    pub fn prev_left(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Info::Db_Block_Info(ref x) => x.prev_left.as_ref(),
            _ => None,
        }
    }

    pub fn prev_right(&self) -> Option<&crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Info::Db_Block_Info(ref x) => x.prev_right.as_ref(),
            _ => None,
        }
    }

    pub fn state(&self) -> Option<&crate::ton::int256> {
        match self {
            Info::Db_Block_Info(ref x) => x.state.as_ref(),
            _ => None,
        }
    }

    pub fn ts(&self) -> Option<&crate::ton::int> {
        match self {
            Info::Db_Block_Info(ref x) => x.ts.as_ref(),
            _ => None,
        }
    }

    pub fn unixtime(&self) -> Option<&crate::ton::int> {
        match self {
            Info::Db_Block_PackedInfo(ref x) => Some(&x.unixtime),
            _ => None,
        }
    }
}
impl Eq for Info {}
impl Default for Info {
    fn default() -> Self {
        Info::Db_Block_ArchivedInfo(crate::ton::db::block::info::ArchivedInfo::default())
    }
}
impl crate::BoxedSerialize for Info {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Info::Db_Block_ArchivedInfo(x) => (crate::ConstructorNumber(0x205f7a51), x),
            Info::Db_Block_Info(x) => (crate::ConstructorNumber(0x4ac6e727), x),
            Info::Db_Block_PackedInfo(x) => (crate::ConstructorNumber(0x46bb9192), x),
        }
    }
}
impl crate::BoxedDeserialize for Info {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x205f7a51),
            crate::ConstructorNumber(0x4ac6e727),
            crate::ConstructorNumber(0x46bb9192),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x205f7a51) => Ok(Info::Db_Block_ArchivedInfo(
                _de.read_bare::<crate::ton::db::block::info::ArchivedInfo>()?,
            )),
            crate::ConstructorNumber(0x4ac6e727) => {
                Ok(Info::Db_Block_Info(_de.read_bare::<crate::ton::db::block::info::Info>()?))
            }
            crate::ConstructorNumber(0x46bb9192) => Ok(Info::Db_Block_PackedInfo(
                _de.read_bare::<crate::ton::db::block::info::PackedInfo>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod info;
