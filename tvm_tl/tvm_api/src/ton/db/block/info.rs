use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.block.archivedInfo`\n\n```text\ndb.block.archivedInfo id:tonNode.blockIdExt flags:# next:flags.0?tonNode.blockIdExt = db.block.Info;\n```\n"]
pub struct ArchivedInfo {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub next: Option<crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for ArchivedInfo {}
impl crate::BareSerialize for ArchivedInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x205f7a51)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ArchivedInfo { id, next } = self;
        let mut _flags = 0u32;
        if next.is_some() {
            _flags |= 1 << 0u32;
        }
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::Flags>(&_flags)?;
        if let Some(inner) = next {
            _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for ArchivedInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let flags = _de.read_bare::<crate::ton::Flags>()?;
            let next = if flags & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?)
            } else {
                None
            };
            Ok(Self { id, next })
        }
    }
}
impl crate::IntoBoxed for ArchivedInfo {
    type Boxed = crate::ton::db::block::Info;

    fn into_boxed(self) -> crate::ton::db::block::Info {
        crate::ton::db::block::Info::Db_Block_ArchivedInfo(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.block.info`\n\n```text\ndb.block.info#4ac6e727 id:tonNode.blockIdExt flags:# prev_left:flags.1?tonNode.blockIdExt\n                                            prev_right:flags.2?tonNode.blockIdExt\n                                            next_left:flags.3?tonNode.blockIdExt\n                                            next_right:flags.4?tonNode.blockIdExt\n                                            lt:flags.13?long \n                                            ts:flags.14?int\n                                            state:flags.17?int256 \n                                            masterchain_ref_seqno:flags.23?int = db.block.Info;\n```\n"]
pub struct Info {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub prev_left: Option<crate::ton::ton_node::blockidext::BlockIdExt>,
    pub prev_right: Option<crate::ton::ton_node::blockidext::BlockIdExt>,
    pub next_left: Option<crate::ton::ton_node::blockidext::BlockIdExt>,
    pub next_right: Option<crate::ton::ton_node::blockidext::BlockIdExt>,
    pub lt: Option<crate::ton::long>,
    pub ts: Option<crate::ton::int>,
    pub state: Option<crate::ton::int256>,
    pub masterchain_ref_seqno: Option<crate::ton::int>,
}
impl Eq for Info {}
impl crate::BareSerialize for Info {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4ac6e727)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Info {
            id,
            prev_left,
            prev_right,
            next_left,
            next_right,
            lt,
            ts,
            state,
            masterchain_ref_seqno,
        } = self;
        let mut _flags = 0u32;
        if prev_left.is_some() {
            _flags |= 1 << 1u32;
        }
        if prev_right.is_some() {
            _flags |= 1 << 2u32;
        }
        if next_left.is_some() {
            _flags |= 1 << 3u32;
        }
        if next_right.is_some() {
            _flags |= 1 << 4u32;
        }
        if lt.is_some() {
            _flags |= 1 << 13u32;
        }
        if ts.is_some() {
            _flags |= 1 << 14u32;
        }
        if state.is_some() {
            _flags |= 1 << 17u32;
        }
        if masterchain_ref_seqno.is_some() {
            _flags |= 1 << 23u32;
        }
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::Flags>(&_flags)?;
        if let Some(inner) = prev_left {
            _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(inner)?;
        }
        if let Some(inner) = prev_right {
            _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(inner)?;
        }
        if let Some(inner) = next_left {
            _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(inner)?;
        }
        if let Some(inner) = next_right {
            _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(inner)?;
        }
        if let Some(inner) = lt {
            _ser.write_bare::<crate::ton::long>(inner)?;
        }
        if let Some(inner) = ts {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = state {
            _ser.write_bare::<crate::ton::int256>(inner)?;
        }
        if let Some(inner) = masterchain_ref_seqno {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for Info {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let flags = _de.read_bare::<crate::ton::Flags>()?;
            let prev_left = if flags & (1 << 1u32) != 0 {
                Some(_de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?)
            } else {
                None
            };
            let prev_right = if flags & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?)
            } else {
                None
            };
            let next_left = if flags & (1 << 3u32) != 0 {
                Some(_de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?)
            } else {
                None
            };
            let next_right = if flags & (1 << 4u32) != 0 {
                Some(_de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?)
            } else {
                None
            };
            let lt = if flags & (1 << 13u32) != 0 {
                Some(_de.read_bare::<crate::ton::long>()?)
            } else {
                None
            };
            let ts = if flags & (1 << 14u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let state = if flags & (1 << 17u32) != 0 {
                Some(_de.read_bare::<crate::ton::int256>()?)
            } else {
                None
            };
            let masterchain_ref_seqno = if flags & (1 << 23u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            Ok(Self {
                id,
                prev_left,
                prev_right,
                next_left,
                next_right,
                lt,
                ts,
                state,
                masterchain_ref_seqno,
            })
        }
    }
}
impl crate::IntoBoxed for Info {
    type Boxed = crate::ton::db::block::Info;

    fn into_boxed(self) -> crate::ton::db::block::Info {
        crate::ton::db::block::Info::Db_Block_Info(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.block.packedInfo`\n\n```text\ndb.block.packedInfo id:tonNode.blockIdExt unixtime:int offset:long = db.block.Info;\n```\n"]
pub struct PackedInfo {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub unixtime: crate::ton::int,
    pub offset: crate::ton::long,
}
impl Eq for PackedInfo {}
impl crate::BareSerialize for PackedInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x46bb9192)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PackedInfo { id, unixtime, offset } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(unixtime)?;
        _ser.write_bare::<crate::ton::long>(offset)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PackedInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let unixtime = _de.read_bare::<crate::ton::int>()?;
            let offset = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { id, unixtime, offset })
        }
    }
}
impl crate::IntoBoxed for PackedInfo {
    type Boxed = crate::ton::db::block::Info;

    fn into_boxed(self) -> crate::ton::db::block::Info {
        crate::ton::db::block::Info::Db_Block_PackedInfo(self)
    }
}
