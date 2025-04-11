use crate::error;
use crate::{
    BocReader, ByteOrderRead, Cell, CellImpl, CellType, DEPTH_SIZE, ExceptionCode, LevelMask,
    MAX_BIG_DATA_BYTES, MAX_DEPTH, SHA256_SIZE, Sha256, UInt256,
};
use crate::{cell, fail};
use std::cmp::max;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::Arc;

const HASH_INDEX_STORED: u32 = u32::MAX;

#[derive(Copy, Clone)]
struct CellInfo {
    raw_offset: u32,
    hash_index: u32,
}

pub struct BocBuf {
    data: Arc<Vec<u8>>,
    ref_size: usize,
    cell_infos: Vec<CellInfo>,
    hashes: Vec<(UInt256, u16)>,
    root_indexes: Vec<u32>,
}

impl BocBuf {
    pub fn new(data: Arc<Vec<u8>>) -> Result<Self, failure::Error> {
        let mut src = Cursor::new(data.as_slice());
        let header = BocReader::read_header(&mut src)?;
        BocReader::precheck_cells_tree_len(&header, src.position(), data.len() as u64, false)?;

        let mut cell_infos = Vec::with_capacity(header.cells_count);
        let mut hashes = Vec::new();

        if header.index_included {
            src.seek(SeekFrom::Current(header.cells_count as i64 * header.offset_size as i64))?;
        }
        for _ in 0_usize..header.cells_count {
            let raw_offset = src.position() as u32;
            let hash_index = Self::pre_scan_cell(&mut src, header.ref_size)?;
            cell_infos.push(CellInfo { raw_offset, hash_index });
        }

        for i in (0..header.cells_count).rev() {
            if cell_infos[i].hash_index != HASH_INDEX_STORED {
                Self::calc_hashes(
                    i,
                    data.as_slice(),
                    header.ref_size,
                    &mut cell_infos,
                    &mut hashes,
                    None,
                )?;
            }
        }

        Ok(Self {
            ref_size: header.ref_size,
            root_indexes: header.roots_indexes,
            data,
            cell_infos,
            hashes,
        })
    }

    fn pre_scan_cell<T>(src: &mut T, ref_size: usize) -> crate::Result<u32>
    where
        T: Read + Seek,
    {
        let mut d1d2 = [0_u8; 2];
        src.read_exact(&mut d1d2[0..1])?;
        let rest_size = if cell::is_big_cell(&d1d2) {
            let len = src.read_be_uint(3)? as usize;
            if len > MAX_BIG_DATA_BYTES {
                fail!("big cell data length {} is too big", len);
            }
            len
        } else {
            src.read_exact(&mut d1d2[1..2])?;
            cell::full_len(&d1d2) + ref_size * cell::refs_count(&d1d2) - 2
        };
        src.seek(SeekFrom::Current(rest_size as i64))?;
        Ok(if cell::store_hashes(&d1d2) { HASH_INDEX_STORED } else { 0 })
    }

