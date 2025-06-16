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
use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_block::Deserializable;
use tvm_block::StateInit;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::IBitstring;
use tvm_types::SliceData;
use tvm_types::error;
use tvm_types::read_single_root_boc;
use zstd::dict::from_files;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::executor::gas::gas_state::Gas;
use crate::executor::math::DivMode;
use crate::executor::serialize_currency_collection;
use crate::executor::token::MyState;
use crate::executor::token::WASM_FUEL_MULTIPLIER;
use crate::executor::token::add_to_linker_gosh;
use crate::executor::token::execute_run_wasm;
use crate::executor::token::rejoin_chain_of_cells;
use crate::executor::token::split_to_chain_of_cells;
use crate::executor::types::Instruction;
use crate::executor::types::InstructionOptions;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::integer::behavior::OperationBehavior;
use crate::stack::integer::behavior::Quiet;
use crate::stack::integer::behavior::Signaling;
use crate::stack::savelist::SaveList;
use crate::types::Exception;
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
    // let cell = TokenValue::write_bytes(&[1u8, 2u8],
    // &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    let cell =
        TokenValue::write_bytes(&[1u8, 100u8], &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    // Push args, func name, instance name, then wasm.
    let wasm_func = "add";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let wasm_func = "docs:adder/add@0.1.0";
    let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell.clone()));
    let filename =
        "/Users/elar/Code/Havok/AckiNacki/wasm/calc_pi/target/wasm32-wasip2/release/add.wasm";
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

use wasmtime::component::ResourceTable;
use wasmtime_wasi::p2::IoImpl;
use wasmtime_wasi::p2::IoView;
use wasmtime_wasi::p2::WasiCtx;
use wasmtime_wasi::p2::WasiCtxBuilder;
use wasmtime_wasi::p2::WasiImpl;
use wasmtime_wasi::p2::WasiView;
// struct MyState {
//     ctx: WasiCtx,
//     table: ResourceTable,
// }
// impl IoView for MyState {
//     fn table(&mut self) -> &mut ResourceTable {
//         &mut self.table
//     }
// }
// impl WasiView for MyState {
//     fn ctx(&mut self) -> &mut WasiCtx {
//         &mut self.ctx
//     }
// }

