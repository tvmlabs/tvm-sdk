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

mod boc3_cell;
mod usage_cell;
pub use boc3_cell::Boc3Cell;
pub use boc3_cell::read_boc3_bytes;
pub use boc3_cell::write_boc3;
pub use boc3_cell::write_boc3_to_bytes;
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
// len) (1) + min tree cells count (1) + min tree bits count (1)
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

impl CellType {
    pub const BIG: u8 = 5;
    pub const EXTERNAL: u8 = 6;
    pub const LIBRARY_REFERENCE: u8 = 2;
    pub const MERKLE_PROOF: u8 = 3;
    pub const MERKLE_UPDATE: u8 = 4;
    pub const ORDINARY: u8 = 0xff;
    pub const PRUNED_BRANCH: u8 = 1;
    pub const UNKNOWN: u8 = 0;

    fn is_merkle(&self) -> bool {
        *self == CellType::MerkleProof || *self == CellType::MerkleUpdate
    }

    fn is_pruned(&self) -> bool {
        *self == CellType::PrunedBranch
    }
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
            Self::PRUNED_BRANCH => Self::PrunedBranch,
            Self::LIBRARY_REFERENCE => Self::LibraryReference,
            Self::MERKLE_PROOF => Self::MerkleProof,
            Self::MERKLE_UPDATE => Self::MerkleUpdate,
            Self::BIG => Self::Big,
            Self::EXTERNAL => Self::External,
            Self::ORDINARY => Self::Ordinary,
            _ => fail!("unknown cell type {}", num),
        };
        Ok(typ)
    }
}

impl From<CellType> for u8 {
    fn from(ct: CellType) -> u8 {
        match ct {
            CellType::Unknown => CellType::UNKNOWN,
            CellType::Ordinary => CellType::ORDINARY,
            CellType::PrunedBranch => CellType::PRUNED_BRANCH,
            CellType::LibraryReference => CellType::LIBRARY_REFERENCE,
            CellType::MerkleProof => CellType::MERKLE_PROOF,
            CellType::MerkleUpdate => CellType::MERKLE_UPDATE,
            CellType::Big => CellType::BIG,
            CellType::External => CellType::EXTERNAL,
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

pub enum Cell {
    Data(Arc<DataCell>),
    Virtual(Arc<VirtualCell>),
    Usage(Arc<UsageCell>),

    Boc3(Boc3Cell), // Experimental, it is not used in production
}

lazy_static::lazy_static! {
    pub(crate) static ref CELL_DEFAULT: Cell = Cell::with_data(DataCell::new());
    static ref CELL_COUNT: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    // static ref FINALIZATION_NANOS: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
}

impl std::cmp::PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(std::cmp::Ordering::is_lt)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(std::cmp::Ordering::is_le)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(std::cmp::Ordering::is_gt)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(std::cmp::Ordering::is_ge)
    }
}

impl std::cmp::Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if other < self { self } else { other }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if other < self { other } else { self }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > std::cmp::max {
            std::cmp::max
        } else {
            self
        }
    }
}

// impl std::hash::Hash for Cell {
//     // TODO
//     fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
//     where
//         Self: Sized,
//     {
//         for piece in data {
//             piece.hash(state);
//         }
//     }

//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         core::mem::discriminant(self).hash(state);
//     }
// }

