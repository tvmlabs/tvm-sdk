use crate::cell::EXTERNAL_CELL_MAX_SIZE;
use crate::cell::cell_data::CellData;
use crate::fail;
use crate::{Cell, CellImpl, CellType, LevelMask, MAX_LEVEL, UInt256, cell};
use crate::{ExceptionCode, error};
use std::io::Write;
use std::sync::Arc;

impl CellImpl for DataCell {
    fn data(&self) -> &[u8] {
        self.cell_data.data()
    }

    fn raw_data(&self) -> crate::Result<&[u8]> {
        Ok(self.cell_data.raw_data())
    }

    fn bit_length(&self) -> usize {
        self.cell_data.bit_length()
    }

    fn references_count(&self) -> usize {
        self.references.len()
    }

    fn reference(&self, index: usize) -> crate::Result<Cell> {
        self.references.get(index).cloned().ok_or_else(|| error!(ExceptionCode::CellUnderflow))
    }

    fn cell_type(&self) -> CellType {
        self.cell_data.cell_type()
    }

    fn level_mask(&self) -> LevelMask {
        self.cell_data.level_mask()
    }

    fn hash(&self, index: usize) -> UInt256 {
        self.cell_data().hash(index)
    }

    fn depth(&self, index: usize) -> u16 {
        self.cell_data().depth(index)
    }

    fn store_hashes(&self) -> bool {
        self.cell_data().store_hashes()
    }

    fn tree_bits_count(&self) -> u64 {
        self.tree_bits_count
    }

    fn tree_cell_count(&self) -> u64 {
        self.tree_cell_count
    }

    fn to_external(&self) -> crate::Result<Arc<dyn CellImpl>> {
        if self.cell_type() != CellType::Ordinary && self.cell_type() != CellType::Big {
            fail!("Only ordinary and big cells can be converted to external")
        }

        let mut data = [0u8; EXTERNAL_CELL_MAX_SIZE];
        let mut cursor = std::io::Cursor::new(data.as_mut());
        cursor.write_all(&[u8::from(CellType::External)])?;
        cursor.write_all(self.hash(MAX_LEVEL).as_slice())?;
        cursor.write_all(&self.depth(MAX_LEVEL).to_be_bytes())?;

        let tree_cells_count = self.tree_cell_count.to_be_bytes();
        let tree_cells_count_len = 8 - self.tree_cell_count.leading_zeros() as u8 / 8;
        let tree_bits_count = self.tree_bits_count.to_be_bytes();
        let tree_bits_count_len = 8 - self.tree_bits_count.leading_zeros() as u8 / 8;
        cursor.write_all(&[(tree_cells_count_len << 4) | tree_bits_count_len])?;
        cursor.write_all(&tree_cells_count[8 - tree_cells_count_len as usize..])?;
        cursor.write_all(&tree_bits_count[8 - tree_bits_count_len as usize..])?;
        cursor.write_all(&[0x80])?;
        let size = cursor.position() as usize;

        let cell = DataCell::with_params(
            vec![],
            &cursor.into_inner()[..size],
            CellType::External,
            0,
            None,
            None,
            None,
        )?;

        Ok(Arc::new(cell) as Arc<dyn CellImpl>)
    }
}

#[derive(Clone, Debug)]
pub struct DataCell {
    cell_data: CellData,
    references: Vec<Cell>, // TODO make array - you already know cells refs count, or may be vector
    tree_bits_count: u64,
    tree_cell_count: u64,
}

impl Default for DataCell {
    fn default() -> Self {
        Self::new()
    }
}

impl DataCell {
    pub fn new() -> Self {
        Self::with_refs_and_data(vec![], &[0x80]).unwrap()
    }

    pub fn with_refs_and_data(
        references: Vec<Cell>,
        data: &[u8], // with completion tag (for big cell - without)!
    ) -> crate::Result<DataCell> {
        Self::with_params(references, data, CellType::Ordinary, 0, None, None, None)
    }

    pub fn with_params(
        references: Vec<Cell>,
        data: &[u8], // with completion tag (for big cell - without)!
        cell_type: CellType,
        level_mask: u8,
        max_depth: Option<u16>,
        hashes: Option<[UInt256; 4]>,
        depths: Option<[u16; 4]>,
    ) -> crate::Result<DataCell> {
        assert_eq!(hashes.is_some(), depths.is_some());
        let store_hashes = hashes.is_some();
        let cell_data = CellData::with_params(
            cell_type,
            data,
            level_mask,
            references.len() as u8,
            store_hashes,
            hashes,
            depths,
        )?;
        Self::construct_cell(cell_data, references, max_depth, true)
    }

