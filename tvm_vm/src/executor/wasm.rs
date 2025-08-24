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
            /// WASI filesystem is a filesystem API primarily intended to let users run WASI
            /// programs that access their files on their existing filesystems, without
            /// significant overhead.
            ///
            /// It is intended to be roughly portable between Unix-family platforms and
            /// Windows, though it does not hide many of the major differences.
            ///
            /// Paths are passed as interface-type `string`s, meaning they must consist of
            /// a sequence of Unicode Scalar Values (USVs). Some filesystems may contain
            /// paths which are not accessible by this API.
            ///
            /// The directory separator in WASI is always the forward-slash (`/`).
            ///
            /// All paths in WASI are relative paths, and are interpreted relative to a
            /// `descriptor` referring to a base directory. If a `path` argument to any WASI
            /// function starts with `/`, or if any step of resolving a `path`, including
            /// `..` and symbolic link steps, reaches a directory outside of the base
            /// directory, or reaches a symlink to an absolute or rooted path in the
            /// underlying filesystem, the function fails with `error-code::not-permitted`.
            ///
            /// For more information about WASI path resolution and sandboxing, see
            /// [WASI filesystem path resolution].
            ///
            /// [WASI filesystem path resolution]: https://github.com/WebAssembly/wasi-filesystem/blob/main/path-resolution.md
            @since(version = 0.2.0)
            interface types {
                @since(version = 0.2.0)
                use wasi:io/streams@0.2.3.{input-stream, output-stream, error};
                @since(version = 0.2.0)
                use wasi:clocks/wall-clock@0.2.3.{datetime};

                /// File size or length of a region within a file.
                @since(version = 0.2.0)
                type filesize = u64;

                /// The type of a filesystem object referenced by a descriptor.
                ///
                /// Note: This was called `filetype` in earlier versions of WASI.
                @since(version = 0.2.0)
                enum descriptor-type {
                    /// The type of the descriptor or file is unknown or is different from
                    /// any of the other types specified.
                    unknown,
                    /// The descriptor refers to a block device inode.
                    block-device,
                    /// The descriptor refers to a character device inode.
                    character-device,
                    /// The descriptor refers to a directory inode.
                    directory,
                    /// The descriptor refers to a named pipe.
                    fifo,
                    /// The file refers to a symbolic link inode.
                    symbolic-link,
                    /// The descriptor refers to a regular file inode.
                    regular-file,
                    /// The descriptor refers to a socket.
                    socket,
                }

                /// Descriptor flags.
                ///
                /// Note: This was called `fdflags` in earlier versions of WASI.
                @since(version = 0.2.0)
                flags descriptor-flags {
                    /// Read mode: Data can be read.
                    read,
                    /// Write mode: Data can be written to.
                    write,
                    /// Request that writes be performed according to synchronized I/O file
                    /// integrity completion. The data stored in the file and the file's
                    /// metadata are synchronized. This is similar to `O_SYNC` in POSIX.
                    ///
                    /// The precise semantics of this operation have not yet been defined for
                    /// WASI. At this time, it should be interpreted as a request, and not a
                    /// requirement.
                    file-integrity-sync,
                    /// Request that writes be performed according to synchronized I/O data
                    /// integrity completion. Only the data stored in the file is
                    /// synchronized. This is similar to `O_DSYNC` in POSIX.
                    ///
                    /// The precise semantics of this operation have not yet been defined for
                    /// WASI. At this time, it should be interpreted as a request, and not a
                    /// requirement.
                    data-integrity-sync,
                    /// Requests that reads be performed at the same level of integrity
                    /// requested for writes. This is similar to `O_RSYNC` in POSIX.
                    ///
                    /// The precise semantics of this operation have not yet been defined for
                    /// WASI. At this time, it should be interpreted as a request, and not a
                    /// requirement.
                    requested-write-sync,
                    /// Mutating directories mode: Directory contents may be mutated.
                    ///
                    /// When this flag is unset on a descriptor, operations using the
                    /// descriptor which would create, rename, delete, modify the data or
                    /// metadata of filesystem objects, or obtain another handle which
                    /// would permit any of those, shall fail with `error-code::read-only` if
                    /// they would otherwise succeed.
                    ///
                    /// This may only be set on directories.
                    mutate-directory,
                }

                /// File attributes.
                ///
                /// Note: This was called `filestat` in earlier versions of WASI.
                @since(version = 0.2.0)
                record descriptor-stat {
                    /// File type.
                    %type: descriptor-type,
                    /// Number of hard links to the file.
                    link-count: link-count,
                    /// For regular files, the file size in bytes. For symbolic links, the
                    /// length in bytes of the pathname contained in the symbolic link.
                    size: filesize,
                    /// Last data access timestamp.
                    ///
                    /// If the `option` is none, the platform doesn't maintain an access
                    /// timestamp for this file.
                    data-access-timestamp: option<datetime>,
                    /// Last data modification timestamp.
                    ///
                    /// If the `option` is none, the platform doesn't maintain a
                    /// modification timestamp for this file.
                    data-modification-timestamp: option<datetime>,
                    /// Last file status-change timestamp.
                    ///
                    /// If the `option` is none, the platform doesn't maintain a
                    /// status-change timestamp for this file.
                    status-change-timestamp: option<datetime>,
                }

                /// Flags determining the method of how paths are resolved.
                @since(version = 0.2.0)
                flags path-flags {
                    /// As long as the resolved path corresponds to a symbolic link, it is
                    /// expanded.
                    symlink-follow,
                }

                /// Open flags used by `open-at`.
                @since(version = 0.2.0)
                flags open-flags {
                    /// Create file if it does not exist, similar to `O_CREAT` in POSIX.
                    create,
                    /// Fail if not a directory, similar to `O_DIRECTORY` in POSIX.
                    directory,
                    /// Fail if file already exists, similar to `O_EXCL` in POSIX.
                    exclusive,
                    /// Truncate file to size 0, similar to `O_TRUNC` in POSIX.
                    truncate,
                }

                /// Number of hard links to an inode.
                @since(version = 0.2.0)
                type link-count = u64;

                /// When setting a timestamp, this gives the value to set it to.
                @since(version = 0.2.0)
                variant new-timestamp {
                    /// Leave the timestamp set to its previous value.
                    no-change,
                    /// Set the timestamp to the current time of the system clock associated
                    /// with the filesystem.
                    now,
                    /// Set the timestamp to the given value.
                    timestamp(datetime),
                }

                /// A directory entry.
                record directory-entry {
                    /// The type of the file referred to by this directory entry.
                    %type: descriptor-type,

                    /// The name of the object.
                    name: string,
                }

                /// Error codes returned by functions, similar to `errno` in POSIX.
                /// Not all of these error codes are returned by the functions provided by this
                /// API; some are used in higher-level library layers, and others are provided
                /// merely for alignment with POSIX.
                enum error-code {
                    /// Permission denied, similar to `EACCES` in POSIX.
                    access,
                    /// Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.
                    would-block,
                    /// Connection already in progress, similar to `EALREADY` in POSIX.
                    already,
                    /// Bad descriptor, similar to `EBADF` in POSIX.
                    bad-descriptor,
                    /// Device or resource busy, similar to `EBUSY` in POSIX.
                    busy,
                    /// Resource deadlock would occur, similar to `EDEADLK` in POSIX.
                    deadlock,
                    /// Storage quota exceeded, similar to `EDQUOT` in POSIX.
                    quota,
                    /// File exists, similar to `EEXIST` in POSIX.
                    exist,
                    /// File too large, similar to `EFBIG` in POSIX.
                    file-too-large,
                    /// Illegal byte sequence, similar to `EILSEQ` in POSIX.
                    illegal-byte-sequence,
                    /// Operation in progress, similar to `EINPROGRESS` in POSIX.
                    in-progress,
                    /// Interrupted function, similar to `EINTR` in POSIX.
                    interrupted,
                    /// Invalid argument, similar to `EINVAL` in POSIX.
                    invalid,
                    /// I/O error, similar to `EIO` in POSIX.
                    io,
                    /// Is a directory, similar to `EISDIR` in POSIX.
                    is-directory,
                    /// Too many levels of symbolic links, similar to `ELOOP` in POSIX.
                    loop,
                    /// Too many links, similar to `EMLINK` in POSIX.
                    too-many-links,
                    /// Message too large, similar to `EMSGSIZE` in POSIX.
                    message-size,
                    /// Filename too long, similar to `ENAMETOOLONG` in POSIX.
                    name-too-long,
                    /// No such device, similar to `ENODEV` in POSIX.
                    no-device,
                    /// No such file or directory, similar to `ENOENT` in POSIX.
                    no-entry,
                    /// No locks available, similar to `ENOLCK` in POSIX.
                    no-lock,
                    /// Not enough space, similar to `ENOMEM` in POSIX.
                    insufficient-memory,
                    /// No space left on device, similar to `ENOSPC` in POSIX.
                    insufficient-space,
                    /// Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.
                    not-directory,
                    /// Directory not empty, similar to `ENOTEMPTY` in POSIX.
                    not-empty,
                    /// State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.
                    not-recoverable,
                    /// Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.
                    unsupported,
                    /// Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.
                    no-tty,
                    /// No such device or address, similar to `ENXIO` in POSIX.
                    no-such-device,
                    /// Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.
                    overflow,
                    /// Operation not permitted, similar to `EPERM` in POSIX.
                    not-permitted,
                    /// Broken pipe, similar to `EPIPE` in POSIX.
                    pipe,
                    /// Read-only file system, similar to `EROFS` in POSIX.
                    read-only,
                    /// Invalid seek, similar to `ESPIPE` in POSIX.
                    invalid-seek,
                    /// Text file busy, similar to `ETXTBSY` in POSIX.
                    text-file-busy,
                    /// Cross-device link, similar to `EXDEV` in POSIX.
                    cross-device,
                }

                /// File or memory access pattern advisory information.
                @since(version = 0.2.0)
                enum advice {
                    /// The application has no advice to give on its behavior with respect
                    /// to the specified data.
                    normal,
                    /// The application expects to access the specified data sequentially
                    /// from lower offsets to higher offsets.
                    sequential,
                    /// The application expects to access the specified data in a random
                    /// order.
                    random,
                    /// The application expects to access the specified data in the near
                    /// future.
                    will-need,
                    /// The application expects that it will not access the specified data
                    /// in the near future.
                    dont-need,
                    /// The application expects to access the specified data once and then
                    /// not reuse it thereafter.
                    no-reuse,
                }

                /// A 128-bit hash value, split into parts because wasm doesn't have a
                /// 128-bit integer type.
                @since(version = 0.2.0)
                record metadata-hash-value {
                /// 64 bits of a 128-bit hash value.
                lower: u64,
                /// Another 64 bits of a 128-bit hash value.
                upper: u64,
                }

                /// A descriptor is a reference to a filesystem object, which may be a file,
                /// directory, named pipe, special file, or other object on which filesystem
                /// calls may be made.
                @since(version = 0.2.0)
                resource descriptor {
                    /// Return a stream for reading from a file, if available.
                    ///
                    /// May fail with an error-code describing why the file cannot be read.
                    ///
                    /// Multiple read, write, and append streams may be active on the same open
                    /// file and they do not interfere with each other.
                    ///
                    /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.
                    @since(version = 0.2.0)
                    read-via-stream: func(
                        /// The offset within the file at which to start reading.
                        offset: filesize,
                    ) -> result<input-stream, error-code>;

                    /// Return a stream for writing to a file, if available.
                    ///
                    /// May fail with an error-code describing why the file cannot be written.
                    ///
                    /// Note: This allows using `write-stream`, which is similar to `write` in
                    /// POSIX.
                    @since(version = 0.2.0)
                    write-via-stream: func(
                        /// The offset within the file at which to start writing.
                        offset: filesize,
                    ) -> result<output-stream, error-code>;

                    /// Return a stream for appending to a file, if available.
                    ///
                    /// May fail with an error-code describing why the file cannot be appended.
                    ///
                    /// Note: This allows using `write-stream`, which is similar to `write` with
                    /// `O_APPEND` in POSIX.
                    @since(version = 0.2.0)
                    append-via-stream: func() -> result<output-stream, error-code>;

                    /// Provide file advisory information on a descriptor.
                    ///
                    /// This is similar to `posix_fadvise` in POSIX.
                    @since(version = 0.2.0)
                    advise: func(
                        /// The offset within the file to which the advisory applies.
                        offset: filesize,
                        /// The length of the region to which the advisory applies.
                        length: filesize,
                        /// The advice.
                        advice: advice
                    ) -> result<_, error-code>;

                    /// Synchronize the data of a file to disk.
                    ///
                    /// This function succeeds with no effect if the file descriptor is not
                    /// opened for writing.
                    ///
                    /// Note: This is similar to `fdatasync` in POSIX.
                    @since(version = 0.2.0)
                    sync-data: func() -> result<_, error-code>;

                    /// Get flags associated with a descriptor.
                    ///
                    /// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.
                    ///
                    /// Note: This returns the value that was the `fs_flags` value returned
                    /// from `fdstat_get` in earlier versions of WASI.
                    @since(version = 0.2.0)
                    get-flags: func() -> result<descriptor-flags, error-code>;

                    /// Get the dynamic type of a descriptor.
                    ///
                    /// Note: This returns the same value as the `type` field of the `fd-stat`
                    /// returned by `stat`, `stat-at` and similar.
                    ///
                    /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided
                    /// by `fstat` in POSIX.
                    ///
                    /// Note: This returns the value that was the `fs_filetype` value returned
                    /// from `fdstat_get` in earlier versions of WASI.
                    @since(version = 0.2.0)
                    get-type: func() -> result<descriptor-type, error-code>;

                    /// Adjust the size of an open file. If this increases the file's size, the
                    /// extra bytes are filled with zeros.
                    ///
                    /// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
                    @since(version = 0.2.0)
                    set-size: func(size: filesize) -> result<_, error-code>;

                    /// Adjust the timestamps of an open file or directory.
                    ///
                    /// Note: This is similar to `futimens` in POSIX.
                    ///
                    /// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
                    @since(version = 0.2.0)
                    set-times: func(
                        /// The desired values of the data access timestamp.
                        data-access-timestamp: new-timestamp,
                        /// The desired values of the data modification timestamp.
                        data-modification-timestamp: new-timestamp,
                    ) -> result<_, error-code>;

                    /// Read from a descriptor, without using and updating the descriptor's offset.
                    ///
                    /// This function returns a list of bytes containing the data that was
                    /// read, along with a bool which, when true, indicates that the end of the
                    /// file was reached. The returned list will contain up to `length` bytes; it
                    /// may return fewer than requested, if the end of the file is reached or
                    /// if the I/O operation is interrupted.
                    ///
                    /// In the future, this may change to return a `stream<u8, error-code>`.
                    ///
                    /// Note: This is similar to `pread` in POSIX.
                    @since(version = 0.2.0)
                    read: func(
                        /// The maximum number of bytes to read.
                        length: filesize,
                        /// The offset within the file at which to read.
                        offset: filesize,
                    ) -> result<tuple<list<u8>, bool>, error-code>;

                    /// Write to a descriptor, without using and updating the descriptor's offset.
                    ///
                    /// It is valid to write past the end of a file; the file is extended to the
                    /// extent of the write, with bytes between the previous end and the start of
                    /// the write set to zero.
                    ///
                    /// In the future, this may change to take a `stream<u8, error-code>`.
                    ///
                    /// Note: This is similar to `pwrite` in POSIX.
                    @since(version = 0.2.0)
                    write: func(
                        /// Data to write
                        buffer: list<u8>,
                        /// The offset within the file at which to write.
                        offset: filesize,
                    ) -> result<filesize, error-code>;

                    /// Read directory entries from a directory.
                    ///
                    /// On filesystems where directories contain entries referring to themselves
                    /// and their parents, often named `.` and `..` respectively, these entries
                    /// are omitted.
                    ///
                    /// This always returns a new stream which starts at the beginning of the
                    /// directory. Multiple streams may be active on the same directory, and they
                    /// do not interfere with each other.
                    @since(version = 0.2.0)
                    read-directory: func() -> result<directory-entry-stream, error-code>;

                    /// Synchronize the data and metadata of a file to disk.
                    ///
                    /// This function succeeds with no effect if the file descriptor is not
                    /// opened for writing.
                    ///
                    /// Note: This is similar to `fsync` in POSIX.
                    @since(version = 0.2.0)
                    sync: func() -> result<_, error-code>;

                    /// Create a directory.
                    ///
                    /// Note: This is similar to `mkdirat` in POSIX.
                    @since(version = 0.2.0)
                    create-directory-at: func(
                        /// The relative path at which to create the directory.
                        path: string,
                    ) -> result<_, error-code>;

                    /// Return the attributes of an open file or directory.
                    ///
                    /// Note: This is similar to `fstat` in POSIX, except that it does not return
                    /// device and inode information. For testing whether two descriptors refer to
                    /// the same underlying filesystem object, use `is-same-object`. To obtain
                    /// additional data that can be used do determine whether a file has been
                    /// modified, use `metadata-hash`.
                    ///
                    /// Note: This was called `fd_filestat_get` in earlier versions of WASI.
                    @since(version = 0.2.0)
                    stat: func() -> result<descriptor-stat, error-code>;

                    /// Return the attributes of a file or directory.
                    ///
                    /// Note: This is similar to `fstatat` in POSIX, except that it does not
                    /// return device and inode information. See the `stat` description for a
                    /// discussion of alternatives.
                    ///
                    /// Note: This was called `path_filestat_get` in earlier versions of WASI.
                    @since(version = 0.2.0)
                    stat-at: func(
                        /// Flags determining the method of how the path is resolved.
                        path-flags: path-flags,
                        /// The relative path of the file or directory to inspect.
                        path: string,
                    ) -> result<descriptor-stat, error-code>;

                    /// Adjust the timestamps of a file or directory.
                    ///
                    /// Note: This is similar to `utimensat` in POSIX.
                    ///
                    /// Note: This was called `path_filestat_set_times` in earlier versions of
                    /// WASI.
                    @since(version = 0.2.0)
                    set-times-at: func(
                        /// Flags determining the method of how the path is resolved.
                        path-flags: path-flags,
                        /// The relative path of the file or directory to operate on.
                        path: string,
                        /// The desired values of the data access timestamp.
                        data-access-timestamp: new-timestamp,
                        /// The desired values of the data modification timestamp.
                        data-modification-timestamp: new-timestamp,
                    ) -> result<_, error-code>;

                    /// Create a hard link.
                    ///
                    /// Fails with `error-code::no-entry` if the old path does not exist,
                    /// with `error-code::exist` if the new path already exists, and
                    /// `error-code::not-permitted` if the old path is not a file.
                    ///
                    /// Note: This is similar to `linkat` in POSIX.
                    @since(version = 0.2.0)
                    link-at: func(
                        /// Flags determining the method of how the path is resolved.
                        old-path-flags: path-flags,
                        /// The relative source path from which to link.
                        old-path: string,
                        /// The base directory for `new-path`.
                        new-descriptor: borrow<descriptor>,
                        /// The relative destination path at which to create the hard link.
                        new-path: string,
                    ) -> result<_, error-code>;

                    /// Open a file or directory.
                    ///
                    /// If `flags` contains `descriptor-flags::mutate-directory`, and the base
                    /// descriptor doesn't have `descriptor-flags::mutate-directory` set,
                    /// `open-at` fails with `error-code::read-only`.
                    ///
                    /// If `flags` contains `write` or `mutate-directory`, or `open-flags`
                    /// contains `truncate` or `create`, and the base descriptor doesn't have
                    /// `descriptor-flags::mutate-directory` set, `open-at` fails with
                    /// `error-code::read-only`.
                    ///
                    /// Note: This is similar to `openat` in POSIX.
                    @since(version = 0.2.0)
                    open-at: func(
                        /// Flags determining the method of how the path is resolved.
                        path-flags: path-flags,
                        /// The relative path of the object to open.
                        path: string,
                        /// The method by which to open the file.
                        open-flags: open-flags,
                        /// Flags to use for the resulting descriptor.
                        %flags: descriptor-flags,
                    ) -> result<descriptor, error-code>;

                    /// Read the contents of a symbolic link.
                    ///
                    /// If the contents contain an absolute or rooted path in the underlying
                    /// filesystem, this function fails with `error-code::not-permitted`.
                    ///
                    /// Note: This is similar to `readlinkat` in POSIX.
                    @since(version = 0.2.0)
                    readlink-at: func(
                        /// The relative path of the symbolic link from which to read.
                        path: string,
                    ) -> result<string, error-code>;

                    /// Remove a directory.
                    ///
                    /// Return `error-code::not-empty` if the directory is not empty.
                    ///
                    /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
                    @since(version = 0.2.0)
                    remove-directory-at: func(
                        /// The relative path to a directory to remove.
                        path: string,
                    ) -> result<_, error-code>;

                    /// Rename a filesystem object.
                    ///
                    /// Note: This is similar to `renameat` in POSIX.
                    @since(version = 0.2.0)
                    rename-at: func(
                        /// The relative source path of the file or directory to rename.
                        old-path: string,
                        /// The base directory for `new-path`.
                        new-descriptor: borrow<descriptor>,
                        /// The relative destination path to which to rename the file or directory.
                        new-path: string,
                    ) -> result<_, error-code>;

                    /// Create a symbolic link (also known as a "symlink").
                    ///
                    /// If `old-path` starts with `/`, the function fails with
                    /// `error-code::not-permitted`.
                    ///
                    /// Note: This is similar to `symlinkat` in POSIX.
                    @since(version = 0.2.0)
                    symlink-at: func(
                        /// The contents of the symbolic link.
                        old-path: string,
                        /// The relative destination path at which to create the symbolic link.
                        new-path: string,
                    ) -> result<_, error-code>;

                    /// Unlink a filesystem object that is not a directory.
                    ///
                    /// Return `error-code::is-directory` if the path refers to a directory.
                    /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
                    @since(version = 0.2.0)
                    unlink-file-at: func(
                        /// The relative path to a file to unlink.
                        path: string,
                    ) -> result<_, error-code>;

                    /// Test whether two descriptors refer to the same filesystem object.
                    ///
                    /// In POSIX, this corresponds to testing whether the two descriptors have the
                    /// same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers.
                    /// wasi-filesystem does not expose device and inode numbers, so this function
                    /// may be used instead.
                    @since(version = 0.2.0)
                    is-same-object: func(other: borrow<descriptor>) -> bool;

                    /// Return a hash of the metadata associated with a filesystem object referred
                    /// to by a descriptor.
                    ///
                    /// This returns a hash of the last-modification timestamp and file size, and
                    /// may also include the inode number, device number, birth timestamp, and
                    /// other metadata fields that may change when the file is modified or
                    /// replaced. It may also include a secret value chosen by the
                    /// implementation and not otherwise exposed.
                    ///
                    /// Implementations are encouraged to provide the following properties:
                    ///
                    ///  - If the file is not modified or replaced, the computed hash value should
                    ///    usually not change.
                    ///  - If the object is modified or replaced, the computed hash value should
                    ///    usually change.
                    ///  - The inputs to the hash should not be easily computable from the
                    ///    computed hash.
                    ///
                    /// However, none of these is required.
                    @since(version = 0.2.0)
                    metadata-hash: func() -> result<metadata-hash-value, error-code>;

                    /// Return a hash of the metadata associated with a filesystem object referred
                    /// to by a directory descriptor and a relative path.
                    ///
                    /// This performs the same hash computation as `metadata-hash`.
                    @since(version = 0.2.0)
                    metadata-hash-at: func(
                        /// Flags determining the method of how the path is resolved.
                        path-flags: path-flags,
                        /// The relative path of the file or directory to inspect.
                        path: string,
                    ) -> result<metadata-hash-value, error-code>;
                }

                /// A stream of directory entries.
                @since(version = 0.2.0)
                resource directory-entry-stream {
                    /// Read a single directory entry from a `directory-entry-stream`.
                    @since(version = 0.2.0)
                    read-directory-entry: func() -> result<option<directory-entry>, error-code>;
                }

                /// Attempts to extract a filesystem-related `error-code` from the stream
                /// `error` provided.
                ///
                /// Stream operations which return `stream-error::last-operation-failed`
                /// have a payload with more information about the operation that failed.
                /// This payload can be passed through to this function to see if there's
                /// filesystem-related information about the error to return.
                ///
                /// Note that this function is fallible because not all stream-related
                /// errors are filesystem-related errors.
                @since(version = 0.2.0)
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
        _when: wasi::clocks::monotonic_clock::Instant,
    ) -> wasmtime::component::Resource<wasi::clocks::monotonic_clock::Pollable> {
        wasmtime::component::Resource::new_own(0)
    }

    #[doc = " Create a `pollable` that will resolve after the specified duration has"]
    #[doc = " elapsed from the time this function is invoked."]
    fn subscribe_duration(
        &mut self,
        _when: wasi::clocks::monotonic_clock::Duration,
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

impl wasi::filesystem::types::HostDirectoryEntryStream for MyState {
    #[doc = " Read a single directory entry from a `directory-entry-stream`."]
    fn read_directory_entry(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::DirectoryEntryStream>,
    ) -> Result<Option<wasi::filesystem::types::DirectoryEntry>, wasi::filesystem::types::ErrorCode>
    {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::filesystem::types::DirectoryEntryStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
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

    #[doc = " Return a stream for reading from a file, if available."]
    #[doc = " "]
    #[doc = " May fail with an error-code describing why the file cannot be read."]
    #[doc = " "]
    #[doc = " Multiple read, write, and append streams may be active on the same open"]
    #[doc = " file and they do not interfere with each other."]
    #[doc = " "]
    #[doc = " Note: This allows using `read-stream`, which is similar to `read` in POSIX."]
    fn read_via_stream(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _offset: wasi::filesystem::types::Filesize,
    ) -> Result<
        wasmtime::component::Resource<wasi::filesystem::types::InputStream>,
        wasi::filesystem::types::ErrorCode,
    > {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Provide file advisory information on a descriptor."]
    #[doc = " "]
    #[doc = " This is similar to `posix_fadvise` in POSIX."]
    fn advise(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _offset: wasi::filesystem::types::Filesize,
        _length: wasi::filesystem::types::Filesize,
        _advice: wasi::filesystem::types::Advice,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Synchronize the data of a file to disk."]
    #[doc = " "]
    #[doc = " This function succeeds with no effect if the file descriptor is not"]
    #[doc = " opened for writing."]
    #[doc = " "]
    #[doc = " Note: This is similar to `fdatasync` in POSIX."]
    fn sync_data(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Ok(())
    }

    #[doc = " Get flags associated with a descriptor."]
    #[doc = " "]
    #[doc = " Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX."]
    #[doc = " "]
    #[doc = " Note: This returns the value that was the `fs_flags` value returned"]
    #[doc = " from `fdstat_get` in earlier versions of WASI."]
    fn get_flags(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<wasi::filesystem::types::DescriptorFlags, wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Adjust the size of an open file. If this increases the file\'s size, the"]
    #[doc = " extra bytes are filled with zeros."]
    #[doc = " "]
    #[doc = " Note: This was called `fd_filestat_set_size` in earlier versions of WASI."]
    fn set_size(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _size: wasi::filesystem::types::Filesize,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Adjust the timestamps of an open file or directory."]
    #[doc = " "]
    #[doc = " Note: This is similar to `futimens` in POSIX."]
    #[doc = " "]
    #[doc = " Note: This was called `fd_filestat_set_times` in earlier versions of WASI."]
    fn set_times(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _data_access_timestamp: wasi::filesystem::types::NewTimestamp,
        _data_modification_timestamp: wasi::filesystem::types::NewTimestamp,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Read from a descriptor, without using and updating the descriptor\'s offset."]
    #[doc = " "]
    #[doc = " This function returns a list of bytes containing the data that was"]
    #[doc = " read, along with a bool which, when true, indicates that the end of the"]
    #[doc = " file was reached. The returned list will contain up to `length` bytes; it"]
    #[doc = " may return fewer than requested, if the end of the file is reached or"]
    #[doc = " if the I/O operation is interrupted."]
    #[doc = " "]
    #[doc = " In the future, this may change to return a `stream<u8, error-code>`."]
    #[doc = " "]
    #[doc = " Note: This is similar to `pread` in POSIX."]
    fn read(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _length: wasi::filesystem::types::Filesize,
        _offset: wasi::filesystem::types::Filesize,
    ) -> Result<(wasmtime::component::__internal::Vec<u8>, bool), wasi::filesystem::types::ErrorCode>
    {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Write to a descriptor, without using and updating the descriptor\'s offset."]
    #[doc = " "]
    #[doc = " It is valid to write past the end of a file; the file is extended to the"]
    #[doc = " extent of the write, with bytes between the previous end and the start of"]
    #[doc = " the write set to zero."]
    #[doc = " "]
    #[doc = " In the future, this may change to take a `stream<u8, error-code>`."]
    #[doc = " "]
    #[doc = " Note: This is similar to `pwrite` in POSIX."]
    fn write(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _buffer: wasmtime::component::__internal::Vec<u8>,
        _offset: wasi::filesystem::types::Filesize,
    ) -> Result<wasi::filesystem::types::Filesize, wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Read directory entries from a directory."]
    #[doc = " "]
    #[doc = " On filesystems where directories contain entries referring to themselves"]
    #[doc = " and their parents, often named `.` and `..` respectively, these entries"]
    #[doc = " are omitted."]
    #[doc = " "]
    #[doc = " This always returns a new stream which starts at the beginning of the"]
    #[doc = " directory. Multiple streams may be active on the same directory, and they"]
    #[doc = " do not interfere with each other."]
    fn read_directory(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<
        wasmtime::component::Resource<wasi::filesystem::types::DirectoryEntryStream>,
        wasi::filesystem::types::ErrorCode,
    > {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Synchronize the data and metadata of a file to disk."]
    #[doc = " "]
    #[doc = " This function succeeds with no effect if the file descriptor is not"]
    #[doc = " opened for writing."]
    #[doc = " "]
    #[doc = " Note: This is similar to `fsync` in POSIX."]
    fn sync(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Ok(())
    }

    #[doc = " Create a directory."]
    #[doc = " "]
    #[doc = " Note: This is similar to `mkdirat` in POSIX."]
    fn create_directory_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path: wasmtime::component::__internal::String,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Return the attributes of a file or directory."]
    #[doc = " "]
    #[doc = " Note: This is similar to `fstatat` in POSIX, except that it does not"]
    #[doc = " return device and inode information. See the `stat` description for a"]
    #[doc = " discussion of alternatives."]
    #[doc = " "]
    #[doc = " Note: This was called `path_filestat_get` in earlier versions of WASI."]
    fn stat_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path_flags: wasi::filesystem::types::PathFlags,
        _path: wasmtime::component::__internal::String,
    ) -> Result<wasi::filesystem::types::DescriptorStat, wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Adjust the timestamps of a file or directory."]
    #[doc = " "]
    #[doc = " Note: This is similar to `utimensat` in POSIX."]
    #[doc = " "]
    #[doc = " Note: This was called `path_filestat_set_times` in earlier versions of"]
    #[doc = " WASI."]
    fn set_times_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path_flags: wasi::filesystem::types::PathFlags,
        _path: wasmtime::component::__internal::String,
        _data_access_timestamp: wasi::filesystem::types::NewTimestamp,
        _data_modification_timestamp: wasi::filesystem::types::NewTimestamp,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Create a hard link."]
    #[doc = " "]
    #[doc = " Fails with `error-code::no-entry` if the old path does not exist,"]
    #[doc = " with `error-code::exist` if the new path already exists, and"]
    #[doc = " `error-code::not-permitted` if the old path is not a file."]
    #[doc = " "]
    #[doc = " Note: This is similar to `linkat` in POSIX."]
    fn link_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _old_path_flags: wasi::filesystem::types::PathFlags,
        _old_path: wasmtime::component::__internal::String,
        _new_descriptor: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _new_path: wasmtime::component::__internal::String,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Open a file or directory."]
    #[doc = " "]
    #[doc = " If `flags` contains `descriptor-flags::mutate-directory`, and the base"]
    #[doc = " descriptor doesn\'t have `descriptor-flags::mutate-directory` set,"]
    #[doc = " `open-at` fails with `error-code::read-only`."]
    #[doc = " "]
    #[doc = " If `flags` contains `write` or `mutate-directory`, or `open-flags`"]
    #[doc = " contains `truncate` or `create`, and the base descriptor doesn\'t have"]
    #[doc = " `descriptor-flags::mutate-directory` set, `open-at` fails with"]
    #[doc = " `error-code::read-only`."]
    #[doc = " "]
    #[doc = " Note: This is similar to `openat` in POSIX."]
    fn open_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path_flags: wasi::filesystem::types::PathFlags,
        _path: wasmtime::component::__internal::String,
        _open_flags: wasi::filesystem::types::OpenFlags,
        _flags: wasi::filesystem::types::DescriptorFlags,
    ) -> Result<
        wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        wasi::filesystem::types::ErrorCode,
    > {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Read the contents of a symbolic link."]
    #[doc = " "]
    #[doc = " If the contents contain an absolute or rooted path in the underlying"]
    #[doc = " filesystem, this function fails with `error-code::not-permitted`."]
    #[doc = " "]
    #[doc = " Note: This is similar to `readlinkat` in POSIX."]
    fn readlink_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path: wasmtime::component::__internal::String,
    ) -> Result<wasmtime::component::__internal::String, wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Remove a directory."]
    #[doc = " "]
    #[doc = " Return `error-code::not-empty` if the directory is not empty."]
    #[doc = " "]
    #[doc = " Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX."]
    fn remove_directory_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path: wasmtime::component::__internal::String,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Rename a filesystem object."]
    #[doc = " "]
    #[doc = " Note: This is similar to `renameat` in POSIX."]
    fn rename_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _old_path: wasmtime::component::__internal::String,
        _new_descriptor: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _new_path: wasmtime::component::__internal::String,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Create a symbolic link (also known as a \"symlink\")."]
    #[doc = " "]
    #[doc = " If `old-path` starts with `/`, the function fails with"]
    #[doc = " `error-code::not-permitted`."]
    #[doc = " "]
    #[doc = " Note: This is similar to `symlinkat` in POSIX."]
    fn symlink_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _old_path: wasmtime::component::__internal::String,
        _new_path: wasmtime::component::__internal::String,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Unlink a filesystem object that is not a directory."]
    #[doc = " "]
    #[doc = " Return `error-code::is-directory` if the path refers to a directory."]
    #[doc = " Note: This is similar to `unlinkat(fd, path, 0)` in POSIX."]
    fn unlink_file_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path: wasmtime::component::__internal::String,
    ) -> Result<(), wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Test whether two descriptors refer to the same filesystem object."]
    #[doc = " "]
    #[doc = " In POSIX, this corresponds to testing whether the two descriptors have the"]
    #[doc = " same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers."]
    #[doc = " wasi-filesystem does not expose device and inode numbers, so this function"]
    #[doc = " may be used instead."]
    fn is_same_object(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _other: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> bool {
        false
    }

    #[doc = " Return a hash of the metadata associated with a filesystem object referred"]
    #[doc = " to by a descriptor."]
    #[doc = " "]
    #[doc = " This returns a hash of the last-modification timestamp and file size, and"]
    #[doc = " may also include the inode number, device number, birth timestamp, and"]
    #[doc = " other metadata fields that may change when the file is modified or"]
    #[doc = " replaced. It may also include a secret value chosen by the"]
    #[doc = " implementation and not otherwise exposed."]
    #[doc = " "]
    #[doc = " Implementations are encouraged to provide the following properties:"]
    #[doc = " "]
    #[doc = "  - If the file is not modified or replaced, the computed hash value should"]
    #[doc = "    usually not change."]
    #[doc = "  - If the object is modified or replaced, the computed hash value should"]
    #[doc = "    usually change."]
    #[doc = "  - The inputs to the hash should not be easily computable from the"]
    #[doc = "    computed hash."]
    #[doc = " "]
    #[doc = " However, none of these is required."]
    fn metadata_hash(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<wasi::filesystem::types::MetadataHashValue, wasi::filesystem::types::ErrorCode>
    {
        Err(wasi::filesystem::types::ErrorCode::Access)
    }

    #[doc = " Return a hash of the metadata associated with a filesystem object referred"]
    #[doc = " to by a directory descriptor and a relative path."]
    #[doc = " "]
    #[doc = " This performs the same hash computation as `metadata-hash`."]
    fn metadata_hash_at(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _path_flags: wasi::filesystem::types::PathFlags,
        _path: wasmtime::component::__internal::String,
    ) -> Result<wasi::filesystem::types::MetadataHashValue, wasi::filesystem::types::ErrorCode>
    {
        Err(wasi::filesystem::types::ErrorCode::Access)
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
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
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
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _len: u64,
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
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _len: u64,
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
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _src: wasmtime::component::Resource<wasi::io::streams::InputStream>,
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
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _src: wasmtime::component::Resource<wasi::io::streams::InputStream>,
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
        _self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        _len: u64,
    ) -> Result<wasmtime::component::__internal::Vec<u8>, wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    #[doc = " Read bytes from a stream, after blocking until at least one byte can"]
    #[doc = " be read. Except for blocking, behavior is identical to `read`."]
    fn blocking_read(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        _len: u64,
    ) -> Result<wasmtime::component::__internal::Vec<u8>, wasi::io::streams::StreamError> {
        Err(wasi::io::streams::StreamError::Closed)
    }

    #[doc = " Skip bytes from a stream. Returns number of bytes skipped."]
    #[doc = " "]
    #[doc = " Behaves identical to `read`, except instead of returning a list"]
    #[doc = " of bytes, returns the number of bytes consumed from the stream."]
    fn skip(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
        len: u64,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Ok(len)
    }

    #[doc = " Skip bytes from a stream, after blocking until at least one byte"]
    #[doc = " can be skipped. Except for blocking behavior, identical to `skip`."]
    fn blocking_skip(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::InputStream>,
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
    fn ready(&mut self, _self_: wasmtime::component::Resource<wasi::io::poll::Pollable>) -> bool {
        true
    }

    #[doc = " `block` returns immediately if the pollable is ready, and otherwise"]
    #[doc = " blocks until ready."]
    #[doc = " "]
    #[doc = " This function is equivalent to calling `poll.poll` on a list"]
    #[doc = " containing only this pollable."]
    fn block(&mut self, _self_: wasmtime::component::Resource<wasi::io::poll::Pollable>) -> () {
        ()
    }

    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::poll::Pollable>,
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
            e => err!(
                ExceptionCode::WasmCellUnpackError,
                "Failed to unpack wasm instruction {:?}",
                e
            )?,
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
                e => err!(
                    ExceptionCode::WasmCellUnpackError,
                    "Failed to unpack wasm instruction {:?}",
                    e
                )?,
            };
        log::debug!("Using WASM Hash {:?}", wasm_hash);
        Ok((
            engine.get_wasm_binary_by_hash(wasm_hash.clone())?,
            Some(match wasm_hash.try_into() {
                Ok(h) => h,
                Err(e) => err!(
                    ExceptionCode::WasmInvalidHash,
                    "Failed to turn valid hash into [u8; 32]. This is probably a bug. {:?}",
                    e
                )?,
            }),
        ))
        // todo!("Add hash lookup here from hash {:?}", wasm_hash);
    } else {
        log::debug!("Using WASM Executable from args");
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
        random_source: rand_chacha::ChaCha20Rng::seed_from_u64(engine.get_wasm_block_time()),
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
    let gas_used: i64 = RUNWASM_GAS_PRICE.try_into()?;
    match engine.gas_remaining() > gas_used {
        true => {}
        false => err!(ExceptionCode::OutOfGas, "Engine out of gas.")?,
    }

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
        Err(e) => err!(ExceptionCode::WasmFuelError, "Failed to set WASm fuel {:?}", e)?,
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
            ExceptionCode::WasmLinkerFail,
            "Failed to instantiate WASM
    instance {:?}",
            e
        )?,
    };

    let f: fn(&mut MyState) -> &mut MyState = |s| s;
    // let f: fn(&mut MyState) -> &mut WasiImpl<IoImpl<&mut MyState>> = |t| t;
    match Localworld::add_to_linker::<MyState, MyLibrary>(&mut wasm_linker, f) {
        Ok(_) => {}
        Err(e) => err!(ExceptionCode::WasmLinkerFail, "Failed to link IO Plugs {:?}", e)?,
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
        Err(e) => {
            err!(ExceptionCode::WasmInstantiateFail, "Failed to instantiate WASM instance {:?}", e)?
        }
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
        None => err!(
            ExceptionCode::WasmInvalidFunction,
            "Failed to find WASM exported function or component",
        )?,
    };
    log::debug!("Func Index {:?}", func_index);
    let wasm_function = match wasm_instance.get_func(&mut wasm_store, func_index) {
        Some(f) => f,
        None => err!(
            ExceptionCode::WasmInvalidFunction,
            "`{}` was not an exported function",
            wasm_func_name
        )?,
    };
    let wasm_function = match wasm_function.typed::<(Vec<u8>,), (Vec<u8>,)>(&wasm_store) {
        Ok(answer) => answer,
        Err(e) => {
            err!(ExceptionCode::WasmInvalidFunction, "Failed to get WASM answer function {:?}", e)?
        }
    };

    let result = match wasm_function.call(&mut wasm_store, (wasm_func_args,)) {
        Ok(result) => result,
        Err(e) => {
            log::debug!("Failed to execute WASM function {:?}", e);
            err!(ExceptionCode::WasmExecFail, "Failed to execute WASM function {:?}", e)?
        }
    };
    log::debug!("WASM Execution result: {:?}", result);

    // TODO: If we switch to dynamic gas usage, reenable this code
    // let gas_used: i64 = match wasm_store.get_fuel() {
    //     Ok(new_fuel) => i64::try_from((wasm_fuel -
    // new_fuel).div_ceil(WASM_FUEL_MULTIPLIER))?,     Err(e) => err!(
    //         ExceptionCode::WasmLoadFail,
    //         "Failed to get WASM engine fuel after execution {:?}",
    //         e
    //     )?,
    // };
    // engine.use_gas(gas_used);
    match engine.try_use_gas(gas_used) {
        Ok(_) => {}
        Err(e) => err!(ExceptionCode::OutOfGas, "Engine out of gas {:?}.", e)?,
    }
    log::debug!("Remaining gas: {:?}", engine.gas_remaining());

    // return result
    log::debug!("EXEC Wasm execution result: {:?}", result);
    let res_vec = result.0;

    let cell = TokenValue::write_bytes(res_vec.as_slice(), &ABI_VERSION_2_4)?.into_cell()?;
    log::debug!("Pushing cell");

    engine.cc.stack.push(StackItem::cell(cell));

    log::debug!("OK");

    Ok(())
}
