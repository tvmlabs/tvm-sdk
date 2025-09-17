use std::cmp::max;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;

use smallvec::SmallVec;
use smallvec::smallvec;

use crate::ByteOrderRead;
use crate::Cell;
use crate::CellType;
use crate::DEPTH_SIZE;
use crate::ExceptionCode;
use crate::LevelMask;
use crate::MAX_DATA_BITS;
use crate::MAX_DEPTH;
use crate::MAX_LEVEL;
use crate::MAX_REFERENCES_COUNT;
use crate::SHA256_SIZE;
use crate::Sha256;
use crate::UInt256;
use crate::cell;
use crate::cell::UNLOADED_ACCOUNT_CELL_MAX_SIZE;
use crate::cell::UNLOADED_ACCOUNT_CELL_MIN_SIZE;
use crate::cell::cell_data::CellData;
use crate::error;
use crate::fail;

#[derive(Clone, Debug)]
pub struct DataCell {
    cell_data: CellData,
    references: SmallVec<[Cell; 4]>, /* TODO make array - you already know cells refs count, or
                                      * may be vector */
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
        Self::with_refs_and_data(smallvec![], &[0x80]).unwrap()
    }

    pub fn with_refs_and_data(
        references: SmallVec<[Cell; 4]>,
        data: &[u8], // with completion tag (for big cell - without)!
    ) -> crate::Result<DataCell> {
        Self::with_params(references, data, CellType::Ordinary, 0, None, None, None)
    }

    pub fn with_params(
        references: SmallVec<[Cell; 4]>,
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
        references: SmallVec<[Cell; 4]>,
        buffer: &Arc<Vec<u8>>,
        offset: usize,
        max_depth: Option<u16>,
        force_finalization: bool,
    ) -> crate::Result<DataCell> {
        let cell_data = CellData::with_external_data(buffer, offset)?;
        Self::construct_cell(cell_data, references, max_depth, force_finalization)
    }

    pub fn with_raw_data(
        references: SmallVec<[Cell; 4]>,
        data: Vec<u8>,
        max_depth: Option<u16>,
        force_finalization: bool,
    ) -> crate::Result<DataCell> {
        let cell_data = CellData::with_raw_data(data)?;
        Self::construct_cell(cell_data, references, max_depth, force_finalization)
    }

    fn construct_cell(
        cell_data: CellData,
        references: SmallVec<[Cell; 4]>,
        max_depth: Option<u16>,
        force_finalization: bool,
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
        cell.finalize(force_finalization, max_depth)?;
        Ok(cell)
    }

    fn finalize(&mut self, force: bool, max_depth: Option<u16>) -> crate::Result<()> {
        if !force && self.store_hashes() {
            return Ok(());
        }

        // let now = std::time::Instant::now();

        // Check data size and references count

        let bit_len = self.bit_length();
        let cell_type = self.cell_type();
        let store_hashes = self.store_hashes();

        // println!("{} {}bits {:03b}", self.cell_type(), bit_len,
        // self.level_mask().mask());

        match cell_type {
            CellType::PrunedBranch => {
                // type + level_mask + level * (hashes + depths)
                let expected =
                    8 * (1 + 1 + (self.level_mask().level() as usize) * (SHA256_SIZE + DEPTH_SIZE));
                if bit_len != expected {
                    fail!("fail creating pruned branch cell: {} != {}", bit_len, expected)
                }
                if !self.references.is_empty() {
                    fail!(
                        "fail creating pruned branch cell: references {} != 0",
                        self.references.len()
                    )
                }
                if self.data()[0] != u8::from(CellType::PrunedBranch) {
                    fail!(
                        "fail creating pruned branch cell: data[0] {} != {}",
                        self.data()[0],
                        u8::from(CellType::PrunedBranch)
                    )
                }
                if self.data()[1] != self.cell_data.level_mask().0 {
                    fail!(
                        "fail creating pruned branch cell: data[1] {} != {}",
                        self.data()[1],
                        self.cell_data.level_mask().0
                    )
                }
                let level = self.level_mask().level() as usize;
                if level == 0 {
                    fail!("Pruned branch cell must have non zero level");
                }
                let data = self.data();
                let mut offset = 1 + 1 + level * SHA256_SIZE;
                for _ in 0..level {
                    let depth = ((data[offset] as u16) << 8) | (data[offset + 1] as u16);
                    if depth > MAX_DEPTH {
                        fail!("Depth of pruned branch cell is too big");
                    }
                    offset += DEPTH_SIZE;
                }
                if store_hashes {
                    fail!("store_hashes flag is not supported for pruned branch cell");
                }
            }
            CellType::MerkleProof => {
                // type + hash + depth
                if bit_len != 8 * (1 + SHA256_SIZE + 2) {
                    fail!(
                        "fail creating merkle proof cell: bit_len {} != {}",
                        bit_len,
                        8 * (1 + SHA256_SIZE + 2)
                    )
                }
                if self.references.len() != 1 {
                    fail!(
                        "fail creating merkle proof cell: references {} != 1",
                        self.references.len()
                    )
                }
            }
            CellType::MerkleUpdate => {
                // type + 2 * (hash + depth)
                if bit_len != 8 * (1 + 2 * (SHA256_SIZE + 2)) {
                    fail!(
                        "fail creating merkle update cell: bit_len {} != {}",
                        bit_len,
                        8 * (1 + 2 * (SHA256_SIZE + 2))
                    )
                }
                if self.references.len() != 2 {
                    fail!(
                        "fail creating merkle update cell: references {} != 2",
                        self.references.len()
                    )
                }
            }
            CellType::Ordinary => {
                if bit_len > MAX_DATA_BITS {
                    fail!("fail creating ordinary cell: bit_len {} > {}", bit_len, MAX_DATA_BITS)
                }
                if self.references.len() > MAX_REFERENCES_COUNT {
                    fail!(
                        "fail creating ordinary cell: references {} > {}",
                        self.references.len(),
                        MAX_REFERENCES_COUNT
                    )
                }
            }
            CellType::LibraryReference => {
                if bit_len != 8 * (1 + SHA256_SIZE) {
                    fail!(
                        "fail creating libray reference cell: bit_len {} != {}",
                        bit_len,
                        8 * (1 + SHA256_SIZE)
                    )
                }
                if !self.references.is_empty() {
                    fail!(
                        "fail creating libray reference cell: references {} != 0",
                        self.references.len()
                    )
                }
            }
            CellType::Big => {
                // all checks were performed before finalization
            }
            CellType::UnloadedAccount => {
                // type + hash + depth + (tree cells count len | tree bits count len) + tree
                // cells count + tree bits count
                let min_required_len = 8 * (UNLOADED_ACCOUNT_CELL_MIN_SIZE);
                if bit_len < min_required_len {
                    fail!(
                        "fail creating unloaded account cell: bit_len {} < {}",
                        bit_len,
                        min_required_len
                    )
                }
                if !self.references.is_empty() {
                    fail!(
                        "fail creating unloaded account cell: references {} != 0",
                        self.references.len()
                    )
                }
                let lengths_offset = 1 + SHA256_SIZE + 2;
                let mut reader = Cursor::new(&self.data()[lengths_offset..]);
                let lengths = reader.read_byte()?;
                let tree_cells_count_len = (lengths >> 4) as usize;
                let tree_bits_count_len = (lengths & 0x0F) as usize;

                if bit_len != 8 * (lengths_offset + 1 + tree_bits_count_len + tree_cells_count_len)
                {
                    fail!(
                        "fail creating unloaded account cell: bit_len {} != {}",
                        bit_len,
                        8 * (lengths_offset + 1 + tree_bits_count_len + tree_cells_count_len)
                    )
                }

                let mut buffer = [0u8; 8];
                let _ = reader.read(&mut buffer[tree_cells_count_len..])?;
                let tree_cell_count = u64::from_be_bytes(buffer);
                let mut buffer = [0u8; 8];
                let _ = reader.read(&mut buffer[tree_bits_count_len..])?;
                let tree_bits_count = u64::from_be_bytes(buffer);

                self.tree_bits_count = tree_bits_count;
                self.tree_cell_count = tree_cell_count;

                // hashes are not calculated for unloaded account cell
                return Ok(());
            }
            CellType::Unknown => {
                fail!("fail creating unknown cell")
            }
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
            CellType::UnloadedAccount => LevelMask::with_mask(0),
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

        let is_merkle_cell = cell_type.is_merkle();
        let is_pruned_cell = cell_type.is_pruned();

        let mut d1d2: [u8; 2] = self.raw_data()?[..2].try_into()?;

        // Hashes are calculated started from the smallest indexes.
        // Representation hash is calculated last and "includes" all previous hashes
        // For pruned branch cell only representation hash is calculated
        let mut hash_array_index = 0;
        for i in 0..=3 {
            // Hash is calculated only for "1" bits of level mask.
            // Hash for i = 0 is calculated anyway.
            // For example if mask = 0b010 i = 0, 2
            // for example if mask = 0b001 i = 0, 1
            // for example if mask = 0b011 i = 0, 1, 2
            if i != 0 && (is_pruned_cell || ((1 << (i - 1)) & level_mask.mask()) == 0) {
                continue;
            }

            let mut hasher = Sha256::new();
            let mut depth = 0;

            if cell_type == CellType::Big {
                // For big cell representation hash is calculated only from data
                hasher.update(self.data());
            } else {
                // descr bytes
                let level_mask =
                    if is_pruned_cell { self.level_mask() } else { LevelMask::with_level(i as u8) };
                d1d2[0] = cell::calc_d1(level_mask, false, cell_type, self.references.len());
                hasher.update(d1d2);

                // data
                if i == 0 {
                    let data_size = (bit_len / 8) + usize::from(bit_len % 8 != 0);
                    hasher.update(&self.data()[..data_size]);
                } else {
                    hasher.update(self.cell_data.raw_hash(i - 1));
                }

                // depth
                for child in self.references.iter() {
                    let child_depth = child.depth(i + is_merkle_cell as usize);
                    depth = max(depth, child_depth + 1);
                    let max_depth = max_depth.unwrap_or(MAX_DEPTH);
                    if depth > max_depth {
                        fail!("fail creating cell: depth {} > {}", depth, max_depth.min(MAX_DEPTH))
                    }
                    hasher.update(child_depth.to_be_bytes());
                }

                // hashes
                for child in self.references.iter() {
                    let child_hash = child.hash(i + is_merkle_cell as usize);
                    hasher.update(child_hash.as_slice());
                }
            }

            let hash = hasher.finalize();
            if store_hashes {
                let stored_depth = self.cell_data.depth(i);
                if depth != stored_depth {
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
                self.cell_data.set_hash_depth(hash_array_index, hash.as_slice(), depth)?;
                hash_array_index += 1;
            }
        }

        // FINALIZATION_NANOS.fetch_add(now.elapsed().as_nanos() as u64,
        // Ordering::Relaxed);

        Ok(())
    }

    pub fn cell_data(&self) -> &CellData {
        &self.cell_data
    }

    pub(crate) fn data(&self) -> &[u8] {
        self.cell_data.data()
    }

    pub(crate) fn raw_data(&self) -> crate::Result<&[u8]> {
        Ok(self.cell_data.raw_data())
    }

    pub(crate) fn bit_length(&self) -> usize {
        self.cell_data.bit_length()
    }

    pub(crate) fn references_count(&self) -> usize {
        self.references.len()
    }

    pub(crate) fn reference(&self, index: usize) -> crate::Result<Cell> {
        self.references.get(index).cloned().ok_or_else(|| error!(ExceptionCode::CellUnderflow))
    }

    pub(crate) fn cell_type(&self) -> CellType {
        self.cell_data.cell_type()
    }

    pub(crate) fn level_mask(&self) -> LevelMask {
        self.cell_data.level_mask()
    }

    pub(crate) fn hash(&self, index: usize) -> UInt256 {
        self.cell_data().hash(index)
    }

    pub(crate) fn depth(&self, index: usize) -> u16 {
        self.cell_data().depth(index)
    }

    pub(crate) fn store_hashes(&self) -> bool {
        self.cell_data().store_hashes()
    }

    pub(crate) fn tree_bits_count(&self) -> u64 {
        self.tree_bits_count
    }

    pub(crate) fn tree_cell_count(&self) -> u64 {
        self.tree_cell_count
    }

    pub(crate) fn to_unloaded_account(&self) -> crate::Result<Cell> {
        if self.cell_type() != CellType::Ordinary && self.cell_type() != CellType::Big {
            fail!("Only ordinary and big cells can be converted to external")
        }

        let mut data = [0u8; UNLOADED_ACCOUNT_CELL_MAX_SIZE + 1]; // including tag byte
        let mut cursor = Cursor::new(data.as_mut());
        cursor.write_all(&[CellType::UNLOADED_ACCOUNT_ROOT])?;
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
            smallvec![],
            &cursor.into_inner()[..size],
            CellType::UnloadedAccount,
            0,
            None,
            None,
            None,
        )?;

        Ok(Cell::with_data(cell))
    }
}
