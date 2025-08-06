use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_block::ACTION_BURNECC;
use tvm_block::ACTION_CNVRTSHELLQ;
use tvm_block::ACTION_MINT_SHELL_TOKEN;
use tvm_block::ACTION_MINT_SHELLQ_TOKEN;
use tvm_block::ACTION_SEND_TO_DAPP_CONFIG;
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

//pub const ARFC: f64 = 1000_f64;
//pub const MINRC: f64 = 1_f64;
//pub const MAXRC: f64 = 3_f64;
pub const MAXRT: u128 = 157_766_400;
//pub const KF: f64 = 0.01_f64;
//pub const KS: f64 = 0.001_f64;
//pub const KM: f64 = 0.00001_f64;
//pub const KRMV: f64 = 0.225_f64;
//pub const MAX_FREE_FLOAT_FRAC: f64 = 1_f64 / 3_f64;

pub const WASM_FUEL_MULTIPLIER: u64 = 2220000u64;
pub const WASM_200MS_FUEL: u64 = 2220000000u64;
pub const RUNWASM_GAS_PRICE: u64 = WASM_200MS_FUEL / WASM_FUEL_MULTIPLIER;

const RC_ONE_Q32: i64 = 1i64 << 32;
const RC_POW2_COEFF: [i64; 6] = [
    4_294_967_296,  // 1 * 2^32
    2_977_044_472,  // ln2 * 2^32
    1_031_764_991,  // (ln2)^2/2 * 2^32
      238_388_332,  // (ln2)^3/6  * 2^32
       41_309_550,  // (ln2)^4/24 * 2^32
        5_726_720,  // (ln2)^5/120 * 2^32
];
const RC_K1_Q32: i64 = 6_196_328_019; // 1 / ln 2 * 2^32
const RC_K2_Q32:      i64 = 188; // ln(ARFC) / MAXRT * 2^32 = ln(1000) / 157766400 * 2^32
const RC_K3_Q32:      i64 = 8_598_533_125; // (MAXRC - MINRC) / (1 - 1 / ARFC) * 2^32 = (3 - 1) / (1 − 1 / 1000) * 2^32
const RCSCALE:  i128 = 1_000_000_000;

const ONE_Q32: i64 = 1i64 << 32;
const TOTALSUPPLY: u128 = 10_400_000_000_000_000_000;
const TTMT:        u128  = 2_000_000_000;
const KM_Q32:      i64  = 42_950;  // KM * 2 ^ 32 = 1e-5 * 2 ^ 32
const ONE_PLUS_KM: i64  = ONE_Q32 + KM_Q32;
const KRBK_NUM: u128 = 675; // 0.675 = 675 / 1000
const KRBK_DEN: u128 = 1_000;
const KRBM_NUM: u128 = 1;
const KRBM_DEN: u128 = 10;
const KRMV_NUM: u128 = 225;
const KRMV_DEN: u128 = 1000;
const UM_Q64: i64 = 106_188_087_029;  // -ln(KM / (KM + 1)) / TTMT * 2^64 = -ln(1e-5 / (1 + 1e-5)) / 2e9 * 2^64

// e^(−n), n = 0...12 in Q‑32
const EXP_NEG_VAL_Q32: [i64; 13] = [
    4_294_967_296, 1_580_030_169,   581_260_615, 213_833_830,  78_665_070,
       28_939_262,    10_646_160,     3_916_503,    1_440_801,     530_041,
          194_991,        71_733,        26_389,
];

// Maclaurin coeffs
const MAC_EXP_NEG_COEFF_Q32: [i64; 9] = [
    4_294_967_296,  // 1 * 2^32
    -4_294_967_296,  // -1 * 2^32
    2_147_483_648,  // 1/2!  * 2^32
    -715_827_883,  // -1/3! * 2^32
    178_956_971,  // 1/4! * 2^32
    -35_791_394,  // -1/5! 2^32
    5_965_232,   // 1/6! * 2^32 
    -852_176,  // -1/7! * 2^32
    106_522,   //1/8! * 2^32
];

const KF_Q32:  i64  = 42_949_673;  // KF * 2 ^ 32 = 1e-2 * 2 ^ 32
const ONE_PLUS_KF_Q32: i64  = ONE_Q32 + KF_Q32; 
const MAX_FREE_FLOAT_FRAC_Q32: i64  = ONE_Q32 / 3;
const UF_Q64: i64 = 42_566_973_522;  // -ln(KF / (KF + 1)) / TTMT * 2^64 = -ln(1e-2 / (1 + 1e-2)) / 2e9 * 2^64


// wasmtime::component::bindgen!({
//     inline: r#"
//         package wasi:io@0.2.3;

//             interface error {
//                 resource error;
//             }
//             interface streams {
//                 use error.{error};

