use tvm_block::ACTION_CNVRTSHELLQ;
use tvm_block::ACTION_MINT_SHELL_TOKEN;
use tvm_block::ACTION_MINTECC;
use tvm_block::ACTION_RUNWASM;
use tvm_block::ExtraCurrencyCollection;
use tvm_block::Serializable;
use tvm_block::VarUInteger32;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::SliceData;
use tvm_types::error;
use wasmtime::component;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::IoView;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiView;

use crate::error::TvmError;
use crate::executor::blockchain::add_action;
use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::items_serialize;
use crate::types::Exception;
use crate::types::ResultRef;
use crate::types::Status;
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;

pub const ECC_NACKL_KEY: u32 = 1;
pub const ECC_SHELL_KEY: u32 = 2;
pub const INFINITY_CREDIT: i128 = -1;

pub const ARFC: f64 = 1000_f64;
pub const MINRC: f64 = 1_f64;
pub const MAXRC: f64 = 3_f64;
pub const TTMT: f64 = 2000000000_f64;
pub const TOTALSUPPLY: f64 = 10400000000000000000_f64;
pub const MAXRT: f64 = 157766400_f64;
pub const KF: f64 = 0.01_f64;
pub const KS: f64 = 0.001_f64;
pub const KM: f64 = 0.00001_f64;
pub const KRBK: f64 = 0.675_f64;
pub const MAX_FREE_FLOAT_FRAC: f64 = 1_f64 / 3_f64;

pub const WASM_FUEL_MULTIPLIER: u64 = 8u64;
struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}
impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

// // Generate all possible ASCII printable strings (length 1-6)
// fn generate_strings(current: String, component:
// &wasmtime::component::Component) {     if current.len() >= 6 {
//         match component.export_index(None, &current) {
//             Some(c) => println!("{:?}: {:?}", current, c),
//             None => {}
//         };
//         return;
//     }

//     // Iterate through all printable ASCII characters (32 to 126)
//     for i in 32u8..=126u8 {
//         let mut new_string = current.clone();
//         new_string.push(i as char);

//         if new_string.len() <= 6 {
//             match component.export_index(None, &new_string) {
//                 Some(c) => println!("{:?}: {:?}", new_string, c),
//                 None => {}
//             };
//             generate_strings(new_string, component);
//         }
//     }
// }

pub(super) fn split_to_chain_of_cells(input: Vec<u8>) -> Cell {
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

pub(super) fn execute_ecc_mint(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("MINTECC"))?;
    fetch_stack(engine, 2)?;
    let x: u32 = engine.cmd.var(0).as_integer()?.into(0..=255)?;
    let y: VarUInteger32 = VarUInteger32::from(engine.cmd.var(1).as_integer()?.into(0..=u64::MAX)?);
    let mut data = ExtraCurrencyCollection::new();
    data.set(&x, &y)?;
    let mut cell = BuilderData::new();
    data.write_to(&mut cell)?;
    add_action(engine, ACTION_MINTECC, None, cell)
}

