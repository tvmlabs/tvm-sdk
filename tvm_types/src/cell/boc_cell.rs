use crate::cell::{EXTERNAL_CELL_MIN_SIZE, calc_d1};
use crate::error;
use crate::{
    BocReader, ByteOrderRead, Cell, CellImpl, CellType, DEPTH_SIZE, ExceptionCode, LevelMask,
    MAX_DATA_BITS, MAX_DEPTH, MAX_REFERENCES_COUNT, SHA256_SIZE, Sha256, UInt256,
};
use crate::{cell, fail};
use std::cmp::max;
use std::io::{Cursor, Read};
use std::ops::Range;
use std::sync::Arc;

pub struct BocBuf {
    data: Vec<u8>,
    index_included: bool,
    offset_size: usize,
    ref_size: usize,
    has_cache_bits: bool,
    index_start: usize,
    cells_start: usize,
    cells_count: usize,
    offsets: Vec<usize>,
    root_indexes: Vec<u32>,
    hashes_depths: Vec<Vec<(UInt256, u16)>>,
}

impl BocBuf {
    pub fn new(data: Vec<u8>) -> Result<Self, failure::Error> {
        let mut src = Cursor::new(&data);
        let header = BocReader::read_header(&mut src)?;
        BocReader::precheck_cells_tree_len(&header, src.position(), data.len() as u64, false)?;
        let index_start = src.position() as usize;

        let mut offsets = vec![];
        let index = &data[src.position() as usize..];
        if !header.index_included {
            offsets = Vec::with_capacity(header.cells_count);
            for _ in 0_usize..header.cells_count {
                offsets.push(src.position() as usize);
                BocReader::skip_cell(&mut src, header.ref_size)?;
            }
        } else if index.len() < header.cells_count * header.offset_size {
            fail!("Invalid data: too small to fit index");
        }

        let cells_start = src.position() as usize + header.cells_count * header.offset_size;
        Ok(Self {
            index_included: header.index_included,
            offset_size: header.offset_size,
            ref_size: header.ref_size,
            has_cache_bits: header.has_cache_bits,
            root_indexes: header.roots_indexes,
            cells_count: header.cells_count,
            data,
            cells_start,
            index_start,
            offsets,
            hashes_depths: Vec::new(),
        })
    }

    pub fn into_root_cells(self) -> Result<Vec<Cell>, failure::Error> {
        let boc = Arc::new(self);
        let mut cells = vec![];
        for index in &boc.root_indexes {
            cells.push(Cell::Boc(BocCell::new(boc.clone(), *index as usize)));
        }
        Ok(cells)
    }

    #[inline(always)]
    fn cell_offset(&self, index: usize) -> usize {
        if self.index_included {
            self.cells_start + self.included_index_entry(index)
        } else {
            self.offsets[index]
        }
    }

    fn cell_range(&self, index: usize) -> Range<usize> {
        let offset = self.cell_offset(index);
        let len = raw_cell_len(&self.data[offset..], self.ref_size);
        offset..offset + len
    }

    fn cell_raw_data(&self, index: usize) -> &[u8] {
        &self.data[self.cell_range(index)]
    }

    // fn ensure_cell_hash_depth(
    //     &self,
    //     cell_index: usize,
    //     hash_index: usize,
    // ) -> Result<(UInt256, u16), failure::Error> {
    //     let Ok(mut hashes_depths) = self.hashes_depths.lock() else {
    //         fail!("fail creating cell: hashes_depths lock poisoned");
    //     };
    //     if hashes_depths.is_empty() {
    //         hashes_depths.resize(self.cells_count, Vec::default());
    //     }
    //     self.internal_ensure_cell_hash_depth(&mut hashes_depths, cell_index, hash_index)
    // }