//                 resource output-stream {
//                     check-write: func() -> result<u64, stream-error>;
//                     write: func(contents: list<u8>) -> result<_,
// stream-error>;                     blocking-write-and-flush: func(contents:
// list<u8>) -> result<_, stream-error>;                     blocking-flush:
// func() -> result<_, stream-error>;                 }

//                 resource input-stream;

//                 variant stream-error {
//                     last-operation-failed(error),
//                     closed,
//                 }
//             }

//         world ioer {
//             import error;
//             import streams;
//         }
//     "#,
// });

wasmtime::component::bindgen!({
    inline: r#"
        package local:demo;
        package wasi:io@0.2.3{

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
        }
        package wasi:cli@0.2.3 {
            interface stdin {
                use wasi:io/streams@0.2.3.{input-stream};

                get-stdin: func() -> input-stream;
            }
            interface stdout {
                use wasi:io/streams@0.2.3.{output-stream};

                get-stdout: func() -> output-stream;
            }
            interface stderr {
                use wasi:io/streams@0.2.3.{output-stream};

                get-stderr: func() -> output-stream;
            }
            world iocli {
            import stdin;
            import stdout;
            import stderr;
            import wasi:io/streams@0.2.3;
        }
        }
        package wasi:filesystem@0.2.3 {
            interface types {
                use wasi:io/streams@0.2.3.{error, output-stream};
                use wasi:clocks/wall-clock@0.2.3.{datetime};

                resource descriptor {
                write-via-stream: func(offset: filesize) -> result<output-stream, error-code>;
                append-via-stream: func() -> result<output-stream, error-code>;
                get-type: func() -> result<descriptor-type, error-code>;
                stat: func() -> result<descriptor-stat, error-code>;
                }

                enum error-code {
                access,
                would-block,
                already,
                bad-descriptor,
                busy,
                deadlock,
                quota,
                exist,
                file-too-large,
                illegal-byte-sequence,
                in-progress,
                interrupted,
                invalid,
                io,
                is-directory,
                loop,
                too-many-links,
                message-size,
                name-too-long,
                no-device,
                no-entry,
                no-lock,
                insufficient-memory,
                insufficient-space,
                not-directory,
                not-empty,
                not-recoverable,
                unsupported,
                no-tty,
                no-such-device,
                overflow,
                not-permitted,
                pipe,
                read-only,
                invalid-seek,
                text-file-busy,
                cross-device,
                }

                type filesize = u64;

                enum descriptor-type {
                unknown,
                block-device,
                character-device,
                directory,
                fifo,
                symbolic-link,
                regular-file,
                socket,
                }

                type link-count = u64;

                record descriptor-stat {
                %type: descriptor-type,
                link-count: link-count,
                size: filesize,
                data-access-timestamp: option<datetime>,
                data-modification-timestamp: option<datetime>,
                status-change-timestamp: option<datetime>,
                }

                filesystem-error-code: func(err: borrow<error>) -> option<error-code>;
            }
            interface preopens {
                use types.{descriptor};

                get-directories: func() -> list<tuple<descriptor, string>>;
            }

            world filesystemtypes {
                import types;
                import preopens;
            }
        }

        package wasi:clocks@0.2.3 {
            interface wall-clock {
                record datetime {
                seconds: u64,
                nanoseconds: u32,
                }
            }
        }

        world localworld {
            import wasi:io/streams@0.2.3;
            import wasi:io/error@0.2.3;
            import wasi:cli/stdin@0.2.3;
            import wasi:cli/stdout@0.2.3;
            import wasi:cli/stderr@0.2.3;
            import wasi:filesystem/types@0.2.3;
            import wasi:filesystem/preopens@0.2.3;
            import wasi:clocks/wall-clock@0.2.3;
        }
    "#,
    // with: {
    //     "wasi:clocks/wall-clock@0.2.3" : wasmtime_wasi::p2::bindings::clocks::wall_clock,
    // }
        // with: {
        // // Specify that our host resource is going to point to the `MyLogger`
        // // which is defined just below this macro.
        // "wasi:io/streams@0.2.3": MyState,
        // },
});
// wasmtime::component::bindgen!({
//     inline: r#"
//         package wasi:cli@0.2.3;
//         interface stdin {
//             use wasi:io/streams@0.2.3.{input-stream};

//             get-stdin: func() -> input-stream;
//         }
//         interface stdout {
//             use wasi:io/streams@0.2.3.{output-stream};

//             get-stdout: func() -> output-stream;
//         }
//         interface stderr {
//             use wasi:io/streams@0.2.3.{output-stream};

//             get-stderr: func() -> output-stream;
//         }
//         world clier {
//             import stdin;
//             import stdout;
//             import stderr;
//         }
//     "#,
// });
pub(crate) struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
    limiter: wasmtime::StoreLimits,
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

