use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `ton.BlockId`\n\n```text\nton.blockId root_cell_hash:int256 file_hash:int256 = ton.BlockId;\n\nton.blockIdApprove root_cell_hash:int256 file_hash:int256 = ton.BlockId;\n```\n"]
pub enum BlockId {
    Ton_BlockId(crate::ton::ton::blockid::BlockId),
    Ton_BlockIdApprove(crate::ton::ton::blockid::BlockIdApprove),
}
impl BlockId {
    pub fn file_hash(&self) -> &crate::ton::int256 {
        match self {
            BlockId::Ton_BlockId(ref x) => &x.file_hash,
            BlockId::Ton_BlockIdApprove(ref x) => &x.file_hash,
        }
    }

    pub fn root_cell_hash(&self) -> &crate::ton::int256 {
        match self {
            BlockId::Ton_BlockId(ref x) => &x.root_cell_hash,
            BlockId::Ton_BlockIdApprove(ref x) => &x.root_cell_hash,
        }
    }
}
impl Eq for BlockId {}
impl Default for BlockId {
    fn default() -> Self {
        BlockId::Ton_BlockId(crate::ton::ton::blockid::BlockId::default())
    }
}
impl crate::BoxedSerialize for BlockId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockId::Ton_BlockId(x) => (crate::ConstructorNumber(0xc50b6e70), x),
            BlockId::Ton_BlockIdApprove(x) => (crate::ConstructorNumber(0x2dd44a49), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc50b6e70), crate::ConstructorNumber(0x2dd44a49)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc50b6e70) => {
                Ok(BlockId::Ton_BlockId(_de.read_bare::<crate::ton::ton::blockid::BlockId>()?))
            }
            crate::ConstructorNumber(0x2dd44a49) => Ok(BlockId::Ton_BlockIdApprove(
                _de.read_bare::<crate::ton::ton::blockid::BlockIdApprove>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod blockid;
