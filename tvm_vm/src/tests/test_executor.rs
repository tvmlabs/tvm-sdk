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

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::IBitstring;
use tvm_types::SliceData;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::executor::gas::gas_state::Gas;
use crate::executor::math::DivMode;
use crate::executor::serialize_currency_collection;
use crate::executor::token::execute_run_wasm;
use crate::executor::token::execute_run_wasm_concat_multiarg;
use crate::executor::types::Instruction;
use crate::executor::types::InstructionOptions;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::integer::behavior::OperationBehavior;
use crate::stack::integer::behavior::Quiet;
use crate::stack::integer::behavior::Signaling;
use crate::stack::savelist::SaveList;
use crate::types::Status;
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;

#[allow(dead_code)]
pub(super) fn split_to_chain_of_cells(input: Vec<u8>) -> Result<Cell, failure::Error> {
    // TODO: Cell size can maybe be increased up to 128?
    let cellsize = 120usize;
    let len = input.len();
    let mut cell_vec = Vec::<Vec<u8>>::new();
    // Process the input in 1024-byte chunks
    for i in (0..len).step_by(cellsize) {
        let end = std::cmp::min(i + cellsize, len);
        let chunk = &input[i..end];

        // Convert slice to Vec<u8> and pass to omnom function
        let chunk_vec = chunk.to_vec();
        cell_vec.push(chunk_vec);

        assert!(
            cell_vec.last().expect("error in split_to_chain_of_cells function").len() == cellsize
                || i + cellsize > len
        );
    }
    let mut cell = BuilderData::with_raw(
        cell_vec[cell_vec.len() - 1].clone(),
        cell_vec[cell_vec.len() - 1].len() * 8,
    )?
    .into_cell()?;
    for i in (0..(cell_vec.len() - 1)).rev() {
        let mut builder = BuilderData::with_raw(cell_vec[i].clone(), cell_vec[i].len() * 8)?;
        let builder = builder.checked_append_reference(cell)?;
        cell = builder.clone().into_cell()?;
    }
    Ok(cell) // return first cell
}

pub(super) fn rejoin_chain_of_cells(input: &Cell) -> Result<Vec<u8>, failure::Error> {
    let mut data_vec = input.data().to_vec();
    let mut cur_cell: Cell = input.clone();
    while cur_cell.reference(0).is_ok() {
        let old_len = data_vec.len();
        cur_cell = cur_cell.reference(0)?;
        data_vec.append(&mut cur_cell.data().to_vec());

        assert!(data_vec.len() - old_len == cur_cell.data().len());
    }
    Ok(data_vec)
}

#[test]
fn test_assert_stack() {
    let mut engine = Engine::with_capabilities(0);
    engine.cc.stack.push(int!(0));
    engine.cc.stack.push(int!(-1));
    engine.cc.stack.push(int!(1));
    let mut stack = Stack::new();
    stack.push(int!(0));
    stack.push(int!(-1));
    stack.push(int!(1));
    engine.assert_stack(&stack);
}

#[test]
fn test_next_cmd_failed() {
    let mut engine = Engine::with_capabilities(0);
    engine.next_cmd().expect_err("Should be generated exception for empty code");
}

#[test]
fn test_div_mode_names_not_intersect() {
    let mut set = HashSet::new();
    for flags in 0..=0b11111111 {
        let mode = DivMode::with_flags(flags);
        if mode.shift_parameter() {
            continue;
        }
        if let Ok(name) = mode.command_name() {
            assert!(set.insert(name.to_string()));
        }
    }
}

#[test]
fn test_division_primitives_execution() {
    let mut count = 0;
    for flags in 0..=0b11111111 {
        let mode = DivMode::with_flags(flags);
        if !mode.is_valid() {
            println!("Flags: {:#010b}, <NOT IMPLEMENTED>", mode.flags);
            continue;
        }
        test_div_primitive_execution::<Signaling>(&mode);
        test_div_primitive_execution::<Quiet>(&mode);
        if !mode.shift_parameter() {
            count += 1;
        }
    }
    assert_eq!(45, count);
}

fn get_command_name<T>(name: &str) -> String
where
    T: OperationBehavior,
{
    let mut result = name.to_owned();
    if let Some(str) = T::name_prefix() {
        result.insert_str(0, str)
    };
    result
}

fn command_name_from_mode<T>(mode: &DivMode) -> String
where
    T: OperationBehavior,
{
    match mode.command_name() {
        Ok(name) => get_command_name::<T>(name),
        Err(_) => {
            panic!("Flags: {:#010b}, Cmd: <NOT IMPLEMENTED>", mode.flags)
        }
    }
}

fn test_div_primitive_execution<T>(mode: &DivMode)
where
    T: OperationBehavior,
{
    let command_name = command_name_from_mode::<T>(mode);
    println!("Flags: {:#010b}, Cmd: {}", mode.flags, command_name);

    let mut value = 15;
    let mul_shift = 3;
    let div_shift = 1;

    let multiplier: i32 = 1 << mul_shift;
    let divisor: i32 = 1 << div_shift;
    let mut stack = Stack::new();
    let mut swap = 0;

    stack.push(int!(value));

    if mode.premultiply() && (!mode.mul_by_shift() || !mode.shift_parameter()) {
        stack.push(int!(if mode.mul_by_shift() {
            swap = 1;
            mul_shift
        } else {
            multiplier
        }));
    }

    if !(mode.div_by_shift() && mode.shift_parameter()) {
        stack.push(int!(if mode.div_by_shift() {
            div_shift
        } else {
            if swap == 1 {
                swap = 2
            }
            divisor
        }));
    }
    if swap == 2 {
        stack.swap(1, 0).unwrap()
    }

    let code = div_generate_bytecode::<T>(mode, mul_shift as u8, div_shift as u8);
    let mut engine =
        Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);

    match engine.execute() {
        Err(e) => panic!("Execute error: {}", e),
        Ok(_) => {
            if mode.premultiply() {
                value *= multiplier
            }

            let (expected_quotient, expected_remainder) = IntegerData::from_i32(value)
                .div::<T>(&IntegerData::from_i32(divisor), mode.rounding_strategy().unwrap())
                .unwrap();

            if mode.need_remainder() {
                let actual_remainder_si = engine.cc.stack.drop(0).unwrap();
                let actual_remainder = actual_remainder_si.as_integer().unwrap();
                assert_eq!(expected_remainder, *actual_remainder, "Remainder");
            }

            if mode.need_quotient() {
                let actual_quotient_si = engine.cc.stack.drop(0).unwrap();
                let actual_quotient = actual_quotient_si.as_integer().unwrap();
                assert_eq!(expected_quotient, *actual_quotient, "Quotient");
            }
        }
    }
}

fn div_generate_bytecode<T>(mode: &DivMode, mul_shift: u8, div_shift: u8) -> SliceData
where
    T: OperationBehavior,
{
    let mut res = Vec::<u8>::with_capacity(5);
    if T::quiet() {
        res.push(0xB7);
    }

    res.push(0xA9);
    res.push(mode.flags);
    if mode.shift_parameter() && (mode.mul_by_shift() || mode.div_by_shift()) {
        if mode.mul_by_shift() {
            res.push(mul_shift - 1);
        } else {
            res.push(div_shift - 1);
        }
    }

    res.push(0x80);
    SliceData::new(res)
}

fn test_slice(offset: usize, r: usize, x: usize) -> Status {
    let mut builder = BuilderData::default();
    builder.append_bits(0x7A53, offset)?; // prefix of command
    builder.append_bits(0, r)?; // references
    builder.append_bits(2, x)?; // bytes
    builder.append_bits(0, (8 - (offset + r + x) % 8) % 8)?; // remainder of data
    builder.append_bits(0xF837, 16)?; // data 2 bytes
    builder.append_bits(0x34, 8)?; // remainder in code slice

    let mut code = SliceData::load_builder(builder).unwrap();
    println!("offset: {}, r: {}, x: {}, code: {}", offset, r, x, code);
    let mut engine =
        Engine::with_capabilities(0).setup_with_libraries(code.clone(), None, None, None, vec![]);
    engine
        .load_instruction(
            Instruction::new("PUSHCTR").set_opts(InstructionOptions::Bitstring(offset, r, x, 0)),
        )
        .unwrap();

    let slice = engine.cmd.slice().clone();
    assert_eq!(engine.seek_next_cmd().unwrap(), None);

    let mut remainder = code.clone();
    remainder.shrink_data(32..);
    assert_eq!(&remainder, engine.cc.code());

    code.shrink_data(offset + r + x..31);
    assert_eq!(code, slice);
    Ok(())
}

#[test]
fn test_extract_slice() {
    test_slice(9, 2, 3).unwrap(); // STSLICECONST a command, x, r and data in the same byte
    test_slice(6, 0, 7).unwrap();
    test_slice(7, 2, 7).unwrap();
    test_slice(12, 0, 4).unwrap();
    test_slice(8, 0, 4).unwrap();
    test_slice(8, 2, 5).unwrap();
    test_slice(0, 3, 7).unwrap();

    for r in 0..4 {
        for x in 2..8 {
            let min_offset = std::cmp::min(8, 16 - r - x);
            for offset in min_offset..16 - r - x {
                test_slice(offset, r, x).unwrap();
            }
        }
    }
}

#[test]
fn test_currency_collection_ser() {
    let b1 = serialize_currency_collection(12345678u128, None).unwrap();
    let b2 = BuilderData::with_raw(vec![0x3b, 0xc6, 0x14, 0xe0], 29).unwrap();
    assert_eq!(b1, b2);
}