impl wasi::filesystem::preopens::Host for MyState {
    fn get_directories(
        &mut self,
    ) -> wasmtime::component::__internal::Vec<(
        wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        wasmtime::component::__internal::String,
    )> {
        Vec::<(
            wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
            wasmtime::component::__internal::String,
        )>::new()
    }
}

impl wasi::clocks::wall_clock::Host for MyState {}

impl wasi::filesystem::types::Host for MyState {
    fn filesystem_error_code(
        &mut self,
        err: wasmtime::component::Resource<wasi::io::streams::Error>,
    ) -> Option<wasi::filesystem::types::ErrorCode> {
        match err {
            _ => Some(wasi::filesystem::types::ErrorCode::Unsupported),
        }
    }
}

impl wasi::filesystem::types::HostDescriptor for MyState {
    fn write_via_stream(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _offset: wasi::filesystem::types::Filesize,
    ) -> Result<
        wasmtime::component::Resource<wasi::filesystem::types::OutputStream>,
        wasi::filesystem::types::ErrorCode,
    > {
        Err(wasi::filesystem::types::ErrorCode::Unsupported)
    }

    fn append_via_stream(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<
        wasmtime::component::Resource<wasi::filesystem::types::OutputStream>,
        wasi::filesystem::types::ErrorCode,
    > {
        Err(wasi::filesystem::types::ErrorCode::Unsupported)
    }

    fn get_type(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<wasi::filesystem::types::DescriptorType, wasi::filesystem::types::ErrorCode> {
        Ok(wasi::filesystem::types::DescriptorType::Unknown)
    }

    fn stat(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<wasi::filesystem::types::DescriptorStat, wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Unsupported)
    }

    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::cli::stderr::Host for MyState {
    fn get_stderr(&mut self) -> wasmtime::component::Resource<wasi::io::streams::OutputStream> {
        wasmtime::component::Resource::<wasi::io::streams::OutputStream>::new_own(10000)
    }
}

impl wasi::cli::stdout::Host for MyState {
    fn get_stdout(&mut self) -> wasmtime::component::Resource<wasi::io::streams::OutputStream> {
        wasmtime::component::Resource::<wasi::io::streams::OutputStream>::new_own(10000)
    }
}

impl wasi::cli::stdin::Host for MyState {
    fn get_stdin(&mut self) -> wasmtime::component::Resource<wasi::io::streams::InputStream> {
        wasmtime::component::Resource::<wasi::io::streams::InputStream>::new_own(10000)
    }
}

impl wasi::io::streams::OutputStream {}

impl wasi::io::streams::HostOutputStream for MyState {
    fn check_write(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Ok(0) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn write(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _contents: wasmtime::component::__internal::Vec<u8>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(()) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn blocking_write_and_flush(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _contents: wasmtime::component::__internal::Vec<u8>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(()) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn blocking_flush(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(()) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::io::streams::HostInputStream for MyState {
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::streams::InputStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::io::streams::Host for MyState {}
impl wasi::io::error::HostError for MyState {
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::error::Error>,
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
// fn type_annotate<T: WasiView, F>(val: F) -> F
// where
//     F: Fn(&mut T) -> WasiImpl<&mut T>,
// {
//     val
// }

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

// struct MyLibrary;

// impl HasData for MyLibrary {
//     type Data<'a> = MyState<'a>;
// }
struct HasWasi<T>(T);

impl<T: 'static> HasData for HasWasi<T> {
    type Data<'a> = WasiImpl<&'a mut T>;
}

struct MyLibrary;

impl HasData for MyLibrary {
    type Data<'a> = &'a mut MyState;
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
    // filesystem::types::add_to_linker::<T, HasWasi<T>>(l, f)?; // DONT USE, async
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
    Ok(())
}

pub(super) fn execute_run_wasm_concat_multiarg(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("RUNWASM"))?;
    fetch_stack(engine, 8)?;

    let (wasm_executable, wasm_hash) = check_and_get_wasm_by_hash(engine, 0, 7)?;

    // let s = engine.cmd.var(0).as_cell()?;
    // let wasm_executable = rejoin_chain_of_cells(s)?;

    // get exported instance name to call
    let s = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let wasm_instance_name = unpack_data_from_cell(s, engine)?;
    let wasm_instance_name = String::from_utf8(wasm_instance_name)?;

    // get exported func to call from within instance
    let s = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let wasm_func_name = unpack_data_from_cell(s, engine)?;
    let wasm_func_name = String::from_utf8(wasm_func_name)?;

    // execute wasm func
    // collect result
    // substract gas based on wasm fuel used
    let s = engine.cmd.var(3).as_cell()?;
    log::debug!("Loading WASM Args");
    let mut wasm_func_args =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            _ => err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction")?,
        };
    let s = engine.cmd.var(4).as_cell()?;
    let mut wasm_args_tail =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction {:?}", e)?,
        };
    wasm_func_args.append(&mut wasm_args_tail);
    let s = engine.cmd.var(5).as_cell()?;
    let mut wasm_args_tail =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction {:?}", e)?,
        };
    wasm_func_args.append(&mut wasm_args_tail);
    let s = engine.cmd.var(6).as_cell()?;
    let mut wasm_args_tail =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction {:?}", e)?,
        };
    wasm_func_args.append(&mut wasm_args_tail);
    log::debug!("WASM Args loaded {:?}", wasm_func_args);

    run_wasm_core(
        engine,
        wasm_executable,
        &wasm_func_name,
        &wasm_instance_name,
        wasm_func_args,
        wasm_hash,
    )
}

// execute wasm binary
pub(super) fn execute_run_wasm(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("RUNWASM"))?;
    fetch_stack(engine, 5)?;

    let (wasm_executable, wasm_hash) = check_and_get_wasm_by_hash(engine, 0, 4)?;
    // let s = engine.cmd.var(0).as_cell()?;
    // let wasm_executable = rejoin_chain_of_cells(s)?;

    // get exported instance name to call
    let s = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let wasm_instance_name = unpack_data_from_cell(s, engine)?;
    let wasm_instance_name = String::from_utf8(wasm_instance_name)?;

    // get exported func to call from within instance
    let s = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let wasm_func_name = unpack_data_from_cell(s, engine)?;
    let wasm_func_name = String::from_utf8(wasm_func_name)?;

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

    run_wasm_core(
        engine,
        wasm_executable,
        &wasm_func_name,
        &wasm_instance_name,
        wasm_func_args,
        wasm_hash,
    )
}

fn check_and_get_wasm_by_hash(
    engine: &mut Engine,
    exec_index: usize,
    hash_index: usize,
) -> Result<(Vec<u8>, Option<[u8; 32]>), failure::Error> {
    // load wasm component binary
    #[cfg(feature = "wasm_external")]
    let wasm_executable = {
        let s = engine.cmd.var(exec_index).as_cell()?;
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(ExceptionCode::WasmLoadFail, "Failed to unpack wasm instruction {:?}", e)?,
        }
    };
    #[cfg(not(feature = "wasm_external"))]
    let wasm_executable = {
        let _e = exec_index; // avoid linter error
        Vec::<u8>::new()
    };
    let wasm_hash_mode = wasm_executable.is_empty();
    if wasm_hash_mode {
        let s = engine.cmd.var(hash_index).as_cell()?;
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
        Ok((
            engine.get_wasm_binary_by_hash(wasm_hash.clone())?,
            Some(match wasm_hash.try_into() {
                Ok(h) => h,
                Err(e) => err!(
                    ExceptionCode::WasmLoadFail,
                    "Failed to turn valid hash into [u8; 32]. This is probably a bug. {:?}",
                    e
                )?,
            }),
        ))
        // todo!("Add hash lookup here from hash {:?}", wasm_hash);
    } else {
        Ok((wasm_executable, None))
    }
}

// Shared functionality for all wasm instructions
fn run_wasm_core(
    engine: &mut Engine,
    wasm_executable: Vec<u8>,
    wasm_func_name: &str,
    wasm_instance_name: &str,
    wasm_func_args: Vec<u8>,
    wasm_hash: Option<[u8; 32]>,
) -> Status {
    let mut builder = WasiCtxBuilder::new();
    let mut wasm_store: wasmtime::Store<MyState> = engine.create_wasm_store(MyState {
        ctx: builder.build(),
        table: wasmtime::component::ResourceTable::new(),
        limiter: wasmtime::StoreLimitsBuilder::new()
            .memory_size(1 << 25 /* 32 MB */)
            .instances(50)
            .memories(100)
            .tables(1000)
            .table_elements(1000000)
            .trap_on_grow_failure(true)
            .build(),
    })?;
    wasm_store.limiter(|state| &mut state.limiter);
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

    let wasm_component = match wasm_hash {
        Some(h) => match engine.get_precompiled_wasm_component(h) {
            Some(c) => c,
            None => &engine.create_single_use_wasm_component(wasm_executable)?,
        },
        None => &engine.create_single_use_wasm_component(wasm_executable)?,
    };

    engine.print_wasm_component_exports_and_imports(&wasm_component)?;

    // Add wasi-cli libs to linker
    let mut wasm_linker = wasmtime::component::Linker::<MyState>::new(engine.get_wasm_engine()?);
    let mut wasm_linker = wasm_linker.allow_shadowing(true);
    // match wasm_linker.define_unknown_imports_as_traps(&wasm_component) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         err!(ExceptionCode::WasmLoadFail, "Failed to instantiate WASM
    // instance traps {:?}", e)?
    //     }
    // };

    // let f: fn(&mut MyState) -> WasiImpl<&mut MyState> = |t| WasiImpl(IoImpl(t));
    // let f: fn(&mut MyState) -> WasiImpl<&mut MyState> = |t| WasiImpl(IoImpl(t));
    // let f: fn(&mut MyState) -> &mut WasiImpl<&mut MyState> = |t| &mut
    // WasiImpl(IoImpl(t));

    // This is a custom linker method, adding only sync, non-io wasi dependencies.
    // If more deps are needed, add them in there!
    match add_to_linker_gosh::<MyState>(&mut wasm_linker) {
        Ok(_) => {}
        Err(e) => err!(
            ExceptionCode::WasmLoadFail,
            "Failed to instantiate WASM
    instance {:?}",
            e
        )?,
    };

    let f: fn(&mut MyState) -> &mut MyState = |s| s;
    // let f: fn(&mut MyState) -> &mut WasiImpl<IoImpl<&mut MyState>> = |t| t;
    match Localworld::add_to_linker::<MyState, MyLibrary>(&mut wasm_linker, f) {
        Ok(_) => {}
        Err(e) => err!(ExceptionCode::WasmLoadFail, "Failed to link IO Plugs {:?}", e)?,
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

    // get callable wasm func
    log::debug!("Callable funcs found:");
    for export in wasm_component.component_type().exports(engine.get_wasm_engine()?) {
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

pub(super) fn execute_calculate_repcoef(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCREPCOEF"))?;
    fetch_stack(engine, 1)?;
    let bkrt = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as u128;
    let repcoef = repcoef_int(bkrt);
    engine.cc.stack.push(int!(repcoef));
    Ok(())
}

pub(super) fn execute_calculate_adjustment_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBKREWARDADJ"))?;
    fetch_stack(engine, 5)?;
    let t = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?; //time from network start
    let rbkprev = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //previous value of rewardadjustment (not minimum)
    let mut drbkavg = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?;
    //_delta_reward = (_delta_reward * _calc_reward_num + (block.timestamp -
    //_delta_reward _reward_last_time)) / (_calc_reward_num + 1);
    //_delta_reward - average time between reward adj calculate
    //_calc_reward_num - number of calculate
    //_reward_last_time - time of last calculate
    let repavgbig = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?; //Average ReputationCoef
    let mbkt = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)?; //sum of reward token (minted, include slash token)
    let mut repavg = repavgbig / 1_000_000_000;
    let rbkmin;
    if t <= TTMT - 1 {
        rbkmin = rbkprev / 3 * 2; 
    } else {
        rbkmin = 0;
    }
    if drbkavg == 0 {
        drbkavg = 1;
    }
    if repavg == 0 {
        repavg = 1;
    }
    let rbk = (((calc_mbk(t + drbkavg, KRBK_NUM, KRBK_DEN) - mbkt) / drbkavg / repavg).max(rbkmin)).min(rbkprev);
    engine.cc.stack.push(int!(rbk as u128));
    Ok(())
}

pub(super) fn execute_calculate_adjustment_reward_bmmv(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBMMVREWARDADJ"))?;
    fetch_stack(engine, 5)?;
    let is_bm = engine.cmd.var(0).as_bool()?; 
    let t = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //time from network start
    let rbmprev = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?; //previous value of rewardadjustment (not minimum)
    let mut drbmavg = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?;
    let mbmt = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)?; //sum of reward token (minted, include slash token)
    let rbmmin;
    if t <= TTMT - 1 {
        rbmmin = rbmprev / 3 * 2; 
    } else {
        rbmmin = 0;
    }   
    let rbm;
    if drbmavg == 0 {
        drbmavg = 1;
    }
    if is_bm {
        rbm = (((calc_mbk(t + drbmavg, KRBM_NUM, KRBM_DEN) - mbmt) / drbmavg).max(rbmmin)).min(rbmprev);
    } else {
        rbm = (((calc_mbk(t + drbmavg, KRMV_NUM, KRMV_DEN) - mbmt) / drbmavg).max(rbmmin)).min(rbmprev);
    }
    engine.cc.stack.push(int!(rbm as u128));
    Ok(())
}

fn exp_neg_q32(v_q32: i64) -> i64 {
    let n = (v_q32 >> 32) as usize;     
    if n >= EXP_NEG_VAL_Q32.len() { return 0; }
    let f = v_q32 & (ONE_Q32 - 1);  
    let int_part  = EXP_NEG_VAL_Q32[n];
    let frac_part = horner_q32(f, &MAC_EXP_NEG_COEFF_Q32);
    ((int_part as i128 * frac_part as i128) >> 32) as i64
}

fn calc_mbk(t: u128, krk_num: u128, krk_den: u128) -> u128 {
    let mbk: u128 = if t > TTMT {
        TOTALSUPPLY
    } else {
        let v_q32 = ((UM_Q64 as i128 * t as i128 + (1 << 31)) >> 32) as i64;
        let exp_q32  = exp_neg_q32(v_q32);
        let diff_q32 = ONE_Q32 - exp_q32;
        let prod_q32 = ((ONE_PLUS_KM as i128 * diff_q32 as i128) >> 32) as i64;
        ((TOTALSUPPLY as i128 * prod_q32 as i128) >> 32) as u128
    };
    (mbk * krk_num) / krk_den
}

pub(super) fn execute_calculate_validator_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBKREWARD"))?;
    fetch_stack(engine, 7)?;
    let mut repcoef = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?; //average reputation coef of licenses in one stake
    let bkstake = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //value of stake
    let totalbkstake = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?; //sum of stakes at start of epoch
    let t = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?; //duration of epoch
    let mbk = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)?; //sum of reward token (minted, include slash token)
    let nbk = engine.cmd.var(5).as_integer()?.into(0..=u128::MAX)?; //numberOfActiveBlockKeepers
    let rbk = engine.cmd.var(6).as_integer()?.into(0..=u128::MAX)?; //last calculated reward_adjustment
    repcoef = repcoef / 1000000000;
    let reward;
    if totalbkstake == 0 {
        if nbk == 0 {
            reward = 0;
        } else {
            reward = rbk * t * repcoef / nbk;
        }
    } else if mbk < TOTALSUPPLY {
        reward = rbk * t * repcoef * bkstake / totalbkstake;
    } else {
        reward = 0;
    }
    engine.cc.stack.push(int!(reward as u128));
    Ok(())
}

