use std::debug_assert;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::vec;

use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

use crate::ByteOrderRead;
use crate::CellType;
use crate::DEPTH_SIZE;
use crate::LevelMask;
use crate::SHA256_SIZE;
use crate::UInt256;
use crate::cell;
use crate::error;
use crate::fail;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CellBuffer {
    Local(Vec<u8>),
    External { buf: Arc<Vec<u8>>, offset: usize },
}

impl CellBuffer {
    pub fn data(&self) -> &[u8] {
        match &self {
            CellBuffer::Local(d) => d,
            CellBuffer::External { buf, offset } => {
                &buf[*offset..*offset + cell::full_len(&buf[*offset..])]
            }
        }
    }

    pub fn unbounded_data(&self) -> &[u8] {
        match &self {
            CellBuffer::Local(d) => d,
            CellBuffer::External { buf, offset } => &buf[*offset..],
        }
    }

    pub fn unbounded_data_mut(&mut self) -> crate::Result<&mut [u8]> {
        match self {
            CellBuffer::Local(d) => Ok(d),
            CellBuffer::External { buf: _, offset: _ } => fail!("Can't change external buffer"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellData {
    pub(crate) buf: CellBuffer,
    pub(crate) hashes_depths: Vec<(UInt256, u16)>,
}

impl Default for CellData {
    fn default() -> Self {
        Self::new()
    }
}

impl CellData {
    pub fn new() -> Self {
        Self::with_params(CellType::Ordinary, &[80], 0, 0, false, None, None).unwrap()
    }

    pub fn with_params(
        cell_type: CellType,
        data: &[u8], // with completion tag!
        level_mask: u8,
        refs: u8,
        store_hashes: bool,
        hashes: Option<[UInt256; 4]>,
        depths: Option<[u16; 4]>,
    ) -> crate::Result<Self> {
        let buffer = if cell_type == CellType::Big {
            cell::build_big_cell_buf(data, level_mask, refs as usize, store_hashes, hashes, depths)?
        } else {
            cell::build_cell_buf(
                cell_type,
                data,
                level_mask,
                refs as usize,
                store_hashes,
                hashes,
                depths,
            )?
        };
        let hashes_count = if cell_type == CellType::PrunedBranch || cell_type == CellType::External
        {
            1
        } else {
            cell::level(&buffer) as usize + 1
        };
        let allocate_for_hashes = (!store_hashes) as usize * hashes_count;
        let mut hashes_depths = Vec::with_capacity(allocate_for_hashes);
        match (store_hashes, hashes, depths) {
            (true, _, _) => (),
            (_, None, None) => (),
            (false, Some(hashes), Some(depths)) => {
                for i in 0..hashes_count {
                    hashes_depths.push((hashes[i], depths[i]));
                }
            }
            _ => fail!("`hashes` and `depths` existence are not correspond each other"),
        }
        Ok(Self { buf: CellBuffer::Local(buffer), hashes_depths })
    }

    pub fn with_external_data(buffer: &Arc<Vec<u8>>, offset: usize) -> crate::Result<Self> {
        cell::check_cell_buf(&buffer[offset..], true)?;

        let allocate_for_hashes = (!cell::store_hashes(&buffer[offset..])) as usize
            * (cell::level(&buffer[offset..]) as usize + 1);
        Ok(Self {
            buf: CellBuffer::External { buf: buffer.clone(), offset },
            hashes_depths: Vec::with_capacity(allocate_for_hashes),
        })
    }

    pub fn with_raw_data(data: Vec<u8>) -> crate::Result<Self> {
        cell::check_cell_buf(&data, false)?;

        let allocate_for_hashes =
            (!cell::store_hashes(&data)) as usize * (cell::level(&data) as usize + 1);
        Ok(Self {
            buf: CellBuffer::Local(data),
            hashes_depths: Vec::with_capacity(allocate_for_hashes),
        })
    }

    pub fn raw_data(&self) -> &[u8] {
        self.buf.data()
    }

    pub fn cell_type(&self) -> CellType {
        cell::cell_type(self.buf.unbounded_data())
    }

    // Might be without tag!!!
    pub fn data(&self) -> &[u8] {
        cell::cell_data(self.buf.unbounded_data())
    }

    pub fn bit_length(&self) -> usize {
        cell::bit_len(self.buf.unbounded_data())
    }

    pub fn level(&self) -> u8 {
        cell::level(self.buf.unbounded_data())
    }

    pub fn level_mask(&self) -> LevelMask {
        cell::level_mask(self.buf.unbounded_data())
    }

    pub fn store_hashes(&self) -> bool {
        cell::store_hashes(self.buf.unbounded_data())
    }

    pub fn references_count(&self) -> usize {
        cell::refs_count(self.buf.unbounded_data())
    }

    pub(crate) fn set_hash_depth(
        &mut self,
        index: usize,
        hash: &[u8],
        depth: u16,
    ) -> crate::Result<()> {
        if self.store_hashes() {
            cell::set_hash(self.buf.unbounded_data_mut()?, index, hash);
            cell::set_depth(self.buf.unbounded_data_mut()?, index, depth);
        } else {
            debug_assert!(self.hashes_depths.len() == index);
            self.hashes_depths.push((hash.into(), depth));
        }
        Ok(())
    }

    pub fn hash(&self, index: usize) -> UInt256 {
        self.raw_hash(index).into()
    }

    pub fn raw_hash(&self, mut index: usize) -> &[u8] {
        index = self.level_mask().calc_hash_index(index);
        if self.cell_type() == CellType::PrunedBranch {
            // pruned cell stores all hashes (except representation) in data
            if index != self.level() as usize {
                let offset = 1 + 1 + index * SHA256_SIZE;
                return &self.data()[offset..offset + SHA256_SIZE];
            } else {
                index = 0;
            }
        }
        // external cell has only representation hash
        if self.cell_type() == CellType::External {
            let offset = 1;
            return &self.data()[offset..offset + SHA256_SIZE];
        }
        if self.store_hashes() {
            cell::hash(self.buf.unbounded_data(), index)
        } else {
            self.hashes_depths[index].0.as_slice()
        }
    }

    pub fn depth(&self, mut index: usize) -> u16 {
        index = self.level_mask().calc_hash_index(index);
        if self.cell_type() == CellType::PrunedBranch {
            // pruned cell stores all depth except "representation" in data
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
            cell::depth(self.buf.unbounded_data(), index)
        } else {
            self.hashes_depths[index].1
        }
    }

    /// Binary serialization of cell data.
    /// Strange things here were made for compatibility
    pub fn serialize<T: Write>(&self, writer: &mut T) -> crate::Result<()> {
        let bitlen = self.bit_length();
        writer.write_all(&[self.cell_type().to_u8().unwrap()])?;
        writer.write_all(&(bitlen as u16).to_le_bytes())?;
        writer.write_all(&self.data()[0..bitlen / 8 + (bitlen % 8 != 0) as usize])?;
        if bitlen % 8 == 0 {
            writer.write_all(&[0])?; // for compatibility
        }
        writer.write_all(&[self.level_mask().0])?;
        writer.write_all(&[self.store_hashes() as u8])?;
        let hashes_count = cell::hashes_count(self.buf.unbounded_data());
        writer.write_all(&[1])?;
        writer.write_all(&[hashes_count as u8])?;
        if self.store_hashes() {
            for i in 0..hashes_count {
                let hash = cell::hash(self.buf.unbounded_data(), i);
                writer.write_all(hash)?;
            }
        } else {
            debug_assert!(hashes_count == self.hashes_depths.len());
            for (hash, _depth) in &self.hashes_depths {
                writer.write_all(hash.as_slice())?;
            }
        }
        writer.write_all(&[1])?;
        writer.write_all(&[hashes_count as u8])?;
        if self.store_hashes() {
            for i in 0..hashes_count {
                let depth = cell::depth(self.buf.unbounded_data(), i);
                writer.write_all(&depth.to_le_bytes())?;
            }
        } else {
            for (_hash, depth) in &self.hashes_depths {
                writer.write_all(&depth.to_le_bytes())?;
            }
        }
        writer.write_all(&[self.references_count() as u8])?;
        Ok(())
    }

    /// Binary deserialization of cell data
    pub fn deserialize<T: Read>(reader: &mut T) -> crate::Result<Self> {
        let cell_type: CellType = FromPrimitive::from_u8(reader.read_byte()?)
            .ok_or_else(|| std::io::Error::from(ErrorKind::InvalidData))?;
        let bitlen = reader.read_le_u16()? as usize;
        let data_len = bitlen / 8 + (bitlen % 8 != 0) as usize;
        let data = if bitlen % 8 == 0 {
            let mut data = vec![0; data_len + 1];
            reader.read_exact(&mut data[..data_len])?;
            let _ = reader.read_byte()?; // for compatibility
            data[data_len] = 0x80;
            data
        } else {
            let mut data = vec![0; data_len];
            reader.read_exact(&mut data)?;
            data
        };
        let level_mask = reader.read_byte()?;
        let store_hashes = Self::read_bool(reader)?;

        let hashes =
            Self::read_short_array_opt(reader, |reader| Ok(UInt256::from(reader.read_u256()?)))?;
        let depths = Self::read_short_array_opt(reader, |reader| Ok(reader.read_le_u16()?))?;

        let refs = reader.read_byte()?;

        Self::with_params(cell_type, &data, level_mask, refs, store_hashes, hashes, depths)
    }

    fn read_short_array_opt<R, T, F>(reader: &mut R, read_func: F) -> crate::Result<Option<[T; 4]>>
    where
        R: Read,
        T: Default,
        F: Fn(&mut R) -> crate::Result<T>,
    {
        if Self::read_bool(reader)? {
            Ok(Some(Self::read_short_array(reader, read_func)?))
        } else {
            Ok(None)
        }
    }

    fn read_short_array<R, T, F>(reader: &mut R, read_func: F) -> crate::Result<[T; 4]>
    where
        R: Read,
        T: Default,
        F: Fn(&mut R) -> crate::Result<T>,
    {
        let count = reader.read_byte()?;
        if count > 4 {
            fail!("count too big {}", count)
        }
        let mut result = [T::default(), T::default(), T::default(), T::default()];
        for i in 0..count {
            result[i as usize] = read_func(reader)?;
        }
        Ok(result)
    }

    fn read_bool<R: Read>(reader: &mut R) -> crate::Result<bool> {
        match reader.read_byte()? {
            1 => Ok(true),
            0 => Ok(false),
            _ => fail!(std::io::Error::from(ErrorKind::InvalidData)),
        }
    }
}
