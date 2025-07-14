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

use rand::RngCore;
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
use crate::executor::math::execute_xor;
use crate::executor::math::DivMode;
use crate::executor::serialize_currency_collection;
use crate::executor::token::execute_run_wasm;
use crate::executor::token::execute_run_wasm_concat_multiarg;
use crate::executor::types::Instruction;
use crate::executor::types::InstructionOptions;
use crate::executor::zk::execute_poseidon_zk_login;
use crate::executor::zk::execute_vergrth16;
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

use crate::utils::pack_string_to_cell;
use crate::utils::unpack_string_from_cell;

use crate::executor::zk_stuff::utils::gen_address_seed;
use crate::executor::zk_stuff::utils::get_zk_login_address;

use ed25519_dalek::Signer;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::KeyPair;
use fastcrypto::traits::ToFromBytes;

use crate::executor::zk_stuff::zk_login::CanonicalSerialize;
use crate::executor::zk_stuff::zk_login::JWK;
use crate::executor::zk_stuff::zk_login::JwkId;
use crate::executor::zk_stuff::zk_login::OIDCProvider;
use crate::executor::zk_stuff::zk_login::ZkLoginInputs;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::error::ZkCryptoError;

use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use std::collections::HashMap;

use base64::decode;
use base64ct::Encoding as bEncoding;

use rand::Rng;

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

    let hash_str = "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2";
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
fn test_wasm_from_invalid_hash() {
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

    let hash_str = "1234567890123456789012345678901234567890123456789012345678901234";
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

    let _res_error = result.expect_err("Test didn't error on fuel use");
}