pub(super) fn execute_calculate_block_manager_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBMREWARD"))?;
    fetch_stack(engine, 5)?;
    let radj = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?;
    let depoch = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?;
    let mbm = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?;
    let count_bm = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?;
    let _pubkey_cell = engine.cmd.var(4).as_cell()?;
    let reward;
    if mbm >= TOTALSUPPLY / 10 || count_bm == 0 {
        reward = 0;
    } else {
        reward = radj * depoch / count_bm;
    }
    engine.cc.stack.push(int!(reward as u128));
    Ok(())
}

fn calc_one_minus_fstk_q32_int(t: u128) -> u128 {
    let fstk_q32: i64 = if t > TTMT {
        MAX_FREE_FLOAT_FRAC_Q32                         
    } else {
        let v_q32 = ((UF_Q64 as i128 * t as i128 + (1 << 31)) >> 32) as i64;
        let diff_q32 = ONE_Q32 - exp_neg_q32(v_q32);
        let tmp_q32 = ((ONE_PLUS_KF_Q32 as i128 * diff_q32 as i128) >> 32) as i64;
        ((MAX_FREE_FLOAT_FRAC_Q32 as i128 * tmp_q32 as i128) >> 32) as i64
    };
    (ONE_Q32 - fstk_q32) as u128
}

