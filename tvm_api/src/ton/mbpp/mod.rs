use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `mbpp.NewMcBlock`\n\n```text\nmbpp.newMcBlockNone = mbpp.NewMcBlock;\n\nmbpp.newMcBlockSome block:tonNode.blockBroadcast = mbpp.NewMcBlock;\n```\n"]
pub enum NewMcBlock {
    #[default]
    Mbpp_NewMcBlockNone,
    Mbpp_NewMcBlockSome(crate::ton::mbpp::newmcblock::NewMcBlockSome),
}
impl NewMcBlock {
    pub fn block(&self) -> Option<&crate::ton::ton_node::broadcast::BlockBroadcast> {
        match self {
            NewMcBlock::Mbpp_NewMcBlockSome(ref x) => Some(&x.block),
            _ => None,
        }
    }
}
impl Eq for NewMcBlock {}
impl crate::BoxedSerialize for NewMcBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            NewMcBlock::Mbpp_NewMcBlockNone => (crate::ConstructorNumber(0x8c046fa1), &()),
            NewMcBlock::Mbpp_NewMcBlockSome(x) => (crate::ConstructorNumber(0xde670c49), x),
        }
    }
}
impl crate::BoxedDeserialize for NewMcBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8c046fa1), crate::ConstructorNumber(0xde670c49)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8c046fa1) => Ok(NewMcBlock::Mbpp_NewMcBlockNone),
            crate::ConstructorNumber(0xde670c49) => Ok(NewMcBlock::Mbpp_NewMcBlockSome(
                _de.read_bare::<crate::ton::mbpp::newmcblock::NewMcBlockSome>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::mbpp::newmcblock::NewMcBlockSome> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x8c046fa1), &()),
            Some(ref x) => (crate::ConstructorNumber(0xde670c49), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::mbpp::newmcblock::NewMcBlockSome> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8c046fa1), crate::ConstructorNumber(0xde670c49)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8c046fa1) => Ok(None),
            crate::ConstructorNumber(0xde670c49) => {
                Ok(Some(_de.read_bare::<crate::ton::mbpp::newmcblock::NewMcBlockSome>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `mbpp.NewShardBlock`\n\n```text\nmbpp.newShardBlock id:tonNode.blockIdExt cc_seqno:int tbd:bytes block:bytes = mbpp.NewShardBlock;\n```\n"]
pub enum NewShardBlock {
    Mbpp_NewShardBlock(crate::ton::mbpp::newshardblock::NewShardBlock),
}
impl NewShardBlock {
    pub fn block(&self) -> &crate::ton::bytes {
        match self {
            NewShardBlock::Mbpp_NewShardBlock(ref x) => &x.block,
        }
    }

    pub fn cc_seqno(&self) -> &crate::ton::int {
        match self {
            NewShardBlock::Mbpp_NewShardBlock(ref x) => &x.cc_seqno,
        }
    }

    pub fn id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            NewShardBlock::Mbpp_NewShardBlock(ref x) => &x.id,
        }
    }

    pub fn tbd(&self) -> &crate::ton::bytes {
        match self {
            NewShardBlock::Mbpp_NewShardBlock(ref x) => &x.tbd,
        }
    }

    pub fn only(self) -> crate::ton::mbpp::newshardblock::NewShardBlock {
        match self {
            NewShardBlock::Mbpp_NewShardBlock(x) => x,
        }
    }
}
impl Eq for NewShardBlock {}
impl Default for NewShardBlock {
    fn default() -> Self {
        NewShardBlock::Mbpp_NewShardBlock(crate::ton::mbpp::newshardblock::NewShardBlock::default())
    }
}
impl crate::BoxedSerialize for NewShardBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            NewShardBlock::Mbpp_NewShardBlock(x) => (crate::ConstructorNumber(0x54d61ee6), x),
        }
    }
}
impl crate::BoxedDeserialize for NewShardBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x54d61ee6)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x54d61ee6) => Ok(NewShardBlock::Mbpp_NewShardBlock(
                _de.read_bare::<crate::ton::mbpp::newshardblock::NewShardBlock>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod newmcblock;
pub mod newshardblock;