#[test]
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

    let hash_str = "8c9cab9d4577b57c7db74ced40cd20ddbf6e07a7aa130c7f34909ffe4d0930c0";
    let hash: Vec<u8> = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let cell =
        TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let tls_data: Vec<u8> = vec![231, 226, 189, 128, 175, 192, 46, 233, 160, 243, 227, 168, 186, 174, 207, 111, 124, 21, 6, 220, 18, 155, 18, 17, 39, 165, 203, 108, 109, 3, 40, 186, 1, 3, 22, 3, 1, 0, 161, 1, 0, 0, 157, 3, 3, 4, 180, 186, 85, 99, 29, 193, 161, 254, 168, 180, 199, 25, 88, 82, 243, 202, 40, 246, 210, 151, 124, 215, 69, 249, 235, 215, 49, 43, 141, 174, 211, 0, 0, 2, 19, 1, 1, 0, 0, 114, 0, 0, 0, 23, 0, 21, 0, 0, 18, 119, 119, 119, 46, 103, 111, 111, 103, 108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 0, 10, 0, 4, 0, 2, 0, 29, 0, 13, 0, 20, 0, 18, 4, 3, 8, 4, 4, 1, 5, 3, 8, 5, 5, 1, 8, 6, 6, 1, 2, 1, 0, 51, 0, 38, 0, 36, 0, 29, 0, 32, 192, 66, 56, 95, 6, 86, 129, 217, 28, 232, 5, 177, 109, 189, 139, 154, 6, 3, 215, 62, 202, 195, 214, 238, 231, 82, 157, 198, 107, 200, 81, 16, 0, 45, 0, 2, 1, 1, 0, 43, 0, 3, 2, 3, 4, 22, 3, 3, 0, 90, 2, 0, 0, 86, 3, 3, 178, 66, 205, 102, 182, 239, 42, 33, 54, 13, 116, 240, 105, 120, 228, 218, 31, 90, 200, 209, 235, 52, 203, 193, 147, 128, 196, 241, 207, 7, 226, 159, 0, 19, 1, 0, 0, 46, 0, 51, 0, 36, 0, 29, 0, 32, 221, 183, 132, 204, 26, 134, 17, 105, 110, 29, 105, 38, 199, 144, 153, 203, 251, 96, 202, 193, 101, 174, 113, 61, 234, 231, 159, 1, 21, 155, 112, 11, 0, 43, 0, 2, 3, 4, 23, 3, 3, 16, 252, 86, 92, 243, 216, 229, 23, 123, 55, 206, 21, 63, 92, 74, 78, 253, 229, 244, 41, 178, 245, 212, 105, 175, 132, 102, 20, 95, 179, 144, 24, 139, 203, 226, 24, 178, 24, 116, 155, 38, 230, 189, 37, 243, 34, 179, 99, 96, 29, 83, 101, 32, 192, 238, 153, 93, 103, 84, 0, 155, 225, 106, 68, 5, 238, 168, 105, 212, 151, 167, 114, 240, 250, 56, 136, 236, 252, 171, 184, 201, 233, 235, 77, 136, 74, 166, 118, 31, 30, 32, 200, 187, 248, 253, 46, 53, 51, 211, 198, 54, 226, 187, 122, 156, 143, 162, 178, 255, 90, 21, 15, 173, 94, 223, 129, 198, 34, 244, 168, 109, 113, 35, 199, 85, 41, 95, 50, 89, 85, 25, 112, 83, 217, 19, 28, 135, 254, 53, 95, 32, 220, 22, 159, 90, 194, 211, 66, 157, 191, 70, 153, 201, 73, 213, 133, 209, 215, 25, 86, 12, 132, 169, 175, 89, 139, 77, 132, 74, 46, 54, 190, 54, 29, 207, 194, 212, 248, 2, 243, 140, 77, 186, 152, 145, 161, 90, 202, 0, 250, 11, 158, 185, 12, 57, 240, 210, 46, 100, 47, 100, 157, 133, 143, 84, 33, 4, 146, 217, 122, 62, 149, 207, 92, 111, 191, 6, 97, 23, 53, 57, 58, 51, 201, 23, 219, 143, 198, 208, 42, 240, 153, 138, 100, 176, 231, 221, 67, 180, 1, 159, 167, 99, 94, 211, 57, 200, 132, 44, 220, 51, 229, 90, 121, 125, 242, 247, 252, 4, 151, 214, 93, 34, 158, 116, 175, 152, 23, 172, 21, 197, 185, 155, 65, 92, 30, 192, 131, 245, 60, 98, 103, 23, 194, 56, 23, 144, 175, 93, 199, 131, 224, 24, 84, 195, 54, 61, 110, 66, 19, 146, 153, 104, 20, 72, 11, 72, 86, 252, 74, 31, 32, 17, 177, 251, 48, 78, 144, 215, 245, 155, 186, 168, 207, 197, 114, 116, 69, 230, 225, 29, 51, 17, 170, 136, 243, 233, 241, 163, 92, 230, 8, 123, 4, 59, 102, 99, 109, 5, 242, 81, 50, 121, 229, 39, 173, 142, 149, 247, 126, 146, 153, 72, 100, 172, 201, 237, 216, 206, 59, 15, 249, 246, 161, 49, 153, 226, 54, 220, 180, 165, 208, 68, 68, 76, 4, 14, 111, 86, 177, 70, 144, 205, 143, 209, 117, 6, 37, 70, 134, 27, 85, 191, 160, 161, 117, 245, 190, 114, 166, 29, 236, 178, 79, 193, 246, 46, 17, 175, 66, 7, 113, 216, 159, 248, 134, 82, 184, 29, 29, 22, 213, 245, 233, 91, 146, 119, 60, 122, 99, 98, 211, 225, 204, 169, 212, 141, 128, 244, 58, 167, 214, 117, 2, 253, 118, 55, 171, 222, 114, 220, 189, 74, 219, 83, 147, 177, 212, 97, 79, 62, 99, 105, 105, 53, 14, 10, 91, 114, 252, 254, 5, 5, 159, 91, 128, 167, 91, 207, 92, 154, 117, 149, 217, 230, 25, 112, 176, 249, 67, 38, 55, 234, 26, 108, 245, 155, 33, 25, 54, 153, 157, 79, 107, 197, 72, 244, 176, 165, 148, 14, 218, 228, 61, 237, 185, 253, 57, 250, 29, 241, 156, 253, 172, 14, 8, 143, 237, 99, 143, 244, 64, 40, 8, 203, 188, 115, 176, 192, 187, 198, 134, 162, 104, 162, 43, 15, 204, 44, 57, 27, 117, 82, 249, 228, 95, 241, 38, 50, 220, 255, 217, 175, 231, 57, 70, 62, 200, 168, 113, 5, 170, 190, 55, 59, 115, 57, 229, 122, 253, 153, 51, 128, 135, 164, 217, 85, 36, 87, 106, 81, 105, 78, 176, 26, 74, 150, 215, 56, 182, 117, 241, 71, 210, 34, 254, 46, 73, 179, 93, 79, 133, 222, 209, 112, 24, 13, 32, 87, 80, 44, 83, 9, 39, 128, 171, 3, 47, 83, 200, 217, 152, 214, 113, 2, 76, 200, 41, 136, 255, 129, 64, 117, 62, 175, 216, 32, 108, 117, 165, 241, 121, 233, 63, 148, 118, 250, 183, 210, 248, 114, 250, 217, 140, 151, 171, 19, 83, 128, 249, 26, 182, 58, 46, 74, 196, 122, 121, 236, 202, 199, 205, 118, 146, 105, 118, 82, 209, 88, 240, 251, 174, 201, 224, 18, 34, 113, 2, 218, 149, 168, 27, 255, 254, 243, 150, 140, 23, 219, 174, 132, 19, 79, 7, 104, 179, 56, 43, 95, 115, 186, 32, 116, 238, 93, 37, 16, 225, 66, 59, 86, 251, 1, 107, 176, 192, 132, 163, 35, 161, 213, 196, 103, 165, 120, 176, 135, 77, 112, 237, 180, 221, 20, 118, 209, 32, 139, 241, 220, 60, 50, 253, 27, 152, 36, 207, 96, 18, 76, 195, 172, 137, 229, 236, 234, 163, 134, 98, 3, 61, 74, 210, 117, 187, 73, 139, 13, 3, 66, 161, 190, 249, 37, 222, 6, 79, 36, 80, 147, 178, 32, 42, 14, 52, 127, 113, 112, 9, 213, 220, 130, 189, 0, 149, 32, 73, 216, 188, 252, 37, 117, 19, 236, 207, 18, 241, 37, 240, 80, 128, 84, 98, 150, 35, 213, 205, 4, 156, 64, 51, 197, 80, 111, 65, 109, 194, 149, 204, 253, 226, 82, 61, 183, 59, 79, 187, 140, 127, 181, 107, 186, 171, 9, 163, 211, 202, 211, 15, 237, 25, 224, 240, 32, 94, 92, 28, 76, 204, 108, 50, 78, 108, 67, 206, 27, 38, 66, 67, 51, 214, 109, 13, 14, 229, 31, 194, 125, 248, 172, 175, 39, 176, 32, 167, 4, 129, 239, 43, 90, 206, 63, 211, 202, 96, 36, 174, 169, 102, 196, 92, 111, 89, 251, 61, 135, 127, 109, 245, 251, 233, 114, 112, 252, 29, 160, 166, 170, 210, 176, 60, 110, 23, 65, 135, 103, 89, 191, 47, 22, 25, 71, 191, 113, 72, 252, 131, 24, 154, 116, 72, 172, 195, 77, 20, 21, 240, 238, 67, 229, 107, 144, 180, 62, 229, 149, 151, 81, 125, 231, 153, 195, 247, 93, 107, 32, 175, 235, 66, 117, 51, 48, 43, 11, 148, 46, 44, 36, 124, 159, 101, 161, 44, 66, 137, 170, 77, 86, 227, 228, 89, 190, 171, 180, 25, 108, 9, 116, 41, 106, 99, 188, 198, 193, 244, 90, 90, 1, 228, 146, 244, 3, 195, 76, 204, 121, 221, 82, 151, 150, 112, 197, 194, 23, 93, 77, 163, 194, 119, 234, 62, 27, 165, 205, 32, 194, 219, 205, 43, 71, 17, 166, 102, 28, 205, 92, 126, 4, 115, 167, 66, 56, 52, 195, 21, 121, 170, 212, 98, 175, 216, 178, 216, 255, 81, 133, 142, 92, 111, 55, 172, 197, 34, 8, 136, 48, 17, 150, 0, 171, 62, 81, 87, 38, 38, 28, 0, 83, 223, 233, 225, 89, 114, 23, 16, 163, 155, 138, 254, 3, 88, 173, 228, 182, 68, 191, 44, 69, 188, 73, 75, 86, 50, 126, 202, 152, 233, 198, 113, 173, 174, 220, 232, 82, 15, 140, 166, 77, 207, 247, 207, 240, 42, 57, 91, 167, 106, 31, 90, 123, 18, 38, 159, 51, 5, 98, 77, 240, 112, 223, 22, 115, 156, 107, 23, 52, 57, 14, 237, 234, 35, 47, 44, 14, 215, 13, 216, 93, 233, 106, 242, 211, 69, 10, 35, 117, 116, 197, 106, 95, 77, 32, 141, 37, 101, 152, 99, 138, 188, 190, 204, 248, 125, 49, 229, 28, 194, 25, 209, 247, 25, 173, 107, 180, 83, 247, 108, 1, 60, 11, 121, 199, 224, 40, 218, 135, 85, 132, 183, 216, 97, 90, 202, 112, 251, 178, 204, 56, 32, 71, 177, 233, 217, 96, 198, 146, 186, 27, 204, 69, 69, 122, 170, 231, 72, 62, 167, 185, 116, 27, 141, 247, 221, 20, 10, 150, 78, 74, 181, 108, 150, 54, 123, 192, 84, 75, 199, 129, 26, 129, 40, 46, 226, 205, 113, 227, 200, 25, 50, 184, 134, 234, 8, 241, 180, 241, 68, 140, 148, 110, 169, 79, 41, 52, 38, 186, 193, 189, 221, 21, 123, 133, 137, 221, 71, 37, 157, 106, 223, 200, 150, 119, 187, 235, 95, 76, 30, 218, 176, 123, 100, 29, 136, 243, 159, 23, 119, 159, 104, 174, 42, 19, 5, 187, 43, 87, 123, 248, 81, 253, 122, 44, 194, 126, 236, 147, 189, 20, 205, 109, 141, 54, 14, 109, 221, 30, 83, 128, 16, 25, 108, 51, 66, 185, 126, 8, 200, 24, 195, 65, 32, 252, 252, 246, 244, 198, 100, 112, 180, 224, 250, 15, 240, 251, 146, 108, 253, 41, 34, 38, 34, 143, 61, 219, 101, 118, 74, 143, 95, 82, 59, 114, 246, 80, 181, 4, 104, 33, 234, 190, 169, 79, 70, 192, 7, 218, 135, 209, 87, 202, 187, 178, 188, 1, 27, 9, 40, 47, 156, 59, 113, 86, 57, 53, 55, 206, 65, 231, 142, 199, 14, 102, 186, 85, 155, 31, 157, 98, 234, 149, 239, 150, 173, 53, 6, 79, 152, 103, 78, 55, 152, 200, 255, 196, 205, 71, 233, 247, 103, 186, 227, 82, 94, 38, 0, 70, 143, 13, 92, 246, 241, 11, 144, 151, 235, 30, 169, 32, 8, 69, 45, 81, 220, 130, 52, 189, 56, 12, 149, 171, 30, 131, 48, 75, 102, 106, 59, 231, 162, 122, 213, 6, 206, 40, 132, 207, 57, 108, 244, 69, 100, 109, 199, 149, 164, 54, 252, 70, 106, 1, 222, 241, 174, 116, 153, 200, 83, 162, 248, 27, 151, 118, 213, 63, 79, 161, 39, 189, 221, 213, 83, 45, 60, 13, 170, 212, 252, 71, 239, 211, 0, 183, 82, 186, 12, 11, 239, 74, 67, 150, 77, 178, 0, 246, 214, 108, 21, 68, 115, 74, 126, 189, 206, 20, 42, 182, 212, 230, 179, 102, 197, 52, 143, 70, 212, 13, 51, 121, 212, 170, 118, 13, 140, 126, 163, 73, 65, 222, 228, 154, 62, 6, 126, 159, 32, 219, 210, 43, 32, 145, 49, 37, 145, 193, 215, 251, 187, 169, 45, 201, 74, 102, 244, 120, 35, 236, 184, 227, 219, 107, 35, 100, 35, 12, 204, 43, 130, 121, 151, 137, 166, 30, 247, 47, 92, 229, 154, 60, 121, 115, 185, 202, 124, 43, 52, 37, 46, 43, 117, 47, 241, 169, 137, 190, 158, 210, 139, 149, 244, 81, 1, 126, 64, 118, 167, 157, 234, 116, 110, 65, 222, 108, 99, 207, 197, 217, 124, 169, 207, 130, 246, 243, 79, 181, 91, 197, 43, 204, 66, 78, 24, 158, 224, 135, 84, 149, 84, 225, 7, 205, 224, 38, 91, 161, 135, 33, 253, 30, 212, 67, 46, 169, 182, 3, 116, 197, 21, 243, 32, 120, 251, 120, 181, 183, 210, 198, 55, 83, 85, 204, 54, 107, 54, 124, 226, 136, 11, 61, 90, 103, 31, 92, 53, 163, 120, 91, 93, 81, 82, 37, 45, 116, 61, 113, 20, 213, 44, 40, 40, 41, 221, 136, 219, 71, 215, 6, 115, 17, 136, 39, 199, 5, 183, 99, 110, 149, 172, 30, 52, 217, 48, 32, 201, 160, 16, 55, 42, 78, 136, 171, 14, 236, 251, 208, 16, 152, 248, 61, 29, 249, 74, 52, 123, 69, 137, 62, 28, 111, 150, 248, 87, 255, 44, 249, 48, 198, 106, 191, 205, 13, 45, 31, 26, 252, 211, 181, 248, 106, 92, 238, 199, 49, 97, 193, 218, 102, 189, 160, 212, 49, 103, 211, 73, 52, 43, 240, 13, 207, 112, 65, 246, 252, 240, 227, 80, 62, 57, 80, 214, 37, 7, 198, 170, 189, 160, 209, 135, 220, 114, 42, 120, 8, 37, 65, 238, 124, 204, 164, 239, 66, 126, 63, 91, 143, 147, 70, 121, 179, 52, 224, 77, 166, 232, 43, 156, 239, 121, 104, 38, 17, 116, 52, 157, 106, 121, 65, 51, 181, 25, 194, 226, 141, 92, 166, 146, 187, 177, 124, 67, 227, 106, 182, 247, 35, 197, 174, 177, 224, 59, 134, 116, 189, 130, 105, 33, 20, 199, 24, 4, 255, 221, 178, 97, 188, 223, 193, 210, 213, 226, 1, 60, 231, 207, 150, 81, 253, 217, 225, 221, 153, 48, 229, 108, 157, 251, 152, 180, 213, 108, 221, 140, 237, 47, 36, 230, 220, 44, 190, 138, 34, 71, 101, 29, 199, 210, 18, 125, 215, 30, 116, 80, 166, 197, 219, 197, 212, 252, 213, 27, 77, 7, 177, 233, 151, 52, 83, 162, 209, 34, 218, 81, 47, 183, 63, 106, 129, 163, 188, 135, 8, 197, 236, 210, 93, 105, 44, 191, 125, 251, 130, 228, 1, 126, 165, 18, 213, 70, 62, 101, 26, 36, 182, 127, 64, 163, 103, 162, 164, 194, 187, 238, 113, 114, 76, 121, 177, 181, 203, 106, 234, 200, 223, 121, 3, 242, 123, 102, 117, 203, 227, 35, 186, 75, 50, 234, 162, 49, 26, 74, 164, 176, 48, 101, 4, 79, 235, 87, 225, 134, 217, 180, 120, 107, 187, 161, 159, 172, 159, 46, 27, 51, 75, 105, 133, 26, 192, 64, 158, 36, 34, 57, 60, 72, 14, 151, 203, 60, 211, 129, 2, 254, 32, 146, 118, 53, 211, 83, 203, 170, 144, 180, 92, 198, 122, 143, 138, 162, 37, 77, 80, 111, 123, 116, 154, 189, 51, 95, 174, 121, 161, 237, 195, 156, 193, 33, 223, 207, 140, 240, 143, 246, 146, 195, 140, 77, 81, 116, 17, 179, 48, 81, 151, 203, 178, 54, 161, 211, 54, 95, 169, 93, 26, 205, 50, 201, 17, 164, 164, 186, 93, 189, 181, 225, 106, 202, 9, 50, 236, 28, 131, 129, 87, 132, 190, 206, 202, 206, 60, 201, 203, 245, 13, 89, 66, 11, 122, 237, 53, 6, 62, 68, 63, 219, 171, 251, 127, 43, 75, 61, 34, 249, 170, 222, 158, 201, 68, 170, 60, 47, 141, 88, 168, 88, 18, 117, 209, 69, 194, 149, 170, 90, 161, 183, 197, 214, 217, 24, 153, 48, 154, 111, 118, 184, 129, 123, 27, 214, 82, 48, 57, 201, 106, 108, 56, 231, 193, 126, 112, 90, 89, 171, 90, 81, 22, 81, 32, 128, 214, 131, 126, 156, 189, 222, 17, 55, 91, 136, 202, 150, 185, 170, 146, 178, 126, 220, 80, 205, 92, 195, 47, 221, 149, 141, 131, 39, 222, 52, 44, 21, 148, 92, 183, 243, 171, 13, 159, 106, 81, 17, 26, 32, 168, 242, 9, 105, 225, 124, 162, 231, 78, 36, 199, 194, 147, 252, 121, 224, 42, 6, 77, 19, 128, 105, 190, 217, 109, 168, 216, 148, 3, 213, 3, 66, 185, 179, 187, 180, 238, 190, 112, 240, 129, 76, 221, 64, 112, 93, 207, 205, 170, 145, 152, 105, 86, 112, 194, 110, 107, 252, 223, 189, 24, 5, 23, 198, 122, 5, 82, 1, 34, 231, 60, 48, 76, 56, 23, 36, 108, 82, 138, 113, 131, 132, 32, 120, 233, 253, 247, 227, 254, 174, 224, 246, 160, 190, 148, 79, 133, 179, 231, 124, 225, 184, 147, 102, 138, 29, 54, 59, 85, 7, 79, 65, 45, 242, 157, 46, 1, 174, 130, 245, 221, 149, 210, 193, 22, 95, 8, 24, 250, 64, 244, 126, 164, 243, 125, 230, 207, 209, 84, 157, 183, 41, 131, 117, 73, 172, 169, 109, 152, 44, 240, 119, 19, 149, 13, 27, 46, 72, 151, 4, 236, 106, 75, 10, 166, 21, 112, 139, 103, 31, 79, 246, 101, 0, 43, 233, 170, 180, 154, 118, 21, 73, 238, 68, 170, 226, 115, 215, 79, 217, 189, 28, 209, 86, 118, 249, 145, 227, 80, 241, 244, 172, 47, 63, 226, 32, 56, 71, 149, 63, 24, 254, 151, 8, 115, 7, 228, 172, 17, 197, 2, 36, 163, 49, 21, 226, 189, 34, 171, 0, 47, 187, 183, 101, 238, 19, 71, 40, 89, 149, 16, 199, 140, 111, 193, 122, 175, 100, 221, 178, 194, 164, 63, 190, 197, 61, 82, 6, 51, 75, 37, 22, 100, 39, 206, 177, 146, 234, 26, 51, 151, 186, 146, 229, 58, 42, 235, 230, 150, 64, 192, 66, 0, 205, 3, 82, 28, 123, 109, 241, 254, 210, 113, 199, 171, 2, 51, 21, 120, 153, 243, 214, 48, 23, 222, 136, 220, 20, 11, 122, 46, 156, 25, 129, 159, 47, 206, 121, 254, 17, 36, 30, 35, 116, 121, 27, 8, 76, 197, 243, 71, 144, 157, 84, 28, 88, 84, 90, 61, 103, 5, 158, 207, 143, 237, 18, 52, 206, 125, 43, 60, 221, 28, 127, 158, 170, 160, 200, 246, 77, 17, 161, 166, 248, 182, 179, 26, 65, 140, 233, 86, 52, 239, 63, 198, 220, 137, 75, 200, 45, 221, 246, 98, 17, 158, 3, 2, 180, 179, 171, 199, 41, 54, 225, 246, 64, 138, 1, 218, 13, 148, 9, 229, 77, 148, 227, 159, 177, 248, 33, 223, 56, 1, 124, 33, 193, 191, 128, 34, 165, 226, 107, 253, 65, 87, 98, 30, 97, 210, 131, 12, 85, 34, 52, 157, 66, 203, 58, 203, 190, 50, 45, 181, 136, 163, 223, 104, 134, 54, 36, 12, 133, 47, 160, 29, 227, 114, 215, 208, 66, 8, 51, 79, 71, 204, 211, 67, 125, 30, 127, 165, 122, 247, 136, 47, 45, 163, 207, 107, 97, 239, 31, 78, 17, 122, 177, 112, 209, 131, 47, 234, 7, 142, 113, 176, 17, 92, 20, 136, 108, 162, 165, 149, 58, 212, 151, 242, 109, 96, 69, 156, 89, 135, 178, 223, 139, 191, 244, 30, 238, 52, 121, 31, 56, 30, 138, 192, 184, 76, 234, 135, 236, 134, 141, 46, 12, 197, 196, 192, 243, 253, 13, 5, 81, 231, 163, 32, 1, 154, 33, 119, 228, 164, 64, 38, 205, 41, 6, 212, 114, 207, 95, 255, 8, 217, 140, 202, 48, 26, 74, 92, 170, 92, 148, 58, 81, 65, 139, 4, 170, 234, 16, 133, 2, 70, 176, 227, 16, 153, 240, 12, 68, 239, 153, 90, 242, 199, 46, 13, 207, 99, 216, 69, 237, 234, 210, 231, 148, 80, 175, 180, 169, 90, 255, 46, 196, 40, 115, 208, 82, 188, 78, 147, 108, 233, 122, 179, 35, 80, 41, 123, 71, 90, 50, 172, 210, 172, 107, 83, 179, 246, 55, 214, 54, 73, 206, 188, 113, 172, 104, 188, 16, 204, 246, 198, 113, 154, 172, 29, 116, 87, 141, 206, 207, 246, 8, 78, 169, 154, 69, 129, 55, 109, 166, 169, 5, 216, 248, 2, 137, 50, 207, 250, 103, 126, 75, 48, 97, 21, 93, 105, 249, 4, 111, 13, 174, 247, 143, 116, 40, 40, 53, 57, 22, 12, 146, 221, 217, 120, 78, 241, 157, 158, 116, 220, 4, 23, 151, 220, 16, 0, 12, 230, 206, 1, 234, 200, 231, 164, 163, 115, 16, 79, 32, 160, 54, 98, 49, 145, 70, 151, 19, 99, 161, 225, 35, 153, 39, 149, 160, 139, 64, 253, 157, 239, 10, 15, 145, 7, 98, 138, 66, 138, 10, 113, 4, 95, 107, 72, 47, 6, 56, 6, 196, 221, 205, 17, 123, 19, 139, 246, 133, 210, 36, 17, 98, 223, 45, 196, 118, 109, 131, 191, 188, 166, 83, 26, 17, 176, 39, 222, 203, 66, 30, 181, 158, 194, 44, 206, 85, 252, 199, 195, 179, 222, 87, 117, 86, 55, 162, 89, 59, 77, 128, 15, 174, 172, 218, 32, 241, 20, 91, 90, 172, 62, 38, 128, 76, 202, 12, 59, 158, 242, 219, 230, 6, 204, 254, 65, 58, 214, 33, 131, 230, 50, 160, 59, 67, 138, 247, 112, 144, 71, 5, 147, 14, 160, 184, 237, 46, 167, 18, 192, 222, 54, 26, 126, 196, 73, 148, 175, 140, 109, 155, 115, 22, 93, 193, 48, 249, 231, 51, 48, 93, 150, 86, 52, 226, 132, 91, 220, 165, 206, 10, 20, 15, 16, 105, 51, 60, 91, 136, 177, 150, 91, 76, 109, 22, 136, 119, 109, 199, 29, 49, 122, 90, 60, 146, 124, 186, 226, 166, 93, 248, 12, 123, 13, 34, 79, 148, 229, 84, 71, 48, 129, 181, 93, 201, 201, 63, 96, 161, 181, 218, 94, 231, 224, 176, 13, 35, 71, 8, 133, 125, 137, 243, 53, 173, 219, 197, 0, 204, 216, 219, 174, 2, 248, 42, 17, 132, 122, 80, 198, 27, 63, 4, 42, 134, 216, 132, 10, 136, 55, 109, 20, 2, 143, 205, 245, 190, 102, 50, 6, 46, 219, 214, 118, 125, 214, 141, 3, 33, 152, 45, 53, 81, 57, 135, 137, 200, 186, 203, 184, 12, 64, 106, 207, 0, 245, 0, 243, 219, 167, 186, 100, 183, 16, 97, 228, 88, 39, 135, 171, 193, 50, 35, 42, 85, 138, 249, 119, 161, 187, 0, 127, 210, 114, 18, 71, 231, 185, 140, 178, 182, 55, 146, 192, 135, 17, 89, 231, 19, 135, 165, 162, 70, 102, 27, 176, 210, 190, 240, 173, 107, 103, 110, 56, 251, 171, 247, 38, 166, 247, 226, 194, 216, 181, 212, 67, 233, 69, 132, 239, 84, 87, 25, 235, 151, 32, 134, 113, 30, 255, 127, 245, 244, 9, 5, 76, 216, 179, 197, 71, 190, 249, 244, 180, 8, 100, 25, 81, 5, 244, 3, 232, 244, 244, 133, 242, 129, 228, 176, 217, 90, 208, 24, 148, 98, 4, 138, 169, 60, 163, 101, 72, 102, 138, 57, 33, 151, 91, 108, 193, 24, 15, 35, 0, 127, 68, 200, 251, 230, 178, 131, 132, 203, 247, 13, 89, 228, 198, 26, 60, 215, 140, 179, 117, 141, 69, 135, 167, 234, 50, 173, 214, 56, 240, 171, 236, 124, 173, 65, 22, 103, 117, 203, 181, 134, 35, 197, 83, 33, 167, 31, 76, 20, 94, 195, 241, 105, 112, 118, 33, 227, 217, 153, 243, 230, 211, 4, 199, 198, 46, 246, 102, 151, 125, 1, 97, 79, 127, 50, 164, 57, 47, 72, 191, 144, 103, 66, 62, 88, 7, 143, 9, 119, 244, 158, 179, 251, 112, 66, 235, 86, 131, 173, 184, 231, 146, 96, 98, 117, 19, 27, 186, 141, 15, 242, 165, 39, 129, 184, 94, 60, 185, 61, 112, 147, 142, 43, 52, 134, 185, 185, 109, 200, 249, 113, 127, 158, 143, 250, 32, 53, 165, 238, 160, 108, 173, 237, 109, 168, 95, 140, 5, 33, 77, 41, 132, 10, 140, 206, 208, 230, 245, 223, 197, 177, 182, 203, 135, 6, 183, 163, 98, 175, 200, 107, 204, 5, 251, 214, 111, 177, 175, 36, 23, 27, 111, 124, 187, 255, 53, 103, 199, 103, 20, 162, 4, 204, 237, 35, 86, 199, 213, 96, 145, 72, 60, 143, 21, 254, 168, 170, 116, 111, 16, 149, 34, 31, 116, 75, 177, 146, 217, 150, 129, 49, 101, 94, 54, 225, 37, 242, 3, 49, 238, 124, 153, 110, 87, 111, 54, 146, 245, 19, 103, 250, 43, 84, 10, 214, 161, 199, 47, 242, 44, 153, 26, 208, 231, 146, 153, 202, 170, 117, 74, 82, 251, 203, 105, 122, 120, 40, 146, 202, 56, 247, 254, 31, 183, 35, 90, 81, 217, 64, 73, 51, 160, 15, 135, 55, 150, 13, 193, 85, 53, 137, 190, 187, 220, 146, 66, 161, 116, 111, 62, 220, 204, 93, 146, 61, 0, 218, 210, 185, 105, 37, 132, 231, 150, 229, 89, 241, 1, 184, 49, 233, 204, 115, 143, 71, 245, 162, 62, 48, 252, 124, 48, 228, 29, 133, 64, 105, 45, 170, 208, 143, 150, 180, 208, 27, 31, 186, 10, 191, 199, 180, 78, 204, 247, 90, 80, 65, 72, 27, 70, 40, 134, 89, 128, 84, 119, 116, 1, 212, 93, 227, 149, 21, 116, 134, 10, 131, 50, 71, 180, 53, 33, 194, 7, 108, 242, 177, 214, 19, 121, 69, 60, 202, 117, 88, 225, 118, 49, 209, 89, 52, 126, 53, 13, 107, 18, 161, 173, 130, 29, 217, 195, 51, 236, 232, 237, 72, 241, 74, 140, 50, 89, 174, 28, 51, 114, 5, 218, 255, 226, 39, 211, 125, 61, 183, 206, 129, 179, 51, 53, 34, 95, 43, 48, 251, 234, 26, 220, 122, 243, 46, 146, 220, 165, 236, 24, 235, 110, 79, 44, 86, 95, 112, 79, 179, 15, 193, 163, 54, 10, 198, 1, 249, 205, 123, 228, 206, 193, 31, 54, 143, 11, 21, 220, 189, 67, 214, 240, 15, 83, 247, 59, 121, 44, 106, 158, 25, 62, 74, 163, 204, 177, 1, 156, 218, 80, 163, 114, 147, 15, 39, 203, 254, 204, 97, 60, 1, 231, 108, 77, 153, 219, 209, 86, 196, 17, 41, 137, 6, 9, 190, 67, 145, 130, 32, 45, 242, 67, 236, 38, 249, 161, 206, 1, 231, 46, 176, 51, 176, 169, 174, 20, 22, 42, 152, 193, 159, 134, 247, 69, 97, 125, 77, 116, 88, 150, 148, 138, 185, 120, 226, 252, 103, 97, 137, 219, 235, 75, 126, 46, 35, 79, 27, 1, 177, 81, 196, 221, 56, 252, 145, 230, 61, 90, 228, 48, 185, 62, 19, 151, 133, 95, 94, 200, 82, 152, 115, 200, 214, 217, 28, 42, 241, 194, 253, 222, 58, 154, 17, 248, 151, 145, 173, 213, 78, 156, 89, 60, 202, 190, 228, 99, 244, 241, 124, 162, 127, 181, 101, 12, 37, 79, 120, 219, 95, 40, 171, 222, 111, 226, 90, 68, 247, 198, 137, 173, 76, 85, 14, 64, 59, 242, 89, 182, 120, 177, 26, 108, 164, 42, 74, 29, 220, 127, 185, 132, 204, 197, 173, 177, 231, 135, 51, 196, 67, 171, 111, 55, 147, 147, 75, 247, 131, 94, 190, 169, 189, 182, 14, 47, 83, 96, 32, 87, 131, 3, 196, 236, 33, 88, 18, 98, 216, 166, 182, 7, 244, 60, 42, 170, 190, 26, 80, 182, 78, 14, 92, 108, 111, 181, 57, 69, 179, 209, 101, 80, 247, 152, 25, 200, 219, 14, 28, 98, 61, 221, 190, 129, 204, 211, 161, 55, 77, 176, 3, 193, 248, 151, 149, 118, 222, 254, 101, 180, 92, 78, 24, 87, 18, 152, 218, 216, 16, 207, 106, 29, 60, 45, 229, 184, 24, 144, 163, 100, 36, 97, 17, 10, 200, 246, 179, 94, 222, 77, 65, 50, 94, 218, 68, 133, 244, 60, 12, 23, 3, 3, 0, 95, 147, 212, 245, 117, 104, 119, 68, 13, 12, 34, 63, 201, 7, 111, 180, 13, 165, 220, 187, 35, 229, 159, 72, 1, 165, 51, 33, 41, 92, 103, 234, 127, 89, 206, 74, 89, 241, 84, 160, 197, 165, 179, 126, 28, 252, 130, 40, 193, 119, 192, 119, 242, 151, 188, 4, 244, 111, 124, 157, 87, 239, 176, 187, 41, 2, 96, 227, 224, 185, 107, 84, 180, 142, 214, 149, 63, 17, 86, 193, 6, 242, 20, 67, 199, 112, 84, 10, 58, 120, 233, 249, 168, 68, 6, 13, 23, 3, 3, 2, 23, 251, 164, 58, 225, 124, 192, 238, 19, 196, 251, 211, 58, 16, 44, 153, 191, 86, 25, 90, 160, 42, 162, 241, 77, 241, 197, 35, 157, 79, 229, 89, 241, 190, 124, 237, 100, 69, 39, 244, 234, 145, 70, 166, 161, 226, 250, 100, 97, 126, 117, 63, 241, 53, 80, 95, 35, 122, 21, 35, 63, 192, 183, 145, 221, 62, 49, 81, 240, 242, 149, 104, 168, 69, 162, 186, 145, 207, 4, 110, 107, 68, 237, 7, 69, 87, 181, 247, 96, 76, 142, 36, 57, 250, 153, 84, 218, 108, 240, 150, 32, 195, 61, 99, 140, 135, 201, 144, 134, 152, 130, 15, 29, 183, 93, 194, 80, 163, 137, 24, 172, 142, 10, 106, 82, 141, 116, 115, 85, 106, 8, 112, 158, 36, 1, 207, 156, 159, 96, 29, 199, 214, 218, 184, 184, 22, 231, 54, 90, 17, 147, 239, 93, 125, 31, 224, 4, 140, 65, 149, 123, 141, 142, 99, 90, 137, 173, 251, 16, 45, 123, 20, 205, 182, 127, 190, 31, 239, 86, 217, 66, 127, 251, 12, 4, 46, 147, 78, 111, 131, 170, 56, 102, 127, 147, 241, 176, 42, 39, 172, 153, 226, 150, 100, 120, 105, 108, 240, 237, 216, 138, 254, 94, 232, 49, 8, 2, 43, 176, 26, 244, 224, 88, 211, 54, 65, 91, 160, 48, 103, 151, 66, 36, 141, 91, 204, 47, 202, 212, 152, 159, 92, 124, 158, 25, 152, 8, 198, 130, 1, 34, 114, 240, 211, 208, 162, 4, 15, 48, 62, 14, 94, 214, 41, 121, 22, 235, 26, 56, 157, 57, 250, 223, 132, 121, 32, 10, 95, 19, 245, 144, 8, 128, 37, 144, 193, 239, 43, 2, 43, 235, 18, 242, 28, 54, 129, 180, 104, 165, 51, 222, 64, 3, 56, 69, 244, 183, 242, 62, 74, 1, 240, 86, 236, 248, 200, 251, 183, 240, 110, 94, 68, 55, 113, 60, 167, 136, 169, 29, 240, 248, 136, 112, 85, 53, 237, 110, 45, 248, 70, 249, 86, 96, 22, 136, 219, 231, 6, 204, 67, 239, 167, 172, 233, 29, 105, 32, 123, 6, 214, 141, 84, 115, 65, 59, 83, 52, 15, 83, 4, 106, 50, 207, 77, 115, 133, 169, 120, 71, 147, 76, 208, 121, 148, 130, 83, 216, 124, 113, 172, 34, 121, 140, 138, 206, 140, 93, 2, 186, 56, 11, 185, 170, 43, 190, 96, 203, 196, 57, 22, 167, 197, 110, 229, 157, 191, 255, 62, 119, 27, 195, 183, 92, 74, 183, 162, 240, 147, 115, 4, 169, 49, 63, 18, 175, 138, 66, 106, 93, 21, 239, 170, 157, 62, 40, 137, 168, 196, 142, 182, 87, 96, 112, 209, 75, 198, 195, 2, 38, 110, 239, 253, 222, 217, 168, 140, 98, 167, 124, 60, 24, 163, 58, 222, 140, 50, 168, 133, 227, 100, 51, 250, 136, 194, 250, 148, 31, 75, 67, 67, 7, 34, 234, 40, 156, 137, 198, 110, 60, 13, 130, 18, 118, 205, 62, 48, 196, 78, 122, 74, 231, 244, 95, 208, 181, 199, 201, 71, 227, 178, 0, 232, 216, 216, 33, 63, 110, 187, 6, 21, 80, 23, 0, 46, 171, 12, 23, 3, 3, 5, 115, 106, 181, 209, 156, 166, 161, 101, 254, 107, 17, 212, 188, 103, 192, 145, 166, 30, 23, 34, 154, 230, 133, 109, 63, 173, 35, 85, 150, 211, 90, 55, 231, 55, 133, 241, 231, 160, 204, 45, 79, 228, 242, 221, 92, 92, 141, 115, 105, 31, 234, 59, 204, 46, 120, 147, 191, 247, 11, 185, 113, 136, 113, 28, 23, 226, 188, 236, 174, 241, 41, 222, 145, 217, 197, 240, 15, 30, 21, 150, 154, 183, 114, 102, 13, 122, 83, 154, 112, 81, 51, 138, 112, 194, 99, 237, 188, 90, 4, 207, 139, 217, 3, 249, 197, 157, 197, 121, 206, 26, 171, 50, 103, 201, 56, 77, 163, 82, 203, 94, 135, 4, 209, 66, 44, 170, 108, 162, 232, 251, 98, 228, 252, 66, 88, 53, 0, 37, 159, 66, 220, 15, 54, 68, 155, 172, 206, 217, 168, 46, 158, 230, 224, 94, 204, 239, 152, 201, 103, 148, 32, 150, 106, 79, 52, 110, 49, 180, 254, 146, 135, 200, 103, 125, 62, 208, 92, 114, 189, 75, 87, 232, 70, 247, 252, 111, 43, 192, 85, 11, 69, 123, 224, 197, 88, 92, 81, 125, 34, 122, 236, 192, 255, 16, 139, 122, 5, 97, 16, 208, 63, 240, 168, 252, 236, 42, 144, 134, 88, 199, 125, 38, 97, 247, 116, 75, 57, 9, 246, 255, 214, 208, 120, 110, 220, 232, 122, 200, 8, 233, 125, 23, 146, 29, 173, 184, 91, 42, 191, 94, 247, 98, 138, 249, 252, 224, 42, 221, 248, 65, 46, 183, 32, 62, 107, 25, 218, 68, 157, 252, 30, 34, 87, 245, 160, 145, 22, 134, 160, 146, 29, 9, 243, 46, 208, 85, 231, 19, 166, 233, 73, 183, 206, 199, 24, 45, 118, 74, 29, 241, 121, 147, 26, 26, 2, 241, 229, 6, 110, 1, 105, 99, 121, 156, 127, 193, 223, 92, 139, 148, 8, 174, 146, 21, 157, 130, 250, 24, 182, 22, 101, 49, 73, 243, 194, 118, 57, 36, 84, 56, 252, 24, 81, 104, 177, 184, 38, 47, 232, 74, 222, 73, 210, 96, 113, 65, 164, 151, 0, 148, 115, 152, 46, 183, 87, 69, 56, 127, 99, 49, 180, 185, 184, 145, 208, 174, 110, 90, 41, 135, 245, 122, 8, 242, 92, 17, 210, 255, 162, 78, 83, 31, 60, 155, 231, 182, 161, 159, 205, 111, 12, 37, 70, 134, 133, 204, 69, 126, 153, 150, 71, 209, 122, 36, 130, 226, 255, 161, 42, 100, 120, 84, 147, 22, 15, 24, 51, 182, 150, 8, 200, 214, 166, 236, 184, 231, 182, 21, 11, 208, 142, 179, 3, 234, 198, 63, 42, 2, 23, 29, 85, 46, 154, 63, 242, 75, 19, 182, 149, 184, 62, 67, 133, 74, 75, 128, 224, 247, 243, 198, 130, 33, 11, 160, 164, 208, 129, 202, 156, 184, 134, 128, 44, 128, 118, 85, 42, 36, 25, 7, 242, 209, 140, 25, 66, 157, 129, 49, 189, 193, 147, 26, 102, 161, 224, 50, 176, 7, 28, 36, 234, 40, 243, 144, 203, 242, 199, 133, 248, 66, 187, 110, 235, 172, 250, 108, 80, 219, 245, 187, 43, 250, 161, 89, 226, 94, 217, 154, 67, 128, 209, 129, 159, 64, 235, 144, 153, 244, 3, 237, 40, 231, 207, 200, 179, 203, 204, 45, 163, 185, 193, 161, 188, 249, 4, 1, 211, 32, 225, 130, 187, 126, 97, 175, 59, 168, 34, 223, 189, 93, 221, 250, 254, 189, 208, 101, 80, 228, 255, 7, 78, 231, 111, 157, 167, 120, 9, 164, 61, 100, 177, 12, 26, 177, 231, 249, 2, 90, 141, 11, 228, 126, 214, 29, 147, 64, 195, 242, 140, 148, 6, 51, 58, 252, 133, 220, 58, 185, 44, 90, 204, 215, 165, 89, 49, 173, 107, 60, 163, 153, 160, 55, 217, 124, 30, 175, 134, 205, 173, 70, 130, 22, 222, 157, 32, 80, 90, 12, 172, 20, 93, 219, 241, 158, 112, 154, 111, 68, 251, 113, 155, 31, 60, 164, 221, 198, 36, 40, 87, 62, 71, 0, 67, 80, 110, 164, 54, 146, 45, 250, 136, 185, 39, 11, 65, 128, 54, 59, 198, 202, 198, 221, 13, 170, 94, 148, 193, 115, 180, 57, 146, 236, 15, 8, 42, 177, 78, 85, 105, 81, 242, 16, 240, 239, 152, 210, 162, 34, 247, 113, 176, 44, 0, 33, 131, 183, 40, 94, 115, 247, 153, 231, 179, 52, 46, 172, 66, 239, 110, 149, 102, 244, 229, 116, 187, 1, 20, 144, 122, 43, 197, 148, 179, 240, 152, 116, 216, 37, 68, 148, 127, 189, 94, 76, 33, 109, 37, 131, 31, 36, 127, 189, 74, 49, 246, 166, 50, 204, 127, 107, 73, 218, 132, 97, 61, 130, 125, 37, 207, 155, 21, 143, 230, 1, 243, 48, 88, 35, 112, 4, 112, 64, 55, 135, 56, 68, 112, 38, 124, 47, 19, 97, 245, 145, 38, 178, 27, 210, 227, 210, 246, 131, 102, 132, 172, 203, 117, 105, 240, 97, 203, 239, 58, 123, 123, 155, 192, 107, 50, 73, 45, 223, 141, 174, 4, 182, 108, 94, 61, 200, 165, 220, 216, 152, 200, 250, 193, 223, 197, 1, 16, 226, 67, 50, 207, 66, 238, 193, 253, 8, 164, 208, 61, 16, 47, 249, 97, 138, 189, 25, 40, 98, 12, 180, 59, 189, 218, 178, 66, 113, 216, 195, 88, 3, 19, 135, 190, 169, 127, 18, 116, 41, 179, 84, 51, 248, 244, 108, 229, 214, 184, 199, 173, 12, 45, 112, 3, 161, 15, 185, 1, 43, 77, 107, 245, 141, 79, 1, 124, 128, 172, 91, 169, 82, 217, 225, 99, 103, 196, 74, 232, 236, 162, 252, 116, 41, 207, 223, 164, 118, 106, 29, 25, 2, 121, 16, 3, 16, 154, 57, 80, 239, 70, 70, 0, 240, 10, 174, 84, 195, 0, 53, 69, 177, 183, 49, 88, 20, 51, 240, 225, 130, 173, 232, 116, 107, 235, 141, 74, 227, 69, 130, 159, 129, 178, 184, 139, 5, 27, 209, 166, 225, 60, 199, 130, 44, 70, 193, 78, 44, 187, 54, 58, 236, 186, 9, 237, 153, 39, 45, 144, 102, 241, 8, 192, 166, 1, 239, 254, 227, 164, 6, 247, 150, 89, 155, 16, 216, 157, 69, 74, 4, 250, 56, 164, 151, 184, 32, 221, 147, 18, 132, 81, 54, 228, 186, 219, 155, 21, 172, 163, 60, 43, 195, 174, 161, 62, 243, 218, 173, 8, 199, 4, 132, 52, 1, 105, 42, 22, 33, 146, 255, 123, 148, 87, 81, 74, 15, 84, 58, 187, 159, 88, 130, 61, 9, 115, 18, 106, 194, 76, 48, 167, 165, 83, 145, 247, 215, 168, 217, 146, 158, 220, 95, 20, 181, 54, 121, 64, 204, 169, 191, 138, 2, 229, 79, 110, 177, 126, 127, 74, 80, 80, 148, 41, 200, 43, 79, 74, 88, 88, 254, 92, 135, 171, 220, 109, 212, 108, 10, 161, 96, 91, 226, 58, 76, 90, 209, 63, 175, 142, 125, 208, 39, 90, 157, 47, 4, 43, 98, 217, 61, 152, 220, 29, 53, 241, 210, 5, 72, 19, 229, 124, 64, 54, 181, 212, 95, 161, 71, 115, 75, 247, 75, 146, 226, 37, 116, 96, 63, 188, 13, 253, 214, 43, 127, 252, 144, 192, 64, 61, 64, 38, 48, 11, 183, 225, 158, 174, 197, 214, 57, 124, 43, 143, 134, 196, 40, 170, 160, 227, 175, 151, 63, 192, 251, 176, 24, 191, 187, 67, 0, 101, 217, 65, 191, 31, 116, 132, 115, 14, 113, 153, 128, 206, 155, 55, 164, 111, 135, 61, 72, 52, 23, 61, 55, 148, 154, 248, 229, 50, 126, 206, 50, 171, 207, 148, 178, 36, 151, 11, 216, 170, 214, 179, 7, 64, 203, 222, 193, 191, 212, 3, 118, 247, 54, 3, 205, 95, 96, 2, 208, 182, 47, 11, 107, 236, 52, 209, 137, 3, 241, 180, 86, 13, 99, 167, 96, 87, 105, 187, 29, 26, 11, 253, 234, 222, 240, 242, 79, 14, 33, 113, 52, 130, 98, 248, 11, 191, 182, 193, 70, 226, 46, 48, 171, 201, 241, 251, 224, 125, 228, 11, 158, 121, 23, 72, 36, 63, 136, 201, 251, 54, 63, 61, 203, 147, 140, 95, 12, 7, 194, 55, 245, 42, 228, 94, 13, 104, 226, 121, 235, 165, 157, 227, 20, 23, 3, 3, 0, 196, 149, 117, 144, 96, 120, 13, 23, 255, 212, 17, 4, 108, 209, 32, 159, 240, 134, 133, 89, 95, 153, 5, 64, 135, 91, 91, 116, 23, 245, 138, 105, 61, 73, 223, 70, 52, 104, 244, 143, 125, 90, 87, 199, 121, 185, 104, 98, 114, 181, 71, 240, 166, 137, 19, 122, 1, 192, 162, 19, 126, 175, 185, 245, 163, 34, 137, 192, 100, 65, 76, 100, 193, 192, 99, 51, 182, 150, 143, 241, 103, 111, 2, 173, 59, 36, 87, 117, 98, 173, 160, 193, 209, 115, 153, 157, 143, 242, 53, 91, 190, 164, 187, 7, 88, 91, 234, 51, 157, 43, 76, 76, 0, 167, 230, 252, 215, 159, 128, 252, 168, 186, 231, 184, 132, 42, 57, 46, 68, 137, 54, 16, 60, 17, 196, 92, 199, 35, 24, 184, 35, 48, 94, 249, 238, 186, 253, 112, 37, 54, 23, 68, 42, 43, 165, 215, 89, 175, 111, 192, 81, 108, 39, 171, 143, 35, 56, 14, 239, 75, 219, 200, 198, 17, 136, 98, 219, 77, 175, 22, 51, 11, 27, 251, 89, 226, 233, 205, 187, 228, 232, 188, 181, 199, 148, 97, 81];
    /*vec![
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
    ];*/
    let cell = TokenValue::write_bytes(/* &[4u8] */ &tls_data, &ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let cert: Vec<u8> = vec![5, 91, 48, 130, 5, 87, 48, 130, 3, 63, 160, 3, 2, 1, 2, 2, 13, 2, 3, 229, 147, 111, 49, 176, 19, 73, 136, 107, 162, 23, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 12, 5, 0, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 49, 48, 30, 23, 13, 49, 54, 48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 23, 13, 51, 54, 48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 49, 48, 130, 2, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 2, 15, 0, 48, 130, 2, 10, 2, 130, 2, 1, 0, 182, 17, 2, 139, 30, 227, 161, 119, 155, 59, 220, 191, 148, 62, 183, 149, 167, 64, 60, 161, 253, 130, 249, 125, 50, 6, 130, 113, 246, 246, 140, 127, 251, 232, 219, 188, 106, 46, 151, 151, 163, 140, 75, 249, 43, 246, 177, 249, 206, 132, 29, 177, 249, 197, 151, 222, 239, 185, 242, 163, 233, 188, 18, 137, 94, 167, 170, 82, 171, 248, 35, 39, 203, 164, 177, 156, 99, 219, 215, 153, 126, 240, 10, 94, 235, 104, 166, 244, 198, 90, 71, 13, 77, 16, 51, 227, 78, 177, 19, 163, 200, 24, 108, 75, 236, 252, 9, 144, 223, 157, 100, 41, 37, 35, 7, 161, 180, 210, 61, 46, 96, 224, 207, 210, 9, 135, 187, 205, 72, 240, 77, 194, 194, 122, 136, 138, 187, 186, 207, 89, 25, 214, 175, 143, 176, 7, 176, 158, 49, 241, 130, 193, 192, 223, 46, 166, 109, 108, 25, 14, 181, 216, 126, 38, 26, 69, 3, 61, 176, 121, 164, 148, 40, 173, 15, 127, 38, 229, 168, 8, 254, 150, 232, 60, 104, 148, 83, 238, 131, 58, 136, 43, 21, 150, 9, 178, 224, 122, 140, 46, 117, 214, 156, 235, 167, 86, 100, 143, 150, 79, 104, 174, 61, 151, 194, 132, 143, 192, 188, 64, 192, 11, 92, 189, 246, 135, 179, 53, 108, 172, 24, 80, 127, 132, 224, 76, 205, 146, 211, 32, 233, 51, 188, 82, 153, 175, 50, 181, 41, 179, 37, 42, 180, 72, 249, 114, 225, 202, 100, 247, 230, 130, 16, 141, 232, 157, 194, 138, 136, 250, 56, 102, 138, 252, 99, 249, 1, 249, 120, 253, 123, 92, 119, 250, 118, 135, 250, 236, 223, 177, 14, 121, 149, 87, 180, 189, 38, 239, 214, 1, 209, 235, 22, 10, 187, 142, 11, 181, 197, 197, 138, 85, 171, 211, 172, 234, 145, 75, 41, 204, 25, 164, 50, 37, 78, 42, 241, 101, 68, 208, 2, 206, 170, 206, 73, 180, 234, 159, 124, 131, 176, 64, 123, 231, 67, 171, 167, 108, 163, 143, 125, 137, 129, 250, 76, 165, 255, 213, 142, 195, 206, 75, 224, 181, 216, 179, 142, 69, 207, 118, 192, 237, 64, 43, 253, 83, 15, 176, 167, 213, 59, 13, 177, 138, 162, 3, 222, 49, 173, 204, 119, 234, 111, 123, 62, 214, 223, 145, 34, 18, 230, 190, 250, 216, 50, 252, 16, 99, 20, 81, 114, 222, 93, 214, 22, 147, 189, 41, 104, 51, 239, 58, 102, 236, 7, 138, 38, 223, 19, 215, 87, 101, 120, 39, 222, 94, 73, 20, 0, 162, 0, 127, 154, 168, 33, 182, 169, 177, 149, 176, 165, 185, 13, 22, 17, 218, 199, 108, 72, 60, 64, 224, 126, 13, 90, 205, 86, 60, 209, 151, 5, 185, 203, 75, 237, 57, 75, 156, 196, 63, 210, 85, 19, 110, 36, 176, 214, 113, 250, 244, 193, 186, 204, 237, 27, 245, 254, 129, 65, 216, 0, 152, 61, 58, 200, 174, 122, 152, 55, 24, 5, 149, 2, 3, 1, 0, 1, 163, 66, 48, 64, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 228, 175, 43, 38, 113, 26, 43, 72, 39, 133, 47, 82, 102, 44, 239, 240, 137, 19, 113, 62, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 12, 5, 0, 3, 130, 2, 1, 0, 159, 170, 66, 38, 219, 11, 155, 190, 255, 30, 150, 146, 46, 62, 162, 101, 74, 106, 152, 186, 34, 203, 125, 193, 58, 216, 130, 10, 6, 198, 246, 165, 222, 192, 78, 135, 102, 121, 161, 249, 166, 88, 156, 170, 249, 181, 230, 96, 231, 224, 232, 177, 30, 66, 65, 51, 11, 55, 61, 206, 137, 112, 21, 202, 181, 36, 168, 207, 107, 181, 210, 64, 33, 152, 207, 34, 52, 207, 59, 197, 34, 132, 224, 197, 14, 138, 124, 93, 136, 228, 53, 36, 206, 155, 62, 26, 84, 30, 110, 219, 178, 135, 167, 252, 243, 250, 129, 85, 20, 98, 10, 89, 169, 34, 5, 49, 62, 130, 214, 238, 219, 87, 52, 188, 51, 149, 211, 23, 27, 232, 39, 162, 139, 123, 78, 38, 26, 122, 90, 100, 182, 209, 172, 55, 241, 253, 160, 243, 56, 236, 114, 240, 17, 117, 157, 203, 52, 82, 141, 230, 118, 107, 23, 198, 223, 134, 171, 39, 142, 73, 43, 117, 102, 129, 16, 33, 166, 234, 62, 244, 174, 37, 255, 124, 21, 222, 206, 140, 37, 63, 202, 98, 112, 10, 247, 47, 9, 102, 7, 200, 63, 28, 252, 240, 219, 69, 48, 223, 98, 136, 193, 181, 15, 157, 195, 159, 74, 222, 89, 89, 71, 197, 135, 34, 54, 230, 130, 167, 237, 10, 185, 226, 7, 160, 141, 123, 122, 74, 60, 113, 210, 226, 3, 161, 31, 50, 7, 221, 27, 228, 66, 206, 12, 0, 69, 97, 128, 181, 11, 32, 89, 41, 120, 189, 249, 85, 203, 99, 197, 60, 76, 244, 182, 255, 219, 106, 95, 49, 107, 153, 158, 44, 193, 107, 80, 164, 215, 230, 24, 20, 189, 133, 63, 103, 171, 70, 159, 160, 255, 66, 167, 58, 127, 92, 203, 93, 176, 112, 29, 43, 52, 245, 212, 118, 9, 12, 235, 120, 76, 89, 5, 243, 51, 66, 195, 97, 21, 16, 27, 119, 77, 206, 34, 140, 212, 133, 242, 69, 125, 183, 83, 234, 239, 64, 90, 148, 10, 92, 32, 95, 78, 64, 93, 98, 34, 118, 223, 255, 206, 97, 189, 140, 35, 120, 210, 55, 2, 224, 142, 222, 209, 17, 55, 137, 246, 191, 237, 73, 7, 98, 174, 146, 236, 64, 26, 175, 20, 9, 217, 208, 78, 178, 162, 247, 190, 238, 238, 216, 255, 220, 26, 45, 222, 184, 54, 113, 226, 252, 121, 183, 148, 37, 209, 72, 115, 91, 161, 53, 231, 179, 153, 103, 117, 193, 25, 58, 43, 71, 78, 211, 66, 142, 253, 49, 200, 22, 102, 218, 210, 12, 60, 219, 179, 142, 201, 161, 13, 128, 15, 123, 22, 119, 20, 191, 255, 219, 9, 148, 178, 147, 188, 32, 88, 21, 233, 219, 113, 67, 243, 222, 16, 195, 0, 220, 168, 42, 149, 182, 194, 214, 63, 144, 107, 118, 219, 108, 254, 140, 188, 242, 112, 53, 12, 220, 153, 25, 53, 220, 215, 200, 70, 99, 213, 54, 113, 174, 87, 251, 183, 130, 109, 220];
    
    /*vec![
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
    ];*/
    let cell =
        TokenValue::write_bytes(/* &[3u8] */ &cert, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));

    let kid: Vec<u8> = vec![20, 142, 143, 200, 229, 86, 247, 167, 109, 8, 211, 88, 41, 214, 249, 10, 226, 225, 44, 253, 13];
    /*vec![
        13, 138, 103, 57, 158, 120, 130, 172, 174, 125, 127, 104, 178, 40, 2, 86, 167, 150, 165,
        130,
    ];*/
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

