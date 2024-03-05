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
use tvm_types::SliceData;

use crate::debug::DbgNode;
use crate::CompileResult;
use crate::DbgInfo;
use crate::OperationError;

#[derive(Clone, Default)]
pub struct Unit {
    builder: BuilderData,
    dbg: DbgNode,
}

impl Unit {
    pub fn new(builder: BuilderData, dbg: DbgNode) -> Self {
        Self { builder, dbg }
    }

    pub fn finalize(self) -> (SliceData, DbgInfo) {
        let cell = self.builder.into_cell().unwrap();
        let slice = SliceData::load_cell_ref(&cell).unwrap();
        let dbg_info = DbgInfo::from(cell, self.dbg);
        (slice, dbg_info)
    }
}

pub struct Units {
    units: Vec<Unit>,
}

impl Default for Units {
    fn default() -> Self {
        Self::new()
    }
}

impl Units {
    /// Constructor
    pub fn new() -> Self {
        Self { units: vec![Unit::default()] }
    }

    /// Writes assembled unit
    pub fn write_unit(&mut self, unit: Unit) -> CompileResult {
        self.units.push(unit);
        Ok(())
    }

    /// Writes simple command
    pub fn write_command(&mut self, command: &[u8], dbg: DbgNode) -> CompileResult {
        self.write_command_bitstring(command, command.len() * 8, dbg)
    }

    pub fn write_command_bitstring(
        &mut self,
        command: &[u8],
        bits: usize,
        dbg: DbgNode,
    ) -> CompileResult {
        if let Some(last) = self.units.last_mut() {
            let orig_offset = last.builder.bits_used();
            if last.builder.append_raw(command, bits).is_ok() {
                last.dbg.inline_node(orig_offset, dbg);
                return Ok(());
            }
        }
        if let Ok(new_last) = BuilderData::with_raw(command, bits) {
            self.units.push(Unit::new(new_last, dbg));
            return Ok(());
        }
        Err(OperationError::NotFitInSlice)
    }

    /// Writes command with additional references
    pub fn write_composite_command(
        &mut self,
        command: &[u8],
        references: Vec<BuilderData>,
        dbg: DbgNode,
    ) -> CompileResult {
        assert_eq!(references.len(), dbg.children.len());
        if let Some(mut last) = self.units.last().cloned() {
            let orig_offset = last.builder.bits_used();
            if last.builder.references_free() > references.len() // one cell remains reserved for finalization
                && last.builder.append_raw(command, command.len() * 8).is_ok()
                && checked_append_references(&mut last.builder, &references)?
            {
                last.dbg.inline_node(orig_offset, dbg);
                *self.units.last_mut().unwrap() = last;
                return Ok(());
            }
        }
        let mut new_last = BuilderData::new();
        if new_last.append_raw(command, command.len() * 8).is_ok()
            && checked_append_references(&mut new_last, &references)?
        {
            self.units.push(Unit::new(new_last, dbg));
            return Ok(());
        }
        Err(OperationError::NotFitInSlice)
    }

    /// Puts recorded cells in a linear sequence
    pub fn finalize(mut self) -> (BuilderData, DbgNode) {
        let mut cursor = self.units.pop().expect("cells can't be empty");
        while let Some(mut destination) = self.units.pop() {
            let orig_offset = destination.builder.bits_used();
            let slice =
                SliceData::load_builder(cursor.builder).expect("failed to convert builder to cell");
            // try to inline cursor into destination
            if destination.builder.checked_append_references_and_data(&slice).is_ok() {
                destination.dbg.inline_node(orig_offset, cursor.dbg);
            } else {
                // otherwise just attach cursor to destination as a reference
                destination.builder.checked_append_reference(slice.into_cell()).unwrap();
                destination.dbg.append_node(cursor.dbg);
            }
            cursor = destination;
        }
        (cursor.builder, cursor.dbg)
    }
}

fn checked_append_references(
    builder: &mut BuilderData,
    refs: &[BuilderData],
) -> Result<bool, OperationError> {
    for reference in refs {
        if builder
            .checked_append_reference(
                reference.clone().into_cell().map_err(|_| OperationError::NotFitInSlice)?,
            )
            .is_err()
        {
            return Ok(false);
        }
    }
    Ok(true)
}
