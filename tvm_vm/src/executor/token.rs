use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_block::ACTION_BURNECC;
use tvm_block::ACTION_CNVRTSHELLQ;
use tvm_block::ACTION_MINT_SHELL_TOKEN;
use tvm_block::ACTION_MINTECC;
use tvm_block::ExtraCurrencyCollection;
use tvm_block::Serializable;
use tvm_block::VarUInteger32;
use tvm_types::BuilderData;
use tvm_types::ExceptionCode;
use tvm_types::SliceData;
use tvm_types::error;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::p2::IoImpl;
use wasmtime_wasi::p2::IoView;
use wasmtime_wasi::p2::WasiCtx;
use wasmtime_wasi::p2::WasiCtxBuilder;
use wasmtime_wasi::p2::WasiImpl;
use wasmtime_wasi::p2::WasiView;

use crate::error::TvmError;
use crate::executor::blockchain::add_action;
use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Exception;
use crate::types::Status;
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
pub const KRBM: f64 = 0.1_f64;
pub const MAX_FREE_FLOAT_FRAC: f64 = 1_f64 / 3_f64;

pub const WASM_FUEL_MULTIPLIER: u64 = 2220000u64;
pub const WASM_200MS_FUEL: u64 = 2220000000u64;
pub const RUNWASM_GAS_PRICE: u64 = WASM_200MS_FUEL / WASM_FUEL_MULTIPLIER;

