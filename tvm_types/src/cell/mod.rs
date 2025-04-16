// Copyright (C) 2019-2023 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::cmp::min;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};
use std::io::Write;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crate::fail;
use crate::types::Result;
use crate::types::UInt256;

mod boc_cell;
mod usage_cell;
pub use boc_cell::{BocBuf, BocCell};
pub use data_cell::DataCell;
pub use usage_cell::UsageTree;

pub const SHA256_SIZE: usize = 32;
pub const DEPTH_SIZE: usize = 2;
pub const MAX_REFERENCES_COUNT: usize = 4;
pub const MAX_DATA_BITS: usize = 1023;
pub const MAX_DATA_BYTES: usize = 128; // including tag
pub const MAX_BIG_DATA_BYTES: usize = 0xff_ff_ff; // 1024 * 1024 * 16 - 1
pub const MAX_LEVEL: usize = 3;
pub const MAX_LEVEL_MASK: u8 = 7;
pub const MAX_DEPTH: u16 = u16::MAX - 1;

// type (1) + hash (256) + depth (2) + (tree cells count len | tree bits count
// len) (1) + max tree cells count (1) + max tree bits count (1)
const EXTERNAL_CELL_MIN_SIZE: usize = 1 + SHA256_SIZE + 2 + 1 + 2;
// type (1) + hash (256) + depth (2) + (tree cells count len | tree bits count
// len) (1) + max tree cells count (8) + max tree bits count (8)
const EXTERNAL_CELL_MAX_SIZE: usize = 1 + SHA256_SIZE + 2 + 1 + 8 * 2;

// recommended maximum depth, this value is safe for stack. Use custom stack
// size to use bigger depths (see `test_max_depth`).
pub const MAX_SAFE_DEPTH: u16 = 2048;

#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Clone,
    Copy,
    Hash,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
)]
pub enum CellType {
    Unknown,
    #[default]
    Ordinary,
    PrunedBranch,
    LibraryReference,
    MerkleProof,
    MerkleUpdate,
    Big,
    External,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
pub struct LevelMask(u8);

impl LevelMask {
    pub fn with_level(level: u8) -> Self {
        LevelMask(match level {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 7,
            _ => {
                log::error!("{} {}", file!(), line!());
                0
            }
        })
    }

    pub fn is_valid(mask: u8) -> bool {
        mask <= 7
    }

    pub fn with_mask(mask: u8) -> Self {
        if Self::is_valid(mask) {
            LevelMask(mask)
        } else {
            log::error!("{} {}", file!(), line!());
            LevelMask(0)
        }
    }

    pub fn for_merkle_cell(children_mask: LevelMask) -> Self {
        LevelMask(children_mask.0 >> 1)
    }

    pub fn level(&self) -> u8 {
        if !Self::is_valid(self.0) {
            log::error!("{} {}", file!(), line!());
            255
        } else {
            // count of set bits (low three)
            (self.0 & 1) + ((self.0 >> 1) & 1) + ((self.0 >> 2) & 1)
        }
    }

    pub fn mask(&self) -> u8 {
        self.0
    }

    // if cell contains required hash() - it will be returned,
    // else = max avaliable, but less then index
    //
    // rows - cell mask
    //       0(0)  1(1)  2(3)  3(7)  columns - index(mask)
    // 000     0     0     0     0     cells - index(AND result)
    // 001     0     1(1)  1(1)  1(1)
    // 010     0     0(0)  1(2)  1(2)
    // 011     0     1(1)  2(3)  2(3)
    // 100     0     0(0)  0(0)  1(4)
    // 101     0     1(1)  0(0)  2(5)
    // 110     0     0(0)  1(2)  2(6)
    // 111     0     1(1)  2(3)  3(7)
    pub fn calc_hash_index(&self, mut index: usize) -> usize {
        index = min(index, 3);
        LevelMask::with_mask(self.0 & LevelMask::with_level(index as u8).0).level() as usize
    }

    pub fn calc_virtual_hash_index(&self, index: usize, virt_offset: u8) -> usize {
        LevelMask::with_mask(self.0 >> virt_offset).calc_hash_index(index)
    }

    pub fn virtualize(&self, virt_offset: u8) -> Self {
        LevelMask::with_mask(self.0 >> virt_offset)
    }

    pub fn is_significant_index(&self, index: usize) -> bool {
        index == 0 || self.0 & LevelMask::with_level(index as u8).0 != 0
    }
}

impl BitOr for LevelMask {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self {
        LevelMask::with_mask(self.0 | rhs.0)
    }
}

impl BitOrAssign for LevelMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Display for LevelMask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:03b}", self.0)
    }
}

impl TryFrom<u8> for CellType {
    type Error = crate::Error;

    fn try_from(num: u8) -> Result<CellType> {
        let typ = match num {
            1 => CellType::PrunedBranch,
            2 => CellType::LibraryReference,
            3 => CellType::MerkleProof,
            4 => CellType::MerkleUpdate,
            5 => CellType::Big,
            6 => CellType::External,
            0xff => CellType::Ordinary,
            _ => fail!("unknown cell type {}", num),
        };
        Ok(typ)
    }
}

