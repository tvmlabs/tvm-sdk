use crate::Cell;
use crate::LevelMask;
use crate::UInt256;
use crate::fail;

#[derive(Clone)]
pub struct VirtualCell {
    offset: u8,
    pub(crate) wrapped: Cell,
}

impl VirtualCell {
    pub fn with_cell_and_offset(inner: Cell, offset: u8) -> Self {
        VirtualCell { offset, wrapped: inner }
    }

    pub(crate) fn raw_data(&self) -> crate::Result<&[u8]> {
        fail!("Virtual cell doesn't support raw_data()");
    }

    pub(crate) fn references_count(&self) -> usize {
        self.wrapped.references_count()
    }

    pub(crate) fn reference(&self, index: usize) -> crate::Result<Cell> {
        Ok(self.wrapped.reference(index)?.virtualize(self.offset))
    }

    pub(crate) fn level_mask(&self) -> LevelMask {
        self.wrapped.level_mask().virtualize(self.offset)
    }

    pub(crate) fn hash(&self, index: usize) -> UInt256 {
        self.wrapped.hash(self.level_mask().calc_virtual_hash_index(index, self.offset))
    }

    pub(crate) fn depth(&self, index: usize) -> u16 {
        self.wrapped.depth(self.level_mask().calc_virtual_hash_index(index, self.offset))
    }

    pub(crate) fn virtualization(&self) -> u8 {
        self.offset
    }
}