wasmtime::component::bindgen!({
    inline: r#"
        package wasi:io@0.2.3;

        interface error {
            resource error;
        }
        interface streams {
            use error.{error};

            resource output-stream {
                check-write: func() -> result<u64, stream-error>;
                write: func(contents: list<u8>) -> result<_, stream-error>;
                blocking-write-and-flush: func(contents: list<u8>) -> result<_, stream-error>;
                blocking-flush: func() -> result<_, stream-error>;
            }

            resource input-stream;

            variant stream-error {
                last-operation-failed(error),
                closed,
            }
        }

        world ioer {
            import error;
            import streams;
        }
    "#,
    with: {
        "wasi:io/error/error": MyWasiIoError,
    },
});
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
pub struct MyWasiIoError;
pub enum StreamError {
    Default,
}
impl wasi::io::streams::HostOutputStream for MyState {
    fn check_write(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    fn write(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        contents: wasmtime::component::__internal::Vec<u8>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    fn blocking_write_and_flush(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        contents: wasmtime::component::__internal::Vec<u8>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    fn blocking_flush(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::io::streams::HostInputStream for MyState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<wasi::io::streams::InputStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::io::streams::Host for MyState {}
impl wasi::io::error::HostError for MyState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<wasi::io::error::Error>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl wasi::io::error::Host for MyState {}

// Async IO annotator for WASI. Do not use unless you know what you're doing.
// fn io_type_annotate<T: IoView, F>(val: F) -> F
// where
//     F: Fn(&mut T) -> IoImpl<&mut T>,
// {
//     val
// }
// Sync annotator for WASI. Used in wasmtime linker
fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
}

// fn get_wasm_binary_by_hash(wasm_hash: Vec<u8>, engine: &mut Engine) ->
// Vec<u8> {     engine.get_wasm_binary_by_hash(wasm_hash)
// }

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

use wasmtime::component::HasData;
struct HasWasi<T>(T);

impl<T: 'static> HasData for HasWasi<T> {
    type Data<'a> = WasiImpl<&'a mut T>;
}
// This is a custom linker method, adding only sync, non-io wasi dependencies.
// If more deps are needed, add them in there!
fn add_to_linker_gosh<'a, T: WasiView + 'static>(
    wasm_linker: &mut wasmtime::component::Linker<T>,
) -> Result<(), wasmtime::Error> {
    use wasmtime_wasi::p2::bindings::cli;
    use wasmtime_wasi::p2::bindings::clocks;
    use wasmtime_wasi::p2::bindings::filesystem;
    use wasmtime_wasi::p2::bindings::random;

    // wasmtime_wasi::p2::add_to_linker_sync(linker)
    let options = wasmtime_wasi::p2::bindings::sync::LinkOptions::default();
    let l = wasm_linker;
    let f: fn(&mut T) -> WasiImpl<&mut T> = |t| WasiImpl(IoImpl(t));
    clocks::wall_clock::add_to_linker::<T, HasWasi<T>>(l, f)?;
    clocks::monotonic_clock::add_to_linker::<T, HasWasi<T>>(l, f)?;
    filesystem::preopens::add_to_linker::<T, HasWasi<T>>(l, f)?;
    random::random::add_to_linker::<T, HasWasi<T>>(l, f)?;
    random::insecure::add_to_linker::<T, HasWasi<T>>(l, f)?;
    random::insecure_seed::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::exit::add_to_linker::<T, HasWasi<T>>(l, &options.into(), f)?;
    cli::environment::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::stdin::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::stdout::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::stderr::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::terminal_input::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::terminal_output::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::terminal_stdin::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::terminal_stdout::add_to_linker::<T, HasWasi<T>>(l, f)?;
    cli::terminal_stderr::add_to_linker::<T, HasWasi<T>>(l, f)?;
    // Ioer::add_to_linker::<T, HasWasi<T>>(l, f)?;
    Ok(())
}

// execute wasm binary
pub(super) fn execute_run_wasm(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("RUNWASM"))?;
    fetch_stack(engine, 5)?;

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
    log::debug!("Starting gas: {:?}", engine.gas_remaining());
    let wasm_fuel: u64 = WASM_200MS_FUEL;

    // TODO: If switching to dunamic fuel limit, use this code:
    // let wasm_fuel: u64 = match engine.gas_remaining() > 0 {
    //     true => match
    // u64::try_from(engine.gas_remaining())?.checked_mul(WASM_FUEL_MULTIPLIER) {
    //         Some(k) => k,
    //         None => err!(ExceptionCode::IntegerOverflow, "Overflow when
    // calculating WASM fuel")?,     },
    //     false => err!(ExceptionCode::OutOfGas, "Engine out of gas.")?,
    // };
    match wasm_store.set_fuel(wasm_fuel) {
        Ok(module) => module,
        Err(e) => err!(ExceptionCode::OutOfGas, "Failed to set WASm fuel {:?}", e)?,
    };

    // load wasm component binary
    let s = engine.cmd.var(0).as_cell()?;
    let wasm_executable =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction {:?}", e)?,
        };
    let wasm_hash_mode = wasm_executable.is_empty();
    let wasm_executable: Vec<u8> = if wasm_hash_mode {
        let s = engine.cmd.var(4).as_cell()?;
        let wasm_hash =
            match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?
                .0
            {
                TokenValue::Bytes(items) => items,
                e => {
                    err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction {:?}", e)?
                }
            };
        log::debug!("Using WASM Hash {:?}", wasm_hash);
        engine.get_wasm_binary_by_hash(wasm_hash)?
        // todo!("Add hash lookup here from hash {:?}", wasm_hash);
    } else {
        wasm_executable
    };
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
            )?,
        };
    let component_type = wasm_component.component_type();

    let mut exports = component_type.exports(&wasm_engine);
    let arg = exports.next();
    log::debug!("List of exports from WASM: {:?}", arg);
    if let Some(arg) = arg {
        log::debug!("{:?}", arg);

        for arg in exports {
            log::debug!(" {:?}", arg);
        }
    }

    // Add wasi-cli libs to linker
    let mut wasm_linker = wasmtime::component::Linker::<MyState>::new(&wasm_engine);

    // This is a custom linker method, adding only sync, non-io wasi dependencies.
    // If more deps are needed, add them in there!
    match add_to_linker_gosh::<MyState>(&mut wasm_linker) {
        Ok(_) => {}
        Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to instantiate WASM instance {:?}", e)?,
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
    log::debug!("Callable funcs found:");
    for export in wasm_component.component_type().exports(&wasm_engine) {
        log::debug!("{:?}", export.0);
    }
    let instance_index = wasm_instance.get_export_index(&mut wasm_store, None, &wasm_instance_name);
    log::debug!("Instance Index {:?}", instance_index);
    let func_index = match wasm_instance.get_export_index(
        &mut wasm_store,
        instance_index.as_ref(),
        &wasm_func_name,
    ) {
        Some(index) => index,
        None => {
            err!(ExceptionCode::WasmLoadFail, "Failed to find WASM exported function or component",)?
        }
    };
    log::debug!("Func Index {:?}", func_index);
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
    log::debug!("Loading WASM Args");
    let wasm_func_args =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            _ => err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction")?,
        };
    log::debug!("WASM Args loaded {:?}", wasm_func_args);
    let result = match wasm_function.call(&mut wasm_store, (wasm_func_args,)) {
        Ok(result) => result,
        Err(e) => {
            log::debug!("Failed to execute WASM function {:?}", e);
            err!(ExceptionCode::WasmExecFail, "Failed to execute WASM function {:?}", e)?
        }
    };
    log::debug!("WASM Execution result: {:?}", result);

    let gas_used: i64 = RUNWASM_GAS_PRICE.try_into()?;
    // TODO: If we switch to dynamic gas usage, reenable this code
    // let gas_used: i64 = match wasm_store.get_fuel() {
    //     Ok(new_fuel) => i64::try_from((wasm_fuel -
    // new_fuel).div_ceil(WASM_FUEL_MULTIPLIER))?,     Err(e) => err!(
    //         ExceptionCode::WasmLoadFail,
    //         "Failed to get WASM engine fuel after execution {:?}",
    //         e
    //     )?,
    // };
    engine.use_gas(gas_used);
    log::debug!("Remaining gas: {:?}", engine.gas_remaining());
    match engine.gas_remaining() > 0 {
        true => {}
        false => err!(ExceptionCode::OutOfGas, "Engine out of gas.")?,
    }

    // return result
    log::debug!("EXEC Wasm execution result: {:?}", result);
    let res_vec = result.0;

    let cell = TokenValue::write_bytes(res_vec.as_slice(), &ABI_VERSION_2_4)?.into_cell()?;
    log::debug!("Pushing cell");

    engine.cc.stack.push(StackItem::cell(cell));

    log::debug!("OK");

    Ok(())
}