impl From<CellType> for u8 {
    fn from(ct: CellType) -> u8 {
        match ct {
            CellType::Unknown => 0,
            CellType::Ordinary => 0xff,
            CellType::PrunedBranch => 1,
            CellType::LibraryReference => 2,
            CellType::MerkleProof => 3,
            CellType::MerkleUpdate => 4,
            CellType::Big => 5,
            CellType::External => 6,
        }
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            CellType::Ordinary => "Ordinary",
            CellType::PrunedBranch => "Pruned branch",
            CellType::LibraryReference => "Library reference",
            CellType::MerkleProof => "Merkle proof",
            CellType::MerkleUpdate => "Merkle update",
            CellType::Big => "Big",
            CellType::External => "External",
            CellType::Unknown => "Unknown",
        };
        f.write_str(msg)
    }
}

pub trait CellImpl: Sync + Send {
    fn data(&self) -> &[u8];
    fn raw_data(&self) -> Result<&[u8]>;
    fn bit_length(&self) -> usize;
    fn references_count(&self) -> usize;
    fn reference(&self, index: usize) -> Result<Cell>;
    fn reference_repr_hash(&self, index: usize) -> Result<UInt256> {
        Ok(self.reference(index)?.hash(MAX_LEVEL))
    }
    fn cell_type(&self) -> CellType;
    fn level_mask(&self) -> LevelMask;
    fn hash(&self, index: usize) -> UInt256;
    fn depth(&self, index: usize) -> u16;
    fn store_hashes(&self) -> bool;

    fn store_hashes_depths_len(&self) -> usize;
    fn store_hashes_depths(&self) -> Vec<(UInt256, u16)>;

    fn level(&self) -> u8 {
        self.level_mask().level()
    }

    fn is_merkle(&self) -> bool {
        self.cell_type() == CellType::MerkleProof || self.cell_type() == CellType::MerkleUpdate
    }

    fn is_pruned(&self) -> bool {
        self.cell_type() == CellType::PrunedBranch
    }

    fn tree_bits_count(&self) -> u64 {
        0
    }

    fn tree_cell_count(&self) -> u64 {
        0
    }

    fn virtualization(&self) -> u8 {
        0
    }

    fn usage_level(&self) -> u64 {
        0
    }

    fn is_usage_cell(&self) -> bool {
        false
    }

    fn downcast_usage(&self) -> Cell {
        unreachable!("Function can be called only for UsageCell")
    }

    fn to_external(&self) -> Result<Cell> {
        fail!("Cell can not be converted to external")
    }
}

pub struct Cell(Arc<dyn CellImpl>);

lazy_static::lazy_static! {
    pub(crate) static ref CELL_DEFAULT: Cell = Cell::with_data(DataCell::new());
    static ref CELL_COUNT: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    // static ref FINALIZATION_NANOS: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Self(self.0.clone())
        // match self {
        //     Cell::Boc(boc) => Cell::Boc(boc.clone()),
        //     Cell::Data(cell) => Cell::Data(cell.clone()),
        //     Cell::Dyn(cell) => Cell::with_dyn_arc(cell.clone()),
        // }
    }
}

impl Drop for Cell {
    fn drop(&mut self) {
        CELL_COUNT.fetch_sub(1, Ordering::Relaxed);
    }
}

impl Cell {
    pub fn virtualize(self, offset: u8) -> Self {
        if self.level_mask().mask() == 0 {
            self
        } else {
            Cell::with_dyn(VirtualCell::with_cell_and_offset(self, offset))
        }
    }

    pub fn virtualization(&self) -> u8 {
        self.0.virtualization()
        // match self {
        //     Cell::Boc(cell) => cell.virtualization(),
        //     Cell::Data(cell) => cell.virtualization(),
        //     Cell::Dyn(cell) => cell.virtualization(),
        // }
    }