    pub fn with_external_data(
        references: Vec<Cell>,
        buffer: &Arc<Vec<u8>>,
        offset: usize,
        max_depth: Option<u16>,
        verify: bool,
    ) -> crate::Result<DataCell> {
        let cell_data = CellData::with_external_data(buffer, offset)?;
        Self::construct_cell(cell_data, references, max_depth, verify)
    }

    pub fn with_raw_data(
        references: Vec<Cell>,
        data: Vec<u8>,
        max_depth: Option<u16>,
    ) -> crate::Result<DataCell> {
        let cell_data = CellData::with_raw_data(data)?;
        Self::construct_cell(cell_data, references, max_depth, true)
    }

    fn construct_cell(
        cell_data: CellData,
        references: Vec<Cell>,
        max_depth: Option<u16>,
        verify: bool,
    ) -> crate::Result<DataCell> {
        const MAX_56_BITS: u64 = 0x00FF_FFFF_FFFF_FFFFu64;
        let mut tree_bits_count = cell_data.bit_length() as u64;
        let mut tree_cell_count = 1u64;
        for reference in &references {
            tree_bits_count = tree_bits_count.saturating_add(reference.tree_bits_count());
            tree_cell_count = tree_cell_count.saturating_add(reference.tree_cell_count());
        }
        if tree_bits_count > MAX_56_BITS {
            tree_bits_count = MAX_56_BITS;
        }
        if tree_cell_count > MAX_56_BITS {
            tree_cell_count = MAX_56_BITS;
        }
        let mut cell = DataCell { cell_data, references, tree_bits_count, tree_cell_count };
        cell.finalize(verify, max_depth)?;
        Ok(cell)
    }

    fn finalize(&mut self, force: bool, max_depth: Option<u16>) -> crate::Result<()> {
        if !force && self.store_hashes() {
            return Ok(());
        }

        let raw_data = self.cell_data.raw_data();

        cell::verify_raw_data(raw_data)?;

        let cell_type = cell::cell_type(raw_data);
        if cell_type == CellType::External {
            (self.tree_cell_count, self.tree_bits_count) = cell::tree_cell_bits_counts(raw_data)?;
            return Ok(());
        }

        // Check level

        let mut children_mask = LevelMask::with_mask(0);
        for child in self.references.iter() {
            children_mask |= child.level_mask();
        }
        let level_mask = match cell_type {
            CellType::Ordinary => children_mask,
            CellType::PrunedBranch => self.level_mask(),
            CellType::LibraryReference => LevelMask::with_mask(0),
            CellType::MerkleProof => LevelMask::for_merkle_cell(children_mask),
            CellType::MerkleUpdate => LevelMask::for_merkle_cell(children_mask),
            CellType::Big => LevelMask::with_mask(0),
            CellType::External => LevelMask::with_mask(0),
            CellType::Unknown => fail!(ExceptionCode::RangeCheckError),
        };
        if self.cell_data.level_mask() != level_mask {
            fail!(
                "Level mask mismatch {} != {}, type: {}",
                self.cell_data.level_mask(),
                level_mask,
                cell_type
            );
        }

        // calculate hashes and depths

        // Hashes are calculated started from smallest indexes.
        // Representation hash is calculated last and "includes" all previous hashes
        // For pruned branch cell only representation hash is calculated
        let mut hashes_depths: [(UInt256, u16); 4] = [
            (UInt256::default(), 0),
            (UInt256::default(), 0),
            (UInt256::default(), 0),
            (UInt256::default(), 0),
        ];
        let hashes_depths_len =
            cell::calc_hashes_depths(raw_data, &self.references, max_depth, &mut hashes_depths)?;

        let store_hashes = self.store_hashes();
        for i in 0..hashes_depths_len {
            let (hash, depth) = &hashes_depths[i];
            if store_hashes {
                let stored_depth = self.cell_data.depth(i);
                if *depth != stored_depth {
                    fail!(
                        "Calculated depth is not equal stored one ({} != {})",
                        depth,
                        stored_depth
                    );
                }
                let stored_hash = self.cell_data.raw_hash(i);
                if hash.as_slice() != stored_hash {
                    fail!("Calculated hash is not equal stored one");
                }
            } else {
                self.cell_data.set_hash_depth(i, hash.as_slice(), *depth)?;
            }
        }

        // FINALIZATION_NANOS.fetch_add(now.elapsed().as_nanos() as u64,
        // Ordering::Relaxed);

        Ok(())
    }

    pub fn cell_data(&self) -> &CellData {
        &self.cell_data
    }
}