static DEFAULT_CAPABILITIES: u64 = 0x572e;

fn read_boc(filename: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut file = std::fs::File::open(filename).unwrap();
    std::io::Read::read_to_end(&mut file, &mut bytes).unwrap();
    bytes
}

fn load_boc(filename: &str) -> tvm_types::Cell {
    let bytes = read_boc(filename);
    tvm_types::read_single_root_boc(bytes).unwrap()
}

#[test]
fn test_termination_deadline() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let mut stack = Stack::new();
    stack.push(StackItem::int(1000000000));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(-2));

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    let from_start = Instant::now();
    // usually this execution requires 250-300 ms
    engine.set_termination_deadline(Some(Instant::now() + Duration::from_millis(50)));
    let err = engine.execute().expect_err("Should be failed with termination deadline reached");
    assert!(from_start.elapsed() < Duration::from_millis(55));
    assert!(matches!(
        err.downcast_ref::<TvmError>().unwrap(),
        TvmError::TerminationDeadlineReached
    ));
}

#[test]
fn test_execution_timeout() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let mut stack = Stack::new();
    stack.push(StackItem::int(1000000000));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(-2));

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        Some(Gas::test_with_credit(1013)),
        vec![],
    );
    let from_start = Instant::now();
    // usually this execution requires 250-300 ms
    engine.set_execution_timeout(Some(Duration::from_millis(50)));
    let err = engine.execute().expect_err("Should be failed with execution timeout");
    assert!(from_start.elapsed() < Duration::from_millis(55));
    let TvmError::TvmExceptionFull(exc, _) = err.downcast_ref::<TvmError>().unwrap() else {
        panic!("Should be TvmExceptionFull");
    };
    assert_eq!(exc.exception_code(), Some(ExceptionCode::ExecutionTimeout));
}

#[test]
#[cfg(feature = "wasm_external")]
fn test_run_wasm_basic_add() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap(); // Should not error on empty hash list!

    let cell = TokenValue::write_bytes(&Vec::<u8>::new().as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8, 2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();

    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let filename = "./src/tests/add.wasm";
    let wasm_dict = std::fs::read(filename).unwrap();

    let cell = TokenValue::write_bytes(&wasm_dict, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);

    assert!(
        rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).unwrap().pop().unwrap()
            == 3u8
    );
}

#[test]
#[cfg(not(feature = "wasm_external"))]
fn test_run_wasm_fail_on_external() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap(); // Should not error on empty hash list!

    let cell = TokenValue::write_bytes(&Vec::<u8>::new().as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8, 2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();

    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let filename = "./src/tests/add.wasm";
    let wasm_dict = std::fs::read(filename).unwrap();

    let cell = TokenValue::write_bytes(&wasm_dict, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm(&mut engine);
    println!("Wasm Return Status: {:?}", status);

    let _res_error = status.expect_err(
        "Test didn't error on external wasm despite disabled feature \"wasm_external\"",
    );
}

#[test]
fn test_run_wasm_io_plug_hashmap() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "e7adc782c05b67bcda5babaca1deabf80f30ca0e6cf668c89825286c3ce0e560";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap();
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8, 2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();

    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let wasm_dict = []; //std::fs::read(filename).unwrap();

    let cell = TokenValue::write_bytes(&wasm_dict, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);

    assert!(
        rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).unwrap().pop().unwrap()
            == 11u8
    );
}

#[test]
fn test_run_wasm_from_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2";

    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap();

    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8, 2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);

    assert!(
        rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).unwrap().pop().unwrap()
            == 3u8
    );
}

#[test]
fn test_tls_wasm_from_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap();
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[4u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[3u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm_concat_multiarg(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);

    assert!(
        rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).unwrap().pop().unwrap()
            == 3u8
    );
}

#[test]
fn test_wasm_from_nonexistent_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "1234567890123456789012345678901234567890123456789012345678901234";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    // we skip precompilation as it would check the hash and error early
    // let mut engine = engine.precompile_all_wasm_by_hash().unwrap();
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8, 2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let result = execute_run_wasm(&mut engine);

    println!("Wasm Return Status: {:?}", result);

    let _res_error = result.expect_err("Test didn't error on unrecognised hash");
}

#[test]
fn test_wasm_from_wrong_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "0000000000000000000000000000000000000000000000000000000000000000";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    // let mut engine = engine.precompile_all_wasm_by_hash().unwrap();
    // we skip precompilation as it would already check the hash and error early
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8, 2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let result = execute_run_wasm(&mut engine);

    println!("Wasm Return Status: {:?}", result);

    let _res_error = result.expect_err("Test didn't error on binary hash mismatch");
}

#[test]
fn test_wasm_from_non_whitelist_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2";
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let cell = TokenValue::write_bytes(&[1u8, 2u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let result = execute_run_wasm(&mut engine);

    println!("Wasm Return Status: {:?}", result);

    let _res_error = result.expect_err("Test didn't error on non-whitelist hash");
}

#[test]
#[cfg(feature = "wasm_external")]
fn test_run_wasm_fuel_error() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let cell = TokenValue::write_bytes(&Vec::<u8>::new().as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let cell =
        TokenValue::write_bytes(&[100u8, 0u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let filename = "./src/tests/calc_pi.wasm";
    let wasm_dict = std::fs::read(filename).unwrap();

    let cell = TokenValue::write_bytes(&wasm_dict, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let result = execute_run_wasm(&mut engine);

    println!("Wasm Return Status: {:?}", result);

    let _res_error = result.expect_err("Test didn't error on fuel use");
}

#[test]
fn test_run_wasm_deterministic_random_from_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap(); // Should not error on empty hash list!

    let hash_str = "9e3095fbdba10d1ea8303c63f7dbc4d1d51248cc2f328db909ee0546d508c954";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let its = 3000u32.to_be_bytes();
    println!("Its {:?}", its);
    let cell = TokenValue::write_bytes(&its, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();

    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "test";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_interface = "gosh:determinism/test-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_interface.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // let filename = "";
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);

    let res = rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).unwrap();
    // println!("Res: {:?}", res);
    // println!("Determinism Res: {:?}", res)
    let mut floats = Vec::new();
    for float in res.chunks(8) {
        floats.push(f64::from_le_bytes(float.try_into().unwrap()));
    }

    for _k in 0..1000 {
        let cell = TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4)
            .unwrap()
            .into_cell()
            .unwrap();
        engine.cc.stack.push(StackItem::cell(cell.clone()));

        let cell = TokenValue::write_bytes(&its, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
        engine.cc.stack.push(StackItem::cell(cell.clone()));

        let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
        engine.cc.stack.push(StackItem::cell(cell.clone()));

        let cell = pack_data_to_cell(&wasm_interface.as_bytes(), &mut engine).unwrap();
        engine.cc.stack.push(StackItem::cell(cell.clone()));

        let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
            .unwrap()
            .into_cell()
            .unwrap();
        // let cell = split_to_chain_of_cells(wasm_dict);
        // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
        engine.cc.stack.push(StackItem::cell(cell.clone()));

        let _status = execute_run_wasm(&mut engine).unwrap();
        // println!("Wasm Return Status: {:?}", status);

        let res = rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).unwrap();
        // println!("Res: {:?}", res);
        // println!("Determinism Res: {:?}", res)
        let mut new_floats = Vec::new();
        for float in res.chunks(8) {
            new_floats.push(f64::from_le_bytes(float.try_into().unwrap()));
        }
        assert_eq!(new_floats, floats);
    }
    // let mut wtr =
    // csv::Writer::from_path("./src/tests/determinism.csv").unwrap();
    // wtr.serialize(floats.clone()).unwrap();
    // wtr.flush().unwrap();

    // print!("Result [");
    // for f in floats {
    //     print!("{:.}, ", f);
    // }
    // println!("]");
    // assert!(
    //     rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).
    // unwrap().pop().unwrap()         == 3u8
    // );
}

#[test]
fn test_run_wasm_clock_from_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap(); // Should not error on empty hash list!
    engine.set_wasm_block_time(3005792924);

    let hash_str = "d61042b13fc07f5d7ece99ddfc57287dd736c4313dbeef2ea6b5282ca627a3b7";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let its = 3000u32.to_be_bytes();
    println!("Its {:?}", its);
    let cell = TokenValue::write_bytes(&its, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();

    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "test";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "gosh:determinism/test-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let cell = TokenValue::write_bytes(&Vec::<u8>::new().as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);

    let res = rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).unwrap();
    println!("Res: {:?}", res);
    // println!("Determinism Res: {:?}", res)
    let mut floats = Vec::new();
    for float in res.chunks(8) {
        floats.push(f64::from_le_bytes(float.try_into().unwrap()));
    }
    // let mut wtr =
    // csv::Writer::from_path("./src/tests/determinism.csv").unwrap();
    // wtr.serialize(floats.clone()).unwrap();
    // wtr.flush().unwrap();

    // print!("Result [");
    // for f in floats {
    //     print!("{:.}, ", f);
    // }
    // println!("]");
    // assert!(
    //     rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).
    // unwrap().pop().unwrap()         == 3u8
    // );
}