    pub fn with_dyn<T: 'static + CellImpl>(cell_impl: T) -> Self {
        let ret = Cell(Arc::new(cell_impl));
        CELL_COUNT.fetch_add(1, Ordering::Relaxed);
        ret
    }

    pub fn with_dyn_arc(cell_impl: Arc<dyn CellImpl>) -> Self {
        let ret = Cell(cell_impl);
        CELL_COUNT.fetch_add(1, Ordering::Relaxed);
        ret
    }

    pub fn with_boc(cell: BocCell) -> Self {
        Self(Arc::new(cell))
        // Self::Boc(cell)
    }

    pub fn with_data(cell: DataCell) -> Self {
        Self(Arc::new(cell))
        // Self::Data(cell)
    }

    pub fn cell_count() -> u64 {
        CELL_COUNT.load(Ordering::Relaxed)
    }

    // pub fn finalization_nanos() -> u64 {
    //     FINALIZATION_NANOS.load(Ordering::Relaxed)
    // }

    pub fn reference(&self, index: usize) -> Result<Cell> {
        self.0.reference(index)
        // match self {
        //     Cell::Boc(cell) => cell.reference(index),
        //     Cell::Data(cell) => cell.reference(index),
        //     Cell::Dyn(cell) => cell.reference(index),
        // }
    }

    pub fn reference_repr_hash(&self, index: usize) -> Result<UInt256> {
        self.0.reference_repr_hash(index)
        // match self {
        //     Cell::Boc(cell) => cell.reference_repr_hash(index),
        //     Cell::Data(cell) => cell.reference_repr_hash(index),
        //     Cell::Dyn(cell) => cell.reference_repr_hash(index),
        // }
    }

    // TODO: make as simple clone
    pub fn clone_references(&self) -> SmallVec<[Cell; 4]> {
        let count = self.references_count();
        let mut refs = SmallVec::with_capacity(count);
        for i in 0..count {
            refs.push(self.reference(i).unwrap())
        }
        refs
    }

    pub fn data(&self) -> &[u8] {
        self.0.data()
        // match self {
        //     Cell::Boc(cell) => cell.data(),
        //     Cell::Data(cell) => cell.data(),
        //     Cell::Dyn(cell) => cell.data(),
        // }
    }

    pub fn raw_data(&self) -> Result<&[u8]> {
        self.0.raw_data()
        // match self {
        //     Cell::Boc(cell) => cell.raw_data(),
        //     Cell::Data(cell) => cell.raw_data(),
        //     Cell::Dyn(cell) => cell.raw_data(),
        // }
    }

    pub fn bit_length(&self) -> usize {
        self.0.bit_length()
        // match self {
        //     Cell::Boc(cell) => cell.bit_length(),
        //     Cell::Data(cell) => cell.bit_length(),
        //     Cell::Dyn(cell) => cell.bit_length(),
        // }
    }

    pub fn cell_type(&self) -> CellType {
        self.0.cell_type()
        // match self {
        //     Cell::Boc(cell) => cell.cell_type(),
        //     Cell::Data(cell) => cell.cell_type(),
        //     Cell::Dyn(cell) => cell.cell_type(),
        // }
    }

    pub fn level(&self) -> u8 {
        self.0.level()
        // match self {
        //     Cell::Boc(cell) => cell.level(),
        //     Cell::Data(cell) => cell.level(),
        //     Cell::Dyn(cell) => cell.level(),
        // }
    }

    pub fn hashes_count(&self) -> usize {
        self.level() as usize + 1
    }

    pub fn store_hashes_depths(&self) -> Vec<(UInt256, u16)> {
        self.0.store_hashes_depths()
        // match self {
        //     Cell::Boc(cell) => cell.store_hashes_depths(),
        //     Cell::Data(cell) => cell.store_hashes_depths(),
        //     Cell::Dyn(cell) => cell.store_hashes_depths(),
        // }
    }

    pub fn count_cells(&self, max: usize) -> Result<usize> {
        let mut count = 0;
        let mut queue = vec![self.clone()];
        while let Some(cell) = queue.pop() {
            if count >= max {
                fail!("count exceeds max {}", max)
            }
            count += 1;
            let count = cell.references_count();
            for i in 0..count {
                queue.push(cell.reference(i)?);
            }
        }
        Ok(count)
    }

    pub fn level_mask(&self) -> LevelMask {
        self.0.level_mask()
        // match self {
        //     Cell::Boc(cell) => cell.level_mask(),
        //     Cell::Data(cell) => cell.level_mask(),
        //     Cell::Dyn(cell) => cell.level_mask(),
        // }
    }

    pub fn references_count(&self) -> usize {
        self.0.references_count()
        // match self {
        //     Cell::Boc(cell) => cell.references_count(),
        //     Cell::Data(cell) => cell.references_count(),
        //     Cell::Dyn(cell) => cell.references_count(),
        // }
    }

    /// Returns cell's higher hash for given index (last one - representation
    /// hash)
    pub fn hash(&self, index: usize) -> UInt256 {
        self.0.hash(index)
        // match self {
        //     Cell::Boc(cell) => cell.hash(index),
        //     Cell::Data(cell) => cell.hash(index),
        //     Cell::Dyn(cell) => cell.hash(index),
        // }
    }

    /// Returns cell's depth for given index
    pub fn depth(&self, index: usize) -> u16 {
        self.0.depth(index)
        // match self {
        //     Cell::Boc(cell) => cell.depth(index),
        //     Cell::Data(cell) => cell.depth(index),
        //     Cell::Dyn(cell) => cell.depth(index),
        // }
    }

    /// Returns cell's hashes (representation and highers)
    pub fn hashes(&self) -> Vec<UInt256> {
        let mut hashes = Vec::new();
        let mut i = 0;
        while hashes.len() < self.level() as usize + 1 {
            if self.level_mask().is_significant_index(i) {
                hashes.push(self.hash(i))
            }
            i += 1;
        }
        hashes
    }

    /// Returns cell's depth (for current state and each level)
    pub fn depths(&self) -> Vec<u16> {
        let mut depths = Vec::new();
        let mut i = 0;
        while depths.len() < self.level() as usize + 1 {
            if self.level_mask().is_significant_index(i) {
                depths.push(self.depth(i))
            }
            i += 1;
        }
        depths
    }

    pub fn repr_hash(&self) -> UInt256 {
        self.0.hash(MAX_LEVEL)
        // match self {
        //     Self::Boc(cell) => cell.hash(MAX_LEVEL),
        //     Self::Data(cell) => cell.hash(MAX_LEVEL),
        //     Self::Dyn(cell) => cell.hash(MAX_LEVEL),
        // }
    }

    pub fn repr_depth(&self) -> u16 {
        self.0.depth(MAX_LEVEL)
        // match self {
        //     Self::Boc(cell) => cell.depth(MAX_LEVEL),
        //     Self::Data(cell) => cell.depth(MAX_LEVEL),
        //     Self::Dyn(cell) => cell.depth(MAX_LEVEL),
        // }
    }

    pub fn store_hashes(&self) -> bool {
        self.0.store_hashes()
        // match self {
        //     Self::Boc(cell) => cell.store_hashes(),
        //     Self::Data(cell) => cell.store_hashes(),
        //     Self::Dyn(cell) => cell.store_hashes(),
        // }
    }

    #[allow(dead_code)]
    pub fn is_merkle(&self) -> bool {
        self.0.is_merkle()
        // match self {
        //     Self::Boc(cell) => cell.is_merkle(),
        //     Self::Data(cell) => cell.is_merkle(),
        //     Self::Dyn(cell) => cell.is_merkle(),
        // }
    }

    #[allow(dead_code)]
    pub fn is_pruned(&self) -> bool {
        self.0.is_pruned()
        // match self {
        //     Self::Boc(cell) => cell.is_pruned(),
        //     Self::Data(cell) => cell.is_pruned(),
        //     Self::Dyn(cell) => cell.is_pruned(),
        // }
    }

    pub fn to_hex_string(&self, lower: bool) -> String {
        let bit_length = self.bit_length();
        if bit_length % 8 == 0 {
            if lower { hex::encode(self.data()) } else { hex::encode_upper(self.data()) }
        } else {
            to_hex_string(self.data(), self.bit_length(), lower)
        }
    }

    fn print_indent(
        f: &mut fmt::Formatter,
        indent: &str,
        last_child: bool,
        first_line: bool,
    ) -> fmt::Result {
        let build = match (first_line, last_child) {
            (true, true) => " └─",
            (true, false) => " ├─",
            (false, true) => "   ",
            (false, false) => " │ ",
        };
        write!(f, "{}{}", indent, build)
    }

    pub fn format_without_refs(
        &self,
        f: &mut fmt::Formatter,
        indent: &str,
        last_child: bool,
        full: bool,
        root: bool,
    ) -> fmt::Result {
        if !root {
            Self::print_indent(f, indent, last_child, true)?;
        }

        if self.cell_type() == CellType::Big {
            let data_len = self.data().len();
            write!(f, "Big   bytes: {}", data_len)?;
            if data_len > 100 {
                writeln!(f)?;
                if !root {
                    Self::print_indent(f, indent, last_child, false)?;
                }
            } else {
                write!(f, "   ")?;
            }
            if data_len < 128 {
                write!(f, "data: {}", hex::encode(self.data()))?;
            } else {
                write!(f, "data: {}...", hex::encode(&self.data()[..128]))?;
            }
            if full {
                writeln!(f)?;
                write!(f, "hash: {:x}", self.repr_hash())?;
            }
        } else {
            if full {
                write!(f, "{}   l: {:03b}   ", self.cell_type(), self.level_mask().mask())?;
            }

            write!(f, "bits: {}", self.bit_length())?;
            write!(f, "   refs: {}", self.references_count())?;

            if self.data().len() > 100 {
                writeln!(f)?;
                if !root {
                    Self::print_indent(f, indent, last_child, false)?;
                }
            } else {
                write!(f, "   ")?;
            }

            write!(f, "data: {}", self.to_hex_string(true))?;

            if full {
                writeln!(f)?;
                if !root {
                    Self::print_indent(f, indent, last_child, false)?;
                }
                write!(f, "hashes:")?;
                for h in self.hashes().iter() {
                    write!(f, " {:x}", h)?;
                }
                writeln!(f)?;
                if !root {
                    Self::print_indent(f, indent, last_child, false)?;
                }
                write!(f, "depths:")?;
                for d in self.depths().iter() {
                    write!(f, " {}", d)?;
                }
            }
        }
        Ok(())
    }

    pub fn format_with_refs_tree(
        &self,
        f: &mut fmt::Formatter,
        mut indent: String,
        last_child: bool,
        full: bool,
        root: bool,
        remaining_depth: u16,
    ) -> std::result::Result<String, fmt::Error> {
        self.format_without_refs(f, &indent, last_child, full, root)?;
        if remaining_depth > 0 {
            if !root {
                indent.push(' ');
                indent.push(if last_child { ' ' } else { '│' });
            }
            for i in 0..self.references_count() {
                let child = self.reference(i).unwrap();
                writeln!(f)?;
                indent = child.format_with_refs_tree(
                    f,
                    indent,
                    i == self.references_count() - 1,
                    full,
                    false,
                    remaining_depth - 1,
                )?;
            }
            if !root {
                indent.pop();
                indent.pop();
            }
        }
        Ok(indent)
    }

    pub fn tree_bits_count(&self) -> u64 {
        self.0.tree_bits_count()
        // match self {
        //     Cell::Boc(cell) => cell.tree_bits_count(),
        //     Cell::Data(cell) => cell.tree_bits_count(),
        //     Cell::Dyn(cell) => cell.tree_bits_count(),
        // }
    }

    pub fn tree_cell_count(&self) -> u64 {
        self.0.tree_cell_count()
        // match self {
        //     Cell::Boc(cell) => cell.tree_cell_count(),
        //     Cell::Data(cell) => cell.tree_cell_count(),
        //     Cell::Dyn(cell) => cell.tree_cell_count(),
        // }
    }

    pub fn to_external(&self) -> Result<Cell> {
        self.0.to_external()
        // match self {
        //     Cell::Boc(cell) => cell.to_external(),
        //     Cell::Data(cell) => cell.to_external(),
        //     Cell::Dyn(cell) => cell.to_external(),
        // }
    }

    pub fn usage_level(&self) -> u64 {
        self.0.usage_level()
        // match self {
        //     Cell::Boc(cell) => cell.usage_level(),
        //     Cell::Data(cell) => cell.usage_level(),
        //     Cell::Dyn(cell) => cell.usage_level(),
        // }
    }

    fn is_usage_cell(&self) -> bool {
        self.0.is_usage_cell()
        // match self {
        //     Cell::Boc(cell) => cell.is_usage_cell(),
        //     Cell::Data(cell) => cell.is_usage_cell(),
        //     Cell::Dyn(cell) => cell.is_usage_cell(),
        // }
    }

    fn downcast_usage(&self) -> Cell {
        self.0.downcast_usage()
        // match self {
        //     Cell::Boc(cell) => cell.downcast_usage(),
        //     Cell::Data(cell) => cell.downcast_usage(),
        //     Cell::Dyn(cell) => cell.downcast_usage(),
        // }
    }

    fn store_hashes_depths_len(&self) -> usize {
        self.0.store_hashes_depths_len()
        // match self {
        //     Cell::Boc(cell) => cell.store_hashes_depths_len(),
        //     Cell::Data(cell) => cell.store_hashes_depths_len(),
        //     Cell::Dyn(cell) => cell.store_hashes_depths_len(),
        // }
    }
}

