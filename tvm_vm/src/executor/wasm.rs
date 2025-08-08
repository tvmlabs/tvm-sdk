use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;
use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
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
use wasmtime_wasi::p2::bindings::clocks::wall_clock::Datetime;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::stack::StackItem;
use crate::types::Exception;
use crate::types::Status;

pub const WASM_FUEL_MULTIPLIER: u64 = 2220000u64;
pub const WASM_200MS_FUEL: u64 = 2220000000u64;
pub const RUNWASM_GAS_PRICE: u64 = WASM_200MS_FUEL / WASM_FUEL_MULTIPLIER;

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

                now: func() -> datetime;

                resolution: func() -> datetime;
            }
        }

        package wasi:random@0.2.3 {
            interface random {
                get-random-bytes: func(len: u64) -> list<u8>;
                get-random-u64: func() -> u64;
            }

            interface insecure {
                get-insecure-random-bytes: func(len: u64) -> list<u8>;

                get-insecure-random-u64: func() -> u64;
            }

            interface insecure-seed {
                insecure-seed: func() -> tuple<u64, u64>;
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
            import wasi:random/random@0.2.3;
            import wasi:random/insecure@0.2.3;
            import wasi:random/insecure-seed@0.2.3;
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
    random_source: rand_chacha::ChaCha20Rng,
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

impl wasi::clocks::wall_clock::Host for MyState {
    fn now(&mut self) -> wasi::clocks::wall_clock::Datetime {
        wasi::clocks::wall_clock::Datetime { nanoseconds: 0, seconds: 0 }
    }

    fn resolution(&mut self) -> wasi::clocks::wall_clock::Datetime {
        wasi::clocks::wall_clock::Datetime { nanoseconds: 0, seconds: 0 }
    }
}

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

impl wasi::random::random::Host for MyState {
    fn get_random_bytes(&mut self, len: u64) -> wasmtime::component::__internal::Vec<u8> {
        let mut vector = Vec::<u8>::with_capacity(match len.try_into() {
            Ok(k) => k,
            Err(_) => u16::max_value().into(),
        });
        self.random_source.fill_bytes(&mut vector);
        vector
    }

    fn get_random_u64(&mut self) -> u64 {
        self.random_source.next_u64()
    }
}

impl wasi::random::insecure::Host for MyState {
    fn get_insecure_random_bytes(&mut self, len: u64) -> wasmtime::component::__internal::Vec<u8> {
        let mut vector = Vec::<u8>::with_capacity(match len.try_into() {
            Ok(k) => k,
            Err(_) => u16::max_value().into(),
        });
        self.random_source.fill_bytes(&mut vector);
        vector
    }

    fn get_insecure_random_u64(&mut self) -> u64 {
        self.random_source.next_u64()
    }
}

impl wasi::random::insecure_seed::Host for MyState {
    fn insecure_seed(&mut self) -> (u64, u64) {
        (self.random_source.next_u64(), self.random_source.next_u64())
    }
}

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
    // use wasmtime_wasi::p2::bindings::random;

    // wasmtime_wasi::p2::add_to_linker_sync(linker)
    let options = wasmtime_wasi::p2::bindings::sync::LinkOptions::default();
    let l = wasm_linker;
    let f: fn(&mut T) -> WasiImpl<&mut T> = |t| WasiImpl(IoImpl(t));
    // clocks::wall_clock::add_to_linker::<T, HasWasi<T>>(l, f)?;
    // clocks::monotonic_clock::add_to_linker::<T, HasWasi<T>>(l, f)?;
    filesystem::preopens::add_to_linker::<T, HasWasi<T>>(l, f)?;
    // filesystem::types::add_to_linker::<T, HasWasi<T>>(l, f)?; // DONT USE, async
    // random::random::add_to_linker::<T, HasWasi<T>>(l, f)?;
    // random::insecure::add_to_linker::<T, HasWasi<T>>(l, f)?;
    // random::insecure_seed::add_to_linker::<T, HasWasi<T>>(l, f)?;
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

pub(crate) fn check_and_get_wasm_by_hash(
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
pub(crate) fn run_wasm_core(
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
        random_source: rand_chacha::ChaCha20Rng::seed_from_u64(42),
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