    fn internal_ensure_cell_hash_depth(
        &self,
        hashes_depths: &mut Vec<Vec<(UInt256, u16)>>,
        cell_index: usize,
        hash_index: usize,
    ) -> Result<(UInt256, u16), failure::Error> {
        if !hashes_depths[cell_index].is_empty() {
            let cell_hash_depth = &hashes_depths[cell_index];
            if hash_index >= cell_hash_depth.len() {
                println!(
                    "hash_index: {}, hashes_depths[cell_index].len(): {}",
                    hash_index,
                    hashes_depths[cell_index].len()
                );
            }
            return Ok(cell_hash_depth[hash_index].clone());
        }
        let raw_data = self.cell_raw_data(cell_index);
        let cell_type = cell::cell_type(raw_data);
        let bit_len = cell::bit_len(raw_data);
        let cell_data = cell::cell_data(raw_data);
        let (refs_buf, refs_count) = cell::child_refs(raw_data, self.ref_size);
        let refs = &refs_buf[0..refs_count];

        self.verify_cell(raw_data, cell_type, bit_len, cell_data, refs_count)?;

        // Check level

        let level_mask = self.verify_level(raw_data, cell_type, refs_count)?;

        let is_merkle_cell =
            cell_type == CellType::MerkleProof || cell_type == CellType::MerkleUpdate;
        let is_pruned_cell = cell_type == CellType::PrunedBranch;

        let mut d1d2: [u8; 2] = raw_data[..2].try_into()?;

        let mut hash_array_index = 0;
        let mut cell_hashes_depths = Vec::<(UInt256, u16)>::new();
        for i in 0..=3 {
            if i != 0 && (is_pruned_cell || ((1 << (i - 1)) & level_mask.mask()) == 0) {
                continue;
            }

            let mut hasher = Sha256::new();
            let mut depth = 0;

            if cell_type == CellType::Big {
                hasher.update(cell_data);
            } else {
                // descr bytes
                let level_mask =
                    if is_pruned_cell { level_mask } else { LevelMask::with_level(i as u8) };
                d1d2[0] = calc_d1(level_mask, false, cell_type, refs_count);
                hasher.update(d1d2);

                // data
                if i == 0 {
                    let data_size = (bit_len / 8) + usize::from(bit_len % 8 != 0);
                    hasher.update(&cell_data[..data_size]);
                } else {
                    hasher.update(&cell_hashes_depths[i - 1].0);
                }

                // depth
                for child_index in refs {
                    let child_depth = self
                        .internal_ensure_cell_hash_depth(
                            hashes_depths,
                            *child_index,
                            i + is_merkle_cell as usize,
                        )?
                        .1;
                    depth = max(depth, child_depth + 1);
                    let max_depth = MAX_DEPTH;
                    if depth > max_depth {
                        fail!("fail creating cell: depth {} > {}", depth, max_depth.min(MAX_DEPTH))
                    }
                    hasher.update(child_depth.to_be_bytes());
                }

                // hashes
                for child_index in refs {
                    let child_hash = self
                        .internal_ensure_cell_hash_depth(
                            hashes_depths,
                            *child_index,
                            i + is_merkle_cell as usize,
                        )?
                        .0;
                    hasher.update(child_hash.as_slice());
                }
            }

            let hash = hasher.finalize();
            debug_assert!(cell_hashes_depths.len() == hash_array_index);
            cell_hashes_depths.push((hash.into(), depth));
            hash_array_index += 1;
        }
        let result = cell_hashes_depths[hash_index].clone();
        hashes_depths[cell_index] = cell_hashes_depths;
        Ok(result)
    }

    fn verify_level(
        &self,
        raw_data: &[u8],
        cell_type: CellType,
        refs_count: usize,
    ) -> Result<LevelMask, failure::Error> {
        let mut children_mask = LevelMask::with_mask(0);
        for child_index in 0..refs_count {
            let child_raw_data = self.cell_child(raw_data, child_index)?.1;
            children_mask |= cell::level_mask(child_raw_data);
        }
        let level_mask = match cell_type {
            CellType::Ordinary => children_mask,
            CellType::PrunedBranch => cell::level_mask(raw_data),
            CellType::LibraryReference => LevelMask::with_mask(0),
            CellType::MerkleProof => LevelMask::for_merkle_cell(children_mask),
            CellType::MerkleUpdate => LevelMask::for_merkle_cell(children_mask),
            CellType::Big => LevelMask::with_mask(0),
            CellType::External => LevelMask::with_mask(0),
            CellType::Unknown => fail!(ExceptionCode::RangeCheckError),
        };
        if cell::level_mask(raw_data) != level_mask {
            fail!(
                "Level mask mismatch {} != {}, type: {}",
                cell::level_mask(raw_data),
                level_mask,
                cell_type
            );
        }
        Ok(level_mask)
    }