#[test]
fn test_run_wasm_fuel_error_from_hash() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "38a68caa4a3d3665b33c361c073664d0284a487ef11589950738362ee9b734da";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell = pack_data_to_cell(&hash.as_slice(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let cell =
        TokenValue::write_bytes(&[100u8, 0u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let result = execute_run_wasm(&mut engine);

    println!("Wasm Return Status: {:?}", result);

    let _res_error = result.expect_err("Test didn't error on fuel use");
}

#[test]
fn test_tls_wasm_from_hash_for_4_args() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let stack = Stack::new();

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    engine.wasm_engine_init_cached().unwrap();

    let hash_str = "f17ce37afc4301d138ad797efadce65387c32b7bba65886fd2b5fc7a48a98e5c";
    let _ = engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned());
    let mut engine = engine.precompile_all_wasm_by_hash().unwrap();
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let tls_data: Vec<u8> = vec![
        231, 226, 189, 128, 175, 192, 46, 233, 160, 243, 227, 168, 186, 174, 207, 111, 124, 21, 6,
        220, 18, 155, 18, 17, 39, 165, 203, 108, 109, 3, 40, 186, 1, 3, 22, 3, 1, 0, 161, 1, 0, 0,
        157, 3, 3, 153, 201, 82, 105, 180, 65, 114, 166, 164, 80, 99, 251, 211, 105, 39, 79, 221,
        226, 254, 4, 235, 219, 212, 9, 133, 1, 46, 158, 84, 154, 155, 224, 0, 0, 2, 19, 1, 1, 0, 0,
        114, 0, 0, 0, 23, 0, 21, 0, 0, 18, 119, 119, 119, 46, 103, 111, 111, 103, 108, 101, 97,
        112, 105, 115, 46, 99, 111, 109, 0, 10, 0, 4, 0, 2, 0, 29, 0, 13, 0, 20, 0, 18, 4, 3, 8, 4,
        4, 1, 5, 3, 8, 5, 5, 1, 8, 6, 6, 1, 2, 1, 0, 51, 0, 38, 0, 36, 0, 29, 0, 32, 192, 66, 56,
        95, 6, 86, 129, 217, 28, 232, 5, 177, 109, 189, 139, 154, 6, 3, 215, 62, 202, 195, 214,
        238, 231, 82, 157, 198, 107, 200, 81, 16, 0, 45, 0, 2, 1, 1, 0, 43, 0, 3, 2, 3, 4, 22, 3,
        3, 0, 90, 2, 0, 0, 86, 3, 3, 91, 88, 94, 30, 144, 45, 210, 2, 250, 82, 28, 37, 48, 190,
        122, 22, 33, 181, 241, 248, 121, 66, 170, 129, 172, 25, 130, 153, 7, 94, 248, 232, 0, 19,
        1, 0, 0, 46, 0, 51, 0, 36, 0, 29, 0, 32, 73, 55, 191, 233, 121, 215, 199, 76, 94, 141, 18,
        156, 54, 155, 16, 139, 239, 248, 235, 39, 96, 174, 94, 152, 89, 38, 172, 193, 32, 160, 238,
        62, 0, 43, 0, 2, 3, 4, 23, 3, 3, 11, 227, 6, 38, 234, 169, 253, 143, 23, 219, 38, 199, 225,
        129, 252, 87, 225, 160, 53, 100, 44, 164, 43, 199, 228, 176, 123, 176, 12, 228, 234, 121,
        174, 26, 73, 74, 193, 20, 142, 46, 50, 127, 217, 248, 174, 183, 145, 190, 56, 89, 143, 84,
        31, 46, 5, 0, 138, 61, 90, 166, 167, 126, 231, 140, 64, 244, 49, 70, 221, 111, 12, 107, 40,
        94, 37, 73, 238, 71, 146, 108, 188, 158, 203, 118, 149, 129, 172, 202, 251, 44, 241, 239,
        128, 159, 192, 71, 207, 223, 15, 65, 62, 18, 132, 32, 29, 114, 248, 66, 183, 125, 213, 49,
        172, 160, 72, 126, 123, 47, 183, 3, 209, 56, 239, 26, 236, 254, 156, 45, 69, 47, 232, 135,
        41, 136, 219, 57, 139, 228, 233, 212, 34, 53, 68, 185, 138, 38, 150, 121, 31, 240, 75, 238,
        243, 55, 1, 235, 133, 87, 214, 158, 244, 210, 135, 152, 232, 16, 109, 120, 51, 107, 92, 24,
        90, 195, 182, 107, 48, 240, 73, 174, 58, 116, 239, 218, 91, 4, 16, 40, 117, 200, 189, 14,
        167, 170, 219, 206, 108, 58, 233, 221, 255, 162, 186, 104, 140, 160, 129, 6, 3, 56, 157,
        209, 185, 63, 169, 110, 44, 61, 167, 137, 133, 63, 113, 105, 61, 186, 194, 107, 243, 122,
        240, 213, 128, 161, 195, 162, 124, 218, 215, 140, 18, 116, 251, 216, 226, 66, 115, 168, 99,
        137, 213, 47, 10, 129, 158, 173, 163, 3, 28, 236, 246, 6, 201, 190, 124, 99, 172, 111, 162,
        107, 254, 12, 156, 229, 216, 240, 34, 87, 231, 183, 170, 95, 49, 91, 227, 104, 99, 208, 96,
        239, 177, 6, 36, 198, 35, 145, 157, 212, 123, 89, 200, 100, 120, 15, 30, 181, 32, 126, 113,
        157, 16, 218, 135, 155, 181, 55, 202, 59, 127, 37, 62, 186, 252, 116, 88, 54, 154, 220, 4,
        91, 100, 78, 223, 250, 5, 91, 110, 18, 95, 150, 92, 193, 121, 221, 77, 178, 217, 170, 78,
        162, 175, 13, 240, 79, 203, 211, 250, 66, 252, 76, 155, 134, 229, 248, 251, 254, 105, 247,
        186, 239, 23, 55, 200, 233, 202, 125, 15, 62, 201, 208, 41, 186, 158, 176, 241, 233, 133,
        228, 248, 42, 166, 82, 25, 89, 46, 14, 226, 39, 165, 130, 182, 140, 87, 95, 240, 63, 51, 5,
        1, 196, 11, 11, 248, 79, 39, 116, 235, 80, 183, 175, 201, 149, 62, 238, 178, 198, 119, 102,
        107, 201, 195, 35, 228, 159, 78, 94, 91, 174, 250, 16, 25, 90, 68, 147, 239, 38, 80, 208,
        94, 27, 176, 61, 183, 173, 0, 134, 208, 139, 109, 125, 239, 29, 190, 239, 132, 119, 87,
        219, 4, 51, 54, 218, 187, 95, 11, 146, 42, 48, 163, 11, 223, 184, 220, 102, 91, 206, 89,
        101, 40, 79, 89, 197, 50, 138, 121, 46, 154, 85, 147, 240, 192, 82, 97, 83, 219, 173, 179,
        224, 127, 160, 32, 32, 190, 108, 192, 158, 222, 147, 156, 116, 188, 17, 92, 114, 78, 135,
        234, 173, 195, 231, 173, 99, 198, 26, 246, 76, 171, 107, 71, 232, 152, 11, 242, 133, 244,
        116, 242, 12, 118, 79, 35, 121, 118, 185, 60, 86, 62, 181, 12, 147, 15, 124, 249, 14, 106,
        181, 136, 251, 119, 246, 70, 145, 98, 107, 215, 185, 236, 207, 34, 107, 190, 110, 38, 169,
        35, 241, 191, 102, 232, 235, 236, 44, 220, 225, 222, 129, 191, 7, 61, 22, 33, 30, 61, 21,
        14, 65, 99, 39, 211, 37, 38, 39, 204, 209, 242, 86, 142, 69, 181, 108, 33, 240, 129, 35,
        69, 249, 82, 226, 103, 139, 89, 204, 11, 188, 27, 56, 48, 105, 145, 121, 121, 89, 105, 207,
        241, 209, 62, 23, 168, 64, 179, 230, 41, 151, 177, 196, 150, 156, 202, 43, 238, 8, 225, 36,
        7, 95, 52, 151, 186, 184, 249, 16, 99, 154, 145, 115, 5, 160, 229, 240, 148, 14, 135, 79,
        121, 133, 223, 162, 97, 75, 227, 18, 122, 4, 139, 123, 139, 63, 82, 155, 65, 104, 216, 4,
        173, 74, 104, 36, 255, 188, 80, 81, 196, 220, 65, 2, 220, 26, 41, 4, 108, 113, 21, 15, 171,
        164, 132, 103, 214, 141, 167, 70, 162, 151, 6, 104, 188, 238, 164, 178, 189, 11, 94, 19,
        174, 255, 194, 200, 162, 79, 235, 231, 73, 231, 141, 121, 176, 199, 0, 96, 58, 216, 83,
        146, 115, 105, 61, 42, 71, 197, 169, 14, 101, 98, 238, 165, 129, 144, 166, 104, 40, 20, 22,
        58, 195, 47, 173, 75, 135, 39, 94, 106, 143, 16, 71, 163, 88, 99, 226, 69, 175, 197, 138,
        250, 236, 145, 123, 59, 70, 250, 191, 228, 233, 101, 153, 168, 150, 46, 185, 234, 10, 152,
        178, 176, 57, 195, 248, 22, 211, 97, 39, 173, 21, 58, 118, 34, 45, 24, 105, 126, 25, 24,
        74, 183, 7, 3, 120, 8, 142, 109, 13, 247, 21, 253, 116, 240, 40, 19, 187, 18, 193, 178, 12,
        23, 181, 6, 4, 152, 176, 94, 48, 26, 197, 167, 152, 5, 22, 184, 181, 109, 56, 36, 198, 156,
        62, 111, 209, 141, 228, 255, 232, 210, 253, 207, 227, 237, 122, 208, 168, 165, 153, 21, 25,
        242, 134, 79, 47, 216, 2, 59, 237, 242, 213, 63, 55, 186, 111, 16, 198, 197, 71, 178, 63,
        226, 18, 67, 60, 166, 74, 249, 134, 47, 50, 66, 179, 33, 59, 167, 199, 101, 243, 106, 227,
        78, 58, 251, 79, 14, 25, 103, 85, 81, 16, 137, 208, 14, 183, 86, 186, 80, 109, 76, 211,
        215, 85, 74, 213, 61, 129, 57, 154, 126, 196, 54, 24, 164, 171, 227, 188, 211, 215, 157,
        121, 174, 137, 125, 113, 167, 126, 48, 241, 119, 12, 41, 78, 150, 251, 177, 110, 14, 232,
        172, 172, 230, 191, 178, 136, 34, 173, 27, 217, 109, 102, 17, 0, 147, 119, 195, 8, 248,
        110, 21, 154, 216, 202, 133, 156, 212, 228, 173, 94, 180, 184, 47, 20, 241, 1, 82, 195,
        233, 28, 76, 164, 170, 242, 150, 36, 148, 22, 41, 71, 73, 173, 152, 97, 8, 128, 250, 125,
        169, 84, 147, 180, 41, 101, 130, 113, 215, 181, 65, 175, 237, 191, 254, 40, 162, 5, 22, 92,
        21, 72, 98, 15, 61, 19, 157, 67, 41, 164, 96, 157, 198, 140, 179, 94, 231, 93, 129, 137,
        246, 141, 127, 149, 28, 91, 122, 167, 188, 65, 221, 112, 208, 38, 132, 199, 176, 9, 106,
        106, 215, 145, 113, 235, 168, 151, 131, 103, 221, 121, 126, 165, 43, 41, 112, 143, 166, 9,
        106, 185, 131, 224, 117, 198, 236, 181, 2, 144, 106, 121, 201, 112, 255, 214, 247, 175, 81,
        136, 74, 2, 80, 148, 116, 205, 159, 140, 232, 4, 157, 164, 148, 222, 150, 83, 173, 3, 113,
        252, 18, 12, 58, 187, 83, 114, 80, 18, 32, 192, 200, 230, 11, 72, 247, 66, 211, 78, 144,
        153, 170, 236, 75, 190, 211, 102, 99, 127, 55, 101, 107, 142, 97, 87, 240, 232, 158, 109,
        143, 24, 204, 158, 134, 10, 235, 2, 84, 221, 37, 122, 73, 47, 5, 160, 91, 21, 9, 162, 140,
        25, 82, 70, 156, 217, 78, 92, 92, 210, 236, 144, 8, 111, 242, 87, 86, 109, 11, 23, 137,
        250, 222, 157, 63, 213, 141, 179, 74, 128, 50, 108, 106, 205, 57, 222, 71, 33, 43, 35, 33,
        191, 220, 182, 6, 77, 174, 169, 255, 165, 199, 186, 36, 210, 46, 174, 246, 120, 248, 190,
        183, 252, 191, 75, 5, 220, 145, 102, 247, 20, 232, 163, 181, 134, 73, 197, 52, 62, 2, 202,
        179, 120, 225, 159, 161, 151, 239, 15, 81, 249, 50, 57, 82, 102, 220, 233, 102, 168, 193,
        231, 10, 94, 171, 167, 80, 73, 212, 86, 52, 56, 53, 103, 155, 235, 102, 66, 171, 151, 10,
        79, 42, 53, 198, 211, 145, 129, 162, 6, 5, 71, 145, 198, 94, 72, 82, 45, 123, 191, 85, 216,
        27, 97, 209, 230, 83, 233, 49, 44, 94, 187, 203, 91, 115, 142, 149, 17, 104, 169, 234, 9,
        6, 100, 245, 154, 148, 131, 25, 230, 139, 46, 222, 102, 113, 255, 217, 216, 76, 221, 125,
        182, 122, 68, 181, 190, 104, 48, 41, 202, 124, 138, 129, 171, 170, 207, 39, 150, 46, 149,
        194, 132, 185, 172, 211, 80, 202, 203, 180, 175, 185, 142, 222, 135, 50, 247, 137, 112,
        252, 149, 122, 97, 187, 152, 107, 233, 227, 181, 255, 254, 170, 88, 8, 50, 134, 237, 78,
        116, 158, 241, 106, 92, 203, 72, 39, 225, 129, 232, 235, 41, 7, 113, 182, 39, 110, 122,
        106, 183, 226, 150, 2, 249, 73, 117, 239, 209, 101, 248, 52, 227, 202, 154, 112, 138, 169,
        241, 35, 195, 146, 125, 91, 139, 60, 70, 0, 53, 229, 202, 145, 166, 97, 122, 187, 106, 170,
        152, 223, 42, 94, 211, 116, 7, 80, 84, 35, 178, 52, 114, 203, 185, 141, 234, 193, 162, 67,
        57, 242, 97, 157, 204, 223, 103, 119, 82, 16, 42, 184, 78, 182, 239, 107, 133, 179, 226,
        180, 110, 61, 205, 197, 213, 126, 132, 197, 30, 108, 230, 108, 207, 188, 231, 198, 135, 87,
        68, 183, 245, 129, 115, 231, 16, 162, 164, 65, 111, 229, 44, 41, 189, 243, 84, 216, 236,
        154, 122, 68, 186, 159, 153, 26, 65, 175, 100, 149, 74, 48, 247, 202, 128, 105, 26, 212,
        120, 238, 15, 94, 125, 208, 130, 187, 91, 180, 44, 22, 242, 192, 16, 178, 16, 184, 157,
        182, 169, 193, 55, 58, 139, 92, 147, 89, 226, 65, 124, 11, 193, 83, 198, 128, 140, 52, 222,
        5, 98, 245, 137, 48, 133, 216, 210, 247, 201, 53, 209, 199, 143, 177, 191, 45, 164, 165,
        220, 192, 60, 4, 190, 10, 170, 141, 254, 189, 234, 218, 9, 194, 27, 72, 242, 138, 95, 192,
        76, 126, 8, 153, 225, 7, 48, 101, 211, 247, 235, 82, 237, 146, 9, 235, 223, 71, 213, 211,
        113, 12, 189, 216, 147, 116, 255, 232, 239, 126, 252, 155, 208, 184, 197, 211, 62, 89, 226,
        110, 65, 30, 168, 196, 197, 225, 27, 100, 157, 245, 152, 181, 76, 93, 225, 30, 65, 165,
        103, 166, 14, 150, 122, 216, 239, 77, 209, 119, 220, 222, 214, 64, 142, 85, 145, 206, 224,
        141, 33, 167, 210, 28, 154, 57, 22, 171, 78, 26, 86, 169, 82, 135, 84, 247, 21, 115, 156,
        178, 28, 72, 168, 174, 33, 215, 21, 142, 244, 144, 234, 252, 119, 157, 87, 188, 141, 19,
        154, 48, 187, 3, 196, 215, 28, 184, 15, 96, 23, 226, 96, 239, 32, 141, 15, 118, 42, 142,
        82, 45, 225, 134, 85, 247, 9, 188, 90, 189, 18, 173, 120, 88, 108, 174, 237, 175, 139, 12,
        203, 43, 156, 197, 207, 98, 5, 209, 98, 14, 247, 17, 163, 101, 180, 103, 226, 207, 47, 110,
        148, 133, 163, 179, 197, 29, 163, 52, 59, 90, 157, 38, 224, 93, 236, 160, 213, 218, 114,
        174, 119, 193, 253, 249, 19, 61, 117, 128, 228, 219, 156, 113, 23, 86, 83, 251, 209, 98,
        209, 34, 60, 67, 138, 108, 91, 200, 27, 65, 255, 55, 109, 245, 115, 205, 249, 222, 1, 173,
        125, 57, 246, 201, 121, 97, 21, 58, 75, 27, 72, 92, 96, 155, 70, 96, 173, 15, 191, 136, 81,
        55, 209, 241, 76, 26, 44, 60, 81, 130, 9, 193, 84, 8, 73, 34, 169, 93, 101, 14, 72, 133,
        127, 62, 253, 123, 120, 131, 186, 75, 52, 8, 220, 90, 101, 141, 3, 149, 81, 227, 165, 121,
        106, 208, 135, 17, 146, 164, 136, 124, 90, 97, 205, 84, 50, 61, 188, 100, 146, 116, 100,
        142, 4, 0, 43, 29, 128, 193, 5, 107, 238, 63, 177, 48, 206, 59, 48, 52, 202, 32, 87, 198,
        216, 189, 189, 89, 68, 114, 119, 38, 203, 136, 228, 27, 139, 192, 146, 168, 51, 175, 161,
        169, 253, 47, 111, 197, 221, 5, 117, 180, 20, 94, 186, 253, 142, 50, 69, 139, 255, 209,
        159, 66, 5, 150, 174, 105, 17, 156, 138, 46, 221, 52, 252, 252, 118, 188, 231, 55, 18, 52,
        136, 52, 154, 131, 47, 83, 9, 243, 75, 35, 208, 154, 48, 55, 54, 170, 204, 87, 86, 235, 30,
        35, 201, 181, 159, 230, 104, 209, 183, 223, 163, 125, 105, 187, 198, 214, 189, 191, 51,
        113, 205, 20, 142, 96, 12, 165, 40, 235, 250, 65, 195, 38, 81, 237, 201, 81, 121, 81, 15,
        100, 51, 164, 161, 175, 57, 114, 226, 213, 38, 142, 3, 122, 74, 228, 156, 16, 34, 39, 204,
        188, 140, 9, 121, 121, 118, 239, 20, 175, 88, 29, 4, 212, 61, 48, 159, 186, 240, 124, 152,
        249, 134, 193, 217, 240, 14, 111, 144, 116, 4, 248, 228, 161, 247, 66, 120, 94, 248, 6,
        113, 33, 239, 20, 2, 37, 143, 112, 109, 27, 200, 24, 209, 103, 51, 44, 110, 123, 101, 52,
        240, 126, 142, 211, 43, 148, 143, 12, 153, 9, 32, 114, 143, 225, 191, 126, 122, 150, 221,
        242, 169, 181, 218, 115, 138, 140, 106, 172, 113, 153, 63, 75, 103, 138, 84, 145, 9, 156,
        8, 95, 255, 97, 16, 83, 206, 158, 245, 36, 194, 80, 30, 166, 252, 160, 244, 139, 48, 30,
        103, 8, 189, 2, 36, 254, 185, 111, 180, 253, 185, 172, 98, 117, 80, 143, 53, 175, 116, 69,
        227, 215, 5, 146, 185, 120, 164, 105, 242, 82, 166, 7, 21, 128, 129, 148, 151, 5, 226, 235,
        150, 89, 178, 207, 7, 245, 70, 102, 145, 130, 209, 228, 42, 44, 252, 171, 59, 141, 20, 33,
        34, 210, 89, 25, 64, 155, 198, 67, 216, 37, 101, 63, 9, 54, 82, 29, 104, 179, 59, 150, 239,
        33, 178, 108, 205, 121, 181, 176, 91, 201, 68, 19, 154, 110, 254, 113, 238, 22, 6, 244, 96,
        238, 220, 34, 165, 109, 250, 151, 154, 37, 136, 32, 56, 139, 122, 39, 153, 99, 38, 125, 4,
        53, 142, 189, 56, 36, 84, 229, 155, 186, 177, 194, 72, 234, 35, 158, 28, 236, 111, 54, 182,
        40, 255, 64, 76, 196, 87, 166, 236, 247, 164, 143, 244, 255, 80, 171, 135, 181, 125, 49,
        154, 182, 206, 15, 80, 132, 81, 133, 15, 119, 241, 189, 88, 2, 112, 45, 252, 236, 5, 204,
        62, 143, 124, 215, 17, 211, 20, 126, 171, 251, 206, 77, 185, 121, 178, 61, 75, 125, 35, 31,
        6, 230, 6, 123, 30, 140, 150, 16, 176, 130, 224, 172, 27, 4, 199, 97, 35, 140, 246, 23,
        138, 229, 119, 39, 101, 96, 242, 129, 69, 10, 56, 15, 69, 199, 176, 73, 156, 61, 237, 237,
        154, 46, 138, 22, 249, 3, 45, 135, 20, 7, 193, 62, 5, 11, 161, 193, 159, 184, 134, 86, 204,
        247, 170, 150, 240, 103, 119, 103, 156, 157, 2, 182, 13, 222, 62, 119, 38, 15, 195, 53,
        111, 198, 225, 180, 153, 59, 18, 29, 70, 192, 61, 195, 53, 108, 64, 239, 12, 202, 250, 89,
        61, 205, 19, 24, 122, 182, 52, 90, 90, 80, 97, 136, 67, 30, 252, 13, 220, 120, 234, 184,
        163, 8, 253, 72, 231, 250, 80, 188, 90, 94, 98, 72, 33, 61, 117, 51, 186, 57, 124, 92, 107,
        203, 219, 136, 161, 99, 106, 177, 211, 76, 47, 131, 183, 211, 229, 105, 73, 126, 116, 36,
        15, 212, 84, 125, 173, 94, 138, 186, 59, 13, 231, 52, 38, 164, 132, 217, 51, 132, 219, 21,
        14, 72, 103, 174, 126, 254, 75, 77, 10, 65, 115, 189, 228, 108, 27, 140, 156, 25, 122, 242,
        214, 33, 240, 228, 187, 28, 221, 53, 46, 160, 81, 94, 172, 123, 250, 0, 54, 25, 132, 12,
        111, 68, 64, 89, 101, 137, 174, 72, 130, 56, 114, 64, 231, 60, 205, 31, 131, 30, 34, 205,
        251, 177, 177, 207, 121, 126, 117, 125, 209, 231, 138, 113, 179, 245, 29, 107, 222, 211,
        217, 223, 64, 2, 23, 187, 237, 3, 150, 165, 91, 129, 109, 193, 168, 81, 27, 101, 118, 10,
        116, 99, 65, 221, 153, 255, 241, 25, 207, 67, 139, 77, 168, 20, 249, 79, 170, 129, 7, 210,
        156, 232, 210, 73, 243, 135, 6, 13, 27, 41, 179, 169, 87, 91, 32, 84, 192, 12, 130, 78, 1,
        7, 184, 181, 132, 146, 251, 85, 196, 208, 74, 178, 175, 11, 43, 1, 98, 105, 18, 72, 136,
        77, 231, 98, 97, 17, 241, 61, 8, 244, 167, 173, 218, 36, 129, 12, 88, 115, 212, 151, 53,
        217, 54, 185, 218, 24, 136, 56, 179, 227, 156, 213, 145, 108, 193, 192, 235, 68, 61, 106,
        106, 111, 25, 148, 81, 19, 182, 210, 56, 232, 86, 191, 108, 148, 210, 157, 90, 36, 160,
        153, 89, 141, 91, 70, 224, 30, 243, 203, 88, 192, 174, 198, 22, 186, 72, 190, 36, 32, 180,
        19, 247, 4, 156, 128, 244, 250, 182, 244, 217, 218, 177, 122, 105, 144, 72, 71, 252, 220,
        51, 220, 209, 249, 18, 96, 227, 50, 187, 36, 86, 122, 210, 115, 237, 48, 143, 191, 6, 196,
        150, 77, 29, 134, 183, 219, 157, 28, 71, 43, 83, 119, 107, 151, 71, 91, 7, 20, 112, 147,
        150, 169, 187, 147, 33, 170, 149, 216, 85, 244, 6, 58, 113, 237, 223, 202, 56, 155, 0, 132,
        23, 109, 137, 112, 28, 210, 45, 148, 86, 71, 143, 244, 176, 97, 83, 208, 171, 6, 211, 197,
        67, 21, 35, 192, 132, 125, 90, 220, 36, 213, 34, 134, 196, 85, 42, 168, 21, 185, 65, 207,
        79, 222, 217, 24, 78, 255, 120, 249, 240, 63, 250, 28, 86, 70, 244, 123, 211, 23, 3, 3, 0,
        95, 31, 180, 246, 42, 71, 22, 71, 86, 198, 65, 22, 142, 199, 87, 95, 232, 226, 20, 162, 35,
        93, 85, 74, 144, 155, 172, 90, 121, 73, 232, 4, 181, 67, 152, 80, 33, 79, 3, 164, 253, 59,
        6, 120, 138, 2, 216, 40, 86, 33, 132, 163, 150, 6, 227, 77, 140, 212, 92, 150, 167, 18, 82,
        20, 250, 76, 55, 209, 25, 77, 115, 44, 117, 9, 241, 54, 0, 59, 248, 26, 217, 196, 81, 53,
        33, 111, 197, 126, 124, 198, 110, 137, 105, 134, 124, 199, 23, 3, 3, 2, 23, 211, 118, 56,
        154, 9, 109, 101, 178, 249, 67, 240, 3, 205, 200, 175, 24, 33, 8, 67, 175, 169, 0, 133,
        198, 192, 126, 40, 222, 103, 20, 24, 90, 188, 237, 173, 51, 24, 202, 241, 148, 87, 241, 56,
        169, 86, 253, 219, 239, 105, 130, 98, 225, 143, 168, 238, 55, 123, 86, 202, 25, 212, 6, 30,
        234, 214, 141, 196, 151, 50, 26, 97, 166, 67, 142, 251, 221, 122, 82, 38, 184, 201, 152,
        241, 113, 228, 145, 9, 24, 52, 69, 199, 182, 35, 206, 201, 130, 216, 22, 50, 147, 79, 50,
        103, 27, 42, 73, 103, 66, 202, 198, 125, 144, 234, 22, 145, 215, 153, 227, 219, 139, 254,
        95, 223, 196, 163, 98, 218, 21, 156, 6, 115, 31, 190, 90, 164, 219, 163, 133, 52, 119, 246,
        227, 66, 249, 46, 23, 77, 39, 23, 206, 119, 139, 133, 112, 124, 125, 48, 132, 116, 103,
        180, 195, 70, 133, 234, 212, 212, 212, 122, 93, 193, 253, 33, 50, 19, 83, 7, 215, 30, 115,
        167, 6, 242, 73, 12, 225, 141, 216, 243, 195, 58, 197, 157, 92, 150, 149, 204, 215, 173,
        14, 237, 8, 49, 200, 123, 49, 193, 9, 149, 116, 234, 102, 214, 207, 161, 102, 69, 79, 215,
        202, 56, 156, 171, 165, 30, 199, 177, 19, 9, 166, 240, 179, 7, 204, 73, 172, 198, 117, 222,
        198, 244, 81, 143, 78, 52, 162, 170, 31, 209, 120, 2, 85, 2, 186, 60, 86, 143, 42, 201,
        236, 225, 146, 105, 81, 50, 146, 97, 132, 125, 194, 81, 87, 74, 4, 31, 198, 91, 235, 87,
        229, 182, 200, 57, 48, 68, 54, 212, 99, 173, 134, 101, 109, 20, 188, 57, 0, 119, 101, 102,
        48, 161, 22, 109, 18, 85, 111, 147, 196, 197, 85, 52, 18, 12, 126, 231, 219, 73, 231, 136,
        11, 136, 56, 190, 155, 75, 228, 21, 27, 131, 60, 7, 150, 74, 117, 38, 129, 165, 158, 37,
        203, 215, 138, 54, 48, 211, 174, 51, 155, 170, 139, 252, 92, 72, 41, 95, 36, 43, 14, 92,
        40, 29, 119, 221, 244, 175, 104, 83, 119, 210, 106, 92, 103, 83, 73, 199, 125, 126, 91,
        138, 100, 119, 154, 29, 143, 148, 134, 94, 168, 15, 218, 3, 221, 77, 68, 163, 49, 69, 186,
        71, 56, 229, 226, 44, 2, 157, 84, 216, 62, 180, 158, 86, 234, 101, 86, 86, 102, 239, 60,
        155, 246, 248, 106, 56, 157, 80, 217, 24, 11, 150, 9, 61, 93, 241, 25, 3, 75, 54, 223, 90,
        130, 113, 187, 127, 46, 172, 220, 109, 55, 183, 119, 126, 198, 58, 104, 244, 69, 148, 212,
        97, 37, 194, 113, 8, 107, 102, 193, 200, 250, 78, 5, 134, 253, 47, 123, 21, 184, 157, 13,
        10, 190, 234, 185, 240, 104, 67, 219, 151, 167, 52, 117, 86, 196, 141, 136, 242, 30, 31,
        217, 142, 112, 182, 59, 170, 195, 188, 59, 197, 68, 193, 216, 200, 69, 250, 234, 182, 208,
        27, 155, 202, 58, 112, 134, 165, 65, 250, 219, 182, 203, 173, 102, 92, 45, 1, 228, 117, 23,
        3, 3, 5, 95, 98, 31, 247, 59, 68, 234, 27, 226, 222, 6, 38, 69, 203, 165, 61, 237, 67, 211,
        89, 229, 51, 122, 129, 42, 157, 58, 22, 84, 115, 20, 7, 119, 249, 161, 35, 52, 128, 250,
        179, 4, 42, 227, 6, 170, 161, 48, 49, 10, 62, 150, 126, 160, 56, 162, 194, 253, 110, 184,
        205, 99, 189, 230, 73, 116, 45, 28, 13, 235, 150, 241, 206, 82, 58, 253, 183, 204, 237, 73,
        173, 217, 100, 162, 98, 6, 25, 105, 114, 163, 210, 104, 150, 168, 11, 95, 183, 104, 98,
        157, 220, 178, 160, 40, 98, 37, 3, 86, 137, 129, 85, 164, 248, 45, 234, 205, 3, 16, 78,
        221, 237, 20, 128, 2, 19, 56, 64, 179, 162, 30, 152, 155, 176, 23, 147, 8, 33, 66, 206,
        183, 86, 164, 60, 24, 152, 77, 10, 16, 45, 193, 194, 5, 222, 9, 177, 70, 167, 6, 153, 60,
        127, 167, 87, 187, 151, 0, 64, 51, 114, 119, 133, 62, 182, 158, 254, 36, 145, 96, 184, 81,
        99, 0, 142, 13, 251, 61, 49, 68, 186, 104, 35, 28, 129, 28, 183, 103, 54, 0, 61, 40, 246,
        226, 17, 179, 213, 254, 193, 198, 120, 80, 78, 61, 147, 195, 30, 218, 225, 56, 41, 84, 202,
        174, 68, 137, 8, 1, 16, 111, 137, 148, 140, 9, 233, 97, 162, 152, 146, 41, 73, 114, 128,
        246, 9, 143, 100, 87, 75, 221, 46, 37, 38, 159, 125, 64, 15, 70, 12, 106, 41, 7, 53, 137,
        113, 244, 209, 228, 133, 201, 65, 250, 88, 8, 41, 235, 56, 244, 112, 108, 255, 15, 212, 31,
        20, 153, 198, 161, 122, 224, 214, 155, 192, 49, 41, 74, 183, 54, 252, 125, 124, 227, 168,
        113, 7, 226, 72, 7, 64, 100, 160, 131, 181, 65, 128, 104, 181, 30, 116, 63, 54, 121, 87,
        41, 105, 233, 176, 164, 106, 191, 209, 243, 187, 161, 218, 51, 106, 225, 39, 98, 190, 42,
        46, 185, 1, 202, 67, 28, 91, 143, 241, 150, 203, 125, 12, 151, 93, 177, 209, 12, 100, 116,
        205, 120, 178, 111, 2, 85, 155, 107, 192, 13, 226, 158, 152, 22, 95, 205, 81, 231, 255,
        154, 106, 155, 215, 157, 221, 120, 195, 191, 236, 198, 157, 238, 233, 47, 148, 249, 45,
        227, 214, 181, 198, 92, 126, 24, 114, 116, 146, 37, 198, 255, 83, 29, 242, 41, 198, 48, 37,
        79, 195, 93, 63, 118, 246, 157, 255, 219, 109, 162, 73, 9, 136, 248, 109, 175, 244, 30, 41,
        199, 193, 111, 87, 94, 92, 14, 79, 157, 153, 166, 221, 246, 73, 240, 184, 34, 85, 69, 129,
        39, 15, 222, 115, 121, 67, 237, 77, 102, 129, 190, 34, 143, 135, 150, 96, 176, 85, 138,
        203, 36, 150, 191, 29, 133, 111, 124, 215, 178, 164, 199, 186, 228, 185, 107, 116, 31, 238,
        236, 204, 62, 94, 180, 246, 227, 198, 82, 90, 230, 151, 109, 23, 65, 194, 242, 75, 95, 245,
        7, 158, 135, 143, 242, 226, 181, 133, 9, 56, 203, 40, 95, 66, 42, 217, 183, 172, 157, 188,
        0, 111, 158, 163, 95, 188, 128, 227, 164, 141, 157, 237, 166, 161, 23, 221, 150, 74, 179,
        36, 151, 2, 149, 53, 194, 135, 97, 244, 124, 55, 26, 165, 109, 44, 9, 123, 236, 125, 112,
        140, 143, 222, 213, 60, 227, 54, 219, 87, 104, 190, 67, 212, 110, 28, 237, 122, 202, 231,
        199, 247, 70, 131, 72, 69, 90, 130, 200, 99, 35, 243, 76, 232, 117, 168, 7, 135, 77, 192,
        31, 115, 93, 167, 215, 167, 192, 120, 157, 76, 69, 190, 164, 8, 2, 94, 81, 32, 14, 158,
        239, 179, 180, 14, 223, 172, 112, 31, 136, 173, 149, 181, 193, 130, 247, 100, 225, 232, 58,
        121, 55, 122, 148, 152, 241, 238, 90, 122, 89, 129, 62, 74, 44, 78, 189, 156, 152, 150,
        106, 233, 101, 44, 146, 254, 195, 48, 220, 22, 238, 53, 198, 16, 250, 54, 231, 106, 82,
        225, 146, 100, 139, 6, 215, 105, 185, 197, 36, 182, 186, 237, 151, 105, 143, 163, 165, 195,
        253, 90, 9, 127, 164, 110, 126, 253, 236, 207, 211, 4, 159, 229, 84, 199, 116, 240, 83,
        222, 192, 101, 29, 123, 177, 97, 16, 218, 6, 119, 48, 82, 94, 72, 155, 19, 63, 19, 42, 152,
        200, 200, 62, 126, 220, 132, 173, 165, 181, 71, 145, 36, 228, 26, 92, 176, 36, 101, 18, 97,
        118, 139, 177, 177, 254, 74, 187, 36, 251, 23, 24, 190, 94, 108, 75, 39, 146, 126, 233,
        119, 90, 11, 85, 193, 180, 202, 143, 102, 146, 236, 165, 143, 19, 10, 183, 109, 230, 107,
        85, 202, 74, 173, 54, 58, 251, 252, 15, 191, 25, 212, 179, 167, 100, 53, 4, 67, 112, 189,
        48, 40, 233, 96, 232, 51, 213, 245, 34, 103, 48, 14, 207, 65, 226, 39, 190, 150, 31, 39,
        139, 159, 62, 141, 114, 242, 252, 63, 217, 216, 24, 114, 246, 151, 215, 49, 197, 82, 139,
        31, 87, 51, 243, 129, 190, 171, 44, 12, 74, 141, 96, 130, 161, 223, 64, 15, 151, 179, 245,
        96, 234, 24, 166, 143, 119, 172, 2, 139, 242, 116, 116, 116, 87, 56, 42, 58, 162, 7, 190,
        22, 89, 111, 112, 34, 56, 87, 178, 243, 215, 112, 182, 235, 136, 103, 159, 61, 105, 191,
        67, 197, 70, 27, 237, 240, 48, 89, 89, 133, 171, 125, 108, 83, 167, 163, 111, 114, 230,
        185, 248, 57, 49, 98, 23, 83, 167, 200, 38, 241, 194, 55, 211, 107, 128, 188, 196, 227,
        138, 140, 203, 3, 53, 241, 19, 208, 88, 11, 223, 203, 15, 253, 203, 175, 42, 166, 65, 40,
        237, 120, 32, 209, 14, 202, 250, 44, 113, 170, 240, 206, 202, 18, 14, 107, 80, 130, 240,
        169, 151, 193, 8, 188, 201, 229, 212, 41, 118, 230, 27, 149, 129, 111, 118, 232, 140, 63,
        1, 179, 47, 198, 60, 117, 120, 197, 223, 209, 169, 168, 157, 5, 34, 146, 144, 217, 216, 15,
        212, 206, 235, 76, 156, 131, 46, 94, 239, 96, 94, 253, 222, 39, 47, 80, 149, 47, 93, 169,
        129, 99, 86, 31, 54, 103, 183, 191, 116, 13, 44, 220, 134, 229, 1, 223, 191, 42, 14, 205,
        6, 245, 136, 205, 76, 75, 170, 89, 142, 88, 79, 79, 114, 80, 125, 59, 7, 13, 191, 24, 224,
        3, 203, 89, 37, 65, 125, 126, 200, 48, 250, 217, 193, 42, 234, 134, 59, 162, 30, 149, 173,
        227, 190, 41, 227, 67, 142, 135, 14, 187, 184, 206, 198, 96, 35, 209, 81, 51, 244, 188,
        166, 229, 237, 124, 97, 153, 42, 85, 102, 237, 165, 92, 153, 58, 70, 162, 220, 175, 195,
        13, 107, 109, 170, 83, 202, 8, 24, 112, 45, 201, 244, 144, 219, 120, 64, 50, 151, 198, 141,
        53, 38, 9, 28, 31, 83, 248, 228, 28, 231, 253, 109, 24, 48, 79, 152, 126, 134, 40, 109,
        157, 174, 160, 52, 4, 25, 78, 163, 158, 84, 69, 235, 72, 43, 240, 248, 151, 18, 163, 170,
        180, 232, 191, 65, 114, 116, 188, 136, 191, 60, 70, 28, 106, 55, 105, 161, 120, 103, 200,
        52, 36, 113, 152, 28, 16, 219, 138, 46, 119, 234, 80, 162, 39, 17, 52, 113, 95, 193, 102,
        163, 229, 101, 218, 96, 179, 243, 34, 92, 124, 239, 95, 109, 216, 28, 224, 136, 113, 143,
        179, 62, 26, 214, 7, 38, 34, 144, 86, 154, 72, 121, 197, 97, 230, 5, 238, 178, 125, 220,
        124, 194, 156, 127, 38, 126, 191, 207, 181, 79, 71, 5, 0, 7, 206, 72, 236, 109, 71, 21, 29,
        28, 194, 190, 56, 33, 155, 209, 156, 180, 208, 225, 27, 18, 11, 126, 46, 43, 215, 64, 51,
        212, 71, 255, 242, 235, 89, 54, 187, 3, 178, 105, 69, 180, 124, 165, 66, 198, 125, 207, 56,
        126, 243, 69, 108, 231, 180, 181, 175, 131, 31, 169, 57, 239, 118, 16, 126, 224, 21, 23, 3,
        3, 2, 211, 178, 150, 180, 206, 153, 40, 147, 33, 230, 132, 110, 146, 140, 9, 213, 105, 178,
        100, 189, 156, 242, 17, 39, 9, 122, 199, 63, 23, 177, 238, 211, 26, 173, 95, 139, 234, 227,
        129, 88, 53, 5, 87, 17, 5, 40, 72, 134, 213, 248, 38, 82, 45, 40, 180, 144, 167, 186, 241,
        135, 158, 147, 187, 130, 150, 156, 80, 216, 195, 27, 52, 137, 166, 53, 174, 168, 62, 182,
        195, 165, 168, 254, 122, 102, 116, 97, 240, 30, 85, 226, 41, 21, 206, 63, 56, 178, 105,
        185, 29, 248, 144, 197, 200, 19, 243, 28, 183, 159, 244, 101, 165, 87, 102, 97, 203, 192,
        201, 83, 21, 9, 91, 144, 66, 218, 20, 124, 121, 230, 225, 200, 185, 233, 68, 162, 83, 150,
        169, 14, 115, 135, 231, 240, 17, 228, 184, 196, 195, 123, 235, 113, 156, 9, 104, 90, 61,
        168, 255, 4, 156, 243, 26, 63, 34, 80, 185, 198, 138, 133, 187, 253, 214, 77, 224, 141,
        124, 202, 166, 14, 244, 49, 76, 74, 77, 99, 38, 182, 105, 137, 107, 56, 179, 17, 60, 9, 54,
        27, 160, 190, 147, 203, 240, 98, 188, 27, 124, 48, 45, 36, 150, 40, 123, 253, 237, 78, 158,
        249, 142, 229, 245, 251, 144, 69, 171, 167, 31, 60, 114, 49, 199, 165, 239, 244, 76, 90,
        94, 118, 238, 216, 219, 27, 19, 194, 29, 192, 231, 86, 176, 204, 173, 13, 102, 51, 108,
        202, 221, 180, 105, 186, 91, 18, 35, 147, 153, 115, 252, 191, 160, 5, 197, 161, 190, 156,
        92, 81, 220, 231, 170, 3, 3, 76, 75, 108, 0, 231, 249, 84, 251, 151, 170, 8, 146, 45, 93,
        149, 178, 145, 90, 127, 249, 112, 149, 227, 48, 110, 199, 240, 130, 116, 52, 250, 232, 246,
        196, 60, 114, 29, 165, 10, 139, 104, 127, 199, 45, 10, 232, 187, 9, 72, 169, 90, 213, 63,
        113, 14, 96, 128, 200, 60, 174, 190, 79, 233, 181, 52, 51, 180, 241, 48, 161, 217, 52, 205,
        80, 167, 36, 55, 237, 32, 217, 67, 34, 182, 60, 189, 219, 17, 120, 63, 17, 249, 25, 92, 37,
        143, 108, 213, 100, 34, 84, 137, 21, 213, 110, 18, 68, 193, 199, 60, 161, 214, 15, 87, 157,
        169, 223, 146, 129, 221, 220, 138, 240, 232, 68, 238, 222, 241, 28, 11, 14, 170, 184, 185,
        121, 172, 75, 32, 228, 6, 123, 18, 34, 64, 132, 102, 35, 31, 125, 158, 73, 80, 182, 189,
        149, 178, 245, 65, 65, 239, 114, 8, 49, 192, 93, 142, 219, 87, 87, 178, 240, 159, 115, 139,
        126, 77, 9, 223, 4, 173, 149, 153, 245, 175, 247, 47, 81, 55, 187, 218, 220, 129, 206, 198,
        61, 73, 56, 194, 200, 194, 78, 18, 174, 152, 50, 73, 34, 61, 91, 172, 97, 70, 193, 131, 56,
        241, 54, 182, 54, 94, 110, 80, 199, 2, 152, 4, 200, 8, 137, 3, 6, 7, 37, 121, 19, 218, 32,
        166, 138, 151, 68, 171, 246, 79, 59, 176, 165, 146, 131, 248, 63, 148, 148, 246, 236, 73,
        100, 105, 146, 70, 224, 40, 57, 192, 211, 245, 3, 207, 29, 237, 218, 213, 143, 205, 110,
        174, 239, 55, 157, 14, 31, 224, 61, 42, 57, 232, 252, 19, 190, 111, 179, 47, 136, 195, 167,
        140, 54, 82, 88, 99, 108, 68, 131, 187, 14, 45, 19, 4, 10, 145, 92, 184, 170, 237, 3, 86,
        43, 27, 46, 23, 215, 102, 167, 172, 175, 222, 10, 124, 155, 132, 78, 198, 84, 106, 98, 229,
        114, 94, 241, 58, 199, 48, 88, 18, 232, 239, 37, 22, 28, 9, 99, 95, 28, 70, 112, 130, 37,
        224, 255, 11, 79, 165, 115, 123, 62, 208, 149, 65, 246, 82, 114, 131, 183, 55, 86, 131,
        134, 183, 99, 175, 178, 199, 7, 26, 108, 31, 138, 230, 230, 92, 114, 176, 53, 29, 134, 203,
        73, 37, 133, 117, 138, 223, 191, 19, 224, 38, 31, 210, 160, 136, 106, 155, 12, 78, 171,
        148, 2, 100, 232, 114, 142, 37, 190, 157, 180, 34, 178, 233, 108, 111, 110, 229, 175, 39,
        14, 56, 241, 96, 217, 83, 243, 245, 237, 101, 165, 229,
    ];
    let cell = TokenValue::write_bytes(/* &[4u8] */ &tls_data, &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let cert: Vec<u8> = vec![
        5, 102, 48, 130, 5, 98, 48, 130, 4, 74, 160, 3, 2, 1, 2, 2, 16, 119, 189, 13, 108, 219, 54,
        249, 26, 234, 33, 15, 196, 240, 88, 211, 13, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1,
        11, 5, 0, 48, 87, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 66, 69, 49, 25, 48, 23, 6, 3, 85,
        4, 10, 19, 16, 71, 108, 111, 98, 97, 108, 83, 105, 103, 110, 32, 110, 118, 45, 115, 97, 49,
        16, 48, 14, 6, 3, 85, 4, 11, 19, 7, 82, 111, 111, 116, 32, 67, 65, 49, 27, 48, 25, 6, 3,
        85, 4, 3, 19, 18, 71, 108, 111, 98, 97, 108, 83, 105, 103, 110, 32, 82, 111, 111, 116, 32,
        67, 65, 48, 30, 23, 13, 50, 48, 48, 54, 49, 57, 48, 48, 48, 48, 52, 50, 90, 23, 13, 50, 56,
        48, 49, 50, 56, 48, 48, 48, 48, 52, 50, 90, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2,
        85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114,
        117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6,
        3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 49, 48, 130, 2, 34, 48, 13,
        6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 2, 15, 0, 48, 130, 2, 10, 2, 130,
        2, 1, 0, 182, 17, 2, 139, 30, 227, 161, 119, 155, 59, 220, 191, 148, 62, 183, 149, 167, 64,
        60, 161, 253, 130, 249, 125, 50, 6, 130, 113, 246, 246, 140, 127, 251, 232, 219, 188, 106,
        46, 151, 151, 163, 140, 75, 249, 43, 246, 177, 249, 206, 132, 29, 177, 249, 197, 151, 222,
        239, 185, 242, 163, 233, 188, 18, 137, 94, 167, 170, 82, 171, 248, 35, 39, 203, 164, 177,
        156, 99, 219, 215, 153, 126, 240, 10, 94, 235, 104, 166, 244, 198, 90, 71, 13, 77, 16, 51,
        227, 78, 177, 19, 163, 200, 24, 108, 75, 236, 252, 9, 144, 223, 157, 100, 41, 37, 35, 7,
        161, 180, 210, 61, 46, 96, 224, 207, 210, 9, 135, 187, 205, 72, 240, 77, 194, 194, 122,
        136, 138, 187, 186, 207, 89, 25, 214, 175, 143, 176, 7, 176, 158, 49, 241, 130, 193, 192,
        223, 46, 166, 109, 108, 25, 14, 181, 216, 126, 38, 26, 69, 3, 61, 176, 121, 164, 148, 40,
        173, 15, 127, 38, 229, 168, 8, 254, 150, 232, 60, 104, 148, 83, 238, 131, 58, 136, 43, 21,
        150, 9, 178, 224, 122, 140, 46, 117, 214, 156, 235, 167, 86, 100, 143, 150, 79, 104, 174,
        61, 151, 194, 132, 143, 192, 188, 64, 192, 11, 92, 189, 246, 135, 179, 53, 108, 172, 24,
        80, 127, 132, 224, 76, 205, 146, 211, 32, 233, 51, 188, 82, 153, 175, 50, 181, 41, 179, 37,
        42, 180, 72, 249, 114, 225, 202, 100, 247, 230, 130, 16, 141, 232, 157, 194, 138, 136, 250,
        56, 102, 138, 252, 99, 249, 1, 249, 120, 253, 123, 92, 119, 250, 118, 135, 250, 236, 223,
        177, 14, 121, 149, 87, 180, 189, 38, 239, 214, 1, 209, 235, 22, 10, 187, 142, 11, 181, 197,
        197, 138, 85, 171, 211, 172, 234, 145, 75, 41, 204, 25, 164, 50, 37, 78, 42, 241, 101, 68,
        208, 2, 206, 170, 206, 73, 180, 234, 159, 124, 131, 176, 64, 123, 231, 67, 171, 167, 108,
        163, 143, 125, 137, 129, 250, 76, 165, 255, 213, 142, 195, 206, 75, 224, 181, 216, 179,
        142, 69, 207, 118, 192, 237, 64, 43, 253, 83, 15, 176, 167, 213, 59, 13, 177, 138, 162, 3,
        222, 49, 173, 204, 119, 234, 111, 123, 62, 214, 223, 145, 34, 18, 230, 190, 250, 216, 50,
        252, 16, 99, 20, 81, 114, 222, 93, 214, 22, 147, 189, 41, 104, 51, 239, 58, 102, 236, 7,
        138, 38, 223, 19, 215, 87, 101, 120, 39, 222, 94, 73, 20, 0, 162, 0, 127, 154, 168, 33,
        182, 169, 177, 149, 176, 165, 185, 13, 22, 17, 218, 199, 108, 72, 60, 64, 224, 126, 13, 90,
        205, 86, 60, 209, 151, 5, 185, 203, 75, 237, 57, 75, 156, 196, 63, 210, 85, 19, 110, 36,
        176, 214, 113, 250, 244, 193, 186, 204, 237, 27, 245, 254, 129, 65, 216, 0, 152, 61, 58,
        200, 174, 122, 152, 55, 24, 5, 149, 2, 3, 1, 0, 1, 163, 130, 1, 56, 48, 130, 1, 52, 48, 14,
        6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5,
        48, 3, 1, 1, 255, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 228, 175, 43, 38, 113, 26, 43,
        72, 39, 133, 47, 82, 102, 44, 239, 240, 137, 19, 113, 62, 48, 31, 6, 3, 85, 29, 35, 4, 24,
        48, 22, 128, 20, 96, 123, 102, 26, 69, 13, 151, 202, 137, 80, 47, 125, 4, 205, 52, 168,
        255, 252, 253, 75, 48, 96, 6, 8, 43, 6, 1, 5, 5, 7, 1, 1, 4, 84, 48, 82, 48, 37, 6, 8, 43,
        6, 1, 5, 5, 7, 48, 1, 134, 25, 104, 116, 116, 112, 58, 47, 47, 111, 99, 115, 112, 46, 112,
        107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 48, 41, 6, 8, 43, 6, 1, 5, 5, 7,
        48, 2, 134, 29, 104, 116, 116, 112, 58, 47, 47, 112, 107, 105, 46, 103, 111, 111, 103, 47,
        103, 115, 114, 49, 47, 103, 115, 114, 49, 46, 99, 114, 116, 48, 50, 6, 3, 85, 29, 31, 4,
        43, 48, 41, 48, 39, 160, 37, 160, 35, 134, 33, 104, 116, 116, 112, 58, 47, 47, 99, 114,
        108, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 47, 103, 115, 114,
        49, 46, 99, 114, 108, 48, 59, 6, 3, 85, 29, 32, 4, 52, 48, 50, 48, 8, 6, 6, 103, 129, 12,
        1, 2, 1, 48, 8, 6, 6, 103, 129, 12, 1, 2, 2, 48, 13, 6, 11, 43, 6, 1, 4, 1, 214, 121, 2, 5,
        3, 2, 48, 13, 6, 11, 43, 6, 1, 4, 1, 214, 121, 2, 5, 3, 3, 48, 13, 6, 9, 42, 134, 72, 134,
        247, 13, 1, 1, 11, 5, 0, 3, 130, 1, 1, 0, 52, 164, 30, 177, 40, 163, 208, 180, 118, 23,
        166, 49, 122, 33, 233, 209, 82, 62, 200, 219, 116, 22, 65, 136, 184, 61, 53, 29, 237, 228,
        255, 147, 225, 92, 95, 171, 187, 234, 124, 207, 219, 228, 13, 209, 139, 87, 242, 38, 111,
        91, 190, 23, 70, 104, 148, 55, 111, 107, 122, 200, 192, 24, 55, 250, 37, 81, 172, 236, 104,
        191, 178, 200, 73, 253, 90, 154, 202, 1, 35, 172, 132, 128, 43, 2, 140, 153, 151, 235, 73,
        106, 140, 117, 215, 199, 222, 178, 201, 151, 159, 88, 72, 87, 14, 53, 161, 228, 26, 214,
        253, 111, 131, 129, 111, 239, 140, 207, 151, 175, 192, 133, 42, 240, 245, 78, 105, 9, 145,
        45, 225, 104, 184, 193, 43, 115, 233, 212, 217, 252, 34, 192, 55, 31, 11, 102, 29, 73, 237,
        2, 85, 143, 103, 225, 50, 215, 211, 38, 191, 112, 227, 61, 244, 103, 109, 61, 124, 229, 52,
        136, 227, 50, 250, 167, 110, 6, 106, 111, 189, 139, 145, 238, 22, 75, 232, 59, 169, 179,
        55, 231, 195, 68, 164, 126, 216, 108, 215, 199, 70, 245, 146, 155, 231, 213, 33, 190, 102,
        146, 25, 148, 85, 108, 212, 41, 178, 13, 193, 102, 91, 226, 119, 73, 72, 40, 237, 157, 215,
        26, 51, 114, 83, 179, 130, 53, 207, 98, 139, 201, 36, 139, 165, 183, 57, 12, 187, 126, 42,
        65, 191, 82, 207, 252, 162, 150, 182, 194, 130, 63,
    ];
    let cell =
        TokenValue::write_bytes(/* &[3u8] */ &cert, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let kid: Vec<u8> = vec![
        13, 138, 103, 57, 158, 120, 130, 172, 174, 125, 127, 104, 178, 40, 2, 86, 167, 150, 165,
        130,
    ];
    let cell =
        TokenValue::write_bytes(&kid /* &[2u8] */, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let timestamp: Vec<u8> = vec![0, 0, 3, 232];
    let cell = TokenValue::write_bytes(/* &[1u8] */ &timestamp, &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    // Push args, func name, instance name, then wasm.
    let wasm_func = "tlscheck";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:tlschecker/tls-check-interface@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_dict = Vec::<u8>::new();

    let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    // let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let status = execute_run_wasm_concat_multiarg(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);

    let res = engine.cc.stack.get(0).as_cell().unwrap(); //engine.cc.stack.get(0).as_slice().unwrap().clone();
    let slice = SliceData::load_cell(res.clone()).unwrap();
    let ress = unpack_data_from_cell(slice, &mut engine).unwrap();
    println!("ress: {:?}", hex::encode(ress));

    // assert!(
    // rejoin_chain_of_cells(engine.cc.stack.get(0).as_cell().unwrap()).
    // unwrap().pop().unwrap() == 3u8
    // );
}