pub(super) fn execute_calculate_min_stake(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKE"))?;
    fetch_stack(engine, 4)?;
    let _nbkreq = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?; // needNumberOfActiveBlockKeepers = 10000
    let nbk = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //numberOfActiveBlockKeepersAtBlockStart
    let tstk = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?; //time from network start + uint128(_waitStep / 3) where waitStep - number of block duration of preEpoch
    let mbkav = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?; //sum of reward token without slash tokens
    let sbkbase;
    if mbkav != 0 {
        let one_minus_fstk_q32 = calc_one_minus_fstk_q32_int(tstk);
        sbkbase = ((mbkav as u128 * one_minus_fstk_q32 as u128) >> 32) / 2 / nbk as u128;
    } else {
        sbkbase = 0;
    }
    engine.cc.stack.push(int!(sbkbase as u128));
    Ok(())
}

pub(super) fn execute_calculate_min_stake_bm(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKEBM"))?;
    fetch_stack(engine, 2)?;
    let tstk = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?; //time from network start 
    let mbkav = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //sum of reward token without slash tokens
    let one_minus_fstk_q32 = calc_one_minus_fstk_q32_int(tstk);
    let sbkmin = ((mbkav as u128 * one_minus_fstk_q32 as u128) >> 32) as u128;
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

/*
fn _boost_coef_integral_calculation(bl: f64, br: f64, xl: f64, xr: f64, yd: f64, yu: f64, k: f64) -> f64 {
    let dx = xr - xl;
    let expk = k.exp();
    ((yu - yd) * dx * ((k * (br - xl) / dx).exp() - (k * (bl - xl) / dx).exp())
        + k * (br - bl) * (yd * expk - yu))
        / (k * (expk - 1_f64))
}


fn _boost_coef_calculation(dl: f64, dr: f64, x1: f64, x2: f64, x3: f64, x4: f64, y1: f64, y2: f64, y3: f64, y4: f64, k1: f64, k2: f64, k3: f64) -> f64 {
    let mut bc = 0_f64;
    if x1 <= dl && dl <= x2 {
        if x1 <= dr && dr <= x2 {
            bc = boost_coef_integral_calculation(dl, dr, x1, x2, y1, y2, k1);
        } else if x2 < dr && dr <= x3 {
            bc = boost_coef_integral_calculation(dl, x2, x1, x2, y1, y2, k1)
                + boost_coef_integral_calculation(x2, dr, x2, x3, y2, y3, k2);
        } else if x3 < dr && dr <= x4 {
            bc = boost_coef_integral_calculation(dl, x2, x1, x2, y1, y2, k1)
                + boost_coef_integral_calculation(x2, x3, x2, x3, y2, y3, k2)
                + boost_coef_integral_calculation(x3, dr, x3, x4, y3, y4, k3);
        }
    } else if x2 < dl && dl <= x3 {
        if x2 < dr && dr <= x3 {
            bc = boost_coef_integral_calculation(dl, dr, x2, x3, y2, y3, k2);
        } else if x3 < dr && dr <= x4 {
            bc = boost_coef_integral_calculation(dl, x3, x2, x3, y2, y3, k2)
                + boost_coef_integral_calculation(x3, dr, x3, x4, y3, y4, k3);
        }
    } else if x3 < dl && dl <= x4 {
        if x3 < dr && dr <= x4 {
            bc = boost_coef_integral_calculation(dl, dr, x3, x4, y3, y4, k3);
        }
    }
    bc
}


fn _calculate_sum_boost_coefficients(
    lst: &[f64], x1: f64, x2: f64, x3: f64, x4: f64,
    y1: f64, y2: f64, y3: f64, y4: f64,
    k1: f64, k2: f64, k3: f64
) -> Vec<f64> {
    let mut total_lst: f64 = lst.iter().sum();
    if total_lst == 0_f64 {
        total_lst = 1_f64;
    }
    let mut cumulative_sum = 0_f64;
    lst
        .iter()
        .map(|&value| {
            let left_border = cumulative_sum / total_lst;
            cumulative_sum += value;
            let right_border = cumulative_sum / total_lst;
            boost_coef_calculation(left_border, right_border, x1, x2, x3, x4, y1, y2, y3, y4, k1, k2, k3)
        })
        .collect()
}

fn _validate_byte_array(bytes: &[u8], name: &str) -> anyhow::Result<()> {
    if bytes.len() % 8 != 0 {
        anyhow::bail!(
            "{}: byte length must be multiple of 8 (got {})", 
            name, bytes.len()
        );
    } else if bytes.len() > 8000 {
        anyhow::bail!(
            "{}: byte length exceeds 8000 bytes (got {})", 
            name, bytes.len()
        );
    }
    Ok(())
}*/

pub(super) fn execute_calculate_boost_coef(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBOOSTCOEF"))?;
    fetch_stack(engine, 2)?;
    let _s = engine.cmd.var(0).as_cell()?;
    let _s1 = engine.cmd.var(1).as_cell()?;
    
    let total_boost_coef_list_bytes: Vec<u8> = Vec::new();
    let cell = TokenValue::write_bytes(total_boost_coef_list_bytes.as_slice(), &ABI_VERSION_2_4)?.into_cell()?;   

    engine.cc.stack.push(StackItem::cell(cell));
    engine.cc.stack.push(int!(0));
    Ok(())
    /*
    let x1 = 0_f64;
    let x2 = 0.3_f64;
    let x3 = 0.7_f64;
    let x4 = 1_f64;
    let y1 = 0_f64;
    let y2 = 0.066696948409_f64;
    let y3 = 2_f64;
    let y4 = 8_f64;
    let k1 = 10_f64;
    let k2 = 1.894163612445_f64;    
    let k3 = 17.999995065464_f64;     
    let (token_value, _) = TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)
        .map_err(|e| exception!(ExceptionCode::TypeCheckError, "Failed to read cell s: {}", e))?;
    let transformed_users_per_item = match token_value {
        TokenValue::Bytes(data) => data,
        _ => return err!(ExceptionCode::TypeCheckError, "Expected Bytes in cell s"),
    };
    validate_byte_array(transformed_users_per_item.as_slice(), "s")
        .map_err(|e| exception!(ExceptionCode::TypeCheckError, "{}", e))?;
        
    let vec_u64: Vec<u64> = transformed_users_per_item
        .as_slice()
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    let mbnlst: Vec<f64> = vec_u64.iter().map(|&x| x as f64).collect();
    let mbnlst_orig = vec_u64.clone();

    let (token_value, _) = TokenValue::read_bytes(SliceData::load_cell(s1.clone())?, true, &ABI_VERSION_2_4)
        .map_err(|e| exception!(ExceptionCode::TypeCheckError, "Failed to read cell s1: {}", e))?;
    let glst_bytes = match token_value {
        TokenValue::Bytes(data) => data,
        _ => return err!(ExceptionCode::TypeCheckError, "Expected Bytes in cell s1"),
    };
    validate_byte_array(glst_bytes.as_slice(), "s1")
        .map_err(|e| exception!(ExceptionCode::TypeCheckError, "{}", e))?;
        
    let glst: Vec<u64> = glst_bytes
        .as_slice()
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    
    let total_boost_coef_list = calculate_sum_boost_coefficients(
        &mbnlst, x1, x2, x3, x4,
        y1, y2, y3, y4,
        k1, k2, k3
    );

    let total_boost_coef_list_u64: Vec<u64> = total_boost_coef_list.iter().map(|&x| (x * 1e9_f64) as u64).collect();

    let total_boost_coef_list_bytes: Vec<u8> = total_boost_coef_list_u64.iter()
        .flat_map(|val| val.to_le_bytes())
        .collect();

    let cell = TokenValue::write_bytes(total_boost_coef_list_bytes.as_slice(), &ABI_VERSION_2_4)?.into_cell()?;   
    let total = mbnlst_orig.iter()
        .zip(glst)
        .map(|(&x, y)| u128::from(x) * u128::from(y))
        .fold(0u128, |acc, val| acc.saturating_add(val));
    engine.cc.stack.push(StackItem::cell(cell));
    engine.cc.stack.push(int!(total));
    Ok(())
    */
}


pub(super) fn execute_calculate_mobile_verifiers_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCMVREWARD"))?;
    fetch_stack(engine, 5)?;
    let _mbn = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let _g = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let _sum = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let _radj = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let _depoch = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as f64;
    engine.cc.stack.push(int!(0 as u128));
    Ok(())
    /*
    let u = mbn * g / sum;
    let reward;
    if sum >= TOTALSUPPLY as f64 * KRMV {
        reward = 0_f64;
    } else {
        reward = radj * depoch * u * 1e9_f64;
    }
    engine.cc.stack.push(int!(reward as u128));
    Ok(())
    */
}



