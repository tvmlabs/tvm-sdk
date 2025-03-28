use crate::{BocReader, Cell, CellImpl, CellType, DEPTH_SIZE, LevelMask, SHA256_SIZE, UInt256};
use crate::{cell, fail};
use std::io::Cursor;
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
    offsets: Vec<usize>,
    root_indexes: Vec<u32>,
}

impl BocBuf {
    pub fn new(data: Vec<u8>) -> Result<Self, failure::Error> {
        let mut src = Cursor::new(&data);
        let header = BocReader::read_header(&mut src)?;
        BocReader::precheck_cells_tree_len(&header, src.position(), data.len() as u64, false)?;
        let index_start = src.position() as usize;

        let mut offsets = vec![];
        let index = &data[index_start..];
        if !header.index_included {
            offsets = Vec::with_capacity(header.cells_count);
            for _ in 0_usize..header.cells_count {
                offsets.push(src.position() as usize);
                BocReader::skip_cell(&mut src, header.ref_size)?;
            }
        } else if index.len() < header.cells_count * header.offset_size {
            fail!("Invalid data: too small to fit index");
        }

        let cells_start = index_start + header.cells_count * header.offset_size;
        Ok(Self {
            index_included: header.index_included,
            offset_size: header.offset_size,
            ref_size: header.ref_size,
            has_cache_bits: header.has_cache_bits,
            root_indexes: header.roots_indexes,
            data,
            cells_start,
            index_start,
            offsets,
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

    fn cell_range(&self, index: usize) -> Range<usize> {
        let offset = self.cell_offset(index);
        let len = raw_cell_len(&self.data[offset..], self.ref_size);
        offset..offset + len
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
    boc_range: Range<usize>,
}

impl BocCell {
    pub fn new(boc: Arc<BocBuf>, index: usize) -> Self {
        Self { boc_range: boc.cell_range(index), boc }
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