impl Default for Cell {
    fn default() -> Self {
        CELL_DEFAULT.clone()
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.repr_hash() == other.repr_hash()
    }
}

impl PartialEq<UInt256> for Cell {
    fn eq(&self, other_hash: &UInt256) -> bool {
        &self.repr_hash() == other_hash
    }
}

impl Eq for Cell {}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.repr_hash())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format_with_refs_tree(
            f,
            "".to_string(),
            true,
            f.alternate(),
            true,
            min(f.precision().unwrap_or(0), MAX_DEPTH as usize) as u16,
        )?;
        Ok(())
    }
}

impl fmt::LowerHex for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex_string(true))
    }
}

impl fmt::UpperHex for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex_string(false))
    }
}

impl fmt::Binary for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bitlen = self.bit_length();
        if bitlen % 8 == 0 {
            write!(
                f,
                "{}",
                self.data().iter().map(|x| format!("{:08b}", *x)).collect::<Vec<_>>().join("")
            )
        } else {
            let data = self.data();
            for b in &data[..data.len() - 1] {
                write!(f, "{:08b}", b)?;
            }
            for i in (8 - (bitlen % 8)..8).rev() {
                write!(f, "{:b}", (data[data.len() - 1] >> i) & 1)?;
            }
            Ok(())
        }
    }
}

