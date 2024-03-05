use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `ton.blockId`\n\n```text\nton.blockId root_cell_hash:int256 file_hash:int256 = ton.BlockId;\n```\n"]
pub struct BlockId {
    pub root_cell_hash: crate::ton::int256,
    pub file_hash: crate::ton::int256,
}
impl Eq for BlockId {}
impl crate::BareSerialize for BlockId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc50b6e70)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockId { root_cell_hash, file_hash } = self;
        _ser.write_bare::<crate::ton::int256>(root_cell_hash)?;
        _ser.write_bare::<crate::ton::int256>(file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let root_cell_hash = _de.read_bare::<crate::ton::int256>()?;
            let file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { root_cell_hash, file_hash })
        }
    }
}
impl crate::IntoBoxed for BlockId {
    type Boxed = crate::ton::ton::BlockId;

    fn into_boxed(self) -> crate::ton::ton::BlockId {
        crate::ton::ton::BlockId::Ton_BlockId(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `ton.blockIdApprove`\n\n```text\nton.blockIdApprove root_cell_hash:int256 file_hash:int256 = ton.BlockId;\n```\n"]
pub struct BlockIdApprove {
    pub root_cell_hash: crate::ton::int256,
    pub file_hash: crate::ton::int256,
}
impl Eq for BlockIdApprove {}
impl crate::BareSerialize for BlockIdApprove {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2dd44a49)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockIdApprove { root_cell_hash, file_hash } = self;
        _ser.write_bare::<crate::ton::int256>(root_cell_hash)?;
        _ser.write_bare::<crate::ton::int256>(file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockIdApprove {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let root_cell_hash = _de.read_bare::<crate::ton::int256>()?;
            let file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { root_cell_hash, file_hash })
        }
    }
}
impl crate::IntoBoxed for BlockIdApprove {
    type Boxed = crate::ton::ton::BlockId;

    fn into_boxed(self) -> crate::ton::ton::BlockId {
        crate::ton::ton::BlockId::Ton_BlockIdApprove(self)
    }
}
