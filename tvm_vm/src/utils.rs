// Copyright (C) 2019-2022 TON Labs. All Rights Reserved.
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
use tvm_types::ExceptionCode;
use tvm_types::GasConsumer;
use tvm_types::MAX_DATA_BITS;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::error;
use tvm_types::fail;

use crate::error::TvmError;
use crate::types::Exception;

/// Pack data as a list of single-reference cells
pub fn pack_data_to_cell(bytes: &[u8], engine: &mut dyn GasConsumer) -> Result<Cell> {
    let mut cell = BuilderData::default();
    let cell_length_in_bytes = MAX_DATA_BITS / 8;
    for cur_slice in bytes.chunks(cell_length_in_bytes).rev() {
        if cell.bits_used() != 0 {
            let mut new_cell = BuilderData::new();
            new_cell.checked_append_reference(engine.finalize_cell(cell)?)?;
            cell = new_cell;
        }
        cell.append_raw(cur_slice, cur_slice.len() * 8)?;
    }
    engine.finalize_cell(cell)
}

/// Pack string as a list of single-reference cells
pub fn pack_string_to_cell(string: &str, engine: &mut dyn GasConsumer) -> Result<Cell> {
    pack_data_to_cell(string.as_bytes(), engine)
}

/// Unpack data as a list of single-reference cells
pub fn unpack_data_from_cell(mut cell: SliceData, engine: &mut dyn GasConsumer) -> Result<Vec<u8>> {
    let mut data = vec![];
    loop {
        if cell.remaining_bits() % 8 != 0 {
            fail!(
                "Cannot parse string from cell because of length of cell bits len: {}",
                cell.remaining_bits()
            )
        }
        data.extend_from_slice(&cell.get_bytestring(0));
        match cell.remaining_references() {
            0 => return Ok(data),
            1 => cell = engine.load_cell(cell.reference(0)?)?,
            _ => {
                return err!(
                    ExceptionCode::TypeCheckError,
                    "Incorrect representation of string in cells"
                );
            }
        }
    }
}

pub(crate) fn bytes_to_string(data: Vec<u8>) -> Result<String> {
    String::from_utf8(data).map_err(|err| {
        exception!(ExceptionCode::TypeCheckError, "Cannot create utf8 string: {}", err)
    })
}

/// Unpack string as a list of single-reference cells
pub fn unpack_string_from_cell(cell: SliceData, engine: &mut dyn GasConsumer) -> Result<String> {
    bytes_to_string(unpack_data_from_cell(cell, engine)?)
}

#[cfg(test)]
mod tests {
    use tvm_types::BuilderData;

    use super::*;
    use crate::error::tvm_exception_code;
    use crate::executor::Engine;

    fn new_engine() -> Engine {
        Engine::with_capabilities(0).setup(SliceData::default(), None, None, None)
    }

    #[test]
    fn pack_and_unpack_roundtrip_small_and_large_payloads() {
        let mut engine = new_engine();
        let small = b"hello".to_vec();
        let cell = pack_data_to_cell(&small, &mut engine).unwrap();
        assert_eq!(
            unpack_data_from_cell(SliceData::load_cell(cell).unwrap(), &mut engine).unwrap(),
            small
        );

        let mut engine = new_engine();
        let large = vec![0x5a; 300];
        let cell = pack_data_to_cell(&large, &mut engine).unwrap();
        assert_eq!(
            unpack_data_from_cell(SliceData::load_cell(cell).unwrap(), &mut engine).unwrap(),
            large
        );
    }

    #[test]
    fn unpack_rejects_non_byte_aligned_cells_and_multiple_references() {
        let mut engine = new_engine();
        let cell = BuilderData::with_raw(vec![0b1010_0000], 4).unwrap().into_cell().unwrap();
        assert!(unpack_data_from_cell(SliceData::load_cell(cell).unwrap(), &mut engine).is_err());

        let mut engine = new_engine();
        let child = BuilderData::with_raw(vec![0xaa], 8).unwrap().into_cell().unwrap();
        let root = BuilderData::with_raw_and_refs(vec![0xbb], 8, vec![child.clone(), child])
            .unwrap()
            .into_cell()
            .unwrap();
        let err =
            unpack_data_from_cell(SliceData::load_cell(root).unwrap(), &mut engine).unwrap_err();
        assert_eq!(tvm_exception_code(&err), Some(ExceptionCode::TypeCheckError));
    }

    #[test]
    fn string_helpers_roundtrip_and_report_invalid_utf8() {
        let mut engine = new_engine();
        let cell = pack_string_to_cell("hello", &mut engine).unwrap();
        assert_eq!(
            unpack_string_from_cell(SliceData::load_cell(cell).unwrap(), &mut engine).unwrap(),
            "hello"
        );

        let err = bytes_to_string(vec![0xff]).unwrap_err();
        assert_eq!(tvm_exception_code(&err), Some(ExceptionCode::TypeCheckError));
    }
}