    fn verify_cell(
        &self,
        raw_data: &[u8],
        cell_type: CellType,
        bit_len: usize,
        cell_data: &[u8],
        refs_count: usize,
    ) -> Result<(), failure::Error> {
        let store_hashes = cell::store_hashes(raw_data);
        match cell_type {
            CellType::PrunedBranch => {
                let expected =
                    8 * (1 + 1 + (cell::level(raw_data) as usize) * (SHA256_SIZE + DEPTH_SIZE));
                if bit_len != expected {
                    fail!("fail creating pruned branch cell: {} != {}", bit_len, expected)
                }
                if refs_count > 0 {
                    fail!("fail creating pruned branch cell: references {} != 0", refs_count)
                }
                if cell_data[0] != u8::from(CellType::PrunedBranch) {
                    fail!(
                        "fail creating pruned branch cell: data[0] {} != {}",
                        cell::cell_data(raw_data)[0],
                        u8::from(CellType::PrunedBranch)
                    )
                }
                if cell_data[1] != cell::level_mask(raw_data).0 {
                    fail!(
                        "fail creating pruned branch cell: data[1] {} != {}",
                        cell::cell_data(raw_data)[1],
                        cell::level_mask(raw_data).0
                    )
                }
                let level = cell::level(raw_data) as usize;
                if level == 0 {
                    fail!("Pruned branch cell must have non zero level");
                }
                let mut offset = 1 + 1 + level * SHA256_SIZE;
                for _ in 0..level {
                    let depth = ((cell_data[offset] as u16) << 8) | (cell_data[offset + 1] as u16);
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
                if bit_len != 8 * (1 + SHA256_SIZE + 2) {
                    fail!(
                        "fail creating merkle proof cell: bit_len {} != {}",
                        bit_len,
                        8 * (1 + SHA256_SIZE + 2)
                    )
                }
                if refs_count != 1 {
                    fail!("fail creating merkle proof cell: references {} != 1", refs_count)
                }
            }
            CellType::MerkleUpdate => {
                if bit_len != 8 * (1 + 2 * (SHA256_SIZE + 2)) {
                    fail!(
                        "fail creating merkle update cell: bit_len {} != {}",
                        bit_len,
                        8 * (1 + 2 * (SHA256_SIZE + 2))
                    )
                }
                if refs_count != 2 {
                    fail!("fail creating merkle unpdate cell: references {} != 2", refs_count)
                }
            }
            CellType::Ordinary => {
                if bit_len > MAX_DATA_BITS {
                    fail!("fail creating ordinary cell: bit_len {} > {}", bit_len, MAX_DATA_BITS)
                }
                if refs_count > MAX_REFERENCES_COUNT {
                    fail!(
                        "fail creating ordinary cell: references {} > {}",
                        refs_count,
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
                if refs_count > 0 {
                    fail!("fail creating libray reference cell: references {} != 0", refs_count)
                }
            }
            CellType::Big => {}
            CellType::External => {
                let min_required_len = 8 * (EXTERNAL_CELL_MIN_SIZE);
                if bit_len < min_required_len {
                    fail!("fail creating external cell: bit_len {} < {}", bit_len, min_required_len)
                }
                if refs_count > 0 {
                    fail!("fail creating external cell: references {} != 0", refs_count)
                }
                let lengths_offset = 1 + SHA256_SIZE + 2;
                let mut reader = Cursor::new(&cell_data[lengths_offset..]);
                let lengths = reader.read_byte()?;
                let tree_cells_count_len = (lengths >> 4) as usize;
                let tree_bits_count_len = (lengths & 0x0F) as usize;

                if bit_len != 8 * (lengths_offset + 1 + tree_bits_count_len + tree_cells_count_len)
                {
                    fail!(
                        "fail creating external cell: bit_len {} != {}",
                        bit_len,
                        8 * (lengths_offset + 1 + tree_bits_count_len + tree_cells_count_len)
                    )
                }

                let mut buffer = [0u8; 8];
                let _ = reader.read(&mut buffer[tree_cells_count_len..])?;
                let mut buffer = [0u8; 8];
                let _ = reader.read(&mut buffer[tree_bits_count_len..])?;

                return Ok(());
            }
            CellType::Unknown => {
                fail!("fail creating unknown cell")
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn included_index_entry(&self, index: usize) -> usize {
        if index > 0 {
            let offset = read_be_int(
                &self.data,
                self.index_start + (index - 1) * self.offset_size,
                self.offset_size,
            );
            if self.has_cache_bits { offset >> 1 } else { offset }
        } else {
            0
        }
    }
    fn cell_child(
        &self,
        cell_raw_data: &[u8],
        index: usize,
    ) -> Result<(usize, &[u8]), failure::Error> {
        let refs_count = cell::refs_count(cell_raw_data);
        if index >= refs_count {
            fail!("reference out of range, cells_count: {}, ref: {}", refs_count, index,)
        }
        let refs_start = cell::full_len(cell_raw_data);
        let child_index =
            read_be_int(cell_raw_data, refs_start + index * self.ref_size, self.ref_size);
        let child_raw_data = self.cell_raw_data(child_index);
        Ok((child_index, child_raw_data))
    }
}

#[inline(always)]
pub(crate) fn read_be_int(buf: &[u8], offset: usize, len: usize) -> usize {
    let available = buf.len().saturating_sub(offset);
    let take = len.min(available).min(8);
    let mut padded = [0u8; 8];
    padded[8 - take..].copy_from_slice(&buf[offset..offset + take]);
    u64::from_be_bytes(padded) as usize
}

#[inline(always)]
fn raw_cell_len(raw_data: &[u8], ref_size: usize) -> usize {
    if cell::is_big_cell(raw_data) {
        4 + read_be_int(raw_data, 1, 3)
    } else {
        cell::full_len(raw_data) + ref_size * cell::refs_count(raw_data)
    }
}

#[derive(Clone)]
pub struct BocCell {
    boc: Arc<BocBuf>,
    index: usize,
    boc_range: Range<usize>,
}

impl BocCell {
    pub fn new(boc: Arc<BocBuf>, index: usize) -> Self {
        Self { index, boc_range: boc.cell_range(index), boc }
    }

    fn boc_raw_data(&self) -> &[u8] {
        &self.boc.data[self.boc_range.clone()]
    }

    fn raw_hash(&self, mut index: usize) -> Result<UInt256, failure::Error> {
        index = self.level_mask().calc_hash_index(index);
        if self.cell_type() == CellType::PrunedBranch {
            // pruned cell stores all hashes (except representation) in data
            if index != self.level() as usize {
                let offset = 1 + 1 + index * SHA256_SIZE;
                return Ok(self.data()[offset..offset + SHA256_SIZE].into());
            } else {
                index = 0;
            }
        }
        // external cell has only representation hash
        if self.cell_type() == CellType::External {
            let offset = 1;
            return Ok(self.data()[offset..offset + SHA256_SIZE].into());
        }
        Ok(if self.store_hashes() {
            cell::hash(self.boc_raw_data(), index).into()
        } else {
            todo!()
            // self.boc.ensure_cell_hash_depth(self.index, index)?.0
        })
    }
}

impl CellImpl for BocCell {
    fn data(&self) -> &[u8] {
        cell::cell_data(self.boc_raw_data())
    }

    fn raw_data(&self) -> crate::Result<&[u8]> {
        Ok(self.boc_raw_data())
    }

    fn bit_length(&self) -> usize {
        cell::bit_len(self.boc_raw_data())
    }

    fn references_count(&self) -> usize {
        cell::refs_count(self.boc_raw_data())
    }

    fn reference(&self, index: usize) -> crate::Result<Cell> {
        let raw_data = self.boc_raw_data();
        let refs_count = cell::refs_count(raw_data);
        if index >= refs_count {
            fail!("reference out of range, cells_count: {}, ref: {}", refs_count, index,)
        }
        let refs_start = cell::full_len(raw_data);
        let cell_index =
            read_be_int(raw_data, refs_start + index * self.boc.ref_size, self.boc.ref_size);

        Ok(Cell::Boc(BocCell::new(self.boc.clone(), cell_index)))
    }

    fn cell_type(&self) -> CellType {
        cell::cell_type(self.boc_raw_data())
    }

    fn level_mask(&self) -> LevelMask {
        cell::level_mask(self.boc_raw_data())
    }

    fn hash(&self, index: usize) -> UInt256 {
        self.raw_hash(index).unwrap().into()
    }

    fn depth(&self, mut index: usize) -> u16 {
        index = self.level_mask().calc_hash_index(index);
        if self.cell_type() == CellType::PrunedBranch {
            // pruned cell stores all depth except "representetion" in data
            if index != self.level() as usize {
                // type + level_mask + level * (hashes + depths)
                let offset = 1 + 1 + (self.level() as usize) * SHA256_SIZE + index * DEPTH_SIZE;
                let data = self.data();
                return ((data[offset] as u16) << 8) | (data[offset + 1] as u16);
            } else {
                index = 0;
            }
        }
        // external cell stores only representation depth
        if self.cell_type() == CellType::External {
            let offset = 1 + SHA256_SIZE;
            let data = self.data();
            return ((data[offset] as u16) << 8) | (data[offset + 1] as u16);
        }
        if self.store_hashes() {
            cell::depth(self.boc_raw_data(), index)
        } else {
            todo!()
            // self.boc.ensure_cell_hash_depth(self.index, index).unwrap().1
        }
    }

    fn store_hashes(&self) -> bool {
        cell::store_hashes(self.boc_raw_data())
    }
}