// execute wasm binary
pub(super) fn execute_run_wasm(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("RUNWASM"))?;
    fetch_stack(engine, 4)?; //TODO match the stack depth change elsewhere

    // load or access WASM engine
    let mut wasm_config = wasmtime::Config::new();
    wasm_config.wasm_component_model(true);
    wasm_config.consume_fuel(true);
    let wasm_engine = match wasmtime::Engine::new(&wasm_config) {
        Ok(module) => module,
        Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to init WASM engine {:?}", e)?,
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
    println!("Starting gas: {:?}", engine.gas_remaining());
    let wasm_fuel: u64 = match engine.gas_remaining() > 0 {
        true => u64::try_from(engine.gas_remaining())? * WASM_FUEL_MULTIPLIER,
        false => err!(ExceptionCode::OutOfGas, "Engine out of gas.")?,
    };
    match wasm_store.set_fuel(wasm_fuel) {
        Ok(module) => module,
        Err(e) => err!(ExceptionCode::OutOfGas, "Failed to set WASm fuel {:?}", e)?,
    };

    // load wasm component binary
    let s = engine.cmd.var(0).as_cell()?;
    let wasm_executable = rejoin_chain_of_cells(s)?;

    let wasm_component =
        match wasmtime::component::Component::new(&wasm_engine, &wasm_executable.as_slice()) {
            Ok(module) => module,
            Err(e) => err!(
                ExceptionCode::WasmLoadFail,
                "Failed to load WASM
    component {:?}",
                e
            )?,
        };
    let component_type = wasm_component.component_type();
    // TODO: Remove debug prints
    let mut exports = component_type.exports(&wasm_engine);
    let arg = exports.next();
    println!("{:?}", arg);
    if let Some(arg) = arg {
        print!("{:?}", arg);

        for arg in exports {
            print!(" {:?}", arg);
        }
    }

    // Add wasi-cli libs to linker
    let mut wasm_linker = wasmtime::component::Linker::<MyState>::new(&wasm_engine);
    match wasmtime_wasi::add_to_linker_sync(&mut wasm_linker) {
        Ok(_) => {}
        Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to add WASI libs to linker {:?}", e)?,
    };

    // Instantiate WASM component. Will error if missing some wasm deps from linker
    let wasm_instance = match wasm_linker.instantiate(&mut wasm_store, &wasm_component) {
        Ok(instance) => instance,
        Err(e) => err!(
            ExceptionCode::WasmLoadFail,
            "Failed to instantiate WASM instance
    {:?}",
            e
        )?,
    };

    // get exported instance name to call
    let s = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let wasm_instance_name = unpack_data_from_cell(s, engine)?;
    let wasm_instance_name = String::from_utf8(wasm_instance_name)?;

    // get exported func to call from within instance
    let s = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let wasm_func_name = unpack_data_from_cell(s, engine)?;
    let wasm_func_name = String::from_utf8(wasm_func_name)?;

    // get callable wasm func
    let instance_index = wasm_instance.get_export(&mut wasm_store, None, &wasm_instance_name);
    println!("Instance Index {:?}", instance_index);
    let func_index =
        match wasm_instance.get_export(&mut wasm_store, instance_index.as_ref(), &wasm_func_name) {
            Some(index) => index,
            None => err!(
                ExceptionCode::WasmLoadFail,
                "Failed to find WASM exported function or component",
            )?,
        };
    println!("Func Index {:?}", func_index);
    let wasm_function = wasm_instance
        .get_func(&mut wasm_store, func_index)
        .expect(&format!("`{}` was not an exported function", wasm_func_name));
    let wasm_function = match wasm_function.typed::<(Vec<u8>,), (Vec<u8>,)>(&wasm_store) {
        Ok(answer) => answer,
        Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to get WASM answer function {:?}", e)?,
    };

    // execute wasm func
    // collect result
    // substract gas based on wasm fuel used
    let s = engine.cmd.var(3).as_cell()?;
    let wasm_func_args = rejoin_chain_of_cells(s)?;
    let result = match wasm_function.call(&mut wasm_store, (wasm_func_args,)) {
        Ok(result) => result,
        Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to execute WASM function {:?}", e)?,
    };
    let gas_used: i64 = match wasm_store.get_fuel() {
        Ok(new_fuel) => i64::try_from((wasm_fuel - new_fuel).div_ceil(WASM_FUEL_MULTIPLIER))?,
        Err(e) => err!(
            ExceptionCode::WasmLoadFail,
            "Failed to get WASM engine fuel after execution {:?}",
            e
        )?,
    };
    engine.use_gas(gas_used);
    println!("Remaining gas: {:?}", engine.gas_remaining());
    match engine.gas_remaining() > 0 {
        true => {}
        false => err!(ExceptionCode::OutOfGas, "Engine out of gas.")?,
    }
    // return result
    println!("EXEC Wasm execution result: {:?}", result);
    let res_vec = result.0;
    // let result = items_serialize(res_vec, engine);
    let cell = split_to_chain_of_cells(res_vec);
    // TODO: Is this stack push enough? do I need an action here?
    engine.cc.stack.push(StackItem::cell(cell));
    // let mut a: u64 = result as u64;
    // let mut cell = BuilderData::new();
    // a.write_to(&mut cell)?;
    // add_action(engine, ACTION_RUNWASM, None, cell) // todo change to OK
    Ok(())
}