/// Calculates data's length in bits with respect to completion tag
pub fn find_tag(bitsting: &[u8]) -> usize {
    let mut length = bitsting.len() * 8;
    for x in bitsting.iter().rev() {
        if *x == 0 {
            length -= 8;
        } else {
            let mut skip = 1;
            let mut mask = 1;
            while (*x & mask) == 0 {
                skip += 1;
                mask <<= 1
            }
            length -= skip;
            break;
        }
    }
    length
}

pub fn append_tag(data: &mut SmallVec<[u8; 128]>, bits: usize) {
    let shift = bits % 8;
    if shift == 0 || data.is_empty() {
        data.truncate(bits / 8);
        data.push(0x80);
    } else {
        data.truncate(1 + bits / 8);
        let mut last_byte = data.pop().unwrap();
        if shift != 7 {
            last_byte >>= 7 - shift;
        }
        last_byte |= 1;
        if shift != 7 {
            last_byte <<= 7 - shift;
        }
        data.push(last_byte);
    }
}

// Cell layout:
// [D1] [D2] [data: 0..128 bytes] (hashes: 0..4 big endian u256) (depths: 0..4
// big endian u16) first byte is so called desription byte 1:
// | level mask| store hashes| exotic| refs count|
// |      7 6 5|            4|      3|      2 1 0|
pub(crate) const LEVELMASK_D1_OFFSET: usize = 5;
pub(crate) const HASHES_D1_FLAG: u8 = 16;
pub(crate) const EXOTIC_D1_FLAG: u8 = 8;
pub(crate) const REFS_D1_MASK: u8 = 7;
pub(crate) const BIG_CELL_D1: u8 = 13; // 0b0000_1101
// next byte is desription byte 2 contains data size (in special encoding, see
// cell_data_len)