/**** VERGRTH16 TEST */

#[test]
fn test_poseidon_and_vergrth16() {
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

 
     // password was 567890 in ascii 535455565748
    let user_pass_salt = "535455565748";
    let secret_key = [222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // 
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());
    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103",
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    )
    .unwrap();
    println!("zk_seed = {:?}", zk_seed);
    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

    println!("iss_and_header_base64details: {}", iss_and_header_base64details);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
     all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(), 
        ),
        content,
    );

    let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
        ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
    }).unwrap();

    let max_epoch = 142; 

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| {
            ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
        })
    .unwrap();

    let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
    println!("====== Start Poseidon ========");
    
    let index_mod_4 = 1;
    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
    println!("modulus_cell = {:?}", modulus_cell);
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    
    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
    println!("iss_base_64_cell = {:?}", iss_base_64_cell);
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
    println!("header_base_64_cell = {:?}", header_base_64_cell);
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
    println!("zk_seed_cell = {:?}", zk_seed_cell);
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let start: Instant = Instant::now();
    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_elapsed = start.elapsed().as_micros();

    println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);  

    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
    let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
    println!("poseidon_res from stack: {:?}", hex::encode(poseidon_res.clone()));

    println!("public_inputs hex (computed in test): {:?}", hex::encode(public_inputs_as_bytes.clone()));
    assert!(poseidon_res == public_inputs_as_bytes);

    println!("====== Start VERGRTH16 ========");
    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    let verification_key_id: u32 = 0; // valid key id
    //let verification_key_id: u32 = 1; //invalid key id
    engine.cc.stack.push(StackItem::int(verification_key_id));

    let start: Instant = Instant::now();
    let status = execute_vergrth16(&mut engine).unwrap();
    let vergrth16_elapsed = start.elapsed().as_micros();

    println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed); 

    let res = engine.cc.stack.get(0).as_integer().unwrap();
    println!("res: {:?}", res);
    assert!(*res == IntegerData::minus_one());

    
}

