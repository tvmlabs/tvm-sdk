// Copyright 2023 EverX Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use thiserror::Error;
use tvm_block::Deserializable;
use tvm_block::Serializable;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::IBitstring;
use tvm_types::Result;
use tvm_types::SliceData;

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("unexpected tlb tag")]
    UnexpectedTLBTag,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct TVC {
    pub code: Option<Cell>,
    pub desc: Option<String>,
}

impl TVC {
    const TVC_TAG: u32 = 0xa2f0b81c;

    pub fn new(code: Option<Cell>, desc: Option<String>) -> Self {
        Self { code, desc }
    }
}

fn builder_store_bytes_ref(b: &mut BuilderData, data: &[u8]) -> Result<()> {
    const CELL_LEN: usize = 127;

    let mut tpb = BuilderData::new();
    let mut len = data.len();
    let mut cap = match len % CELL_LEN {
        0 => CELL_LEN,
        x => x,
    };

    while len > 0 {
        len -= cap;
        tpb.append_raw(&data[len..len + cap], cap * 8)?;

        if len > 0 {
            let mut nb = BuilderData::new();
            nb.checked_append_reference(tpb.clone().into_cell()?)?;
            cap = std::cmp::min(CELL_LEN, len);
            tpb = nb;
        }
    }

    b.checked_append_reference(tpb.into_cell()?)?;
    Ok(())
}

pub fn builder_store_string_ref(builder: &mut BuilderData, data: &str) -> Result<()> {
    builder_store_bytes_ref(builder, data.as_bytes())
}

pub fn slice_load_bytes_ref(slice: &mut SliceData) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    let rrb = slice.remaining_references();
    let mut curr: Cell = Cell::construct_from(slice)?;
    assert_eq!(rrb - 1, slice.remaining_references(), "ref not loaded from slice");

    loop {
        let cs = SliceData::load_cell(curr)?;
        let bb = cs.get_bytestring(0);
        bytes.append(&mut bb.clone());

        if cs.remaining_references() > 0 {
            curr = cs.reference(0)?;
        } else {
            break;
        }
    }

    Ok(bytes)
}

pub fn slice_load_string_ref(slice: &mut SliceData) -> Result<String> {
    Ok(String::from_utf8(slice_load_bytes_ref(slice)?)?)
}

impl Serializable for TVC {
    fn write_to(&self, builder: &mut BuilderData) -> tvm_types::Result<()> {
        builder.append_u32(Self::TVC_TAG)?;

        if let Some(c) = &self.code {
            builder.append_bit_one()?;
            builder.checked_append_reference(c.to_owned())?;
        } else {
            builder.append_bit_zero()?;
        }

        if let Some(s) = &self.desc {
            builder.append_bit_one()?;
            builder_store_string_ref(builder, s)?;
        } else {
            builder.append_bit_zero()?;
        }

        Ok(())
    }
}

impl Deserializable for TVC {
    fn read_from(&mut self, slice: &mut SliceData) -> tvm_types::Result<()> {
        let tag = slice.get_next_u32()?;
        if tag != Self::TVC_TAG {
            return Err(DeserializationError::UnexpectedTLBTag.into());
        }

        if slice.get_next_bit()? {
            self.code = Some(Cell::construct_from(slice)?);
        }

        if slice.get_next_bit()? {
            self.desc = Some(slice_load_string_ref(slice)?);
        }

        Ok(())
    }
}
