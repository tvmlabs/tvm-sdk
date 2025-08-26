use std::io::Seek;
use std::io::Write;
use std::sync::Arc;

use crate::Cell;
use crate::CellType;
use crate::DEPTH_SIZE;
use crate::LevelMask;
use crate::SHA256_SIZE;
use crate::UInt256;
use crate::boc::BOC_V3_TAG;
use crate::cell;
use crate::fail;

type Offset = u32;
const OFFSET_SIZE: usize = 4;

pub fn write_boc3_to_bytes(root_cells: &[Cell]) -> Result<Vec<u8>, failure::Error> {
    let mut result = Vec::new();
    write_boc3(&mut std::io::Cursor::new(&mut result), root_cells)?;
    Ok(result)
}

trait WriteInts {
    fn write_u32_be(&mut self, value: u32) -> std::io::Result<()>;
    fn write_u8(&mut self, value: u8) -> std::io::Result<()>;
}

impl<W: Write> WriteInts for W {
    fn write_u32_be(&mut self, value: u32) -> std::io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_u8(&mut self, value: u8) -> std::io::Result<()> {
        self.write_all(&[value])
    }
}

pub fn write_boc3<W: Write + Seek>(
    writer: &mut W,
    root_cells: &[Cell],
) -> Result<(), failure::Error> {
    writer.write_u32_be(BOC_V3_TAG)?;
    writer.write_u32_be(root_cells.len() as u32)?;

    let root_offsets_start = writer.stream_position()?;
    for _ in 0..root_cells.len() {
        writer.write_u32_be(0)?;
    }
    let mut offset = 4 + 4 + (OFFSET_SIZE * root_cells.len()) as u32;

    let mut root_offsets = Vec::with_capacity(root_cells.len());
    for cell in root_cells {
        root_offsets.push(write_cell_tree(writer, &mut offset, cell.clone())?);
    }

    let end = writer.stream_position()?;
    writer.seek(std::io::SeekFrom::Start(root_offsets_start))?;
    for offset in root_offsets {
        writer.write_u32_be(offset)?;
    }
    writer.seek(std::io::SeekFrom::Start(end))?;
    Ok(())
}

fn write_cell_tree<W: Write>(
    writer: &mut W,
    offset: &mut u32,
    cell: Cell,
) -> Result<Offset, failure::Error> {
    struct Processing {
        cell: Cell,
        children_processed: usize,
        children_count: usize,
        child_offsets: [u32; 4],
    }

    impl Processing {
        fn new(cell: Cell) -> Self {
            let children_count = cell.references_count();
            Self { cell, children_processed: 0, children_count, child_offsets: [0u32; 4] }
        }
    }

    let mut stack = Vec::new();
    let mut return_offset = 0;

    stack.push(Processing::new(cell));

    while let Some(current) = stack.last_mut() {
        if current.children_processed < current.children_count {
            // Still need to process next child
            let child = Processing::new(current.cell.reference(current.children_processed)?);
            stack.push(child);
        } else {
            // All children processed now write the current cell
            let raw_data = current.cell.raw_data()?;
            let cell_offset = *offset;

            if cell::is_big_cell(raw_data) {
                write_big_cell(writer, offset, &current.cell, raw_data)?;
            } else {
                write_non_big_cell(
                    writer,
                    offset,
                    &current.cell,
                    raw_data,
                    &current.child_offsets,
                    current.children_count,
                )?;
            }

            stack.pop();
            return_offset = cell_offset;

            if let Some(parent) = stack.last_mut() {
                parent.child_offsets[parent.children_processed] = cell_offset;
                parent.children_processed += 1;
            }
        }
    }

    Ok(return_offset)
}

fn write_big_cell<W: Write>(
    writer: &mut W,
    offset: &mut u32,
    cell: &Cell,
    raw_data: &[u8],
) -> Result<(), failure::Error> {
    writer.write_all(raw_data)?;
    *offset += raw_data.len() as u32;
    writer.write_all(cell.hash(0).as_slice())?;
    *offset += SHA256_SIZE as u32;
    Ok(())
}