pub const TEST_AUTH_DATA_1: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImJ4bW5KVzMxcnV6S01HaXIwMVlQR1lMMHhEWSIsIm5iZiI6MTcxNTY4NzAzNiwiaWF0IjoxNzE1Njg3MzM2LCJleHAiOjE3MTU2OTA5MzYsImp0aSI6IjliNjAxZDI1ZjAwMzY0MGMyODg5YTJhMDQ3Nzg5MzgyY2IxY2ZlODcifQ.rTa9KA9HoYm04Agj71D0kDkvsCZ35SeeihBGbABYckBRxaUlCy6LQ-sEaVOTgvnL_DgVn7hx8g3sSmnhJ9kHzj5e6gtUoxoWAe8PuGyK2bmqhmPrQMeEps9f6m2EToQCIA_Id4fGCjSCktjJBi47QHT_Dhe6isHdKk1pgSshOyvCF1VjIvyyeGY5iWQ4cIRBMQNlNBT11o6T01SY6B9DtiiFN_0-ok5taIjQgtMNG6Cwr3tCnqXftuGGQrHlx15y8VgCPODYi-wOtvUbzI2yfx53PmRD_L8O50cMNCrCRE3yYR5MNOu1LlQ_EACy5UFsCJR35xRz84nv-6Iyrufx1g\",\"user_pass_to_int_format\":\"981021191041055255531141165751\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":155,\"1\":147,\"2\":37,\"3\":82,\"4\":183,\"5\":109,\"6\":227,\"7\":144,\"8\":85,\"9\":248,\"10\":20,\"11\":45,\"12\":92,\"13\":103,\"14\":160,\"15\":221,\"16\":101,\"17\":44,\"18\":30,\"19\":86,\"20\":96,\"21\":85,\"22\":24,\"23\":224,\"24\":106,\"25\":63,\"26\":13,\"27\":130,\"28\":8,\"29\":119,\"30\":247,\"31\":67},\"secret_key\":{\"0\":192,\"1\":16,\"2\":35,\"3\":54,\"4\":100,\"5\":14,\"6\":88,\"7\":217,\"8\":164,\"9\":21,\"10\":154,\"11\":233,\"12\":248,\"13\":208,\"14\":188,\"15\":4,\"16\":52,\"17\":244,\"18\":125,\"19\":103,\"20\":99,\"21\":26,\"22\":225,\"23\":60,\"24\":140,\"25\":75,\"26\":228,\"27\":157,\"28\":137,\"29\":220,\"30\":1,\"31\":65,\"32\":155,\"33\":147,\"34\":37,\"35\":82,\"36\":183,\"37\":109,\"38\":227,\"39\":144,\"40\":85,\"41\":248,\"42\":20,\"43\":45,\"44\":92,\"45\":103,\"46\":160,\"47\":221,\"48\":101,\"49\":44,\"50\":30,\"51\":86,\"52\":96,\"53\":85,\"54\":24,\"55\":224,\"56\":106,\"57\":63,\"58\":13,\"59\":130,\"60\":8,\"61\":119,\"62\":247,\"63\":67}}},\"zk_addr\":\"0x290623ea2fe67e77502c931e015e910720b59cf99994bfe872da851245a6adb8\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"4240296169193969312736577528388333411353554120022978085193148043577551744781\",\"5805161066003598301896048908428560240907086333477483881772048922050706263054\",\"1\"],\"b\":[[\"12834391737669124973917765536412427456985620342194191639017091262766903638891\",\"17565396762846717347409742387259908749145765976354144805005547481529916658455\"],[\"10704310067924910937030159163683742097178285875135929496314190235513445131794\",\"5158907077493606386023392148737817037260820737072162547798816810512684527243\"],[\"1\",\"0\"]],\"c\":[\"1422540522119231707130773229384414857146368773886805969586218853559909475064\",\"8843079196273712399340537238369227864378150337693574970239878271571912585171\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AJuTJVK3beOQVfgULVxnoN1lLB5WYFUY4Go/DYIId/dD\"}";
pub const TEST_AUTH_DATA_2: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjJKd0VMbjJfUV9Rd0VsTC1rWTFPRnFqdXZCMCIsIm5iZiI6MTcxNTY4NzAyOSwiaWF0IjoxNzE1Njg3MzI5LCJleHAiOjE3MTU2OTA5MjksImp0aSI6ImU2YjM1ZjJmNmFkNjIzOWEwMDAxMTJiMWI5YWI2MWQ0MjRkMGM1OTIifQ.QcrEDE9qmPZKX83nU3Tx2BN8fsinb_mmXkO1Qf7Uv1QTd0NjirSeu7C4Vn9WDNWDaIR-BgCfhOlkwMQPljcahqC4AN43N_66tvbEsXjtEdFejslXrGG4D_BEKvtmD7_WkW388LyU2PxKgtdDfpYFgmuT6wTM2TO5dTbrGrDyn88q3pkPfefC5a8Wi1V6zECfFdSV-pKQlxtPaImi7s3CKAUMDu1n-jcT-Ho2aTgrWKAzhXE56tgEWOpXQO06eJsWCSOqoZSLYtatTrZr4d38U7QRQiNlH-ydHv4zXt1tixLLJ0wvPx-dQaCnCl1kW1orYkJGFfHgjx6A9z5Ol4afuw\",\"user_pass_to_int_format\":\"101119106102103\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":194,\"1\":38,\"2\":203,\"3\":255,\"4\":219,\"5\":127,\"6\":105,\"7\":129,\"8\":234,\"9\":222,\"10\":71,\"11\":169,\"12\":108,\"13\":94,\"14\":28,\"15\":48,\"16\":111,\"17\":221,\"18\":113,\"19\":110,\"20\":5,\"21\":226,\"22\":19,\"23\":230,\"24\":232,\"25\":67,\"26\":255,\"27\":179,\"28\":6,\"29\":10,\"30\":209,\"31\":63},\"secret_key\":{\"0\":44,\"1\":32,\"2\":251,\"3\":184,\"4\":109,\"5\":252,\"6\":105,\"7\":67,\"8\":208,\"9\":111,\"10\":86,\"11\":214,\"12\":192,\"13\":135,\"14\":169,\"15\":48,\"16\":162,\"17\":36,\"18\":216,\"19\":145,\"20\":232,\"21\":64,\"22\":17,\"23\":14,\"24\":29,\"25\":56,\"26\":39,\"27\":118,\"28\":143,\"29\":250,\"30\":31,\"31\":66,\"32\":194,\"33\":38,\"34\":203,\"35\":255,\"36\":219,\"37\":127,\"38\":105,\"39\":129,\"40\":234,\"41\":222,\"42\":71,\"43\":169,\"44\":108,\"45\":94,\"46\":28,\"47\":48,\"48\":111,\"49\":221,\"50\":113,\"51\":110,\"52\":5,\"53\":226,\"54\":19,\"55\":230,\"56\":232,\"57\":67,\"58\":255,\"59\":179,\"60\":6,\"61\":10,\"62\":209,\"63\":63}}},\"zk_addr\":\"0x9d28c04a423b33d6901065b2e23440d80c963e2d8cf60619aed131cf302a3345\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"10113442204684515220664612836724727112601024759319365467272456423129044788607\",\"1622056145268645528934658046911045406324940175278473377024147189407527440953\",\"1\"],\"b\":[[\"16638441944380099215425740101953753038808466958852552979180365845498468757656\",\"15160836857346434734063515954042830497610079883703780011464867547889770445695\"],[\"18562910453341688699790780964434211467815845944672185772065803860963710445937\",\"8200691834141582017549140597895023392490964486044036655696113278873832146838\"],[\"1\",\"0\"]],\"c\":[\"4229037146526046139176767312447148765936834700862335953317784850097077554287\",\"14155516063621997063825085002662503289554536312724791903045026922766401869119\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AMImy//bf2mB6t5HqWxeHDBv3XFuBeIT5uhD/7MGCtE/\"}";
pub const TEST_AUTH_DATA_3: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkFadlVHUkI5MU5VZmdnYVV5bUdCQU9kSmM2ayIsIm5iZiI6MTcxNTY4NzAyMiwiaWF0IjoxNzE1Njg3MzIyLCJleHAiOjE3MTU2OTA5MjIsImp0aSI6ImZiNGNhMzdjOGE5MjEzOTFjZTE2ZDQwNmE2NmVmYjA1MTQxNTg5YjYifQ.C7zNP2sxRMF62irwNjO2y_JVMjYLqGk6sAWy0rKoXswa7SA6KhPrWocMAB2GKaQW-CeqUzMJdypgJz1RcMzmOWg30cv4diEgqBSM1I1ocOI5ivRE2Atj8g-Oj2uAm_DBvuJBLzTA6wfb34QTasOTZqLsMyoaQavxUprzPi-1z-MUE-darDjZ-IkWu7SctdEzNhSuUfQPJo_sbN5_38dQm300plXK-9iJgDxMWmT4NPO91hSQaGKbBm_euMI-fBAfYwARMnlaTETvSiCNSAyzphNrBi9kU49BFi5X04GoIkSW4zFwb74OeFbL49_14AZZ9Z2Mw7EPQ9sAAjzanxPUfA\",\"user_pass_to_int_format\":\"98118101102104106100\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":219,\"1\":225,\"2\":68,\"3\":197,\"4\":249,\"5\":59,\"6\":249,\"7\":200,\"8\":218,\"9\":242,\"10\":184,\"11\":214,\"12\":247,\"13\":159,\"14\":9,\"15\":162,\"16\":60,\"17\":174,\"18\":162,\"19\":13,\"20\":111,\"21\":5,\"22\":61,\"23\":179,\"24\":155,\"25\":167,\"26\":207,\"27\":6,\"28\":174,\"29\":163,\"30\":23,\"31\":23},\"secret_key\":{\"0\":28,\"1\":117,\"2\":37,\"3\":14,\"4\":166,\"5\":188,\"6\":125,\"7\":36,\"8\":70,\"9\":193,\"10\":162,\"11\":142,\"12\":79,\"13\":218,\"14\":210,\"15\":131,\"16\":217,\"17\":32,\"18\":88,\"19\":246,\"20\":195,\"21\":214,\"22\":135,\"23\":80,\"24\":27,\"25\":198,\"26\":131,\"27\":31,\"28\":3,\"29\":240,\"30\":199,\"31\":129,\"32\":219,\"33\":225,\"34\":68,\"35\":197,\"36\":249,\"37\":59,\"38\":249,\"39\":200,\"40\":218,\"41\":242,\"42\":184,\"43\":214,\"44\":247,\"45\":159,\"46\":9,\"47\":162,\"48\":60,\"49\":174,\"50\":162,\"51\":13,\"52\":111,\"53\":5,\"54\":61,\"55\":179,\"56\":155,\"57\":167,\"58\":207,\"59\":6,\"60\":174,\"61\":163,\"62\":23,\"63\":23}}},\"zk_addr\":\"0xeccbb76b41c1fd5e19950f0c005e5d2a2596b9cc510e98b6f69bb3cf590b3cf8\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"11651445672013969095011012560101085321682180365624939394143647198080899422642\",\"1099774502834574451399947043208869188329872135932897351612210181871486714260\",\"1\"],\"b\":[[\"4095258550782358133185755302461547336434190495389756275789648565352453295275\",\"11290282088300413285686821769617771231670721476484846359206004074570380534935\"],[\"10130196410049440247754977520268298700433580296307256932070052957562923587210\",\"18578315450133100598244014262861961858129311260491371986249505812898194068790\"],[\"1\",\"0\"]],\"c\":[\"3621803486710965065098877836422521469652420656514094958857631583114966034063\",\"10775419351495516109888010278620848514990288696189982169937651175162131341248\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ANvhRMX5O/nI2vK41vefCaI8rqINbwU9s5unzwauoxcX\"}";
pub const TEST_AUTH_DATA_4: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6Inh4VjZnc3RReGY2WW5Kb0QyMXMtZkNWdm9ESSIsIm5iZiI6MTcxNTY4NzAxNSwiaWF0IjoxNzE1Njg3MzE1LCJleHAiOjE3MTU2OTA5MTUsImp0aSI6ImQzOWJjMTUyOWVhODMwNTRiNGU0MDRlNDRhMmE4ZWRjOTJlZWFiYWYifQ.u23WQFEtc4TldMtNqrU7DiYdL33X2QySNxueCW79LQHc00P7g-Pu7xPX0XK_TLxP6ReZEdpdCmjfG6g--XBYXh313FKcqVhcrtKdBE06jf5acAf4fQ3TVzG5CFWqjISRhLL0eGjX20DZm8drrSFYTgfWPl9ANo6TV2IFF6BR9TOO_flxzmXPRVvER9ZA4QO52JCqagVYBw4bFZcUebiN_KXYuXOYWzUAiHM7lKUdVKoCte9JDKnTfRNg3r-i5tt5Oiovswwh9jubQd5c8nCQckQ8Fj9T5nPmlfPtF282kfd76xlHckvL94mM3HUKNuFrxeiFX07f_5Ff7NxvQ3QPgw\",\"user_pass_to_int_format\":\"118102101104106\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":25,\"1\":68,\"2\":102,\"3\":77,\"4\":86,\"5\":118,\"6\":203,\"7\":106,\"8\":41,\"9\":192,\"10\":205,\"11\":144,\"12\":20,\"13\":158,\"14\":42,\"15\":167,\"16\":18,\"17\":30,\"18\":27,\"19\":103,\"20\":51,\"21\":222,\"22\":226,\"23\":224,\"24\":168,\"25\":111,\"26\":16,\"27\":214,\"28\":128,\"29\":165,\"30\":10,\"31\":183},\"secret_key\":{\"0\":204,\"1\":128,\"2\":233,\"3\":135,\"4\":233,\"5\":64,\"6\":127,\"7\":97,\"8\":231,\"9\":135,\"10\":123,\"11\":149,\"12\":126,\"13\":145,\"14\":173,\"15\":252,\"16\":33,\"17\":141,\"18\":251,\"19\":181,\"20\":223,\"21\":9,\"22\":77,\"23\":32,\"24\":19,\"25\":187,\"26\":3,\"27\":180,\"28\":110,\"29\":49,\"30\":114,\"31\":167,\"32\":25,\"33\":68,\"34\":102,\"35\":77,\"36\":86,\"37\":118,\"38\":203,\"39\":106,\"40\":41,\"41\":192,\"42\":205,\"43\":144,\"44\":20,\"45\":158,\"46\":42,\"47\":167,\"48\":18,\"49\":30,\"50\":27,\"51\":103,\"52\":51,\"53\":222,\"54\":226,\"55\":224,\"56\":168,\"57\":111,\"58\":16,\"59\":214,\"60\":128,\"61\":165,\"62\":10,\"63\":183}}},\"zk_addr\":\"0x9440174050c8a69f3736aade438d256444387d7f99afaf9b5a9f29c6f0fba0c3\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"12337032776119704699956096862904448418119911526311506121881119564201699892276\",\"21261432927871679671381948020842646421823600661053961908567605147368225372658\",\"1\"],\"b\":[[\"361501104451650926380087094710685809078127996371826342961671838349546013669\",\"6224896865231367783073876006741593926823975323893517814398563485217838362592\"],[\"17991862631010087641911530148948529285385885925990265147692471125933697566220\",\"3919918348467391624469564417209140189505145619892305626999747602773689849635\"],[\"1\",\"0\"]],\"c\":[\"2974798412198231516644318932878285282801453498857240613838304706754188993145\",\"18411763423260631630440151338922210964792206590205572668118109635459867927504\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ABlEZk1WdstqKcDNkBSeKqcSHhtnM97i4KhvENaApQq3\"}";
pub const TEST_AUTH_DATA_5: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IlZXM0VrSmp5ZXM0M2pvNENXU0FyU1pORHk5ayIsIm5iZiI6MTcxNTY4NzAwNywiaWF0IjoxNzE1Njg3MzA3LCJleHAiOjE3MTU2OTA5MDcsImp0aSI6IjMwNDczMjk0MmI3MDQzOTUzN2M3OTE4YTIyZDMxODA1YTVjYzkzZTYifQ.sa6ee8fcMhF3JgGQcl03IY0alries0KC7SRH-HVUnA5cqTVYomJ6fr0NTDJmYXNKOeIcaT85LLN0ALsKtEQdZjhu1g4m16kbS-5MybFIXT85JIPhBOz7zYldrbiy-Me8XRNWPkR3X_lV9pwqvYJTnZ0ley5dDITRvIXE1w2ZmjGNDlDxG3aM2XOQDICQ1ztsCZkn20ShuvG7tZHq7cp7K0hd6JdX0fFRY85eSIeapW7NnnWdvJi2xvuiCcwqm8sshldcJI5uU9xikhoN2WA7c8fJ5rtshqp5-RtTOfbzLn2a6m0WeDE0JqUd8jbh6_T8mGtYYeYMAWfWb-jVPa8aNg\",\"user_pass_to_int_format\":\"118981041155255\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":76,\"1\":252,\"2\":245,\"3\":48,\"4\":8,\"5\":126,\"6\":211,\"7\":60,\"8\":249,\"9\":241,\"10\":111,\"11\":107,\"12\":148,\"13\":35,\"14\":224,\"15\":237,\"16\":179,\"17\":74,\"18\":84,\"19\":7,\"20\":130,\"21\":96,\"22\":198,\"23\":40,\"24\":23,\"25\":4,\"26\":50,\"27\":62,\"28\":191,\"29\":222,\"30\":119,\"31\":195},\"secret_key\":{\"0\":217,\"1\":194,\"2\":91,\"3\":84,\"4\":244,\"5\":214,\"6\":113,\"7\":57,\"8\":79,\"9\":43,\"10\":104,\"11\":85,\"12\":61,\"13\":225,\"14\":26,\"15\":139,\"16\":139,\"17\":206,\"18\":110,\"19\":48,\"20\":118,\"21\":99,\"22\":130,\"23\":122,\"24\":59,\"25\":6,\"26\":224,\"27\":144,\"28\":146,\"29\":25,\"30\":147,\"31\":225,\"32\":76,\"33\":252,\"34\":245,\"35\":48,\"36\":8,\"37\":126,\"38\":211,\"39\":60,\"40\":249,\"41\":241,\"42\":111,\"43\":107,\"44\":148,\"45\":35,\"46\":224,\"47\":237,\"48\":179,\"49\":74,\"50\":84,\"51\":7,\"52\":130,\"53\":96,\"54\":198,\"55\":40,\"56\":23,\"57\":4,\"58\":50,\"59\":62,\"60\":191,\"61\":222,\"62\":119,\"63\":195}}},\"zk_addr\":\"0x71444450505074fe9d9205f02747fb34f49dda22eb33eaf7929bb8561ffd45f2\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"4549343359411304649846201661164616647369749072820883051997393354186530425088\",\"3937997833930688873121017483900547354352969510911658484353113904856895725039\",\"1\"],\"b\":[[\"737118397015523176881783675037843258491735390512712007670938320351154476838\",\"18093386738096496776241258608856280732173952478987786488484944779094702670649\"],[\"17783469782238073070748856104623185946400565050372789961482242728023613389739\",\"15824649467012100671772283318060553156148444804907193757065241285355958322525\"],[\"1\",\"0\"]],\"c\":[\"15112690010634489290938122084488710379345235713605729023472643459768097669053\",\"21568492795931010980780236148561695295582527237009199544419907898465140630575\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AEz89TAIftM8+fFva5Qj4O2zSlQHgmDGKBcEMj6/3nfD\"}";
pub const TEST_AUTH_DATA_6: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjMySWx6VWJuSFY4MV9QTTBJSDBSZmFveVN1TSIsIm5iZiI6MTcxNTY4NzAwMCwiaWF0IjoxNzE1Njg3MzAwLCJleHAiOjE3MTU2OTA5MDAsImp0aSI6ImRhOTU5NmIwMTljOTQ2NWE4MzA0MWIxMDA3OTI2OGU3NDgwY2ZjMDkifQ.HFkMZFhHu6BGBbWhC1NwCvJ9_bKOL8jOdOHuRG21mKh-CaJPffnGtaVNcwEJjf4jOVVPPZNfcJPWOd7KoT_R2Giw7An2dUcJFvVJHUv4h55u4DinU50R7h7ACyEl5GwbKCI-cgxORbcoUdQRukDt1zJHe1eeWm1S8URlE2f4U0w2tPPaE_NmChIRyvU_CjB0dLwxzIWU74pvnbkLSSD2pTWhGbLT1yNhfMTh6yukLyEt2kWvNdZOgGbDfIU6xFjxJLtnPrm5WGiOiWyBmMuDput47-ns4821l3KogdIbWr6TLWW0PMwyJuHnif5pV7wJI9JL5XdFv8KZ0IReAYOIEg\",\"user_pass_to_int_format\":\"1021035256\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":169,\"1\":238,\"2\":219,\"3\":251,\"4\":231,\"5\":87,\"6\":175,\"7\":233,\"8\":185,\"9\":44,\"10\":161,\"11\":207,\"12\":48,\"13\":166,\"14\":79,\"15\":104,\"16\":225,\"17\":53,\"18\":68,\"19\":236,\"20\":49,\"21\":204,\"22\":99,\"23\":208,\"24\":2,\"25\":134,\"26\":101,\"27\":212,\"28\":221,\"29\":142,\"30\":69,\"31\":196},\"secret_key\":{\"0\":94,\"1\":128,\"2\":26,\"3\":130,\"4\":137,\"5\":40,\"6\":61,\"7\":27,\"8\":79,\"9\":58,\"10\":100,\"11\":117,\"12\":200,\"13\":118,\"14\":156,\"15\":202,\"16\":165,\"17\":34,\"18\":238,\"19\":237,\"20\":90,\"21\":63,\"22\":84,\"23\":119,\"24\":86,\"25\":2,\"26\":221,\"27\":177,\"28\":224,\"29\":4,\"30\":233,\"31\":99,\"32\":169,\"33\":238,\"34\":219,\"35\":251,\"36\":231,\"37\":87,\"38\":175,\"39\":233,\"40\":185,\"41\":44,\"42\":161,\"43\":207,\"44\":48,\"45\":166,\"46\":79,\"47\":104,\"48\":225,\"49\":53,\"50\":68,\"51\":236,\"52\":49,\"53\":204,\"54\":99,\"55\":208,\"56\":2,\"57\":134,\"58\":101,\"59\":212,\"60\":221,\"61\":142,\"62\":69,\"63\":196}}},\"zk_addr\":\"0xb1dfac568641e785f1fbd385f43f9ab5751f30e942ffd0618ea3cacf2feb884f\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"12140334820013650239749561964826061158522594132954339836339110630367427672527\",\"21543355833919541708094668850466443067263177907229807762067953508321817783804\",\"1\"],\"b\":[[\"11929519532343982399968491980281874410531815035766070083344475081372092452425\",\"13741260533480647813301201467326069876472210148610447598292633272004546481630\"],[\"14605296808789442404291984821803068302067977919075239981788942874792752578522\",\"20230214791286972912596895174545361255719543417377972941442631629070781210055\"],[\"1\",\"0\"]],\"c\":[\"6046227686259383004231849145260526357580306829730644608118177932582255490991\",\"1343314209137088066016224766407952045954639818725548553059063245802388749310\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AKnu2/vnV6/puSyhzzCmT2jhNUTsMcxj0AKGZdTdjkXE\"}";
pub const TEST_AUTH_DATA_7: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjhJNGF5NDF5THpHWnk4czNCUGR3QzItR2Q5ayIsIm5iZiI6MTcxNTY4Njk5MywiaWF0IjoxNzE1Njg3MjkzLCJleHAiOjE3MTU2OTA4OTMsImp0aSI6ImFjYmJiNmQ3NGY0ZmU3MjMxYzc2ZDQxNzE0ZDM4NDJiNzZlNDU4YzIifQ.nsmqj7tDDv7wJSn47YfaFBXabPYVBjZosGzH_bPHZZToPfvdQyXZrO5CXbaJmojxTPRmzZ2bPI39K9GMX7Y8gaOqk_LYHR7eemVaEj0wNpPPtmUFmHmyrL8nPkTN0a-87L2eu6t7yBZtEiT5e2Jz46RBu9rQL138seOvK3vm0YwhtnLGxhZQnoAKu076qZ_ItlsRn9PqM-sd83bqQoG_SPQVCZL6spWoFunXtj1FeKE-3gRRD8BopORDhFp4xytWDamd1XgIdCNp0a8u7mvElPZCjc3ZUAtFYBWwvfI9r2wN5X4gbNe_pbfpBmgg-2zxwt6c32IhNXlrDQkLxJYqkg\",\"user_pass_to_int_format\":\"9899115100106104\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":35,\"1\":64,\"2\":69,\"3\":29,\"4\":242,\"5\":9,\"6\":183,\"7\":224,\"8\":98,\"9\":254,\"10\":210,\"11\":82,\"12\":213,\"13\":2,\"14\":137,\"15\":66,\"16\":71,\"17\":61,\"18\":80,\"19\":154,\"20\":135,\"21\":100,\"22\":176,\"23\":189,\"24\":187,\"25\":96,\"26\":245,\"27\":194,\"28\":163,\"29\":250,\"30\":15,\"31\":37},\"secret_key\":{\"0\":117,\"1\":94,\"2\":35,\"3\":85,\"4\":116,\"5\":80,\"6\":126,\"7\":55,\"8\":166,\"9\":193,\"10\":94,\"11\":109,\"12\":238,\"13\":86,\"14\":132,\"15\":192,\"16\":225,\"17\":240,\"18\":26,\"19\":65,\"20\":211,\"21\":18,\"22\":195,\"23\":36,\"24\":225,\"25\":158,\"26\":143,\"27\":141,\"28\":21,\"29\":174,\"30\":139,\"31\":13,\"32\":35,\"33\":64,\"34\":69,\"35\":29,\"36\":242,\"37\":9,\"38\":183,\"39\":224,\"40\":98,\"41\":254,\"42\":210,\"43\":82,\"44\":213,\"45\":2,\"46\":137,\"47\":66,\"48\":71,\"49\":61,\"50\":80,\"51\":154,\"52\":135,\"53\":100,\"54\":176,\"55\":189,\"56\":187,\"57\":96,\"58\":245,\"59\":194,\"60\":163,\"61\":250,\"62\":15,\"63\":37}}},\"zk_addr\":\"0x2130548addf21464dba0598e4306193fc658433793260241bd224fa5a186eea1\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"11563763779979887221682129962776185026792805331576343366100386476995832665737\",\"11230623338801856741023013148077980370341441565413488652841279984753971030674\",\"1\"],\"b\":[[\"459996434316864652818633810305056376561329097756558823429320916262609240883\",\"9149790799426074072032368390512074348954812141386022619414187192076850710684\"],[\"21136831034524197906636931934376551157061262869485003235799208746070603082410\",\"7423352680736750974836973800304252036668418183885087029886854244313632685127\"],[\"1\",\"0\"]],\"c\":[\"13616579662900237409901679872544397096722160915603059752960265571802149963290\",\"17724386432768174493966206493099783171212386514205046762827409640509581679264\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ACNARR3yCbfgYv7SUtUCiUJHPVCah2Swvbtg9cKj+g8l\"}";
pub const TEST_AUTH_DATA_8: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjlzWGhqc0xVZlNmdE12bl9iYmQtWkEwZnY5OCIsIm5iZiI6MTcxNTY4Njk4NiwiaWF0IjoxNzE1Njg3Mjg2LCJleHAiOjE3MTU2OTA4ODYsImp0aSI6IjhiNDUzYjUzNTY4MGU3OWZkZWUyOGE3NDVmMzgzMDBkMmNmYjNmODQifQ.a2fpzvW0PxyOcvE8P6WEtIs_mdfTQ9kJb4MIUC5T5uRYJ9ySqSa2qT-MICspGYBuNzCtWIvI6KHY9cIWE2XF3yv7d7gTk_IkhXJud0s5hMhsIxWuNXla_-HducNufaXxXxWYJ2g8dy2xsIMnPr5OC-r4dAX3DM3AchB8qA-RYJdtgwlLytyANp6I35BRT7ewXyDDdlqMLnz5dv4xh1y1wrXFL7VDyzV2XVTK3ap12Cev9IZtHnSGsDgl-vEXj1OYIyiaDgtDhA7rfLXWRTQEeVnRpF-v3AIwZmRu1qaXFoqUbMaSQpFotwb6m8fMQ1q9efOK1Xrv8dL3jBDcUA3w3w\",\"user_pass_to_int_format\":\"98118115106\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":52,\"1\":224,\"2\":199,\"3\":19,\"4\":180,\"5\":128,\"6\":181,\"7\":171,\"8\":55,\"9\":210,\"10\":168,\"11\":100,\"12\":198,\"13\":241,\"14\":150,\"15\":156,\"16\":226,\"17\":233,\"18\":32,\"19\":175,\"20\":153,\"21\":53,\"22\":23,\"23\":58,\"24\":196,\"25\":29,\"26\":16,\"27\":170,\"28\":245,\"29\":46,\"30\":71,\"31\":177},\"secret_key\":{\"0\":47,\"1\":184,\"2\":41,\"3\":167,\"4\":98,\"5\":225,\"6\":50,\"7\":146,\"8\":173,\"9\":129,\"10\":201,\"11\":41,\"12\":181,\"13\":239,\"14\":8,\"15\":249,\"16\":159,\"17\":200,\"18\":159,\"19\":80,\"20\":194,\"21\":79,\"22\":41,\"23\":26,\"24\":200,\"25\":82,\"26\":74,\"27\":200,\"28\":38,\"29\":172,\"30\":84,\"31\":187,\"32\":52,\"33\":224,\"34\":199,\"35\":19,\"36\":180,\"37\":128,\"38\":181,\"39\":171,\"40\":55,\"41\":210,\"42\":168,\"43\":100,\"44\":198,\"45\":241,\"46\":150,\"47\":156,\"48\":226,\"49\":233,\"50\":32,\"51\":175,\"52\":153,\"53\":53,\"54\":23,\"55\":58,\"56\":196,\"57\":29,\"58\":16,\"59\":170,\"60\":245,\"61\":46,\"62\":71,\"63\":177}}},\"zk_addr\":\"0xd704fd1fa5b1d8603b91081d104c08a025e9a952cd6b5b44324fcca2ed432737\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"12104554633236481277286668189930576438264898269322388260846346074721767290773\",\"12396613925509861793005815245783240999567113519994130032649036118285018908597\",\"1\"],\"b\":[[\"1950992742588131071369658940220202257834946772534232957497529743913085624908\",\"13592611568444679350754388983552527571019415309901710535712414143531288069409\"],[\"16680699225604481493782973126773355417557338915104879244979908308676269902149\",\"7242446539394843603528008588061352122030003516933411896066602483137632866329\"],[\"1\",\"0\"]],\"c\":[\"17095909781059243761149234557016161052123209525874162987135833613569429453315\",\"8531296608559822287633863219696197152375138627859243631029781182381653695377\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ADTgxxO0gLWrN9KoZMbxlpzi6SCvmTUXOsQdEKr1Lkex\"}";
pub const TEST_AUTH_DATA_9: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImRLMlRMdjktRXFnaGR3SWQzMzlPNXZmN1JENCIsIm5iZiI6MTcxNTY4Njk3OSwiaWF0IjoxNzE1Njg3Mjc5LCJleHAiOjE3MTU2OTA4NzksImp0aSI6IjQxYTgwZTI3N2U5NzIxYjcwZDkyMjRkMWY5MzRlNjJhYTYwNzcyZGEifQ.NLM6YIR61HOzlEVS1ianwnFoG6OfSeLyuGpjH-Wt7eiWt27fHbDhOWTo-2ysx7cXuAl3gV8ZzMta24QSpjIiiaooGdurX92cWuDcARyewX5_4UuwBWBTXe66irHuqjwIOB2WwyN6PuOwvM6Y_IcL9vPwg76iJoupbeCHXBswiRVzVyBQus1k9SGigU8_ZuwGYoTLPd68MX7Z68NrK7mCF04Xaijs__zwJigIhVOK3TXN2Xy84Ha76mrXJRJZuWErrSNWagVO-dxb2oMT8vm5ND9aJ4q4NaIeGa8PIN2X1cfg9A6LZVBsGIc9JV2FG39yK4T2XAH6tn_HtoMzy_Vuvg\",\"user_pass_to_int_format\":\"10011898115106104\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":78,\"1\":247,\"2\":200,\"3\":7,\"4\":84,\"5\":131,\"6\":33,\"7\":223,\"8\":6,\"9\":241,\"10\":100,\"11\":90,\"12\":91,\"13\":2,\"14\":31,\"15\":23,\"16\":138,\"17\":130,\"18\":115,\"19\":150,\"20\":202,\"21\":79,\"22\":12,\"23\":132,\"24\":168,\"25\":153,\"26\":155,\"27\":131,\"28\":31,\"29\":69,\"30\":170,\"31\":112},\"secret_key\":{\"0\":98,\"1\":144,\"2\":57,\"3\":245,\"4\":40,\"5\":191,\"6\":248,\"7\":149,\"8\":147,\"9\":12,\"10\":229,\"11\":76,\"12\":157,\"13\":3,\"14\":241,\"15\":94,\"16\":134,\"17\":124,\"18\":226,\"19\":177,\"20\":31,\"21\":140,\"22\":224,\"23\":58,\"24\":57,\"25\":95,\"26\":235,\"27\":246,\"28\":120,\"29\":89,\"30\":33,\"31\":149,\"32\":78,\"33\":247,\"34\":200,\"35\":7,\"36\":84,\"37\":131,\"38\":33,\"39\":223,\"40\":6,\"41\":241,\"42\":100,\"43\":90,\"44\":91,\"45\":2,\"46\":31,\"47\":23,\"48\":138,\"49\":130,\"50\":115,\"51\":150,\"52\":202,\"53\":79,\"54\":12,\"55\":132,\"56\":168,\"57\":153,\"58\":155,\"59\":131,\"60\":31,\"61\":69,\"62\":170,\"63\":112}}},\"zk_addr\":\"0x4493e2aab6fcd5d7259e066291ed6f42f6e0b732ecbd38bbaf8a98546a7d0cba\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"6187760498712900389422022394560825973187662358740291343829568808375698843239\",\"3663904360488418820404220406786944885702547623862334490191838865255632801941\",\"1\"],\"b\":[[\"17208058907245387104889127891010282196539728213379257608213444054211064433036\",\"9822512703540345824827246410723992174766686970531763618190197664729418117984\"],[\"9555481236549941306688205540885297760448987185399187813240300069134845655152\",\"17967781633941820778916846359708064205041390458485667635199415296702341964940\"],[\"1\",\"0\"]],\"c\":[\"12374452924342055287727719327288397498526425907741014437332085255604038084453\",\"7084903967634108603521121616612807600817728267672878238097194166039392876060\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AE73yAdUgyHfBvFkWlsCHxeKgnOWyk8MhKiZm4MfRapw\"}";
pub const TEST_AUTH_DATA_10: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6InZjdk1UaEZFdjFzVTBGVGR1M3Z5TmxlVUd0MCIsIm5iZiI6MTcxNTY4Njk3MiwiaWF0IjoxNzE1Njg3MjcyLCJleHAiOjE3MTU2OTA4NzIsImp0aSI6ImFiMDMwNTMzZjQyOWJmOGFhMDNmYzZhYjAxZjg2MGQ3MTg5ZDBlNjkifQ.f11q7mTu1uScsGvj4-KgVHHEhfAqk53JbAIC0PT8-CU40D4fSWbBoyXrUUQw6zly4KsyqazAFJ_1JqjiFvYFOhCAsoGWpgiA-hnL4QK-uxqUV4ule7Wt9xs8QVPivYxTrK2jmDgPGosvTUmlrGeyZk2XwilO3mbTe5wN-zMkUF0zUTdlIBTPrKbXMS1PklWTjUgDa1bXb-hOaFILkfZ4UgQI3PYHjZul3Rm_UUHHHVRkLgt0M449CGjuKSsIFvVkslfL319_71DLo7W0sYkJkWGOa482vTvyHgR9SjalUPV4TzPhpe_6DZZlKna7MXgq4FWOS9710PC6_HAXF2n-ag\",\"user_pass_to_int_format\":\"11898100104102106115\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":146,\"1\":239,\"2\":26,\"3\":188,\"4\":228,\"5\":23,\"6\":17,\"7\":118,\"8\":183,\"9\":248,\"10\":93,\"11\":219,\"12\":0,\"13\":213,\"14\":164,\"15\":161,\"16\":140,\"17\":200,\"18\":97,\"19\":183,\"20\":135,\"21\":18,\"22\":103,\"23\":137,\"24\":234,\"25\":122,\"26\":246,\"27\":20,\"28\":155,\"29\":72,\"30\":212,\"31\":15},\"secret_key\":{\"0\":107,\"1\":202,\"2\":67,\"3\":226,\"4\":108,\"5\":41,\"6\":149,\"7\":181,\"8\":238,\"9\":3,\"10\":97,\"11\":189,\"12\":216,\"13\":94,\"14\":143,\"15\":210,\"16\":192,\"17\":213,\"18\":224,\"19\":200,\"20\":253,\"21\":67,\"22\":168,\"23\":88,\"24\":140,\"25\":106,\"26\":235,\"27\":247,\"28\":54,\"29\":146,\"30\":251,\"31\":123,\"32\":146,\"33\":239,\"34\":26,\"35\":188,\"36\":228,\"37\":23,\"38\":17,\"39\":118,\"40\":183,\"41\":248,\"42\":93,\"43\":219,\"44\":0,\"45\":213,\"46\":164,\"47\":161,\"48\":140,\"49\":200,\"50\":97,\"51\":183,\"52\":135,\"53\":18,\"54\":103,\"55\":137,\"56\":234,\"57\":122,\"58\":246,\"59\":20,\"60\":155,\"61\":72,\"62\":212,\"63\":15}}},\"zk_addr\":\"0xb86a18deea59af2850ab3800e2d46f63cfbea3bae309359089945d55949aef84\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"2575938484642353074459611431508941853614856803645537593538048270397701877180\",\"18525747234426619072147704335372433454079655655225636793928970068265541595508\",\"1\"],\"b\":[[\"5146896444986257903458872614168031344366471557324420746422302593221564486610\",\"19134791144810013840937258347062701987554745426617919650818846823708095832550\"],[\"3133101512761334334340993079649721452024653991833325456466256722050883608250\",\"21877263483512108853787895465249721341909931993800128255134630466114688578666\"],[\"1\",\"0\"]],\"c\":[\"3069457366306376197755607218741517434199413283376424243014529567457206056402\",\"4929625283757609606431630951067242799347282963225969540629139985267066740824\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AJLvGrzkFxF2t/hd2wDVpKGMyGG3hxJniep69hSbSNQP\"}";
pub const TEST_AUTH_DATA_11: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkJ4RzlwNFhoV1B4WEt3NVRRdG9FbDljZzI5NCIsIm5iZiI6MTcxNTY4Njk0MSwiaWF0IjoxNzE1Njg3MjQxLCJleHAiOjE3MTU2OTA4NDEsImp0aSI6IjMyYmMxN2VjNWY3ZTQ5ZWRkOTM2MzVkZjE5MDk2N2E3NTg5Y2ZmNTgifQ.w3cT9MVhKTvnmAlmKClFFG6hjB2zrwHonYuN6l5S2unwyR6P_tGE42KhaFSNCY-imysy8k42awfmAafXwftKClLvqzk1T6bi5Li6caVd6-la8wj_FxNWkE5Cy-N4grOiEYJtV5SZezFzifmL6LOstv-Nc4X2b9Z6utuGOWYq3W9LNPveD0v5GnBCR6JRtHJkI6e5yZnMwDDE5o1P-LZbGuFXP75P6jseGem956the_WbrwIsnnTdFgjgjbXn_1gkh4SYGQ1ig0NVKcs75hUhKuQi7V6VqycuyXTgACOCsIfh2guoKha-APZUeul3z33zNbsqUcgkWwl6CkvDSdGWiQ\",\"user_pass_to_int_format\":\"1145652515748\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":153,\"1\":152,\"2\":146,\"3\":133,\"4\":135,\"5\":137,\"6\":8,\"7\":27,\"8\":197,\"9\":109,\"10\":12,\"11\":221,\"12\":49,\"13\":15,\"14\":10,\"15\":1,\"16\":64,\"17\":236,\"18\":222,\"19\":97,\"20\":181,\"21\":214,\"22\":200,\"23\":214,\"24\":130,\"25\":247,\"26\":204,\"27\":212,\"28\":49,\"29\":33,\"30\":169,\"31\":172},\"secret_key\":{\"0\":159,\"1\":96,\"2\":35,\"3\":206,\"4\":32,\"5\":121,\"6\":5,\"7\":32,\"8\":37,\"9\":203,\"10\":15,\"11\":252,\"12\":99,\"13\":107,\"14\":57,\"15\":211,\"16\":139,\"17\":123,\"18\":6,\"19\":233,\"20\":56,\"21\":15,\"22\":35,\"23\":224,\"24\":243,\"25\":148,\"26\":44,\"27\":114,\"28\":112,\"29\":161,\"30\":226,\"31\":255,\"32\":153,\"33\":152,\"34\":146,\"35\":133,\"36\":135,\"37\":137,\"38\":8,\"39\":27,\"40\":197,\"41\":109,\"42\":12,\"43\":221,\"44\":49,\"45\":15,\"46\":10,\"47\":1,\"48\":64,\"49\":236,\"50\":222,\"51\":97,\"52\":181,\"53\":214,\"54\":200,\"55\":214,\"56\":130,\"57\":247,\"58\":204,\"59\":212,\"60\":49,\"61\":33,\"62\":169,\"63\":172}}},\"zk_addr\":\"0x41c25944949f0e3bf80fea41d9ab27acfa26e0b25ecd7e468235b2284e5b0c09\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"10607121052143170357142710430122120898934487918266599021712929788471219763472\",\"13359690919698524885136984693561112109891470903700041135375248695741012306373\",\"1\"],\"b\":[[\"3247989990207989646120856507929936403874972366284220250880918537588838028173\",\"20347831818628957019286012207626379731554938194907710010892594024137236752987\"],[\"18217798786390957788883983024823206348636485136705276787854998111125834676541\",\"11824109578691812603938426242725149605448845948255194504928078330266973720614\"],[\"1\",\"0\"]],\"c\":[\"16499583001208064509247079494271177710897656329498349773613236383353749984739\",\"1944718879141050229961827816471755841829876012643055740792265283564642185697\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AJmYkoWHiQgbxW0M3TEPCgFA7N5htdbI1oL3zNQxIams\"}";
pub const TEST_AUTH_DATA_12: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IndNcWhEa3BxQllkSlowQVBwXzUtbVZDLUU2OCIsIm5iZiI6MTcxNTY4NjkzMywiaWF0IjoxNzE1Njg3MjMzLCJleHAiOjE3MTU2OTA4MzMsImp0aSI6IjEwNGRhZTE1ZWMwODRjODU3MzBjYTRiZGI1MThiZTkwZDdkMTQ3NmEifQ.KBzhI2UOTstRFpgkZiFFlCmhy-E0PwoWdfWhXem6Kr0HjOgCfr-a5TGVRyMf0b7-Tnf712tMPf4N7-uPSoyaBsmtiYmAudj8whha2obUVhzWjghiURrbYkiCBWys5Z4v3SnVKDqXPsUFmNucBSA3l6DIWbhLT4WqTszGY-Qc_cKhR-7y5i3t90lhGNmwrvCR72jAXaF-xbBvsaiMXxhfCS5fnMFNRibIE3tRx1r3mkx59etA8E3xQAu8LPzFyC0ecEKL0K6a5ZWNWBFPbGSzAhSK9D3ak1gzON6rhccCPpLRErk2MIhUQq4HBnOywg5Lf1w0onxhkJtU6docO2VVAA\",\"user_pass_to_int_format\":\"11099104102117\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":173,\"1\":37,\"2\":48,\"3\":14,\"4\":54,\"5\":38,\"6\":225,\"7\":52,\"8\":254,\"9\":178,\"10\":32,\"11\":56,\"12\":162,\"13\":128,\"14\":135,\"15\":55,\"16\":10,\"17\":222,\"18\":131,\"19\":175,\"20\":166,\"21\":161,\"22\":145,\"23\":219,\"24\":44,\"25\":231,\"26\":183,\"27\":245,\"28\":141,\"29\":178,\"30\":237,\"31\":92},\"secret_key\":{\"0\":108,\"1\":38,\"2\":149,\"3\":222,\"4\":132,\"5\":184,\"6\":128,\"7\":164,\"8\":27,\"9\":101,\"10\":217,\"11\":92,\"12\":24,\"13\":245,\"14\":209,\"15\":31,\"16\":88,\"17\":174,\"18\":237,\"19\":144,\"20\":78,\"21\":127,\"22\":73,\"23\":195,\"24\":194,\"25\":229,\"26\":208,\"27\":176,\"28\":220,\"29\":60,\"30\":229,\"31\":253,\"32\":173,\"33\":37,\"34\":48,\"35\":14,\"36\":54,\"37\":38,\"38\":225,\"39\":52,\"40\":254,\"41\":178,\"42\":32,\"43\":56,\"44\":162,\"45\":128,\"46\":135,\"47\":55,\"48\":10,\"49\":222,\"50\":131,\"51\":175,\"52\":166,\"53\":161,\"54\":145,\"55\":219,\"56\":44,\"57\":231,\"58\":183,\"59\":245,\"60\":141,\"61\":178,\"62\":237,\"63\":92}}},\"zk_addr\":\"0xe5433ade6e56883e0cc13044783fc6e0d835db866e8ef69d305622f4dbfd7730\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"20315021530892971959830664693110327999639349964485536174303351139810441711270\",\"9363226245552972448215999928614529638129956136095863617353608229521342156596\",\"1\"],\"b\":[[\"13215029653817105228530429395766730210769586389024965762310641194113200165202\",\"7799676398333409903573594921069872917500921399080042730183754684502821618481\"],[\"13048821293399627652827197503115267831066766008561767009809325017447715880491\",\"331361016081752781071859245948286166830568341165278760117629920699739892753\"],[\"1\",\"0\"]],\"c\":[\"7347702391542317289078324477957712035210582056186479239076715504548941012834\",\"795883936884678581860170407596096541519605830081875833581950897247827301651\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AK0lMA42JuE0/rIgOKKAhzcK3oOvpqGR2yznt/WNsu1c\"}";
pub const TEST_AUTH_DATA_13: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjVqNVBySXFpYW9xaUNFWkR0eklNazY2MUotRSIsIm5iZiI6MTcxNTY4NjkyNCwiaWF0IjoxNzE1Njg3MjI0LCJleHAiOjE3MTU2OTA4MjQsImp0aSI6IjJmYzk0MmM1MDBiMmJmNGE5YzZiZjUwN2Y0MjU4NTg3MGM4YmQ5N2QifQ.GU70HImKkqZyGmWAC_onzc-ccUhALeT7ebQ0LrE0QGqjCZyCnonjOeDhatB4Q1GQCVQ-KPWKCdg4NNPCPvKwLYAjwNF0sorwS5h6jKKVvRgT_t12dbDzrPKJE7xW0_0kfmfj7lKGZp_W4HNVxd_hlPiwJb56X0ZVkt3pwpkwBe8MU-Nzb3QyrJtDRJDDb4v_bVdOJSyUNEtssFvAgFB4diGI_GFQzZpbQnBeciST-lS7rGHpItnlwe0mRNf3e34S7A7wUOo_YTvy-TKTViSekMdkMKt9hgGkti9c4dYwI8NMExe4wtnLFVOh6XZ0FtrdnVGrYZFMWTJjNGizUmFMZQ\",\"user_pass_to_int_format\":\"525451555057114102\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":221,\"1\":11,\"2\":223,\"3\":171,\"4\":2,\"5\":140,\"6\":112,\"7\":100,\"8\":233,\"9\":182,\"10\":68,\"11\":219,\"12\":126,\"13\":215,\"14\":96,\"15\":164,\"16\":201,\"17\":227,\"18\":132,\"19\":169,\"20\":157,\"21\":120,\"22\":187,\"23\":16,\"24\":40,\"25\":208,\"26\":174,\"27\":209,\"28\":89,\"29\":163,\"30\":255,\"31\":62},\"secret_key\":{\"0\":5,\"1\":6,\"2\":91,\"3\":164,\"4\":51,\"5\":203,\"6\":161,\"7\":246,\"8\":61,\"9\":156,\"10\":92,\"11\":96,\"12\":69,\"13\":141,\"14\":93,\"15\":73,\"16\":208,\"17\":85,\"18\":37,\"19\":52,\"20\":167,\"21\":121,\"22\":63,\"23\":221,\"24\":215,\"25\":165,\"26\":48,\"27\":232,\"28\":136,\"29\":10,\"30\":71,\"31\":92,\"32\":221,\"33\":11,\"34\":223,\"35\":171,\"36\":2,\"37\":140,\"38\":112,\"39\":100,\"40\":233,\"41\":182,\"42\":68,\"43\":219,\"44\":126,\"45\":215,\"46\":96,\"47\":164,\"48\":201,\"49\":227,\"50\":132,\"51\":169,\"52\":157,\"53\":120,\"54\":187,\"55\":16,\"56\":40,\"57\":208,\"58\":174,\"59\":209,\"60\":89,\"61\":163,\"62\":255,\"63\":62}}},\"zk_addr\":\"0x0934ba96e39b32a66b83afdd089d9534b91336d0c72324acb72b718e2d8adcd8\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"3292113297742390468701372446942400025026948502434627571571387058022780524172\",\"4608365882159831859997420943605862565647863626478617897572911626264555729258\",\"1\"],\"b\":[[\"5662407938030293510048180382159430467791189346676904212329490391470516566946\",\"14655907382794614210872210515582570998106075620115645016125280695488094003217\"],[\"3337061425406207163991320131711738442766654603337106758166291266688030689117\",\"4469383376673348053098454774700074508703514397281065469277327859575940584146\"],[\"1\",\"0\"]],\"c\":[\"6592007510647447256322156763481821378802835999285873915184749854236303252416\",\"16208563039085392733361585085996378606127672981771155339865393880548209917912\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AN0L36sCjHBk6bZE237XYKTJ44SpnXi7ECjQrtFZo/8+\"}";
pub const TEST_AUTH_DATA_14: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjN6eHc2SUFERzVXcjJDR244SGczS1Q3Nm1qOCIsIm5iZiI6MTcxNTY4NjkwNSwiaWF0IjoxNzE1Njg3MjA1LCJleHAiOjE3MTU2OTA4MDUsImp0aSI6IjNkOTM4NmFlODMxZDVhYjdiZTI1NjcxMDhmZjhkMTM1N2YzNDZjOTUifQ.R3s_OfTiDlMMSFsEfp4xM6rLoJ99GALalEE1TVG8aneruEWuI1qxz241YmX9r9-49t1ja5BfO0eh3Fu_p6lg1O32sNSLR626Mvrv1Ph60syPQN01Tam4RCV_YBK3b2Pj-rWeJq3WSCGQg2rab2QyHy3Al9VPdXlkbaaH69QzRSXFyNojixgo92cPhABxbAxI1a5pYmzwwfkDDO0FY5uRUt3w4wuBhx9gQ6g_kboF03pIzQ5kvGUYBPGax66faTzulAGdTADmU9xgG6denQoZWn3Lh6dfdQX8KXkn9jVY8gMIY_rbobc8nkIMmslsjjio7BXb90-YD_WJT5so5Cre3A\",\"user_pass_to_int_format\":\"1031021041155552\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":97,\"1\":100,\"2\":35,\"3\":169,\"4\":212,\"5\":9,\"6\":238,\"7\":108,\"8\":186,\"9\":80,\"10\":106,\"11\":26,\"12\":209,\"13\":87,\"14\":84,\"15\":117,\"16\":235,\"17\":25,\"18\":81,\"19\":248,\"20\":137,\"21\":197,\"22\":146,\"23\":139,\"24\":214,\"25\":127,\"26\":143,\"27\":179,\"28\":137,\"29\":79,\"30\":181,\"31\":216},\"secret_key\":{\"0\":70,\"1\":76,\"2\":130,\"3\":97,\"4\":75,\"5\":0,\"6\":7,\"7\":122,\"8\":166,\"9\":56,\"10\":85,\"11\":179,\"12\":143,\"13\":55,\"14\":136,\"15\":47,\"16\":75,\"17\":211,\"18\":125,\"19\":145,\"20\":130,\"21\":206,\"22\":118,\"23\":212,\"24\":87,\"25\":200,\"26\":130,\"27\":38,\"28\":65,\"29\":93,\"30\":37,\"31\":44,\"32\":97,\"33\":100,\"34\":35,\"35\":169,\"36\":212,\"37\":9,\"38\":238,\"39\":108,\"40\":186,\"41\":80,\"42\":106,\"43\":26,\"44\":209,\"45\":87,\"46\":84,\"47\":117,\"48\":235,\"49\":25,\"50\":81,\"51\":248,\"52\":137,\"53\":197,\"54\":146,\"55\":139,\"56\":214,\"57\":127,\"58\":143,\"59\":179,\"60\":137,\"61\":79,\"62\":181,\"63\":216}}},\"zk_addr\":\"0x87b9236aadcbc8de1a2bce17bb104cbae2f8c955f89808ee2d258cf2bc1cce1f\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"18415333747085688267133796133445868671450647215619171648016630248725573444572\",\"13021999644739913954648136527237689315935942107782566659768353668730521796833\",\"1\"],\"b\":[[\"10379715945772584677584721710592153467187645980157575584584703890180885281296\",\"21114541349211062821701871386552875726196087055162878583823021987759476907947\"],[\"21741245524391086016724288544952241247835975701957615054057894483829435111137\",\"19675246006347690391662817422022652459552259504790883596539945355325572896761\"],[\"1\",\"0\"]],\"c\":[\"6388980351498388564470364481867721519510272532387680761911853865824806443040\",\"2927953057998420964296253396822428516251336255094433794401337892358172944522\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AGFkI6nUCe5sulBqGtFXVHXrGVH4icWSi9Z/j7OJT7XY\"}";
pub const TEST_AUTH_DATA_15: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjRtbk1TajFjTmdlSUU4Unk3Z2dBOWNRVWRLSSIsIm5iZiI6MTcxNTY4Njg3MywiaWF0IjoxNzE1Njg3MTczLCJleHAiOjE3MTU2OTA3NzMsImp0aSI6ImVmYzU3YjVmMGEzZmEwZGI2ZTQzNWFhZjI4OTEwNjY2YjM0NWViNmEifQ.YRFPl_szPP8iBid__ACAj4Etr4YDZEmeawFTas_MFw7rR_sD_tQ268F2g9O4VOU3VWSWT-LCG1gp_NdRVvb5SFBzuMIYp4YrUEvzJdaO_ab1a2Xp_EVVEmjMwNHVpnFZjS9El0e0oOmaw_PQgC2soauJkfLvRhayx-_Vps7htHm94PW1aHBOxwr2HpR58mjzT4JyutyiioCLgLqhnvGW4N6CBlx6iLNfITk0wwsAOHRcdjW_hk0hHarjMy3U2VdbcPkmq1OIg8ZDQo2jbUGEWevUC6zrGeNWYjp38f3Wo1NUqf7_ne0YeJEBtyK5r9BuDxdr6YRyUXKnpxJpr9cZ-Q\",\"user_pass_to_int_format\":\"5256515057\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":226,\"1\":91,\"2\":63,\"3\":50,\"4\":152,\"5\":120,\"6\":233,\"7\":249,\"8\":177,\"9\":205,\"10\":1,\"11\":233,\"12\":153,\"13\":199,\"14\":101,\"15\":124,\"16\":112,\"17\":15,\"18\":160,\"19\":228,\"20\":124,\"21\":169,\"22\":57,\"23\":196,\"24\":118,\"25\":117,\"26\":94,\"27\":132,\"28\":228,\"29\":108,\"30\":145,\"31\":117},\"secret_key\":{\"0\":168,\"1\":100,\"2\":5,\"3\":144,\"4\":15,\"5\":220,\"6\":219,\"7\":42,\"8\":52,\"9\":1,\"10\":7,\"11\":203,\"12\":43,\"13\":71,\"14\":99,\"15\":90,\"16\":8,\"17\":66,\"18\":137,\"19\":155,\"20\":200,\"21\":27,\"22\":69,\"23\":112,\"24\":209,\"25\":173,\"26\":109,\"27\":93,\"28\":152,\"29\":210,\"30\":96,\"31\":194,\"32\":226,\"33\":91,\"34\":63,\"35\":50,\"36\":152,\"37\":120,\"38\":233,\"39\":249,\"40\":177,\"41\":205,\"42\":1,\"43\":233,\"44\":153,\"45\":199,\"46\":101,\"47\":124,\"48\":112,\"49\":15,\"50\":160,\"51\":228,\"52\":124,\"53\":169,\"54\":57,\"55\":196,\"56\":118,\"57\":117,\"58\":94,\"59\":132,\"60\":228,\"61\":108,\"62\":145,\"63\":117}}},\"zk_addr\":\"0xc2e01f23756fd4fc3e8ee98f96751729c911acc1b8abc4e5d8f732a0b6a69602\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"6734924940747627006546678824977458478287976951246203795880487352562116664933\",\"14763323532227801517600705873776227782564830701170466315373208681644431001874\",\"1\"],\"b\":[[\"19846719805329609703868726781640931109590522837532762309497922458996335263239\",\"15420764526732603133646176483042915906318651960194518136943858294265541434918\"],[\"3657954841783806502381750774780312041530173171470043250309926815017975476219\",\"3502207265482905042029962996793932548717468210237619905023157797841132512624\"],[\"1\",\"0\"]],\"c\":[\"1288521393482492105362792882426011805774869298603270001189992299082351112997\",\"3336108234609612516660580995781529303851605528785003185796473743343393403477\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AOJbPzKYeOn5sc0B6ZnHZXxwD6DkfKk5xHZ1XoTkbJF1\"}";
pub const TEST_AUTH_DATA_16: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImpKS0poeXJtbHE3T1UxU0NwZnlfX2F3MEZxVSIsIm5iZiI6MTcxNTY4Njg2NSwiaWF0IjoxNzE1Njg3MTY1LCJleHAiOjE3MTU2OTA3NjUsImp0aSI6Ijg4ZDVmNTg1OGMzNDIwMmY1OTAyOWE5ODM4YzhhOWMzYWVlMDZjNDMifQ.cjuamUo90ycOmkGffs4qe6Ozb0q-UhG6oG4pLf3a5zMgRUXr_PcKNj9GcHujqYzWFbVsiuYdoVwMmPsHeKmLnkuIDS4mwT0z-LhWvYrXdx2FksXyv0ECIBJNGHWNtf6JyhA_3XGYSqzn4sQncKxHK82aFAJZYaPfXCgKJJK0c9PFjONxY2nQoDV-IM89vm6x9vpNPjYxMxxE60p_5qceLLU9pgy4jgP2Eyco0sGfCFTry7zVqgYsMSinh_UIWk4naihDtgrZxAdNkoAA-4PQkWxrlTO8b68YQp8K4ncerUwmOJDs-0NUxDm8mTjwq47Qgf_UAcTxVN9_YpIqEoxdwQ\",\"user_pass_to_int_format\":\"100102104119101105101121102\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":109,\"1\":20,\"2\":190,\"3\":101,\"4\":236,\"5\":16,\"6\":171,\"7\":49,\"8\":222,\"9\":170,\"10\":22,\"11\":241,\"12\":224,\"13\":116,\"14\":18,\"15\":124,\"16\":48,\"17\":1,\"18\":20,\"19\":126,\"20\":94,\"21\":16,\"22\":164,\"23\":173,\"24\":180,\"25\":226,\"26\":71,\"27\":184,\"28\":218,\"29\":162,\"30\":145,\"31\":87},\"secret_key\":{\"0\":131,\"1\":117,\"2\":15,\"3\":104,\"4\":243,\"5\":100,\"6\":1,\"7\":157,\"8\":31,\"9\":54,\"10\":163,\"11\":215,\"12\":45,\"13\":202,\"14\":70,\"15\":51,\"16\":77,\"17\":200,\"18\":206,\"19\":59,\"20\":210,\"21\":59,\"22\":129,\"23\":250,\"24\":53,\"25\":166,\"26\":201,\"27\":57,\"28\":9,\"29\":13,\"30\":255,\"31\":18,\"32\":109,\"33\":20,\"34\":190,\"35\":101,\"36\":236,\"37\":16,\"38\":171,\"39\":49,\"40\":222,\"41\":170,\"42\":22,\"43\":241,\"44\":224,\"45\":116,\"46\":18,\"47\":124,\"48\":48,\"49\":1,\"50\":20,\"51\":126,\"52\":94,\"53\":16,\"54\":164,\"55\":173,\"56\":180,\"57\":226,\"58\":71,\"59\":184,\"60\":218,\"61\":162,\"62\":145,\"63\":87}}},\"zk_addr\":\"0x3a26feb6fa552d6e2796e37cfcfaa19ff8d09b9b3e30060557f313ec82e7809a\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"17553248964460513660899064860794334313854327380740726458707730696116622715951\",\"19780935404993030841853448182973442738138094386982905919654095638585438825727\",\"1\"],\"b\":[[\"21560192083940754229490187081097411180154947135453319375957763951829010741758\",\"19864576266509862087012277908356289924851686435133221538927641836878678315039\"],[\"5332198541444016097635381835036279771892300735490162251066050727152100828695\",\"4562785582599067136108384927870755899035073041220030123445496806313655366742\"],[\"1\",\"0\"]],\"c\":[\"17180793399699270264610473764500109290307106335241771936808740744446379111802\",\"19531923144281240440166451089649574065952850605237982747209921274042428958350\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AG0UvmXsEKsx3qoW8eB0EnwwARR+XhCkrbTiR7jaopFX\"}";
pub const TEST_AUTH_DATA_17: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkxPYUk4MkJHS1lSWXVETVRMVXU4Z0ktV3ZsayIsIm5iZiI6MTcxNTY4Njg1NywiaWF0IjoxNzE1Njg3MTU3LCJleHAiOjE3MTU2OTA3NTcsImp0aSI6ImMyOGE2MTEwYzMwYmQ0ZDkzMmEwOTNlOWVmZjllNzEyZjUzYWI2MjQifQ.fHYn7sAFbOtPneSX_YBA52ASidwosnl42uWF7RmroUU132sPO3Jmzf7tqZrqQFu04Y1G2LeGvTeHklUowVdWdKQkomV9bCeputcMRkDPD9-5-UJdpDY8eIAzfHzWN9nWyu5St0Iz0S0FIth6cesMmPUkCrq6pCUyHLWgrxUoICuYIbbtEO5ZVnF8lIeMjUTLXT4_9svFBRhugkD0nvHnQnWS8H0ijS53lCs8z7xVy0cm_MawsCMpApMQvWm-4CeIq69p3m2HXclXNmwSxg7oeGDKn-yqhPaXX3Pn4PfHKPj-XHXOR2rr9uG2lYi73yOyDve84wCXzV9kmiUnc0YGUQ\",\"user_pass_to_int_format\":\"515253575654\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":100,\"1\":216,\"2\":222,\"3\":171,\"4\":27,\"5\":27,\"6\":171,\"7\":132,\"8\":172,\"9\":13,\"10\":174,\"11\":188,\"12\":196,\"13\":208,\"14\":35,\"15\":125,\"16\":10,\"17\":214,\"18\":5,\"19\":29,\"20\":118,\"21\":41,\"22\":114,\"23\":70,\"24\":166,\"25\":37,\"26\":189,\"27\":136,\"28\":37,\"29\":106,\"30\":245,\"31\":15},\"secret_key\":{\"0\":75,\"1\":60,\"2\":159,\"3\":196,\"4\":243,\"5\":180,\"6\":224,\"7\":198,\"8\":228,\"9\":147,\"10\":22,\"11\":104,\"12\":69,\"13\":182,\"14\":80,\"15\":232,\"16\":127,\"17\":195,\"18\":43,\"19\":2,\"20\":99,\"21\":206,\"22\":161,\"23\":47,\"24\":106,\"25\":44,\"26\":131,\"27\":5,\"28\":133,\"29\":110,\"30\":82,\"31\":140,\"32\":100,\"33\":216,\"34\":222,\"35\":171,\"36\":27,\"37\":27,\"38\":171,\"39\":132,\"40\":172,\"41\":13,\"42\":174,\"43\":188,\"44\":196,\"45\":208,\"46\":35,\"47\":125,\"48\":10,\"49\":214,\"50\":5,\"51\":29,\"52\":118,\"53\":41,\"54\":114,\"55\":70,\"56\":166,\"57\":37,\"58\":189,\"59\":136,\"60\":37,\"61\":106,\"62\":245,\"63\":15}}},\"zk_addr\":\"0x89045412e3f5c808e7bf0ea6d47008a6c75f14b48a7fea54f420b03d3298ef4e\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"257137064185145448465242836924275827726618300508610920187643303682623341809\",\"13136104385342155450873138185971548464179081219858145555616259274682267182602\",\"1\"],\"b\":[[\"9623367967771069752036280248035047299371597257306258748768218269896381701321\",\"12210765432002064938981141260402135327544184192147240766501813387730760651726\"],[\"15251118264052002493837427778759923199895437430037469672801786148252966111936\",\"12243121821747384937988506024826071890328897029202152518609157933400978560340\"],[\"1\",\"0\"]],\"c\":[\"4201350126073080124441494110984461007902792066210632786985402248838880518314\",\"10425614983366289743736253875955608779721351186796918402238008669517994775682\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AGTY3qsbG6uErA2uvMTQI30K1gUddilyRqYlvYglavUP\"}";
pub const TEST_AUTH_DATA_18: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IlhhakktZ0NqbFVVY3ZBTFdmcDlyTURCelBzVSIsIm5iZiI6MTcxNTY4Njg0OSwiaWF0IjoxNzE1Njg3MTQ5LCJleHAiOjE3MTU2OTA3NDksImp0aSI6IjZmYmI3NmEzN2NjOTAwYTQ5NThlYmNlZmNmYmVhNjMzMzkyMzM2OTUifQ.RqLBHMZMuXbsZGW5YGDNbfTSGG5Ezv_XtJRvMbBIXytAqGoT70RrfZSwU3e8yXaq-o4RBoeypQIygj_Sjxq0JJXVRuypVqkbismASkWKWH77avFgRUe0Etvc8EFXupmwj1biRpURUukroVUyjktOI17m3DvFIIan7_rq3SQBNxLyjFZav517zaJaUVXdYMDAYIEVs1Es04G2kWTxBYQ6iu0jyHtuNcg9_kosGQEZjnp2HsnvegrRwloyjuFByMRv90bRuV6cc3f-3GPO23tcrhFzeoOQXUfcSdlqE3C92gb6E_3uBld414mNj2LelnagKtpvPjTCgX3tic2c7fB_CQ\",\"user_pass_to_int_format\":\"989911510011710554\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":81,\"1\":243,\"2\":172,\"3\":238,\"4\":183,\"5\":132,\"6\":17,\"7\":7,\"8\":200,\"9\":125,\"10\":73,\"11\":248,\"12\":71,\"13\":220,\"14\":159,\"15\":138,\"16\":16,\"17\":207,\"18\":25,\"19\":103,\"20\":70,\"21\":23,\"22\":193,\"23\":72,\"24\":27,\"25\":94,\"26\":241,\"27\":155,\"28\":98,\"29\":155,\"30\":212,\"31\":118},\"secret_key\":{\"0\":89,\"1\":13,\"2\":132,\"3\":115,\"4\":95,\"5\":59,\"6\":196,\"7\":68,\"8\":136,\"9\":46,\"10\":22,\"11\":70,\"12\":5,\"13\":188,\"14\":76,\"15\":116,\"16\":156,\"17\":15,\"18\":226,\"19\":232,\"20\":167,\"21\":204,\"22\":143,\"23\":148,\"24\":230,\"25\":69,\"26\":18,\"27\":166,\"28\":234,\"29\":47,\"30\":178,\"31\":31,\"32\":81,\"33\":243,\"34\":172,\"35\":238,\"36\":183,\"37\":132,\"38\":17,\"39\":7,\"40\":200,\"41\":125,\"42\":73,\"43\":248,\"44\":71,\"45\":220,\"46\":159,\"47\":138,\"48\":16,\"49\":207,\"50\":25,\"51\":103,\"52\":70,\"53\":23,\"54\":193,\"55\":72,\"56\":27,\"57\":94,\"58\":241,\"59\":155,\"60\":98,\"61\":155,\"62\":212,\"63\":118}}},\"zk_addr\":\"0xa41d812b2137a9e701512dd1e77643b94c3eff566c5be50365d174d1db60a415\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"17614076016833085587577424708926810169376585820815427961767207039192652769013\",\"14135032500684844024372302628625185135058981148802759791768119531914068811069\",\"1\"],\"b\":[[\"12223738407851653989769057205672802523120105196291312843502064346721414495287\",\"633499823246797838323329834422571844397737716575556571535890211670105511423\"],[\"6003190178099558462113377195569012506764289704920013648574803015959961275195\",\"2773541228770509456407096233964565540804779880988894871588471857181885931620\"],[\"1\",\"0\"]],\"c\":[\"1069242590881057236271046634996302431045048055413724998035360474439616694142\",\"4170832142623397447640837445675045579300900045561776497865454715900704844006\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AFHzrO63hBEHyH1J+Efcn4oQzxlnRhfBSBte8Ztim9R2\"}";
pub const TEST_AUTH_DATA_19: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkxyN2JvekNHNUw4UHlLVVNPRzNwMDZYYVM1USIsIm5iZiI6MTcxNTY4Njg0MSwiaWF0IjoxNzE1Njg3MTQxLCJleHAiOjE3MTU2OTA3NDEsImp0aSI6Ijc2MzdiMGMyM2ZiMWIwZTFjMGEyYWE3NWVkOGMxNjA1YzE1YWFiZjAifQ.uNMFOgl9xdG5wljwZrIDzWm3SS_F9OLhR9avDGRhHSxYNSzexcOHtGT7HY9zsWloN9LWFZxu2t3yG-jWduo5qYgyM-OXpdAXLzfXZwQSNxgtXl2yisxeBU18_7lPpmjMzTMUPCXtJxrB75VYoZAybkyGnFmC_tPD13MIShT04iUGkNLFPpaof4BGxnmCE4hNob-tVijFTH_EIdNXg0fr-rQ-qxd3vw7NVDIF0yDNxCeSYMz0GKuGPlvXk3SPtUzfUfZaJFau3QpfcrXhkNrUS0fW3HcXRLMhiVqNIJ5Y5wYJdq5IvEe_lElrv4NS4apswDNVI1s7B_iMDvcjFASD9Q\",\"user_pass_to_int_format\":\"5255515057565453\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":223,\"1\":83,\"2\":48,\"3\":161,\"4\":204,\"5\":195,\"6\":149,\"7\":141,\"8\":132,\"9\":65,\"10\":55,\"11\":201,\"12\":245,\"13\":60,\"14\":139,\"15\":236,\"16\":39,\"17\":130,\"18\":130,\"19\":162,\"20\":215,\"21\":104,\"22\":235,\"23\":117,\"24\":152,\"25\":71,\"26\":252,\"27\":46,\"28\":73,\"29\":54,\"30\":170,\"31\":251},\"secret_key\":{\"0\":90,\"1\":130,\"2\":14,\"3\":79,\"4\":237,\"5\":213,\"6\":128,\"7\":240,\"8\":11,\"9\":61,\"10\":50,\"11\":225,\"12\":67,\"13\":212,\"14\":26,\"15\":215,\"16\":84,\"17\":207,\"18\":4,\"19\":3,\"20\":95,\"21\":124,\"22\":35,\"23\":123,\"24\":72,\"25\":189,\"26\":115,\"27\":153,\"28\":16,\"29\":105,\"30\":73,\"31\":216,\"32\":223,\"33\":83,\"34\":48,\"35\":161,\"36\":204,\"37\":195,\"38\":149,\"39\":141,\"40\":132,\"41\":65,\"42\":55,\"43\":201,\"44\":245,\"45\":60,\"46\":139,\"47\":236,\"48\":39,\"49\":130,\"50\":130,\"51\":162,\"52\":215,\"53\":104,\"54\":235,\"55\":117,\"56\":152,\"57\":71,\"58\":252,\"59\":46,\"60\":73,\"61\":54,\"62\":170,\"63\":251}}},\"zk_addr\":\"0x37c4424a1b9970b94dd7276aecae5b9d1c035c7e3c88f4f1155aa0e5127ef6e4\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"7945493796354284921600453177054285773255312631482061079984629835363646515586\",\"2166517138136751277833084326208436279446476764263036612847849710284540628729\",\"1\"],\"b\":[[\"14768147580014515274920059533450969526917243519129235569375547705391356814034\",\"10926704359346438742364088104571886636979515204481541507299552373423645137538\"],[\"18345707220306299341155061798987886250677895640406984732019863169577306401665\",\"13781450607771983148196301814354815025344242496715512941320154501577226245887\"],[\"1\",\"0\"]],\"c\":[\"16100487697721354409255314346417284275475569122937970611421991273969908317416\",\"20037727069966515075925458192010761910249599063237527691300280470015098486501\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AN9TMKHMw5WNhEE3yfU8i+wngoKi12jrdZhH/C5JNqr7\"}";
pub const TEST_AUTH_DATA_20: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImJlMTgtZlBDdV9ZRndybE43TDhKV1BHX3hFbyIsIm5iZiI6MTcxNTY4NjgzMSwiaWF0IjoxNzE1Njg3MTMxLCJleHAiOjE3MTU2OTA3MzEsImp0aSI6IjdmZTFkZTM5NDVkMjliYTBhOWQ4MGFlODZiZGRmNjkyMDE1N2RlMDcifQ.RMM8wIiEzZ97DVdngkDhKapTMZq-R7woI2yjclLqTgnYZKTZ5N9y67zFJLDfcg017VyyRK18OS1OLsnUgnphi3ULotImnJ2292VDBd7kxhyq9QAqfHVDK2-MYNlJXy53UIr2xS9td1aoHUDZkvBy690IhV4nPrxLOUhI8c4gAvpkfHFmAvxYuQoUu69c_hSzREhrVOa979t5nZuJjNWwUcwgD40To1DM6Dxwy186basvY4AyWPHcI4ARFoyPEMRFUOtO05fUrwUH8O63Ay6K1DwxaLXDzx4T7O9X9nlrCj2uROdahsv-Dj24hruudSYxi4GH2uO6u0a1RlTIvWJ-1A\",\"user_pass_to_int_format\":\"525451525655\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":66,\"1\":155,\"2\":237,\"3\":117,\"4\":45,\"5\":166,\"6\":245,\"7\":92,\"8\":78,\"9\":225,\"10\":218,\"11\":156,\"12\":7,\"13\":132,\"14\":164,\"15\":47,\"16\":114,\"17\":174,\"18\":4,\"19\":86,\"20\":18,\"21\":212,\"22\":182,\"23\":62,\"24\":50,\"25\":219,\"26\":104,\"27\":185,\"28\":183,\"29\":108,\"30\":38,\"31\":252},\"secret_key\":{\"0\":13,\"1\":127,\"2\":13,\"3\":29,\"4\":128,\"5\":121,\"6\":142,\"7\":51,\"8\":210,\"9\":28,\"10\":131,\"11\":160,\"12\":209,\"13\":42,\"14\":214,\"15\":198,\"16\":137,\"17\":147,\"18\":155,\"19\":40,\"20\":86,\"21\":167,\"22\":168,\"23\":10,\"24\":249,\"25\":180,\"26\":188,\"27\":132,\"28\":41,\"29\":146,\"30\":192,\"31\":28,\"32\":66,\"33\":155,\"34\":237,\"35\":117,\"36\":45,\"37\":166,\"38\":245,\"39\":92,\"40\":78,\"41\":225,\"42\":218,\"43\":156,\"44\":7,\"45\":132,\"46\":164,\"47\":47,\"48\":114,\"49\":174,\"50\":4,\"51\":86,\"52\":18,\"53\":212,\"54\":182,\"55\":62,\"56\":50,\"57\":219,\"58\":104,\"59\":185,\"60\":183,\"61\":108,\"62\":38,\"63\":252}}},\"zk_addr\":\"0x86ab13e3c90b7f5b52a0e7d045425f1a5ce4f2938d82fe32013b5c5dffc8aa40\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"13280060882937967421268531103181473070897547065707830941266439167277535861998\",\"9934433138500558951890280258062370504217382435917636186873727481367280202864\",\"1\"],\"b\":[[\"3838124130316726849360987686807592227651253623250263168834039482151640975443\",\"10050190797101422174255450354163308725608018844614813729840170282126147936409\"],[\"18360080471111693027482741715722945557865825591442098780536696036281663618095\",\"1378964582828950987975075563637558653759765511530268169302574447782691787466\"],[\"1\",\"0\"]],\"c\":[\"1373142722414479432483215105546507593017308819682036641663292686387425172376\",\"3353210342014729825799146687716012229927760750084040417279416030868174996451\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AEKb7XUtpvVcTuHanAeEpC9yrgRWEtS2PjLbaLm3bCb8\"}";
pub const TEST_AUTH_DATA_21: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6Im12NUNKY1dsMXE1ek03Y0lyR1ZUdHF6SnFTNCIsIm5iZiI6MTcxNTY4NjgxOSwiaWF0IjoxNzE1Njg3MTE5LCJleHAiOjE3MTU2OTA3MTksImp0aSI6IjYxMDM2YmQwZWE3YjI5MDY4MjgwODYxMzMyODZhODZlNmY0ZmMwNGEifQ.VE2a8s2ZuyTVklFwSvh05y_mGrDMJXww-5Pu3-UUIQi3sBQnMzpnvWo3MIb32rXxwU6Obtx9izsR-Csk-U0QH4WuseGHnhHA90lACdeXNXHUWNktsY62_z2lkseTlJQV_ccNVctNgqornxmtV6gRvihLKkYCJt08umhAcRe8-Fh9iNmlCf5sMngaA-k0bvIbdnxkoP0KI9em7sgpTDB0FJFCgVAVYkzQTuJJlfuKjeF0lgpLnkjTOtgMyCpuZrrxf9GH6wY2VSme3Zk6xVJfl5cC6YugQFs-t56CEhPDrm-LIlLTD9JuNAKctlRRaTmkTembZAzweu6Wqh322MDx1g\",\"user_pass_to_int_format\":\"100102100102100115106107\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":227,\"1\":142,\"2\":234,\"3\":83,\"4\":36,\"5\":125,\"6\":219,\"7\":233,\"8\":159,\"9\":30,\"10\":60,\"11\":195,\"12\":110,\"13\":130,\"14\":105,\"15\":107,\"16\":44,\"17\":46,\"18\":151,\"19\":154,\"20\":116,\"21\":131,\"22\":237,\"23\":231,\"24\":159,\"25\":119,\"26\":35,\"27\":130,\"28\":56,\"29\":90,\"30\":121,\"31\":26},\"secret_key\":{\"0\":34,\"1\":107,\"2\":197,\"3\":227,\"4\":209,\"5\":156,\"6\":36,\"7\":233,\"8\":231,\"9\":171,\"10\":100,\"11\":210,\"12\":113,\"13\":247,\"14\":59,\"15\":222,\"16\":214,\"17\":129,\"18\":238,\"19\":254,\"20\":13,\"21\":13,\"22\":3,\"23\":151,\"24\":9,\"25\":173,\"26\":77,\"27\":113,\"28\":126,\"29\":7,\"30\":203,\"31\":52,\"32\":227,\"33\":142,\"34\":234,\"35\":83,\"36\":36,\"37\":125,\"38\":219,\"39\":233,\"40\":159,\"41\":30,\"42\":60,\"43\":195,\"44\":110,\"45\":130,\"46\":105,\"47\":107,\"48\":44,\"49\":46,\"50\":151,\"51\":154,\"52\":116,\"53\":131,\"54\":237,\"55\":231,\"56\":159,\"57\":119,\"58\":35,\"59\":130,\"60\":56,\"61\":90,\"62\":121,\"63\":26}}},\"zk_addr\":\"0xb092062dc38ee15b239fedd8955547cae553068e350add6a186a900308ca1704\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"19083893384522082364200015848081882762611466511855277173108395395690822433582\",\"15765871630522826744343212165387977339454134778029147662517994756792436892191\",\"1\"],\"b\":[[\"3347275249816013439391836622904014049361323482158752310150932588414843416768\",\"9261935324040115069949005166058871304117632278716385673922763868694265924905\"],[\"10774327302040930015542399179222458502634829694095804484749135988841930351850\",\"409015645239595129631982791901837203813000443262394810220799589635024410401\"],[\"1\",\"0\"]],\"c\":[\"805618312212200473153836203801534856685701602304097276485035246177305246575\",\"12984127923817330198936709848850019846193356630613954592225359007088568774616\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AOOO6lMkfdvpnx48w26CaWssLpeadIPt5593I4I4Wnka\"}";