pub(super) fn execute_exchange_shell(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CNVRTSHELLQ"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_CNVRTSHELLQ, None, cell)
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_repcoef(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCREPCOEF"))?;
    fetch_stack(engine, 1)?;
    let bkrt = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let mut repcoef = if bkrt < MAXRT {
        MINRC
            + (MAXRC - MINRC) / (1_f64 - 1_f64 / ARFC)
                * (1_f64 - (-1_f64 * ARFC.ln() * bkrt / MAXRT).exp())
    } else {
        MAXRC
    };
    repcoef *= 1e9_f64;
    engine.cc.stack.push(int!(repcoef as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_adjustment_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBKREWARDADJ"))?;
    fetch_stack(engine, 5)?;
    let t = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64; //time from network start
    let rbkprev = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64; //previous value of rewardadjustment (not minimum)
    let drbkavg = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    //_delta_reward = (_delta_reward * _calc_reward_num + (block.timestamp -
    //_delta_reward _reward_last_time)) / (_calc_reward_num + 1);
    //_delta_reward - average time between reward adj calculate
    //_calc_reward_num - number of calculate
    //_reward_last_time - time of last calculate
    let repavgbig = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64; //Average ReputationCoef
    let mbkt = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as f64; //sum of reward token (minted, include slash token)
    let um = (-1_f64 / TTMT) * (KM / (KM + 1_f64)).ln();
    let repavg = repavgbig / 1e9_f64;
    let rbkmin;
    if t <= TTMT - 1_f64 {
        rbkmin = TOTALSUPPLY
            * 0.675_f64
            * (1_f64 + KM)
            * ((-1_f64 * um * t).exp() - (-1_f64 * um * (t + 1_f64)).exp())
            / 3.5_f64;
    } else {
        rbkmin = 0_f64;
    }
    let rbk = (((calc_mbk(t + drbkavg) - mbkt) / drbkavg / repavg).max(rbkmin)).min(rbkprev);
    engine.cc.stack.push(int!(rbk as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
fn calc_mbk(t: f64) -> f64 {
    let um = (-1_f64 / TTMT) * (KM / (KM + 1_f64)).ln();
    let mt;
    if t > TTMT {
        mt = TOTALSUPPLY;
    } else {
        mt = TOTALSUPPLY * (1_f64 + KM) * (1_f64 - (-1_f64 * um * t).exp());
    }
    let mbk = KRBK * mt;
    return mbk;
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_validator_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBKREWARD"))?;
    fetch_stack(engine, 7)?;
    let mut repcoef = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64; //average reputation coef of licenses in one stake
    let bkstake = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64; //value of stake
    let totalbkstake = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64; //sum of stakes at start of epoch
    let t = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64; //duration of epoch
    let mbk = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as f64; //sum of reward token (minted, include slash token)
    let nbk = engine.cmd.var(5).as_integer()?.into(0..=u128::MAX)? as f64; //numberOfActiveBlockKeepers
    let rbk = engine.cmd.var(6).as_integer()?.into(0..=u128::MAX)? as f64; //last calculated reward_adjustment
    repcoef = repcoef / 1e9_f64;
    let reward;
    if mbk == 0_f64 {
        reward = rbk * t * repcoef / nbk;
    } else if mbk < TOTALSUPPLY {
        reward = rbk * t * repcoef * bkstake / totalbkstake;
    } else {
        reward = 0_f64;
    }
    engine.cc.stack.push(int!(reward as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_min_stake(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKE"))?;
    fetch_stack(engine, 4)?;
    let nbkreq = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64; // needNumberOfActiveBlockKeepers = 10000
    let nbk = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64; //numberOfActiveBlockKeepersAtBlockStart
    let tstk = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64; //time from network start + uint128(_waitStep / 3) where waitStep - number of block duration of preEpoch
    let mbkav = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64; //sum of reward token without slash tokens
    let sbkbase;
    if mbkav != 0_f64 {
        let fstk;
        if tstk > TTMT {
            fstk = MAX_FREE_FLOAT_FRAC;
        } else {
            let uf = (-1_f64 / TTMT) * (KF / (1_f64 + KF)).ln();
            fstk = MAX_FREE_FLOAT_FRAC * (1_f64 + KF) * (1_f64 - (-1_f64 * tstk * uf).exp());
        }
        sbkbase = (mbkav * (1_f64 - fstk) / 2_f64) / nbkreq;
    } else {
        sbkbase = 0_f64;
    }
    let sbkmin;
    let us = -1_f64 * (KS / (KS + 1_f64)).ln() / nbkreq;
    if (nbk >= 0_f64) && (nbk <= nbkreq) {
        sbkmin = sbkbase * (1_f64 + KS) * (1_f64 - (-1_f64 * us * nbk).exp());
    } else {
        let unbk = 2_f64 * nbkreq - nbk;
        sbkmin = sbkbase * (2_f64 - (1_f64 + KS) * (1_f64 - (-1_f64 * us * unbk).exp()));
    }
    engine.cc.stack.push(int!(sbkmin as u128));
    Ok(())
}

pub(super) fn execute_mint_shell(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("MINTSHELL"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_MINT_SHELL_TOKEN, None, cell)
}