fn write_non_big_cell<W: Write>(
    writer: &mut W,
    offset: &mut u32,
    cell: &Cell,
    raw_data: &[u8],
    child_offsets: &[u32; 4],
    child_count: usize,
) -> Result<(), failure::Error> {
    if cell::store_hashes(raw_data) {
        let len = cell::full_len(raw_data);
        writer.write_all(&raw_data[0..len])?;
        *offset += len as u32;
    } else {
        write_with_hashes(writer, offset, cell, raw_data)?;
    }
    #[allow(clippy::needless_range_loop)]
    for i in 0..child_count {
        writer.write_u32_be(child_offsets[i])?;
    }
    writer.write_u32_be(cell.tree_cell_count() as u32)?;
    writer.write_u32_be(cell.tree_bits_count() as u32)?;
    *offset += (child_count * OFFSET_SIZE + 4 + 4) as u32;
    Ok(())
}

fn write_with_hashes<W: Write>(
    writer: &mut W,
    offset: &mut u32,
    cell: &Cell,
    raw_data: &[u8],
) -> Result<(), failure::Error> {
    writer.write_u8(raw_data[0] | cell::HASHES_D1_FLAG)?;
    writer.write_u8(raw_data[1])?;

    let hashes_count: usize;
    if cell::cell_type(raw_data) == CellType::PrunedBranch {
        hashes_count = 1;
        writer.write_all(cell.repr_hash().as_slice())?;
        writer.write_all(&cell.repr_depth().to_be_bytes())?;
    } else {
        hashes_count = cell::hashes_count(raw_data);
        for i in 0..hashes_count {
            writer.write_all(cell.hash(i).as_slice())?;
        }
        for i in 0..hashes_count {
            writer.write_all(&cell.depth(i).to_be_bytes())?;
        }
    }
    let data = cell::cell_data(raw_data);
    writer.write_all(data)?;
    *offset += (2 + hashes_count * (DEPTH_SIZE + SHA256_SIZE) + data.len()) as u32;
    Ok(())
}

pub fn read_boc3_bytes(data: Arc<Vec<u8>>, boc_offset: usize) -> Result<Vec<Cell>, failure::Error> {
    let magic = get_u32_checked(&data, boc_offset)?;
    if magic != BOC_V3_TAG {
        fail!("Invalid BOC3 magic: {}", magic);
    }
    let root_count = get_u32_checked(&data, boc_offset + 4)? as usize;
    let mut root_cells = Vec::with_capacity(root_count);
    for i in 0..root_count {
        let cell_rel_offset = get_u32_checked(&data, boc_offset + 4 + 4 + i * 4)?;
        root_cells.push(Cell::with_boc3(Boc3Cell::new(
            data.clone(),
            boc_offset as Offset,
            cell_rel_offset,
        )));
    }
    Ok(root_cells)
}

fn get_u32_checked(buf: &[u8], offset: usize) -> Result<u32, failure::Error> {
    if buf.len() - offset >= 4 {
        let mut offset_buf = [0u8; 4];
        offset_buf.copy_from_slice(&buf[offset..offset + 4]);
        Ok(u32::from_be_bytes(offset_buf))
    } else {
        fail!("Failed to get offset: buf len less than 4")
    }
}

pub(crate) fn get_be_int(buf: &[u8], offset: usize, len: usize) -> usize {
    let available = buf.len().saturating_sub(offset);
    let take = len.min(available).min(8);
    let mut padded = [0u8; 8];
    padded[8 - take..].copy_from_slice(&buf[offset..offset + take]);
    u64::from_be_bytes(padded) as usize
}

fn raw_cell_len(raw_data: &[u8]) -> usize {
    if cell::is_big_cell(raw_data) {
        4 + get_be_int(raw_data, 1, 3)
    } else {
        cell::full_len(raw_data) + OFFSET_SIZE * cell::refs_count(raw_data)
    }
}

#[derive(Clone)]
pub struct Boc3Cell {
    data: Arc<Vec<u8>>,
    boc_offset: Offset,
    cell_offset: Offset,
}

impl Boc3Cell {
    pub fn new(data: Arc<Vec<u8>>, boc_offset: Offset, cell_rel_offset: Offset) -> Self {
        Self { data, boc_offset, cell_offset: boc_offset + cell_rel_offset }
    }

    fn unbounded_raw_data(&self) -> &[u8] {
        &self.data[self.cell_offset as usize..]
    }

    fn bounded_raw_data(&self) -> &[u8] {
        let unbounded = self.unbounded_raw_data();
        &unbounded[..raw_cell_len(unbounded)]
    }