#[derive(Debug, Deserialize)]
pub struct JwtData {
    pub jwt: String,
    pub user_pass_to_int_format: String,
    pub ephemeral_key_pair: EphemeralKeyPair,
    pub zk_addr: String,
    pub zk_proofs: ZkProofs,
    pub extended_ephemeral_public_key: String,
}

 #[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart1 {
    pub alg: String,
    pub kid: String,
     pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2 {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub nbf: u32,
    pub iat: u32,
    pub exp: u32,
    pub jti: String,
}

#[derive(Debug, Deserialize)]
pub struct EphemeralKeyPair {
    pub keypair: Keypair,
}

#[derive(Debug, Deserialize)]
pub struct Keypair {
    pub public_key: HashMap<String, u8>,
    pub secret_key: HashMap<String, u8>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZkProofs {
    pub proof_points: ProofPoints,
    pub iss_base64_details: IssBase64Details,
    pub header_base64: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofPoints {
    pub a: Vec<String>,
    pub b: Vec<Vec<String>>,
    pub c: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssBase64Details {
    pub value: String,
    pub index_mod4: i32,
}

#[test]
fn test_poseidon_and_vergrth16_for_multiple_data() {
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

    //////

    let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww".to_string(), // Alina's data
            alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "323b214ae6975a0f034ea77354dc0c25d03642dc".to_string(), 
        ),
        content,
    );
    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";

    let data = [
            TEST_AUTH_DATA_1,
            TEST_AUTH_DATA_2,
            TEST_AUTH_DATA_3,
            TEST_AUTH_DATA_4,
            TEST_AUTH_DATA_5,
            TEST_AUTH_DATA_6,
            TEST_AUTH_DATA_7,
            TEST_AUTH_DATA_8,
            TEST_AUTH_DATA_9,
            TEST_AUTH_DATA_10,
            TEST_AUTH_DATA_11,
            TEST_AUTH_DATA_12,
            TEST_AUTH_DATA_13,
            TEST_AUTH_DATA_14,
            TEST_AUTH_DATA_15,
            TEST_AUTH_DATA_16,
            TEST_AUTH_DATA_17,
            TEST_AUTH_DATA_18,
            TEST_AUTH_DATA_19,
            TEST_AUTH_DATA_20,
            TEST_AUTH_DATA_21,
    ];

    let mut average_poseidon: u128 = 0;
    let mut average_vergrth16: u128 = 0;

    for i in 0..data.len() {
        println!("");
        println!("====================== Iter@ is {i} =========================");
        let jwt_data: JwtData = serde_json::from_str(&data[i]).unwrap();
        println!("jwt_data: {:?}", jwt_data);

        let user_pass_salt = jwt_data.user_pass_to_int_format.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        println!("{:?}", jwt_data.ephemeral_key_pair.keypair.public_key);
        let eph_secret_key = secret_key_from_integer_map(jwt_data.ephemeral_key_pair.keypair.secret_key);

        let ephemeral_kp = Ed25519KeyPair::from_bytes(&eph_secret_key).unwrap();
        let mut eph_pubkey: Vec<u8> = Vec::new(); // vec![0x00];
        eph_pubkey.extend(ephemeral_kp.public().as_ref());

        println!("ephemeral secret_key is {:?}", eph_secret_key);
        println!("ephemeral public_key is {:?}", eph_pubkey);

        let eph_pubkey_len = eph_pubkey.clone().len();
        println!("len eph_pubkey: {:?}", eph_pubkey_len);

        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is
        // "{\"alg\":\"RS256\",\"kid\":\"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"
        // typ\":\"JWT\"}"
        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 =
            serde_json::from_str(&jwt_string_1).unwrap();
        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); // "{\"iss\":\"https://accounts.google.com\",\"azp\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"aud\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"sub\":\"112897468626716626103\",\"nonce\":\"bxmnJW31ruzKMGir01YPGYL0xDY\",\"nbf\":1715687036,\"iat\":1715687336,\"exp\":1715690936,\"jti\":\"9b601d25f003640c2889a2a047789382cb1cfe87\"}"

        // JwtDataDecodedPart2
        let jwt_data_decoded2: JwtDataDecodedPart2 =
            serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(), 
            jwt_data_decoded2.aud.as_str(),
            )
        .unwrap();

