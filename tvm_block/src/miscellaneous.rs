// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::HashmapE;
use tvm_types::HashmapSubtree;
use tvm_types::HashmapType;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;
use tvm_types::fail;

use crate::Deserializable;
use crate::Serializable;
use crate::define_HashmapE;

// key is [ shard:uint64 mc_seqno:uint32 ]
// _ (HashmapE 96 ProcessedUpto) = ProcessedInfo;
define_HashmapE!(ProcessedInfo, 96, ProcessedUpto);

/// Struct ProcessedInfoKey describe key for ProcessedInfo
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ProcessedInfoKey {
    pub shard: u64,
    pub mc_seqno: u32,
}

impl ProcessedInfoKey {
    // New instance ProcessedInfoKey structure
    pub fn with_params(shard: u64, mc_seqno: u32) -> Self {
        ProcessedInfoKey { shard, mc_seqno }
    }

    pub fn seq_no(&self) -> u32 {
        self.mc_seqno
    }
}

impl Serializable for ProcessedInfoKey {
    fn write_to(&self, cell: &mut BuilderData) -> Result<()> {
        self.shard.write_to(cell)?;
        self.mc_seqno.write_to(cell)?;
        Ok(())
    }
}

impl Deserializable for ProcessedInfoKey {
    fn read_from(&mut self, cell: &mut SliceData) -> Result<()> {
        self.shard.read_from(cell)?;
        self.mc_seqno.read_from(cell)?;
        Ok(())
    }
}

/// Struct ProcessedUpto
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ProcessedUpto {
    pub last_msg_lt: u64,
    pub last_msg_hash: UInt256,
    pub original_shard: Option<u64>,
}

impl ProcessedUpto {
    // New instance ProcessedUpto structure
    pub fn with_params(
        last_msg_lt: u64,
        last_msg_hash: UInt256,
        original_shard: Option<u64>,
    ) -> Self {
        Self { last_msg_lt, last_msg_hash, original_shard }
    }
}

impl Serializable for ProcessedUpto {
    fn write_to(&self, cell: &mut BuilderData) -> Result<()> {
        self.last_msg_lt.write_to(cell)?;
        self.last_msg_hash.write_to(cell)?;
        if let Some(s) = &self.original_shard {
            s.write_to(cell)?;
        }
        Ok(())
    }
}

impl Deserializable for ProcessedUpto {
    fn read_from(&mut self, cell: &mut SliceData) -> Result<()> {
        self.last_msg_lt.read_from(cell)?;
        self.last_msg_hash.read_from(cell)?;
        self.original_shard = match cell.remaining_bits() {
            0 => None,
            64 => Some(cell.get_next_u64()?),
            // Compatibility with data serialized with old version of the library
            // (with "fast_finality" feature)
            1 => None,
            65 => Deserializable::construct_maybe_from(cell)?,
            _ => fail!("Invalid cell size"),
        };
        Ok(())
    }
}

// IhrPendingInfo structure
define_HashmapE!(IhrPendingInfo, 320, IhrPendingSince);

impl IhrPendingInfo {
    pub fn split_inplace(&mut self, split_key: &SliceData) -> Result<()> {
        self.0.into_subtree_with_prefix(split_key, &mut 0)
    }
}

/// IhrPendingSince structure
///
/// ihr_pending$_
///     import_lt:uint64
/// = IhrPendingSince;
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct IhrPendingSince {
    import_lt: u64,
}

impl IhrPendingSince {
    /// New default instance IhrPendingSince structure
    pub fn new() -> Self {
        Self::default()
    }

    // New instance IhrPendingSince structure
    pub fn with_import_lt(import_lt: u64) -> Self {
        IhrPendingSince { import_lt }
    }

    pub fn import_lt(&self) -> u64 {
        self.import_lt
    }
}

impl Serializable for IhrPendingSince {
    fn write_to(&self, cell: &mut BuilderData) -> Result<()> {
        self.import_lt.write_to(cell)?;
        Ok(())
    }
}

impl Deserializable for IhrPendingSince {
    fn read_from(&mut self, cell: &mut SliceData) -> Result<()> {
        self.import_lt.read_from(cell)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tvm_types::IBitstring;

    use super::*;

    fn processed_prefix(last_msg_lt: u64, last_msg_hash: UInt256) -> BuilderData {
        let mut builder = BuilderData::new();
        last_msg_lt.write_to(&mut builder).unwrap();
        last_msg_hash.write_to(&mut builder).unwrap();
        builder
    }

    #[test]
    fn processed_info_key_roundtrip_and_seq_no() {
        let key = ProcessedInfoKey::with_params(0x0123_4567_89ab_cdef, 77);
        let parsed = ProcessedInfoKey::construct_from_cell(key.serialize().unwrap()).unwrap();

        assert_eq!(parsed, key);
        assert_eq!(parsed.seq_no(), 77);
    }

    #[test]
    fn processed_upto_roundtrip_with_and_without_original_shard() {
        let with_shard = ProcessedUpto::with_params(15, UInt256::from([7; 32]), Some(42));
        let without_shard = ProcessedUpto::with_params(16, UInt256::from([8; 32]), None);

        assert_eq!(
            ProcessedUpto::construct_from_cell(with_shard.serialize().unwrap()).unwrap(),
            with_shard
        );
        assert_eq!(
            ProcessedUpto::construct_from_cell(without_shard.serialize().unwrap()).unwrap(),
            without_shard
        );
    }

    #[test]
    fn processed_upto_reads_compatibility_forms_and_rejects_invalid_tail_sizes() {
        let hash = UInt256::from([9; 32]);

        let no_original = ProcessedUpto::construct_from(
            &mut SliceData::load_builder(processed_prefix(1, hash.clone())).unwrap(),
        )
        .unwrap();
        assert_eq!(no_original.original_shard, None);

        let mut old_compat = processed_prefix(2, hash.clone());
        old_compat.append_bit_zero().unwrap();
        let old_compat =
            ProcessedUpto::construct_from(&mut SliceData::load_builder(old_compat).unwrap())
                .unwrap();
        assert_eq!(old_compat.original_shard, None);

        let mut maybe_some = processed_prefix(3, hash.clone());
        maybe_some.append_bit_one().unwrap();
        0xfeed_beef_u64.write_to(&mut maybe_some).unwrap();
        let maybe_some =
            ProcessedUpto::construct_from(&mut SliceData::load_builder(maybe_some).unwrap())
                .unwrap();
        assert_eq!(maybe_some.original_shard, Some(0xfeed_beef));

        let mut invalid = processed_prefix(4, hash);
        invalid.append_raw(&[0b1100_0000], 2).unwrap();
        assert!(
            ProcessedUpto::construct_from(&mut SliceData::load_builder(invalid).unwrap()).is_err()
        );
    }

    #[test]
    fn ihr_pending_since_constructors_and_roundtrip() {
        let default_value = IhrPendingSince::new();
        let explicit = IhrPendingSince::with_import_lt(123456);

        assert_eq!(default_value.import_lt(), 0);
        assert_eq!(explicit.import_lt(), 123456);
        assert_eq!(
            IhrPendingSince::construct_from_cell(explicit.serialize().unwrap()).unwrap(),
            explicit
        );
    }
}