impl Clone for Cell {
    fn clone(&self) -> Self {
        match self {
            Cell::Data(cell) => Cell::Data(cell.clone()),
            Cell::Usage(cell) => Cell::Usage(cell.clone()),
            Cell::Virtual(cell) => Cell::Virtual(cell.clone()),
            Cell::Boc3(boc) => Cell::Boc3(boc.clone()),
        }
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
            Cell::with_virtual(VirtualCell::with_cell_and_offset(self, offset))
        }
    }

    pub fn virtualization(&self) -> u8 {
        match self {
            Cell::Virtual(cell) => cell.virtualization(),
            _ => 0,
        }
    }

    pub fn with_usage(cell: Arc<UsageCell>) -> Self {
        let ret = Cell::Usage(cell);
        CELL_COUNT.fetch_add(1, Ordering::Relaxed);
        ret
    }

    pub fn with_virtual(cell: VirtualCell) -> Self {
        let ret = Cell::Virtual(Arc::new(cell));
        CELL_COUNT.fetch_add(1, Ordering::Relaxed);
        ret
    }

    pub fn with_boc3(cell: Boc3Cell) -> Self {
        let ret = Cell::Boc3(cell);
        CELL_COUNT.fetch_add(1, Ordering::Relaxed);
        ret
    }

    pub fn with_data(cell: DataCell) -> Self {
        let ret = Cell::Data(Arc::new(cell));
        CELL_COUNT.fetch_add(1, Ordering::Relaxed);
        ret
    }

    pub fn cell_count() -> u64 {
        CELL_COUNT.load(Ordering::Relaxed)
    }

    // pub fn finalization_nanos() -> u64 {
    //     FINALIZATION_NANOS.load(Ordering::Relaxed)
    // }

    pub fn reference(&self, index: usize) -> Result<Cell> {
        match self {
            Cell::Data(cell) => cell.reference(index),
            Cell::Usage(cell) => UsageCell::reference(cell, index),
            Cell::Virtual(cell) => cell.reference(index),
            Cell::Boc3(cell) => cell.reference(index),
        }
    }

    pub fn reference_repr_hash(&self, index: usize) -> Result<UInt256> {
        Ok(self.reference(index)?.hash(MAX_LEVEL))
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
        match self {
            Cell::Data(cell) => cell.data(),
            Cell::Usage(cell) => UsageCell::data(cell),
            Cell::Virtual(cell) => cell.wrapped.data(),
            Cell::Boc3(cell) => cell.data(),
        }
    }

    pub fn raw_data(&self) -> Result<&[u8]> {
        match self {
            Cell::Data(cell) => cell.raw_data(),
            Cell::Usage(cell) => UsageCell::raw_data(cell),
            Cell::Virtual(cell) => cell.raw_data(),
            Cell::Boc3(cell) => cell.raw_data(),
        }
    }

    pub fn bit_length(&self) -> usize {
        match self {
            Cell::Data(cell) => cell.bit_length(),
            Cell::Usage(cell) => cell.wrapped.bit_length(),
            Cell::Virtual(cell) => cell.wrapped.bit_length(),
            Cell::Boc3(cell) => cell.bit_length(),
        }
    }

    pub fn cell_type(&self) -> CellType {
        match self {
            Cell::Data(cell) => cell.cell_type(),
            Cell::Usage(cell) => cell.wrapped.cell_type(),
            Cell::Virtual(cell) => cell.wrapped.cell_type(),
            Cell::Boc3(cell) => cell.cell_type(),
        }
    }

    pub fn level(&self) -> u8 {
        self.level_mask().level()
    }

    pub fn hashes_count(&self) -> usize {
        self.level() as usize + 1
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
        match self {
            Cell::Data(cell) => cell.level_mask(),
            Cell::Usage(cell) => cell.wrapped.level_mask(),
            Cell::Virtual(cell) => cell.level_mask(),
            Cell::Boc3(cell) => cell.level_mask(),
        }
    }

    pub fn references_count(&self) -> usize {
        match self {
            Cell::Data(cell) => cell.references_count(),
            Cell::Usage(cell) => cell.wrapped.references_count(),
            Cell::Virtual(cell) => cell.references_count(),
            Cell::Boc3(cell) => cell.references_count(),
        }
    }

    /// Returns cell's higher hash for given index (last one - representation
    /// hash)
    pub fn hash(&self, index: usize) -> UInt256 {
        match self {
            Cell::Data(cell) => cell.hash(index),
            Cell::Usage(cell) => cell.wrapped.hash(index),
            Cell::Virtual(cell) => cell.hash(index),
            Cell::Boc3(cell) => cell.hash(index),
        }
    }

    /// Returns cell's depth for given index
    pub fn depth(&self, index: usize) -> u16 {
        match self {
            Cell::Data(cell) => cell.depth(index),
            Cell::Usage(cell) => cell.wrapped.depth(index),
            Cell::Virtual(cell) => cell.depth(index),
            Cell::Boc3(cell) => cell.depth(index),
        }
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
        match self {
            Self::Data(cell) => cell.hash(MAX_LEVEL),
            Self::Usage(cell) => cell.wrapped.hash(MAX_LEVEL),
            Self::Virtual(cell) => cell.hash(MAX_LEVEL),
            Self::Boc3(cell) => cell.hash(MAX_LEVEL),
        }
    }

    pub fn repr_depth(&self) -> u16 {
        match self {
            Self::Data(cell) => cell.depth(MAX_LEVEL),
            Self::Usage(cell) => cell.wrapped.depth(MAX_LEVEL),
            Self::Virtual(cell) => cell.depth(MAX_LEVEL),
            Self::Boc3(cell) => cell.depth(MAX_LEVEL),
        }
    }

    pub fn store_hashes(&self) -> bool {
        match self {
            Self::Data(cell) => cell.store_hashes(),
            Self::Usage(cell) => cell.wrapped.store_hashes(),
            Self::Virtual(cell) => cell.wrapped.store_hashes(),
            Self::Boc3(cell) => cell.store_hashes(),
        }
    }

    #[allow(dead_code)]
    pub fn is_merkle(&self) -> bool {
        self.cell_type().is_merkle()
    }

    #[allow(dead_code)]
    pub fn is_pruned(&self) -> bool {
        self.cell_type().is_pruned()
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
        match self {
            Cell::Data(cell) => cell.tree_bits_count(),
            Cell::Usage(cell) => cell.wrapped.tree_bits_count(),
            Cell::Virtual(cell) => cell.wrapped.tree_bits_count(),
            Cell::Boc3(cell) => cell.tree_bits_count(),
        }
    }

    pub fn tree_cell_count(&self) -> u64 {
        match self {
            Cell::Data(cell) => cell.tree_cell_count(),
            Cell::Usage(cell) => cell.wrapped.tree_cell_count(),
            Cell::Virtual(cell) => cell.wrapped.tree_cell_count(),
            Cell::Boc3(cell) => cell.tree_cell_count(),
        }
    }

    pub fn to_external(&self) -> Result<Cell> {
        match self {
            Cell::Data(cell) => cell.to_external(),
            Cell::Usage(cell) => UsageCell::to_external(cell),
            _ => {
                fail!("Cell can not be converted to external")
            }
        }
    }

    fn is_usage_cell(&self) -> bool {
        match self {
            Cell::Usage(_) => true,
            Cell::Virtual(cell) => cell.wrapped.is_usage_cell(),
            _ => false,
        }
    }

    fn downcast_usage(&self) -> Cell {
        match self {
            Cell::Usage(cell) => cell.wrapped.clone(),
            _ => {
                unreachable!("Function can be called only for UsageCell")
            }
        }
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

use smallvec::SmallVec;
use smallvec::smallvec;
use virtual_cell::VirtualCell;

pub use self::builder_operations::*;
use crate::cell::usage_cell::UsageCell;

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
    references: SmallVec<[Cell; 4]>,
    data: &[u8], // with completion tag (for big cell - without)!
) -> Result<Cell> {
    Ok(Cell::with_data(DataCell::with_refs_and_data(references, data)?))
}

pub fn create_big_cell(data: &[u8]) -> Result<Cell> {
    Ok(Cell::with_data(DataCell::with_params(
        smallvec![],
        data,
        CellType::Big,
        0,
        None,
        None,
        None,
    )?))
}
