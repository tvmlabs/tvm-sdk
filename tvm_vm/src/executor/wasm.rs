use std::io::Write;

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
            
            /// WASI I/O is an I/O abstraction API which is currently focused on providing
            /// stream types.
            ///
            /// In the future, the component model is expected to add built-in stream types;
            /// when it does, they are expected to subsume this API.
            @since(version = 0.2.0)
            interface streams {
                @since(version = 0.2.0)
                use error.{error};
                @since(version = 0.2.0)
                use poll.{pollable};

                /// An error for input-stream and output-stream operations.
                @since(version = 0.2.0)
                variant stream-error {
                    /// The last operation (a write or flush) failed before completion.
                    ///
                    /// More information is available in the `error` payload.
                    ///
                    /// After this, the stream will be closed. All future operations return
                    /// `stream-error::closed`.
                    last-operation-failed(error),
                    /// The stream is closed: no more input will be accepted by the
                    /// stream. A closed output-stream will return this error on all
                    /// future operations.
                    closed
                }

                /// An input bytestream.
                ///
                /// `input-stream`s are *non-blocking* to the extent practical on underlying
                /// platforms. I/O operations always return promptly; if fewer bytes are
                /// promptly available than requested, they return the number of bytes promptly
                /// available, which could even be zero. To wait for data to be available,
                /// use the `subscribe` function to obtain a `pollable` which can be polled
                /// for using `wasi:io/poll`.
                @since(version = 0.2.0)
                resource input-stream {
                    /// Perform a non-blocking read from the stream.
                    ///
                    /// When the source of a `read` is binary data, the bytes from the source
                    /// are returned verbatim. When the source of a `read` is known to the
                    /// implementation to be text, bytes containing the UTF-8 encoding of the
                    /// text are returned.
                    ///
                    /// This function returns a list of bytes containing the read data,
                    /// when successful. The returned list will contain up to `len` bytes;
                    /// it may return fewer than requested, but not more. The list is
                    /// empty when no bytes are available for reading at this time. The
                    /// pollable given by `subscribe` will be ready when more bytes are
                    /// available.
                    ///
                    /// This function fails with a `stream-error` when the operation
                    /// encounters an error, giving `last-operation-failed`, or when the
                    /// stream is closed, giving `closed`.
                    ///
                    /// When the caller gives a `len` of 0, it represents a request to
                    /// read 0 bytes. If the stream is still open, this call should
                    /// succeed and return an empty list, or otherwise fail with `closed`.
                    ///
                    /// The `len` parameter is a `u64`, which could represent a list of u8 which
                    /// is not possible to allocate in wasm32, or not desirable to allocate as
                    /// as a return value by the callee. The callee may return a list of bytes
                    /// less than `len` in size while more bytes are available for reading.
                    @since(version = 0.2.0)
                    read: func(
                        /// The maximum number of bytes to read
                        len: u64
                    ) -> result<list<u8>, stream-error>;

                    /// Read bytes from a stream, after blocking until at least one byte can
                    /// be read. Except for blocking, behavior is identical to `read`.
                    @since(version = 0.2.0)
                    blocking-read: func(
                        /// The maximum number of bytes to read
                        len: u64
                    ) -> result<list<u8>, stream-error>;

                    /// Skip bytes from a stream. Returns number of bytes skipped.
                    ///
                    /// Behaves identical to `read`, except instead of returning a list
                    /// of bytes, returns the number of bytes consumed from the stream.
                    @since(version = 0.2.0)
                    skip: func(
                        /// The maximum number of bytes to skip.
                        len: u64,
                    ) -> result<u64, stream-error>;

                    /// Skip bytes from a stream, after blocking until at least one byte
                    /// can be skipped. Except for blocking behavior, identical to `skip`.
                    @since(version = 0.2.0)
                    blocking-skip: func(
                        /// The maximum number of bytes to skip.
                        len: u64,
                    ) -> result<u64, stream-error>;

                    /// Create a `pollable` which will resolve once either the specified stream
                    /// has bytes available to read or the other end of the stream has been
                    /// closed.
                    /// The created `pollable` is a child resource of the `input-stream`.
                    /// Implementations may trap if the `input-stream` is dropped before
                    /// all derived `pollable`s created with this function are dropped.
                    @since(version = 0.2.0)
                    subscribe: func() -> pollable;
                }


                /// An output bytestream.
                ///
                /// `output-stream`s are *non-blocking* to the extent practical on
                /// underlying platforms. Except where specified otherwise, I/O operations also
                /// always return promptly, after the number of bytes that can be written
                /// promptly, which could even be zero. To wait for the stream to be ready to
                /// accept data, the `subscribe` function to obtain a `pollable` which can be
                /// polled for using `wasi:io/poll`.
                ///
                /// Dropping an `output-stream` while there's still an active write in
                /// progress may result in the data being lost. Before dropping the stream,
                /// be sure to fully flush your writes.
                @since(version = 0.2.0)
                resource output-stream {
                    /// Check readiness for writing. This function never blocks.
                    ///
                    /// Returns the number of bytes permitted for the next call to `write`,
                    /// or an error. Calling `write` with more bytes than this function has
                    /// permitted will trap.
                    ///
                    /// When this function returns 0 bytes, the `subscribe` pollable will
                    /// become ready when this function will report at least 1 byte, or an
                    /// error.
                    @since(version = 0.2.0)
                    check-write: func() -> result<u64, stream-error>;

                    /// Perform a write. This function never blocks.
                    ///
                    /// When the destination of a `write` is binary data, the bytes from
                    /// `contents` are written verbatim. When the destination of a `write` is
                    /// known to the implementation to be text, the bytes of `contents` are
                    /// transcoded from UTF-8 into the encoding of the destination and then
                    /// written.
                    ///
                    /// Precondition: check-write gave permit of Ok(n) and contents has a
                    /// length of less than or equal to n. Otherwise, this function will trap.
                    ///
                    /// returns Err(closed) without writing if the stream has closed since
                    /// the last call to check-write provided a permit.
                    @since(version = 0.2.0)
                    write: func(
                        contents: list<u8>
                    ) -> result<_, stream-error>;

                    /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                    /// until all of these operations are complete, or an error occurs.
                    ///
                    /// Returns success when all of the contents written are successfully
                    /// flushed to output. If an error occurs at any point before all
                    /// contents are successfully flushed, that error is returned as soon as
                    /// possible. If writing and flushing the complete contents causes the
                    /// stream to become closed, this call should return success, and
                    /// subsequent calls to check-write or other interfaces should return
                    /// stream-error::closed.
                    @since(version = 0.2.0)
                    blocking-write-and-flush: func(
                        contents: list<u8>
                    ) -> result<_, stream-error>;

                    /// Request to flush buffered output. This function never blocks.
                    ///
                    /// This tells the output-stream that the caller intends any buffered
                    /// output to be flushed. the output which is expected to be flushed
                    /// is all that has been passed to `write` prior to this call.
                    ///
                    /// Upon calling this function, the `output-stream` will not accept any
                    /// writes (`check-write` will return `ok(0)`) until the flush has
                    /// completed. The `subscribe` pollable will become ready when the
                    /// flush has completed and the stream can accept more writes.
                    @since(version = 0.2.0)
                    flush: func() -> result<_, stream-error>;

                    /// Request to flush buffered output, and block until flush completes
                    /// and stream is ready for writing again.
                    @since(version = 0.2.0)
                    blocking-flush: func() -> result<_, stream-error>;

                    /// Create a `pollable` which will resolve once the output-stream
                    /// is ready for more writing, or an error has occurred. When this
                    /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                    /// error.
                    ///
                    /// If the stream is closed, this pollable is always ready immediately.
                    ///
                    /// The created `pollable` is a child resource of the `output-stream`.
                    /// Implementations may trap if the `output-stream` is dropped before
                    /// all derived `pollable`s created with this function are dropped.
                    @since(version = 0.2.0)
                    subscribe: func() -> pollable;

                    /// Write zeroes to a stream.
                    ///
                    /// This should be used precisely like `write` with the exact same
                    /// preconditions (must use check-write first), but instead of
                    /// passing a list of bytes, you simply pass the number of zero-bytes
                    /// that should be written.
                    @since(version = 0.2.0)
                    write-zeroes: func(
                        /// The number of zero-bytes to write
                        len: u64
                    ) -> result<_, stream-error>;

                    /// Perform a write of up to 4096 zeroes, and then flush the stream.
                    /// Block until all of these operations are complete, or an error
                    /// occurs.
                    ///
                    /// Functionality is equivelant to `blocking-write-and-flush` with
                    /// contents given as a list of len containing only zeroes.
                    @since(version = 0.2.0)
                    blocking-write-zeroes-and-flush: func(
                        /// The number of zero-bytes to write
                        len: u64
                    ) -> result<_, stream-error>;

                    /// Read from one stream and write to another.
                    ///
                    /// The behavior of splice is equivalent to:
                    /// 1. calling `check-write` on the `output-stream`
                    /// 2. calling `read` on the `input-stream` with the smaller of the
                    /// `check-write` permitted length and the `len` provided to `splice`
                    /// 3. calling `write` on the `output-stream` with that read data.
                    ///
                    /// Any error reported by the call to `check-write`, `read`, or
                    /// `write` ends the splice and reports that error.
                    ///
                    /// This function returns the number of bytes transferred; it may be less
                    /// than `len`.
                    @since(version = 0.2.0)
                    splice: func(
                        /// The stream to read from
                        src: borrow<input-stream>,
                        /// The number of bytes to splice
                        len: u64,
                    ) -> result<u64, stream-error>;

                    /// Read from one stream and write to another, with blocking.
                    ///
                    /// This is similar to `splice`, except that it blocks until the
                    /// `output-stream` is ready for writing, and the `input-stream`
                    /// is ready for reading, before performing the `splice`.
                    @since(version = 0.2.0)
                    blocking-splice: func(
                        /// The stream to read from
                        src: borrow<input-stream>,
                        /// The number of bytes to splice
                        len: u64,
                    ) -> result<u64, stream-error>;
                }
            }
                    
            world ioer {
            import error;
            import streams;
            }


            /// A poll API intended to let users wait for I/O events on multiple handles
            /// at once.
            @since(version = 0.2.0)
            interface poll {
                /// `pollable` represents a single I/O event which may be ready, or not.
                @since(version = 0.2.0)
                resource pollable {

                    /// Return the readiness of a pollable. This function never blocks.
                    ///
                    /// Returns `true` when the pollable is ready, and `false` otherwise.
                    @since(version = 0.2.0)
                    ready: func() -> bool;

                    /// `block` returns immediately if the pollable is ready, and otherwise
                    /// blocks until ready.
                    ///
                    /// This function is equivalent to calling `poll.poll` on a list
                    /// containing only this pollable.
                    @since(version = 0.2.0)
                    block: func();
                }

                /// Poll for completion on a set of pollables.
                ///
                /// This function takes a list of pollables, which identify I/O sources of
                /// interest, and waits until one or more of the events is ready for I/O.
                ///
                /// The result `list<u32>` contains one or more indices of handles in the
                /// argument list that is ready for I/O.
                ///
                /// This function traps if either:
                /// - the list is empty, or:
                /// - the list contains more elements than can be indexed with a `u32` value.
                ///
                /// A timeout can be implemented by adding a pollable from the
                /// wasi-clocks API to the list.
                ///
                /// This function does not return a `result`; polling in itself does not
                /// do any I/O so it doesn't fail. If any of the I/O sources identified by
                /// the pollables has an error, it is indicated by marking the source as
                /// being ready for I/O.
                @since(version = 0.2.0)
                poll: func(in: list<borrow<pollable>>) -> list<u32>;
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

            /// WASI Monotonic Clock is a clock API intended to let users measure elapsed
            /// time.
            ///
            /// It is intended to be portable at least between Unix-family platforms and
            /// Windows.
            ///
            /// A monotonic clock is a clock which has an unspecified initial value, and
            /// successive reads of the clock will produce non-decreasing values.
            @since(version = 0.2.0)
            interface monotonic-clock {
                @since(version = 0.2.0)
                use wasi:io/poll@0.2.3.{pollable};

                /// An instant in time, in nanoseconds. An instant is relative to an
                /// unspecified initial value, and can only be compared to instances from
                /// the same monotonic-clock.
                @since(version = 0.2.0)
                type instant = u64;

                /// A duration of time, in nanoseconds.
                @since(version = 0.2.0)
                type duration = u64;

                /// Read the current value of the clock.
                ///
                /// The clock is monotonic, therefore calling this function repeatedly will
                /// produce a sequence of non-decreasing values.
                @since(version = 0.2.0)
                now: func() -> instant;

                /// Query the resolution of the clock. Returns the duration of time
                /// corresponding to a clock tick.
                @since(version = 0.2.0)
                resolution: func() -> duration;

                /// Create a `pollable` which will resolve once the specified instant
                /// has occurred.
                @since(version = 0.2.0)
                subscribe-instant: func(
                    when: instant,
                ) -> pollable;

                /// Create a `pollable` that will resolve after the specified duration has
                /// elapsed from the time this function is invoked.
                @since(version = 0.2.0)
                subscribe-duration: func(
                    when: duration,
                ) -> pollable;
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
            import wasi:io/poll@0.2.3;
            import wasi:cli/stdin@0.2.3;
            import wasi:cli/stdout@0.2.3;
            import wasi:cli/stderr@0.2.3;
            import wasi:filesystem/types@0.2.3;
            import wasi:filesystem/preopens@0.2.3;
            import wasi:clocks/wall-clock@0.2.3;
            import wasi:clocks/monotonic-clock@0.2.3;
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
    time: u64,
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
        wasi::clocks::wall_clock::Datetime { nanoseconds: 0, seconds: self.time }
    }

    fn resolution(&mut self) -> wasi::clocks::wall_clock::Datetime {
        wasi::clocks::wall_clock::Datetime { nanoseconds: 0, seconds: 1 }
    }
}