#[inline(always)]
pub(crate) fn calc_d1(
    level_mask: LevelMask,
    store_hashes: bool,
    cell_type: CellType,
    refs_count: usize,
) -> u8 {
    (level_mask.mask() << LEVELMASK_D1_OFFSET)
        | (store_hashes as u8 * HASHES_D1_FLAG)
        | ((cell_type != CellType::Ordinary) as u8 * EXOTIC_D1_FLAG)
        | refs_count as u8
}

#[inline(always)]
pub(crate) fn calc_d2(data_bit_len: usize) -> u8 {
    ((data_bit_len / 8) << 1) as u8 + (data_bit_len % 8 != 0) as u8
}

// A lot of helper-functions which incapsulates cell's layout.
// All this functions (except returning Result) can panic in case of going out
// of slice bounds.
#[inline(always)]
pub(crate) fn level(buf: &[u8]) -> u8 {
    level_mask(buf).level()
}

#[inline(always)]
pub(crate) fn level_mask(buf: &[u8]) -> LevelMask {
    debug_assert!(!buf.is_empty());
    LevelMask::with_mask(buf[0] >> LEVELMASK_D1_OFFSET)
}

#[inline(always)]
pub(crate) fn store_hashes(buf: &[u8]) -> bool {
    if is_big_cell(buf) {
        false
    } else {
        debug_assert!(!buf.is_empty());
        (buf[0] & HASHES_D1_FLAG) == HASHES_D1_FLAG
    }
}

#[inline(always)]
pub(crate) fn exotic(buf: &[u8]) -> bool {
    debug_assert!(!buf.is_empty());
    (buf[0] & EXOTIC_D1_FLAG) == EXOTIC_D1_FLAG
}

#[inline(always)]
pub(crate) fn cell_type(buf: &[u8]) -> CellType {
    // exotic?
    if !exotic(buf) {
        // no
        CellType::Ordinary
    } else if is_big_cell(buf) {
        CellType::Big
    } else {
        match cell_data(buf).first() {
            Some(byte) => CellType::try_from(*byte).unwrap_or(CellType::Unknown),
            None => {
                debug_assert!(false, "empty exotic cell data");
                CellType::Unknown
            }
        }
    }
}

#[inline(always)]
pub(crate) fn refs_count(buf: &[u8]) -> usize {
    if is_big_cell(buf) {
        0
    } else {
        debug_assert!(!buf.is_empty());
        (buf[0] & REFS_D1_MASK) as usize
    }
}

#[inline(always)]
pub(crate) fn is_big_cell(buf: &[u8]) -> bool {
    debug_assert!(!buf.is_empty());
    buf[0] == BIG_CELL_D1
}

#[inline(always)]
pub(crate) fn cell_data_len(buf: &[u8]) -> usize {
    if is_big_cell(buf) {
        debug_assert!(buf.len() >= 4);
        ((buf[1] as usize) << 16) | ((buf[2] as usize) << 8) | buf[3] as usize
    } else {
        debug_assert!(buf.len() >= 2);
        ((buf[1] >> 1) + (buf[1] & 1)) as usize
    }
}

#[inline(always)]
pub(crate) fn bit_len(buf: &[u8]) -> usize {
    if is_big_cell(buf) {
        debug_assert!(buf.len() >= 4);
        let bytes = ((buf[1] as usize) << 16) | ((buf[2] as usize) << 8) | buf[3] as usize;
        bytes * 8
    } else {
        debug_assert!(buf.len() >= 2);
        if buf[1] & 1 == 0 { (buf[1] >> 1) as usize * 8 } else { find_tag(cell_data(buf)) }
    }
}

#[inline(always)]
pub(crate) fn data_offset(buf: &[u8]) -> usize {
    if is_big_cell(buf) {
        4
    } else {
        2 + (store_hashes(buf) as usize) * hashes_count(buf) * (SHA256_SIZE + DEPTH_SIZE)
    }
}

#[inline(always)]
pub(crate) fn cell_data(buf: &[u8]) -> &[u8] {
    let data_offset = data_offset(buf);
    let cell_data_len = cell_data_len(buf);
    debug_assert!(buf.len() >= data_offset + cell_data_len);
    &buf[data_offset..data_offset + cell_data_len]
}

