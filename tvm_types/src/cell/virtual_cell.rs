use crate::{fail, Cell, CellImpl, CellType, LevelMask, UInt256};

#[derive(Clone)]
pub struct VirtualCell {
    offset: u8,
    cell: Cell,
}

impl VirtualCell {
    pub fn with_cell_and_offset(cell: Cell, offset: u8) -> Self {
        VirtualCell { offset, cell }
    }
}

impl CellImpl for VirtualCell {
    fn data(&self) -> &[u8] {
        self.cell.data()
    }

    fn raw_data(&self) -> crate::Result<&[u8]> {
        fail!("Virtual cell doesn't support raw_data()");
    }

    fn bit_length(&self) -> usize {
        self.cell.bit_length()
    }

    fn references_count(&self) -> usize {
        self.cell.references_count()
    }

    fn reference(&self, index: usize) -> crate::Result<Cell> {
        Ok(self.cell.reference(index)?.virtualize(self.offset))
    }

    fn cell_type(&self) -> CellType {
        self.cell.cell_type()
    }

    fn level_mask(&self) -> LevelMask {
        self.cell.level_mask().virtualize(self.offset)
    }

    fn hash(&self, index: usize) -> UInt256 {
        self.cell.hash(self.level_mask().calc_virtual_hash_index(index, self.offset))
    }

    fn depth(&self, index: usize) -> u16 {
        self.cell.depth(self.level_mask().calc_virtual_hash_index(index, self.offset))
    }

    fn store_hashes(&self) -> bool {
        self.cell.store_hashes()
    }

    fn tree_bits_count(&self) -> u64 {
        self.cell.tree_bits_count()
    }

    fn tree_cell_count(&self) -> u64 {
        self.cell.tree_cell_count()
    }

    fn virtualization(&self) -> u8 {
        self.offset
    }
}