    pub(crate) fn data(&self) -> &[u8] {
        cell::cell_data(self.unbounded_raw_data())
    }

    pub(crate) fn raw_data(&self) -> crate::Result<&[u8]> {
        Ok(self.bounded_raw_data())
    }

    pub(crate) fn bit_length(&self) -> usize {
        cell::bit_len(self.unbounded_raw_data())
    }

    pub(crate) fn references_count(&self) -> usize {
        cell::refs_count(self.unbounded_raw_data())
    }

    pub(crate) fn reference(&self, index: usize) -> crate::Result<Cell> {
        let raw_data = self.unbounded_raw_data();
        let refs_count = cell::refs_count(raw_data);
        if index >= refs_count {
            fail!("reference out of range, cells_count: {}, ref: {}", refs_count, index,)
        }
        let child_offset =
            get_u32_checked(raw_data, cell::full_len(raw_data) + index * OFFSET_SIZE)?;
        Ok(Cell::with_boc3(Boc3Cell::new(self.data.clone(), self.boc_offset, child_offset)))
    }

    pub(crate) fn cell_type(&self) -> CellType {
        cell::cell_type(self.unbounded_raw_data())
    }

    pub(crate) fn level_mask(&self) -> LevelMask {
        cell::level_mask(self.unbounded_raw_data())
    }

    pub(crate) fn hash(&self, index: usize) -> UInt256 {
        let raw_data = self.unbounded_raw_data();
        if cell::is_big_cell(raw_data) {
            let hash_offset = cell::full_len(raw_data);
            return raw_data[hash_offset..hash_offset + SHA256_SIZE].into();
        }
        let mut index = cell::level_mask(raw_data).calc_hash_index(index);
        let cell_type = cell::cell_type(raw_data);
        if cell_type == CellType::PrunedBranch {
            // pruned cell stores all hashes (except representation) in data
            if index != cell::level(raw_data) as usize {
                let offset = 1 + 1 + index * SHA256_SIZE;
                return cell::cell_data(raw_data)[offset..offset + SHA256_SIZE].into();
            } else {
                index = 0;
            }
        }
        // external cell has only representation hash
        if cell_type == CellType::UnloadedAccount {
            let offset = 1;
            return cell::cell_data(raw_data)[offset..offset + SHA256_SIZE].into();
        }
        cell::hash(raw_data, index).into()
    }

    pub(crate) fn depth(&self, index: usize) -> u16 {
        let cell_raw = self.unbounded_raw_data();
        if cell::is_big_cell(cell_raw) {
            return 1;
        }
        let mut index = cell::level_mask(cell_raw).calc_hash_index(index);
        let cell_type = cell::cell_type(cell_raw);
        if cell_type == CellType::PrunedBranch {
            // pruned cell stores all hashes (except representation) in data
            if index != cell::level(cell_raw) as usize {
                let offset =
                    1 + 1 + (cell::level(cell_raw) as usize) * SHA256_SIZE + index * DEPTH_SIZE;
                let data = cell::cell_data(cell_raw);
                return ((data[offset] as u16) << 8) | (data[offset + 1] as u16);
            } else {
                index = 0;
            }
        }
        // external cell has only representation hash
        if cell_type == CellType::UnloadedAccount {
            let offset = 1 + SHA256_SIZE;
            let data = cell::cell_data(cell_raw);
            return ((data[offset] as u16) << 8) | (data[offset + 1] as u16);
        }
        cell::depth(cell_raw, index)
    }

    pub(crate) fn store_hashes(&self) -> bool {
        true
    }

    pub(crate) fn tree_bits_count(&self) -> u64 {
        let raw_data = self.unbounded_raw_data();
        if cell::is_big_cell(raw_data) {
            cell::cell_data_len(raw_data) as u64 * 8
        } else {
            get_u32_checked(raw_data, stats_offset(raw_data) + 4).unwrap_or(0) as u64
        }
    }

    pub(crate) fn tree_cell_count(&self) -> u64 {
        let raw_data = self.unbounded_raw_data();
        if cell::is_big_cell(raw_data) {
            1
        } else {
            get_u32_checked(raw_data, stats_offset(raw_data)).unwrap_or(0) as u64
        }
    }
}

fn stats_offset(raw_data: &[u8]) -> usize {
    cell::full_len(raw_data) + cell::refs_count(raw_data) * OFFSET_SIZE
}