#[inline(always)]
pub(crate) fn hashes_count(buf: &[u8]) -> usize {
    // Hashes count depends on cell's type and level
    // - for pruned branch it's always 1
    // - for other types it's level + 1
    // To get cell type we need to calculate data's offset, but we can't do it
    // without hashes_count. So we will recognise pruned branch cell by some
    // indirect signs - 0 refs and level != 0

    if exotic(buf) && refs_count(buf) == 0 && level(buf) != 0 {
        // pruned branch
        1
    } else {
        level(buf) as usize + 1
    }
}

#[inline(always)]
pub(crate) fn full_len(buf: &[u8]) -> usize {
    data_offset(buf) + cell_data_len(buf)
}

#[inline(always)]
pub(crate) fn hashes_len(buf: &[u8]) -> usize {
    hashes_count(buf) * SHA256_SIZE
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn hashes(buf: &[u8]) -> &[u8] {
    debug_assert!(store_hashes(buf));
    let hashes_len = hashes_len(buf);
    debug_assert!(buf.len() >= 2 + hashes_len);
    &buf[2..2 + hashes_len]
}

#[inline(always)]
pub(crate) fn hash(buf: &[u8], index: usize) -> &[u8] {
    debug_assert!(store_hashes(buf));
    let offset = 2 + index * SHA256_SIZE;
    debug_assert!(buf.len() >= offset + SHA256_SIZE);
    &buf[offset..offset + SHA256_SIZE]
}

#[inline(always)]
pub(crate) fn depths_offset(buf: &[u8]) -> usize {
    2 + hashes_len(buf)
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn depths_len(buf: &[u8]) -> usize {
    hashes_count(buf) * DEPTH_SIZE
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn depths(buf: &[u8]) -> &[u8] {
    debug_assert!(store_hashes(buf));
    let offset = depths_offset(buf);
    let depths_len = depths_len(buf);
    debug_assert!(buf.len() >= offset + depths_len);
    &buf[offset..offset + depths_len]
}

#[inline(always)]
pub(crate) fn depth(buf: &[u8], index: usize) -> u16 {
    debug_assert!(store_hashes(buf));
    let offset = depths_offset(buf) + index * DEPTH_SIZE;
    let d = &buf[offset..offset + DEPTH_SIZE];
    ((d[0] as u16) << 8) | (d[1] as u16)
}

fn build_big_cell_buf(
    data: &[u8], // without completion tag, all data will use as cell's data
    level_mask: u8,
    refs: usize,
    store_hashes: bool,
    hashes: Option<[UInt256; 4]>,
    depths: Option<[u16; 4]>,
) -> Result<Vec<u8>> {
    if level_mask != 0 {
        fail!("Big cell must have level_mask 0");
    }
    if refs != 0 {
        fail!("Big cell must have 0 refs");
    }
    if store_hashes | hashes.is_some() | depths.is_some() {
        fail!("Big cell doesn't support stored hashes");
    }
    if data.len() > MAX_BIG_DATA_BYTES {
        fail!("Data is too big for big cell: {} > {}", data.len(), MAX_BIG_DATA_BYTES);
    }

    let full_len = 4 + data.len();
    let mut buf = Vec::with_capacity(full_len);
    buf.write_all(&[BIG_CELL_D1])?;
    buf.write_all(&data.len().to_be_bytes()[5..8])?;
    buf.write_all(data)?;

    Ok(buf)
}

fn build_cell_buf(
    cell_type: CellType,
    data: &[u8], // with completion tag
    level_mask: u8,
    refs: usize,
    store_hashes: bool,
    hashes: Option<[UInt256; 4]>,
    depths: Option<[u16; 4]>,
) -> Result<Vec<u8>> {
    if cell_type == CellType::Big {
        fail!("CellType::Big is not supported, use build_big_cell_buf function instead");
    }
    if cell_type != CellType::Ordinary && data.len() == 1 {
        fail!("Exotic cell can't have empty data");
    }
    if data.len() > MAX_DATA_BYTES {
        fail!("Cell's data can't has {} length", data.len());
    }
    if refs > MAX_REFERENCES_COUNT {
        fail!("Cell can't has {} refs", refs);
    }
    if level_mask > MAX_LEVEL_MASK {
        fail!("Level mask can't be {}", level_mask);
    }

    let data_bit_len = find_tag(data);
    let data_len = (data_bit_len / 8) + (data_bit_len % 8 != 0) as usize;
    let level_mask = LevelMask::with_mask(level_mask);
    let level = level_mask.level();
    let hashes_count = if store_hashes {
        if cell_type == CellType::PrunedBranch { 1 } else { level as usize + 1 }
    } else {
        0
    };
    let full_length = 2 + data_len + hashes_count * (SHA256_SIZE + DEPTH_SIZE);

    debug_assert!(refs <= MAX_REFERENCES_COUNT);
    debug_assert!(data.len() <= MAX_DATA_BYTES);
    debug_assert!(hashes.is_some() == depths.is_some());
    debug_assert!(level_mask.mask() <= MAX_LEVEL_MASK);
    debug_assert!(data.len() >= data_len);

    let mut buf = vec![0; full_length];
    buf[0] = calc_d1(level_mask, store_hashes, cell_type, refs);
    buf[1] = calc_d2(data_bit_len);
    let mut offset = 2;
    if store_hashes {
        if hashes.is_none() || depths.is_none() {
            fail!("`hashes` or `depths` can't be none while `store_hashes` is true");
        }
        if let Some(hashes) = hashes {
            for hash in hashes.iter().take(hashes_count) {
                buf[offset..offset + SHA256_SIZE].copy_from_slice(hash.as_slice());
                offset += SHA256_SIZE;
            }
        }
        if let Some(depths) = depths {
            for depth in depths.iter().take(hashes_count) {
                buf[offset] = (depth >> 8) as u8;
                buf[offset + 1] = (depth & 0xff) as u8;
                offset += DEPTH_SIZE;
            }
        }
    }
    buf[offset..offset + data_len].copy_from_slice(&data[..data_len]);
    Ok(buf)
}

#[inline(always)]
fn set_hash(buf: &mut [u8], index: usize, hash: &[u8]) {
    debug_assert!(index <= level(buf) as usize);
    debug_assert!(hash.len() == SHA256_SIZE);
    let offset = 2 + index * SHA256_SIZE;
    debug_assert!(buf.len() >= offset + SHA256_SIZE);
    buf[offset..offset + SHA256_SIZE].copy_from_slice(hash);
}

#[inline(always)]
fn set_depth(buf: &mut [u8], index: usize, depth: u16) {
    debug_assert!(index <= level(buf) as usize);
    let offset = depths_offset(buf) + index * DEPTH_SIZE;
    debug_assert!(buf.len() >= offset + DEPTH_SIZE);
    buf[offset] = (depth >> 8) as u8;
    buf[offset + 1] = (depth & 0xff) as u8;
}

fn check_cell_buf(buf: &[u8], unbounded: bool) -> Result<()> {
    if buf.len() < 2 {
        fail!("Buffer is too small to read description bytes")
    }

    if is_big_cell(buf) {
        if buf.len() < 4 {
            fail!("Buffer is too small to read big cell's length (min 4 bytes)");
        }
        let full_data_len = full_len(buf);
        if buf.len() < full_data_len {
            fail!("buf is too small ({}) to fit this big cell ({})", buf.len(), full_data_len);
        }
        if !unbounded && buf.len() > full_data_len {
            fail!("buf is too big ({}) for this big cell ({})", buf.len(), full_data_len);
        }
    } else {
        let refs_count = refs_count(buf);
        if refs_count > MAX_REFERENCES_COUNT {
            fail!("Too big references count: {}", refs_count);
        }

        let full_data_len = full_len(buf);
        if buf.len() < full_data_len {
            fail!("Buffer is too small ({}) to fit cell ({})", buf.len(), full_data_len);
        }
        if !unbounded && buf.len() > full_data_len {
            log::warn!(
                "Buffer is too big ({}), needed only {} to fit cell",
                buf.len(),
                full_data_len
            );
        }

        let cell_data = cell_data(buf);
        if exotic(buf) && cell_data.is_empty() {
            fail!("exotic cells must have non zero data length")
        }
        let data_bit_len = bit_len(buf);
        let expected_len = data_bit_len / 8 + (data_bit_len % 8 != 0) as usize;
        if cell_data.len() != expected_len {
            log::warn!(
                "Data len wrote in description byte 2 ({} bytes) does not correspond to real length \
                calculated by tag ({} bytes, {} bits, data: {})",
                cell_data.len(),
                expected_len,
                data_bit_len,
                hex::encode(cell_data)
            );
        }
    }

    Ok(())
}

mod slice;

pub use self::slice::*;

pub mod builder;

pub use self::builder::*;

mod builder_operations;
mod cell_data;
mod data_cell;
mod virtual_cell;

pub use self::builder_operations::*;
use smallvec::SmallVec;
use virtual_cell::VirtualCell;

pub(crate) fn to_hex_string(data: impl AsRef<[u8]>, len: usize, lower: bool) -> String {
    if len == 0 {
        return String::new();
    }
    let mut result = if lower { hex::encode(data) } else { hex::encode_upper(data) };
    match len % 8 {
        0 => {
            result.pop();
            result.pop();
        }
        1..=3 => {
            result.pop();
            result.push('_')
        }
        4 => {
            result.pop();
        }
        _ => result.push('_'),
    }
    result
}

pub fn create_cell(
    references: Vec<Cell>,
    data: &[u8], // with completion tag (for big cell - without)!
) -> Result<Cell> {
    Ok(Cell::with_dyn(DataCell::with_refs_and_data(references, data)?))
}

pub fn create_big_cell(data: &[u8]) -> Result<Cell> {
    Ok(Cell::with_dyn(DataCell::with_params(vec![], data, CellType::Big, 0, None, None, None)?))
}
