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

use std::cell;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

use ark_std::iterable::Iterable;
use tvm_block::Deserializable;
use tvm_block::StateInit;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::IBitstring;
use tvm_types::SliceData;
use tvm_types::read_single_root_boc;
use zstd::dict::from_files;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::executor::gas::gas_state::Gas;
use crate::executor::math::DivMode;
use crate::executor::serialize_currency_collection;
use crate::executor::token::execute_run_wasm;
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

fn split_to_chain_of_cells(input: Vec<u8>) -> Cell {
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
        // println!(
        //     "chunk: {:?}, cell: {:?}, size: {:?}",
        //     chunk.len(),
        //     cell_vec.last().expect("msg").len(),
        //     cellsize
        // );
        assert!(
            cell_vec.last().expect("error in split_to_chain_of_cells function").len() == cellsize
                || i + cellsize > len
        );
    }
    let mut cell = BuilderData::with_raw(
        cell_vec[cell_vec.len() - 1].clone(),
        cell_vec[cell_vec.len() - 1].len() * 8,
    )
    .unwrap()
    .into_cell()
    .unwrap();
    for i in (0..(cell_vec.len() - 1)).rev() {
        let mut builder =
            BuilderData::with_raw(cell_vec[i].clone(), cell_vec[i].len() * 8).unwrap();
        let builder = builder.checked_append_reference(cell).unwrap();
        cell = builder.clone().into_cell().unwrap();
        // println!("data: {:?}, vec: {:?}, i: {:?}", cell.data().len(),
        // cell_vec[i].len(), i);
    }
    cell // return first cell
}

fn rejoin_chain_of_cells(mut input: Cell) -> Vec<u8> {
    let mut data_vec = input.data().to_vec();
    while input.reference(0).is_ok() {
        input = input.reference(0).unwrap();
        data_vec.append(&mut input.data().to_vec());
    }
    data_vec
}

fn load_stateinit(filename: &str) -> StateInit {
    StateInit::construct_from_file(filename).unwrap()
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
fn test_run_wasm_fortytwo() {
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

    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );
    let cell = split_to_chain_of_cells((Vec::<u8>::from([1u8, 2u8])));
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let filename =
        "/Users/elar/Code/Havok/AckiNacki/wasm/add/target/wasm32-wasip1/release/add.wasm";
    let wasm_dict = std::fs::read(filename).unwrap();

    let cell = split_to_chain_of_cells(wasm_dict);
    // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let status = execute_run_wasm(&mut engine).unwrap();
    println!("Wasm Return Status: {:?}", status);
    // let err = engine.execute();
    assert!(
        unpack_data_from_cell(
            SliceData::load_cell_ref(engine.cc.stack.get(0).as_cell().unwrap()).unwrap(),
            &mut engine
        )
        .unwrap()
        .pop()
        .unwrap()
            == 3u8
    );

    // let TvmError::TvmExceptionFull(exc, _) =
    // err.downcast_ref::<TvmError>().unwrap() else {     panic!("Should be
    // TvmExceptionFull"); };
    // assert_eq!(exc.exception_code(), Some(ExceptionCode::ExecutionTimeout));
}