pub(super) fn execute_ecc_burn(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("BURNECC"))?;
    fetch_stack(engine, 2)?;
    let x: u32 = engine.cmd.var(0).as_integer()?.into(0..=255)?;
    let y = engine.cmd.var(1).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    y.write_to(&mut cell)?;
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_BURNECC, None, cell)
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
    let rbk = (((calc_mbk(t + drbkavg, KRBK) - mbkt) / drbkavg / repavg).max(rbkmin)).min(rbkprev);
    engine.cc.stack.push(int!(rbk as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_adjustment_reward_bm(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBMREWARDADJ"))?;
    fetch_stack(engine, 4)?;
    let t = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64; //time from network start
    let rbmprev = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64; //previous value of rewardadjustment (not minimum)
    let drbmavg = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let mbmt = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64; //sum of reward token (minted, include slash token)
    let um = (-1_f64 / TTMT) * (KM / (KM + 1_f64)).ln();
    let rbmmin;
    if t <= TTMT - 1_f64 {
        rbmmin = TOTALSUPPLY
            * 0.1_f64
            * (1_f64 + KM)
            * ((-1_f64 * um * t).exp() - (-1_f64 * um * (t + 1_f64)).exp())
            / 3.5_f64;
    } else {
        rbmmin = 0_f64;
    }
    let rbm = (((calc_mbk(t + drbmavg, KRBM) - mbmt) / drbmavg).max(rbmmin)).min(rbmprev);
    engine.cc.stack.push(int!(rbm as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
fn calc_mbk(t: f64, krk: f64) -> f64 {
    let um = (-1_f64 / TTMT) * (KM / (KM + 1_f64)).ln();
    let mt;
    if t > TTMT {
        mt = TOTALSUPPLY;
    } else {
        mt = TOTALSUPPLY * (1_f64 + KM) * (1_f64 - (-1_f64 * um * t).exp());
    }
    let mbk = krk * mt;
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
pub(super) fn execute_calculate_block_manager_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBMREWARD"))?;
    fetch_stack(engine, 4)?;
    let radj = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let depoch = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let mbm = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let count_bm = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let reward;
    if mbm >= TOTALSUPPLY * 0.1_f64 {
        reward = 0_f64;
    } else {
        reward = radj * depoch / count_bm;
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

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_min_stake_bm(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKEBM"))?;
    fetch_stack(engine, 2)?;
    let tstk = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64; //time from network start 
    let mbkav = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64; //sum of reward token without slash tokens
    let fstk;
    if tstk > TTMT {
        fstk = MAX_FREE_FLOAT_FRAC;
    } else {
        let uf = (-1_f64 / TTMT) * (KF / (1_f64 + KF)).ln();
        fstk = MAX_FREE_FLOAT_FRAC * (1_f64 + KF) * (1_f64 - (-1_f64 * tstk * uf).exp());
    }
    let sbkmin = mbkav * (1_f64 - fstk);
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