impl wasi::clocks::monotonic_clock::Host for MyState {
    #[doc = " Read the current value of the clock."]
    #[doc = " "]
    #[doc = " The clock is monotonic, therefore calling this function repeatedly will"]
    #[doc = " produce a sequence of non-decreasing values."]
    fn now(&mut self) -> wasi::clocks::monotonic_clock::Instant {
        self.time
    }

    #[doc = " Query the resolution of the clock. Returns the duration of time"]
    #[doc = " corresponding to a clock tick."]
    fn resolution(&mut self) -> wasi::clocks::monotonic_clock::Duration {
        1000000000u64
    }

    #[doc = " Create a `pollable` which will resolve once the specified instant"]
    #[doc = " has occurred."]
    fn subscribe_instant(
        &mut self,
        when: wasi::clocks::monotonic_clock::Instant,
    ) -> wasmtime::component::Resource<wasi::clocks::monotonic_clock::Pollable> {
        wasmtime::component::Resource::new_own(0)
    }

    #[doc = " Create a `pollable` that will resolve after the specified duration has"]
    #[doc = " elapsed from the time this function is invoked."]
    fn subscribe_duration(
        &mut self,
        when: wasi::clocks::monotonic_clock::Duration,
    ) -> wasmtime::component::Resource<wasi::clocks::monotonic_clock::Pollable> {
        wasmtime::component::Resource::new_own(0)
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

    #[doc = " Request to flush buffered output. This function never blocks."]
    #[doc = " "]
    #[doc = " This tells the output-stream that the caller intends any buffered"]
    #[doc = " output to be flushed. the output which is expected to be flushed"]
    #[doc = " is all that has been passed to `write` prior to this call."]
    #[doc = " "]
    #[doc = " Upon calling this function, the `output-stream` will not accept any"]
    #[doc = " writes (`check-write` will return `ok(0)`) until the flush has"]
    #[doc = " completed. The `subscribe` pollable will become ready when the"]
    #[doc = " flush has completed and the stream can accept more writes."]
    fn flush(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(())
    }

    #[doc = " Create a `pollable` which will resolve once the output-stream"]
    #[doc = " is ready for more writing, or an error has occurred. When this"]
    #[doc = " pollable is ready, `check-write` will return `ok(n)` with n>0, or an"]
    #[doc = " error."]
    #[doc = " "]
    #[doc = " If the stream is closed, this pollable is always ready immediately."]
    #[doc = " "]
    #[doc = " The created `pollable` is a child resource of the `output-stream`."]
    #[doc = " Implementations may trap if the `output-stream` is dropped before"]
    #[doc = " all derived `pollable`s created with this function are dropped."]
    fn subscribe(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> wasmtime::component::Resource<wasi::io::streams::Pollable> {
        wasmtime::component::Resource::new_own(self_.rep())
    }

    #[doc = " Write zeroes to a stream."]
    #[doc = " "]
    #[doc = " This should be used precisely like `write` with the exact same"]
    #[doc = " preconditions (must use check-write first), but instead of"]
    #[doc = " passing a list of bytes, you simply pass the number of zero-bytes"]
    #[doc = " that should be written."]
    fn write_zeroes(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        len: u64,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(())
    }

    #[doc = " Perform a write of up to 4096 zeroes, and then flush the stream."]
    #[doc = " Block until all of these operations are complete, or an error"]
    #[doc = " occurs."]
    #[doc = " "]
    #[doc = " Functionality is equivelant to `blocking-write-and-flush` with"]
    #[doc = " contents given as a list of len containing only zeroes."]
    fn blocking_write_zeroes_and_flush(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        len: u64,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(())
    }

    #[doc = " Read from one stream and write to another."]
    #[doc = " "]
    #[doc = " The behavior of splice is equivalent to:"]
    #[doc = " 1. calling `check-write` on the `output-stream`"]
    #[doc = " 2. calling `read` on the `input-stream` with the smaller of the"]
    #[doc = " `check-write` permitted length and the `len` provided to `splice`"]
    #[doc = " 3. calling `write` on the `output-stream` with that read data."]
    #[doc = " "]
    #[doc = " Any error reported by the call to `check-write`, `read`, or"]
    #[doc = " `write` ends the splice and reports that error."]
    #[doc = " "]
    #[doc = " This function returns the number of bytes transferred; it may be less"]
    #[doc = " than `len`."]
    fn splice(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        src: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        len: u64,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Ok(len)
    }

    #[doc = " Read from one stream and write to another, with blocking."]
    #[doc = " "]
    #[doc = " This is similar to `splice`, except that it blocks until the"]
    #[doc = " `output-stream` is ready for writing, and the `input-stream`"]
    #[doc = " is ready for reading, before performing the `splice`."]
    fn blocking_splice(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        src: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        len: u64,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Ok(len)
    }
}

impl wasi::io::streams::HostInputStream for MyState {
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::streams::InputStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }

    #[doc = " Perform a non-blocking read from the stream."]
    #[doc = " "]
    #[doc = " When the source of a `read` is binary data, the bytes from the source"]
    #[doc = " are returned verbatim. When the source of a `read` is known to the"]
    #[doc = " implementation to be text, bytes containing the UTF-8 encoding of the"]
    #[doc = " text are returned."]
    #[doc = " "]
    #[doc = " This function returns a list of bytes containing the read data,"]
    #[doc = " when successful. The returned list will contain up to `len` bytes;"]
    #[doc = " it may return fewer than requested, but not more. The list is"]
    #[doc = " empty when no bytes are available for reading at this time. The"]
    #[doc = " pollable given by `subscribe` will be ready when more bytes are"]
    #[doc = " available."]
    #[doc = " "]
    #[doc = " This function fails with a `stream-error` when the operation"]
    #[doc = " encounters an error, giving `last-operation-failed`, or when the"]
    #[doc = " stream is closed, giving `closed`."]
    #[doc = " "]
    #[doc = " When the caller gives a `len` of 0, it represents a request to"]
    #[doc = " read 0 bytes. If the stream is still open, this call should"]
    #[doc = " succeed and return an empty list, or otherwise fail with `closed`."]
    #[doc = " "]
    #[doc = " The `len` parameter is a `u64`, which could represent a list of u8 which"]
    #[doc = " is not possible to allocate in wasm32, or not desirable to allocate as"]
    #[doc = " as a return value by the callee. The callee may return a list of bytes"]
    #[doc = " less than `len` in size while more bytes are available for reading."]
    fn read(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        len: u64,
    ) -> Result<wasmtime::component::__internal::Vec<u8>, wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    #[doc = " Read bytes from a stream, after blocking until at least one byte can"]
    #[doc = " be read. Except for blocking, behavior is identical to `read`."]
    fn blocking_read(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        len: u64,
    ) -> Result<wasmtime::component::__internal::Vec<u8>, wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    #[doc = " Skip bytes from a stream. Returns number of bytes skipped."]
    #[doc = " "]
    #[doc = " Behaves identical to `read`, except instead of returning a list"]
    #[doc = " of bytes, returns the number of bytes consumed from the stream."]
    fn skip(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        len: u64,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Ok(len)
    }

    #[doc = " Skip bytes from a stream, after blocking until at least one byte"]
    #[doc = " can be skipped. Except for blocking behavior, identical to `skip`."]
    fn blocking_skip(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        len: u64,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Ok(len)
    }

    #[doc = " Create a `pollable` which will resolve once either the specified stream"]
    #[doc = " has bytes available to read or the other end of the stream has been"]
    #[doc = " closed."]
    #[doc = " The created `pollable` is a child resource of the `input-stream`."]
    #[doc = " Implementations may trap if the `input-stream` is dropped before"]
    #[doc = " all derived `pollable`s created with this function are dropped."]
    fn subscribe(
        &mut self,
        self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
    ) -> wasmtime::component::Resource<wasi::io::streams::Pollable> {
        wasmtime::component::Resource::new_own(self_.rep())
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

impl wasi::io::poll::HostPollable for MyState {
    #[doc = " Return the readiness of a pollable. This function never blocks."]
    #[doc = " "]
    #[doc = " Returns `true` when the pollable is ready, and `false` otherwise."]
    fn ready(&mut self, self_: wasmtime::component::Resource<wasi::io::poll::Pollable>) -> bool {
        true
    }

    #[doc = " `block` returns immediately if the pollable is ready, and otherwise"]
    #[doc = " blocks until ready."]
    #[doc = " "]
    #[doc = " This function is equivalent to calling `poll.poll` on a list"]
    #[doc = " containing only this pollable."]
    fn block(&mut self, self_: wasmtime::component::Resource<wasi::io::poll::Pollable>) -> () {
        ()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<wasi::io::poll::Pollable>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::io::poll::Host for MyState {
    #[doc = " Poll for completion on a set of pollables."]
    #[doc = " "]
    #[doc = " This function takes a list of pollables, which identify I/O sources of"]
    #[doc = " interest, and waits until one or more of the events is ready for I/O."]
    #[doc = " "]
    #[doc = " The result `list<u32>` contains one or more indices of handles in the"]
    #[doc = " argument list that is ready for I/O."]
    #[doc = " "]
    #[doc = " This function traps if either:"]
    #[doc = " - the list is empty, or:"]
    #[doc = " - the list contains more elements than can be indexed with a `u32` value."]
    #[doc = " "]
    #[doc = " A timeout can be implemented by adding a pollable from the"]
    #[doc = " wasi-clocks API to the list."]
    #[doc = " "]
    #[doc = " This function does not return a `result`; polling in itself does not"]
    #[doc = " do any I/O so it doesn\'t fail. If any of the I/O sources identified by"]
    #[doc = " the pollables has an error, it is indicated by marking the source as"]
    #[doc = " being ready for I/O."]
    fn poll(
        &mut self,
        in_: wasmtime::component::__internal::Vec<
            wasmtime::component::Resource<wasi::io::poll::Pollable>,
        >,
    ) -> wasmtime::component::__internal::Vec<u32> {
        let p = (0..{
            match in_.len().try_into() {
                Ok(l) => l,
                Err(_e) => u32::MAX,
            }
        })
            .collect();
        p
    }
}

impl wasi::random::random::Host for MyState {
    fn get_random_bytes(&mut self, len: u64) -> wasmtime::component::__internal::Vec<u8> {
        let mut vector: Vec<u8> = vec![
            0;
            match len.try_into() {
                Ok(k) => k,
                Err(_) => u16::max_value().into(),
            }
        ];
        self.random_source.fill_bytes(&mut vector);
        vector.to_vec()
    }

    fn get_random_u64(&mut self) -> u64 {
        self.random_source.next_u64()
    }
}

impl wasi::random::insecure::Host for MyState {
    fn get_insecure_random_bytes(&mut self, len: u64) -> wasmtime::component::__internal::Vec<u8> {
        let mut vector: Vec<u8> = vec![
            0;
            match len.try_into() {
                Ok(k) => k,
                Err(_) => u16::max_value().into(),
            }
        ];
        self.random_source.fill_bytes(&mut vector);
        vector.to_vec()
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
    // use wasmtime_wasi::p2::bindings::clocks;
    // use wasmtime_wasi::p2::bindings::filesystem;
    // use wasmtime_wasi::p2::bindings::random;

    // wasmtime_wasi::p2::add_to_linker_sync(linker)
    let options = wasmtime_wasi::p2::bindings::sync::LinkOptions::default();
    let l = wasm_linker;
    let f: fn(&mut T) -> WasiImpl<&mut T> = |t| WasiImpl(IoImpl(t));
    // clocks::wall_clock::add_to_linker::<T, HasWasi<T>>(l, f)?;
    // clocks::monotonic_clock::add_to_linker::<T, HasWasi<T>>(l, f)?;
    // filesystem::preopens::add_to_linker::<T, HasWasi<T>>(l, f)?;
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
        time: engine.get_wasm_block_time(),
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
    let wasm_function = match wasm_instance.get_func(&mut wasm_store, func_index) {
        Some(f) => f,
        None => {
            err!(ExceptionCode::WasmLoadFail, "`{}` was not an exported function", wasm_func_name)?
        }
    };
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
