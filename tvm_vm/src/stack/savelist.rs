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

use std::fmt;

use tvm_types::BuilderData;
use tvm_types::ExceptionCode;
use tvm_types::HashmapE;
use tvm_types::HashmapType;
use tvm_types::IBitstring;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::error;

use crate::error::TvmError;
use crate::executor::gas::gas_state::Gas;
use crate::stack::StackItem;
use crate::types::Exception;
use crate::types::ResultOpt;

#[derive(Clone, Debug, PartialEq)]
pub struct SaveList {
    storage: [Option<StackItem>; Self::NUMREGS],
}

impl Default for SaveList {
    fn default() -> Self {
        Self::new()
    }
}

impl SaveList {
    pub const NUMREGS: usize = 7;
    pub const REGS: [usize; Self::NUMREGS] = [0, 1, 2, 3, 4, 5, 7];

    const fn adjust(index: usize) -> usize {
        if index == 7 { 6 } else { index }
    }

    pub fn new() -> Self {
        Self { storage: Default::default() }
    }

    pub fn can_put(index: usize, value: &StackItem) -> bool {
        match index {
            0 | 1 | 3 => value.as_continuation().is_ok(),
            2 => value.as_continuation().is_ok() || value.is_null(),
            4 | 5 => value.as_cell().is_ok(),
            7 => value.as_tuple().is_ok(),
            _ => false,
        }
    }

    pub fn check_can_put(index: usize, value: &StackItem) -> Result<()> {
        if Self::can_put(index, value) {
            Ok(())
        } else {
            err!(ExceptionCode::TypeCheckError, "wrong item {} for index {}", value, index)
        }
    }

    pub fn get(&self, index: usize) -> Option<&StackItem> {
        self.storage[Self::adjust(index)].as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut StackItem> {
        self.storage[Self::adjust(index)].as_mut()
    }

    pub fn is_empty(&self) -> bool {
        for v in &self.storage {
            if v.is_some() {
                return false;
            }
        }
        true
    }

    pub fn put(&mut self, index: usize, value: &mut StackItem) -> ResultOpt<StackItem> {
        Self::check_can_put(index, value)?;
        Ok(self.put_opt(index, value))
    }

    pub fn put_opt(&mut self, index: usize, value: &mut StackItem) -> Option<StackItem> {
        debug_assert!(Self::can_put(index, value));
        self.storage[Self::adjust(index)].replace(value.withdraw())
    }

    pub fn apply(&mut self, other: &mut Self) {
        for index in 0..Self::NUMREGS {
            if other.storage[index].is_some() {
                self.storage[index] = std::mem::take(&mut other.storage[index]);
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<StackItem> {
        std::mem::take(&mut self.storage[Self::adjust(index)])
    }
}

impl SaveList {
    pub fn serialize_old(&self) -> Result<(BuilderData, i64)> {
        let mut gas = 0;
        let mut dict = HashmapE::with_bit_len(4);
        for index in 0..Self::NUMREGS {
            if let Some(ref item) = self.storage[index] {
                let mut builder = BuilderData::new();
                builder.append_bits(if index == 6 { 7 } else { index }, 4)?;
                let key = SliceData::load_builder(builder)?;
                let (value, gas2) = item.serialize_old()?;
                gas += gas2;
                dict.set_builder(key, &value)?;
            }
        }
        let mut builder = BuilderData::new();
        match dict.data() {
            Some(cell) => {
                builder.append_bit_one()?;
                builder.checked_append_reference(cell.clone())?;
                gas += Gas::finalize_price();
            }
            None => {
                builder.append_bit_zero()?;
            }
        }
        Ok((builder, gas))
    }

    pub fn deserialize_old(slice: &mut SliceData) -> Result<(Self, i64)> {
        let mut gas = 0;
        match slice.get_next_bit()? {
            false => Ok((Self::new(), gas)),
            true => {
                let dict = HashmapE::with_hashmap(4, slice.checked_drain_reference().ok());
                gas += Gas::load_cell_price(true);
                let mut savelist = SaveList::new();
                dict.iterate_slices(|mut key, mut value| {
                    let key = key.get_next_int(4)? as usize;
                    let (mut value, gas2) = StackItem::deserialize_old(&mut value)?;
                    gas += gas2;
                    savelist.put(key, &mut value)?;
                    Ok(true)
                })?;
                Ok((savelist, gas))
            }
        }
    }
}

impl fmt::Display for SaveList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- Control registers ------------------")?;
        for i in 0..Self::NUMREGS {
            if let Some(item) = &self.storage[i] {
                writeln!(f, "{}: {}", i, item)?
            }
        }
        writeln!(f, "{:-<40}", "")
    }
}

#[cfg(test)]
mod tests {
    use tvm_types::SliceData;

    use super::*;
    use crate::stack::continuation::ContinuationData;

    fn continuation_item() -> StackItem {
        StackItem::continuation(ContinuationData::with_code(SliceData::new(vec![0x80])))
    }

    #[test]
    fn can_put_matches_register_contract() {
        let cont = continuation_item();
        let cell = StackItem::cell(Default::default());
        let tuple = StackItem::tuple(vec![StackItem::int(1)]);

        assert!(SaveList::can_put(0, &cont));
        assert!(SaveList::can_put(1, &cont));
        assert!(SaveList::can_put(2, &cont));
        assert!(SaveList::can_put(2, &StackItem::None));
        assert!(SaveList::can_put(4, &cell));
        assert!(SaveList::can_put(5, &cell));
        assert!(SaveList::can_put(7, &tuple));

        assert!(!SaveList::can_put(0, &cell));
        assert!(!SaveList::can_put(4, &tuple));
        assert!(!SaveList::can_put(6, &cont));
    }

    #[test]
    fn put_remove_apply_and_empty_work_as_expected() {
        let mut first = SaveList::new();
        let mut second = SaveList::new();
        let mut cont = continuation_item();
        let mut tuple = StackItem::tuple(vec![StackItem::int(1), StackItem::int(2)]);

        assert!(first.is_empty());
        first.put(0, &mut cont).unwrap();
        second.put(7, &mut tuple).unwrap();
        assert!(!first.is_empty());
        assert!(first.get(0).unwrap().as_continuation().is_ok());

        first.apply(&mut second);
        assert!(first.get(7).unwrap().as_tuple().is_ok());
        assert!(second.is_empty());

        let removed = first.remove(7).unwrap();
        assert!(removed.as_tuple().is_ok());
        assert!(first.get(7).is_none());
    }

    #[test]
    fn serialize_old_roundtrips_empty_and_populated_lists() {
        let (builder, gas) = SaveList::new().serialize_old().unwrap();
        assert_eq!(gas, 0);
        let mut slice = SliceData::load_builder(builder).unwrap();
        let (decoded, gas) = SaveList::deserialize_old(&mut slice).unwrap();
        assert_eq!(decoded, SaveList::new());
        assert_eq!(gas, 0);

        let mut savelist = SaveList::new();
        let mut cont = continuation_item();
        let mut cell = StackItem::cell(Default::default());
        let mut tuple = StackItem::tuple(vec![StackItem::int(7)]);
        savelist.put(0, &mut cont).unwrap();
        savelist.put(4, &mut cell).unwrap();
        savelist.put(7, &mut tuple).unwrap();

        let (builder, _) = savelist.serialize_old().unwrap();
        let mut slice = SliceData::load_builder(builder).unwrap();
        let (decoded, _) = SaveList::deserialize_old(&mut slice).unwrap();
        assert_eq!(decoded, savelist);
    }
}