pub(super) fn execute_mint_shellq(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("MINTSHELLQ"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_MINT_SHELLQ_TOKEN, None, cell)
}

pub(super) fn execute_send_to_dapp_config(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("SENDTODAPPCONFIG"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_SEND_TO_DAPP_CONFIG, None, cell)
}

pub(super) fn execute_get_available_balance(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("GETAVAILABLEBALANCE"))?;
    let mut balance = engine.get_available_credit();  
    if balance < 0 {
        balance = 0;
    }
    engine.cc.stack.push(int!(balance as u128));
    Ok(())
}

fn horner_q32<const N: usize>(f_q32: i64, coeffs: &[i64; N]) -> i64 {
    let mut acc = coeffs[N - 1];
    for &c in coeffs[..N - 1].iter().rev() {
        acc = (((acc as i128 * f_q32 as i128) >> 32) as i64) + c;
    }
    acc
}

fn rep_coef_pow2_horner_q32(f: i64) -> i64 {
    let mut acc = RC_POW2_COEFF[5];
    for &c in RC_POW2_COEFF[..5].iter().rev() {
        acc = (((acc as i128 * f as i128) >> 32) as i64) + c;
    }
    acc
}

fn rep_coef_exp_q32(x_q32: i64) -> i64 {
    let y_q64 = x_q32 as i128 * RC_K1_Q32 as i128;
    let y_q32 = (y_q64 >> 32) as i64;
    let i = (y_q32 >> 32) as i32;
    let f = y_q32 & (RC_ONE_Q32 - 1);
    let pow2_i_q32 = RC_ONE_Q32 >> (-i as u32);
    let pow2_f_q32 = rep_coef_pow2_horner_q32(f);
    ((pow2_i_q32 as i128 * pow2_f_q32 as i128) >> 32) as i64
}

fn repcoef_int(bkrt: u128) -> u128 {
    if bkrt == 0 { return 1_000_000_000; }
    if bkrt >= MAXRT { return 3_000_000_000; }
    let x_q32 = -(RC_K2_Q32 * bkrt as i64);
    let diff_q32 = RC_ONE_Q32 - rep_coef_exp_q32(x_q32);
    let rep_q32  = RC_ONE_Q32 + (((RC_K3_Q32 as i128 * diff_q32 as i128) >> 32) as i64);
    ((rep_q32 as i128 * RCSCALE) >> 32) as u128
}