    fn calc_hashes(
        cell_index: usize,
        boc: &[u8],
        ref_size: usize,
        cell_infos: &mut [CellInfo],
        hashes: &mut Vec<(UInt256, u16)>,
        max_depth: Option<u16>,
    ) -> crate::Result<()> {
        let raw_data = &boc[cell_infos[cell_index].raw_offset as usize..];
        let cell_type = cell::cell_type(raw_data);

        if cell_type == CellType::External {
            // hashes are not calculated for external cell
            return Ok(());
        }

        // Check level

        let refs = cell::refs_count(raw_data);
        let refs_start = &raw_data[cell::full_len(raw_data)..];
        let mut children_mask = LevelMask::with_mask(0);
        for r in 0..refs {
            let (child_raw, _) = raw_child(refs_start, r, ref_size, boc, &cell_infos);
            children_mask |= cell::level_mask(child_raw);
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

        // calculate hashes and depths

        let is_merkle_cell = matches!(cell_type, CellType::MerkleProof | CellType::MerkleUpdate);
        let is_pruned_cell = cell_type == CellType::PrunedBranch;

        let cell_data = cell::cell_data(raw_data);
        let bit_len = cell::bit_len(raw_data);

        let mut d1d2: [u8; 2] = raw_data[..2].try_into()?;

        // Hashes are calculated started from the smallest indexes.
        // Representation hash is calculated last and "includes" all previous hashes
        // For pruned branch cell only representation hash is calculated
        cell_infos[cell_index].hash_index = hashes.len() as u32;
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
                hasher.update(cell_data);
            } else {
                // descr bytes
                let level_mask = if is_pruned_cell {
                    cell::level_mask(raw_data)
                } else {
                    LevelMask::with_level(i as u8)
                };
                d1d2[0] = cell::calc_d1(level_mask, false, cell_type, refs);
                hasher.update(d1d2);

                // data
                if i == 0 {
                    let data_size = (bit_len / 8) + usize::from(bit_len % 8 != 0);
                    hasher.update(&cell_data[..data_size]);
                } else {
                    let prev_hash =
                        raw_hash(raw_data, i - 1, cell_infos[cell_index].hash_index, &hashes)?;
                    hasher.update(prev_hash);
                }

                // depth
                for r in 0..refs {
                    let (child_raw, child_info) =
                        raw_child(refs_start, r, ref_size, boc, &cell_infos);
                    let child_depth = raw_depth(
                        child_raw,
                        i + is_merkle_cell as usize,
                        child_info.hash_index,
                        &hashes,
                    )?;
                    depth = max(depth, child_depth + 1);
                    let max_depth = max_depth.unwrap_or(MAX_DEPTH);
                    if depth > max_depth {
                        fail!("fail creating cell: depth {} > {}", depth, max_depth.min(MAX_DEPTH))
                    }
                    hasher.update(child_depth.to_be_bytes());
                }

                // hashes
                for r in 0..refs {
                    let (child_raw, child_info) =
                        raw_child(refs_start, r, ref_size, boc, &cell_infos);
                    let child_hash = raw_hash(
                        child_raw,
                        i + is_merkle_cell as usize,
                        child_info.hash_index,
                        &hashes,
                    )?;
                    hasher.update(child_hash.as_slice());
                }
            }

            let hash = hasher.finalize();
            hashes.push((UInt256::from(hash), depth));
        }

        Ok(())
    }

    pub fn into_root_cells(self) -> Result<Vec<Cell>, failure::Error> {
        let boc = Arc::new(self);
        let mut cells = vec![];
        for index in &boc.root_indexes {
            cells.push(Cell::Boc(BocCell::new(boc.clone(), *index as usize)));
        }
        Ok(cells)
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
    info: CellInfo,
}

impl BocCell {
    pub fn new(boc: Arc<BocBuf>, index: usize) -> Self {
        Self { info: boc.cell_infos[index], boc }
    }

    fn unbounded_raw_data(&self) -> &[u8] {
        &self.boc.data[self.info.raw_offset as usize..]
    }

    fn bounded_raw_data(&self) -> &[u8] {
        let offset = self.info.raw_offset as usize;
        let len = raw_cell_len(self.unbounded_raw_data(), self.boc.ref_size);
        &self.boc.data[offset..offset + len]
    }
}

fn raw_child<'a>(
    refs_start: &[u8],
    r: usize,
    ref_size: usize,
    boc: &'a [u8],
    cell_infos: &[CellInfo],
) -> (&'a [u8], CellInfo) {
    let child_index = read_be_int(refs_start, r * ref_size, ref_size);
    let child_info = cell_infos[child_index];
    let child_raw = &boc[child_info.raw_offset as usize..];
    (child_raw, child_info)
}