        println!("jwt_data.zk_proofs = {:?}", jwt_data.zk_proofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zk_proofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string())
        .unwrap();

        // let verification_key_id: u32 = 2;
        let verification_key_id: u32 = 0;

        let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
        let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
        ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
        }).unwrap();

        let max_epoch = 142; 

        let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| {
            ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
        })
        .unwrap();

        let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

        let mut public_inputs_as_bytes = vec![];
        public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
        println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
        println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
        println!("====== Start Poseidon ========");
    
        let index_mod_4 = 1;
        engine.cc.stack.push(StackItem::int(index_mod_4));
        engine.cc.stack.push(StackItem::int(max_epoch));
        engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

        let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
        println!("modulus_cell = {:?}", modulus_cell);
        engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    
        let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
        println!("iss_base_64_cell = {:?}", iss_base_64_cell);
        engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
        println!("header_base_64_cell = {:?}", header_base_64_cell);
        engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));

        let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
        println!("zk_seed_cell = {:?}", zk_seed_cell);
        engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

        let start: Instant = Instant::now();
        let status = execute_poseidon_zk_login(&mut engine).unwrap();
        let poseidon_elapsed = start.elapsed().as_micros();

        

        let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
        let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
        let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
        println!("poseidon_res from stack: {:?}", hex::encode(poseidon_res.clone()));

        println!("public_inputs hex (computed in test): {:?}", hex::encode(public_inputs_as_bytes.clone()));
        assert!(poseidon_res == public_inputs_as_bytes);

        println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);  

        average_poseidon = average_poseidon + poseidon_elapsed;


        println!("====== Start VERGRTH16 ========");
        let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

        let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

        let verification_key_id: u32 = 0; // valid key id
        //let verification_key_id: u32 = 1; //invalid key id
        engine.cc.stack.push(StackItem::int(verification_key_id));

        let start: Instant = Instant::now();
        let status = execute_vergrth16(&mut engine).unwrap();
        let vergrth16_elapsed = start.elapsed().as_micros();

        println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed); 

        let res = engine.cc.stack.get(0).as_integer().unwrap();
        println!("res: {:?}", res);
        assert!(*res == IntegerData::minus_one());

        average_vergrth16 = average_vergrth16 + vergrth16_elapsed;
    }

    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("===================================");

    let average_poseidon_=  average_poseidon / (data.len() as u128);
    println!("average_poseidon_ in microsecond: {:?}", average_poseidon_);  
    let average_vergrth16_=  average_vergrth16 / (data.len() as u128);
    println!("average_vergrth16_ in microsecond: {:?}", average_vergrth16_); 

}

pub fn secret_key_from_integer_map(key_data: HashMap<String, u8>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..=31 {
        if let Some(value) = key_data.get(&i.to_string()) {
            vec.push(value.clone());
        }
    }
    return vec;
}

#[test]
fn test_xor() {
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
   
    let num_iter = 10;
    let mut average_: u128 = 0;
    for i in 0..num_iter {
        println!("======================");
        println!("iter = {i}");
        let x = rand::thread_rng().next_u32();
        println!("x = {x}");
        engine.cc.stack.push(StackItem::int(x));
        let y = rand::thread_rng().next_u32();
        println!("y = {y}");
        engine.cc.stack.push(StackItem::int(y));

        let start: Instant = Instant::now();
        execute_xor::<Signaling>(&mut engine).unwrap();
        let elapsed = start.elapsed().as_nanos();

        println!("elapsed in nanoseconds: {:?}", elapsed);

        let res = engine.cc.stack.get(0).as_integer().unwrap();
        println!("res: {:?}", res);

        average_ = average_ + elapsed;
    }

    let average_ =  average_ / num_iter;
    println!("average_ in nanoseconds: {:?}", average_);

        
}