#[test]
fn test_run_wasm_generic() -> Result<(), wasmtime::Error> {
    let start = Instant::now();
    // load or access WASM engine
    let mut wasm_config = wasmtime::Config::new();
    wasm_config.wasm_component_model(true);
    wasm_config.consume_fuel(true);
    let wasm_engine = match wasmtime::Engine::new(&wasm_config) {
        Ok(module) => module,
        Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to init WASM engine {:?}", e).unwrap(),
    };
    let mut builder = WasiCtxBuilder::new();
    let mut wasm_store = wasmtime::Store::new(
        &wasm_engine,
        MyState { ctx: builder.build(), table: wasmtime::component::ResourceTable::new() },
    );
    // set WASM fuel limit based on available gas
    // TODO: Consider adding a constant offset to account for cell pack/unpack and
    // other actions to be run after WASM instruction
    // TODO: Add a catch for out-of-fuel and remove matching consumed gas from
    // instruction (or set to 0?)
    // println!("Starting gas: {:?}", engine.gas_remaining());
    let wasm_fuel: u64 = 999998990u64 * WASM_FUEL_MULTIPLIER;
    match wasm_store.set_fuel(wasm_fuel) {
        Ok(module) => module,
        Err(e) => err!(ExceptionCode::OutOfGas, "Failed to set WASm fuel {:?}", e).unwrap(),
    };

    // load wasm component binary
    let filename =
        "/Users/elar/Code/Havok/AckiNacki/wasm/calc_pi/target/wasm32-wasip2/release/add.wasm";
    let wasm_instance_name = "docs:adder/add@0.1.0";
    let wasm_func_name = "add";
    let wasm_func_args = [1u8, 100u8].to_vec();

    let wasm_executable = std::fs::read(filename).unwrap();

    // let s = engine.cmd.var(0).as_cell()?;
    // let wasm_executable = rejoin_chain_of_cells(s)?;

    let wasm_component =
        match wasmtime::component::Component::new(&wasm_engine, &wasm_executable.as_slice()) {
            Ok(module) => module,
            Err(e) => err!(
                ExceptionCode::WasmLoadFail,
                "Failed to load WASM
    component {:?}",
                e
            )
            .unwrap(),
        };
    let component_type = wasm_component.component_type();

    let mut exports = component_type.exports(&wasm_engine);
    let arg = exports.next();
    println!("List of exports from WASM: {:?}", arg);
    if let Some(arg) = arg {
        println!("{:?}", arg);

        for arg in exports {
            println!(" {:?}", arg);
        }
    }

    // Add wasi-cli libs to linker
    let mut wasm_linker = wasmtime::component::Linker::<MyState>::new(&wasm_engine);

    // This is a custom linker method, adding only sync, non-io wasi dependencies.
    // If more deps are needed, add them in there!
    match add_to_linker_gosh(&mut wasm_linker) {
        Ok(_) => {}
        Err(e) => err!(
            ExceptionCode::WasmLoadFail,
            "Failed to instantiate WASM instance
    {:?}"
        )
        .unwrap(),
    };

    // This is the default add to linker method, we dont use it as it will add async
    // calls for IO stuff, which fails inside out Tokio runtime
    // match wasmtime_wasi::p2::add_to_linker_sync(&mut wasm_linker) {
    //     Ok(_) => {}
    //     Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to add WASI libs to
    // linker {:?}", e)?, };

    // Instantiate WASM component. Will error if missing some wasm deps from linker
    let wasm_instance = match wasm_linker.instantiate(&mut wasm_store, &wasm_component) {
        Ok(instance) => instance,
        Err(e) => err!(
            ExceptionCode::WasmLoadFail,
            "Failed to instantiate WASM instance
    {:?}",
            e
        )
        .unwrap(),
    };

    // get callable wasm func
    println!("Callable funcs found:");
    for export in wasm_component.component_type().exports(&wasm_engine) {
        println!("{:?}", export.0);
    }
    let instance_index = wasm_instance.get_export_index(&mut wasm_store, None, &wasm_instance_name);
    println!("Instance Index {:?}", instance_index);
    let func_index = match wasm_instance.get_export_index(
        &mut wasm_store,
        instance_index.as_ref(),
        &wasm_func_name,
    ) {
        Some(index) => index,
        None => {
            err!(ExceptionCode::WasmLoadFail, "Failed to find WASM exported function or component",)
                .unwrap()
        }
    };
    println!("Func Index {:?}", func_index);
    let wasm_function = wasm_instance
        .get_func(&mut wasm_store, func_index)
        .expect(&format!("`{}` was not an exported function", wasm_func_name));
    let wasm_function = match wasm_function.typed::<(Vec<u8>,), (Vec<u8>,)>(&wasm_store) {
        Ok(answer) => answer,
        Err(e) => {
            err!(ExceptionCode::WasmLoadFail, "Failed to get WASM answer function {:?}", e).unwrap()
        }
    };

    // execute wasm func
    // collect result
    // substract gas based on wasm fuel used
    println!("Loading WASM Args");
    println!("WASM Args loaded {:?}", wasm_func_args);
    let result = match wasm_function.call(&mut wasm_store, (wasm_func_args,)) {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to execute WASM function {:?}", e);
            err!(ExceptionCode::WasmLoadFail, "Failed to execute WASM function {:?}", e).unwrap()
        }
    };
    println!("WASM Execution result: {:?}", result);

    let gas_used: i64 = match wasm_store.get_fuel() {
        Ok(new_fuel) => {
            i64::try_from((wasm_fuel - new_fuel).div_ceil(WASM_FUEL_MULTIPLIER)).unwrap()
        }
        Err(e) => err!(
            ExceptionCode::WasmLoadFail,
            "Failed to get WASM engine fuel after execution {:?}",
            e
        )
        .unwrap(),
    };
    // engine.use_gas(gas_used);
    // println!("Remaining gas: {:?}", engine.gas_remaining());
    // match engine.gas_remaining() > 0 {
    //     true => {}
    //     false => err!(ExceptionCode::OutOfGas, "Engine out of gas.")?,
    // }

    // return result
    println!("EXEC Wasm execution result: {:?}", result);
    let res_vec = result.0;

    // let cell =
    //     TokenValue::write_bytes(res_vec.as_slice(),
    // &ABI_VERSION_2_4).unwrap().into_cell().unwrap(); println!("Pushing
    // cell");

    // engine.cc.stack.push(StackItem::cell(cell));

    println!("OK");
    println!("Duration: {:?}", Instant::now().duration_since(start));

    Ok(())
}