fn raw_hash(
    cell_raw: &[u8],
    index: usize,
    hash_index: u32,
    hashes: &[(UInt256, u16)],
) -> Result<UInt256, failure::Error> {
    let mut index = cell::level_mask(cell_raw).calc_hash_index(index);
    let cell_type = cell::cell_type(cell_raw);
    if cell_type == CellType::PrunedBranch {
        // pruned cell stores all hashes (except representation) in data
        if index != cell::level(cell_raw) as usize {
            let offset = 1 + 1 + index * SHA256_SIZE;
            return Ok(cell::cell_data(cell_raw)[offset..offset + SHA256_SIZE].into());
        } else {
            index = 0;
        }
    }
    // external cell has only representation hash
    if cell_type == CellType::External {
        let offset = 1;
        return Ok(cell::cell_data(cell_raw)[offset..offset + SHA256_SIZE].into());
    }
    Ok(if cell::store_hashes(cell_raw) {
        cell::hash(cell_raw, index).into()
    } else {
        let offset = hash_index;
        if offset == HASH_INDEX_STORED {
            fail!("hash is not stored in cell");
        }
        hashes[offset as usize + index].0.clone()
    })
}

fn raw_depth(
    cell_raw: &[u8],
    index: usize,
    hash_index: u32,
    hashes: &[(UInt256, u16)],
) -> Result<u16, failure::Error> {
    let mut index = cell::level_mask(cell_raw).calc_hash_index(index);
    let cell_type = cell::cell_type(cell_raw);
    if cell_type == CellType::PrunedBranch {
        // pruned cell stores all hashes (except representation) in data
        if index != cell::level(cell_raw) as usize {
            let offset =
                1 + 1 + (cell::level(cell_raw) as usize) * SHA256_SIZE + index * DEPTH_SIZE;
            let data = cell::cell_data(cell_raw);
            return Ok(((data[offset] as u16) << 8) | (data[offset + 1] as u16));
        } else {
            index = 0;
        }
    }
    // external cell has only representation hash
    if cell_type == CellType::External {
        let offset = 1 + SHA256_SIZE;
        let data = cell::cell_data(cell_raw);
        return Ok(((data[offset] as u16) << 8) | (data[offset + 1] as u16));
    }
    Ok(if cell::store_hashes(cell_raw) {
        cell::depth(cell_raw, index)
    } else {
        if hash_index == HASH_INDEX_STORED {
            fail!("hash is not stored in cell");
        }
        let hash_index = hash_index as usize + index;
        hashes[hash_index].1
    })
}

impl CellImpl for BocCell {
    fn data(&self) -> &[u8] {
        cell::cell_data(self.unbounded_raw_data())
    }

    fn raw_data(&self) -> crate::Result<&[u8]> {
        Ok(self.bounded_raw_data())
    }

    fn bit_length(&self) -> usize {
        cell::bit_len(self.unbounded_raw_data())
    }

    fn references_count(&self) -> usize {
        cell::refs_count(self.unbounded_raw_data())
    }

    fn reference(&self, index: usize) -> crate::Result<Cell> {
        let raw_data = self.unbounded_raw_data();
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
        cell::cell_type(self.unbounded_raw_data())
    }

    fn level_mask(&self) -> LevelMask {
        cell::level_mask(self.unbounded_raw_data())
    }

    fn hash(&self, index: usize) -> UInt256 {
        raw_hash(self.unbounded_raw_data(), index, self.info.hash_index, &self.boc.hashes).unwrap()
    }

    fn depth(&self, index: usize) -> u16 {
        raw_depth(self.unbounded_raw_data(), index, self.info.hash_index, &self.boc.hashes).unwrap()
    }

    fn store_hashes(&self) -> bool {
        cell::store_hashes(self.unbounded_raw_data())
    }

    fn store_hashes_depths_len(&self) -> usize {
        cell::hashes_count(self.unbounded_raw_data())
    }

    fn store_hashes_depths(&self) -> Vec<(UInt256, u16)> {
        let raw_data = self.unbounded_raw_data();
        let hashes_count = cell::hashes_count(raw_data);
        let mut result = Vec::with_capacity(hashes_count);
        for i in 0..hashes_count {
            if cell::store_hashes(raw_data) {
                result.push((cell::hash(raw_data, i).into(), cell::depth(raw_data, i)));
            } else {
                result.push(self.boc.hashes[self.info.hash_index as usize + i].clone());
            };
        }
        result
    }
}
