# Release Notes

All notable changes to this project will be documented in this file.


## [3.1.0] - 2026-10-16

### Breaking Changes
- `ZKHALO2VERIFYWITHVK`: VkBlob header byte 11, previously part of the reserved region and ignored, is now `accumulator_limbs`. Only `0` (no accumulator) and `12` (KZG accumulator) are accepted; any other value fails parsing and the opcode throws `FatalError`. The remaining reserved bytes `12..16` are still ignored.
- Removed the `ZKHALO2VERIFY` instruction (C749). Its handler module `executor::zk_halo2` is gone (`warmup_halo2`, `consume_uint64`, `check_is_u64`), and so is the Dark DEX key it verified against: `zk_halo2_utils::DARK_DEX_W128_VK_BYTES`, `dark_dex_w128_config_params` and `build_dark_dex_w128_vk`. C749 is left unassigned and throws `InvalidOpcode`. Verify with `ZKHALO2VERIFYWITHVK` (C74A) instead, passing the key as a `VkBlob`; the Dark DEX key in that form is `halo2_test_data/dark_dex_w128_vk_blob.bin`, the same bytes `RootPN.sol` embeds.

### Added
- `KECCAK256` (`C74B`, `gosh` feature), stack `c - x`. Takes a cell holding a byte string — a chain of cells with at most one reference each, the layout `pack_data_to_cell` produces and Solidity `bytes` uses — and pushes its Ethereum keccak-256 digest as a 256-bit unsigned integer. Gas: flat `KECCAK256_GAS_PRICE` = 1000, plus the cell loads of the chain. A cell whose data is not a whole number of bytes, or that has more than one reference, throws; an empty stack throws a stack underflow.
- `Gas::keccak256_price()` (`gosh` feature).
- `VK_ACCUMULATOR_NONE` (`0`), `VK_ACCUMULATOR_LIMBS_KZG` (`12`) and the public field `VkBlob::accumulator_limbs`.

### Changed
- `ZKHALO2VERIFYWITHVK`: when the VkBlob carries `accumulator_limbs = 12`, the opcode runs the KZG accumulation decider after the SHPLONK verify: it decodes the snark-verifier accumulator from `instances[0..12]` (`lhs.x ‖ lhs.y ‖ rhs.x ‖ rhs.y`, three little-endian 88-bit limbs each) and checks `e(lhs, [1]G2) == e(rhs, [s]G2)` against the embedded Hermez SRS points. It pushes `true` only when both checks hold; an instance column shorter than 12, an off-curve accumulator point or a failed pairing pushes `false`. Blobs with `accumulator_limbs = 0` verify exactly as before.

## [3.0.6] - 2026-09-03

### Fixed
- Four `TvmError` variants reported `0` in place of the value they were built with, their `#[error]` attributes interpolating bare integer literals rather than the variants' own fields: `Fatal error`, `Invalid argument`, `Invalid data`, and `VM Exception`, which rendered both its exception code and its description as zero. A VM exception now names the code it carries.

## Version 2.7.0

Added VM execution time control parameters:

- `termination_deadline` – set time deadline for VM execution. If VM reaches this deadline, it aborts 
  the current execution and returns `TvmError::TerminationDeadline`.
  Transaction executor will map this error to the `ExecutorError::TerminationDeadline`.
  
- `execution_timeout` – set maximum duration for VM execution. If VM reaches this limit, it aborts 
  the current execution and returns `TvmError::TvmExceptionFull` with `ExceptionCode::ExecutionTimeout` (17).
  Behaviour of this error is the same as `ExceptionCode::OutOfGas`.

Both parameters are optional. Thus, if the limit is omitted it is not checked during execution.

If both limits are specified, then the VM execution will be aborted with reason of the limit with an earlier time.

## Version 1.8.217

- BLS pubkey dummy

## Version 1.8.212

- Public slice_serialize and slice_deserialize functions

## Version 1.8.209

- Benchmark massive cell making

## Version 1.8.208

- Optimize IntegerData::as_slice()

## Version 1.8.207

- Add a tuples benchmark

## Version 1.8.206

- Recover load_hashed_cell() perf

## Version 1.8.194

- Drop trees of StackItem::Tuple iteratively

## Version 1.8.192

- Add cc() and ctrls() methods

## Version 1.8.190

- Fix depth bug of exotic cell

## Version 1.8.187

- Add new GASREMAINING insn

## Version 1.8.186

- Add new TRYKEEP instruction

## Version 1.8.177

- Supported changed interface of BocWriter::with_roots

## Version 1.8.170

- Disable debug symbols by default

## Version 1.8.161

- Fix TRY nargs bogus modification

## Version 1.8.160

- Use SHA algos from ever-types

## Version: 1.8.159

- Minor fix for support changes in types

## Version: 1.8.131
### Fixed
 - check_signature: on bad pubkey/signature return false

## Version 1.8.128
- Supported ever-types version 2.0

## Version: 1.8.97
### Fixed
 - Fix DATASIZE primitives with GlobalCapability

## Version: 1.8.94
### Fixed
 - Put new code under Capability

## Version: 1.8.93
### Fixed
 - Removed recursion in STCONT/LDCONT primitives

## Version: 1.8.82
### Fixed
 - Fixed STCONT/LDCONT primitives for tuples and continuations

## Version: 1.8.79

### Fixed
 - Adapt some functions (CDATASIZE, XCTOS, XLOAD, CTOS) to library cells

## Version: 1.8.78
### Fixed
 - Fixed debug output

## Version: 1.8.77

### Fixed
 - Stack overflow during load library cell

## Version: 1.8.71

### New
- Add common as submodule

### Fixed
- Some minor fixes

## Version: 1.8.60

### Fixed
 - Fix for cells loading
 - Some refactoring for remove direct usage of Arcs
 - Bumped versions of some creates

## Version: 1.8.40

### Fixed
 - Fixed several potential panics

## Version: 1.8.39

### New
 - Implemented MYCODE primitive
 - Implemented COPYLEFT primitive
 - Implemented COPYLEFT primitive
 - Implemented STORAGE_FEE primitive
 - Implemented TRYELECT primitive
 - Implemented SEQNO primitive
 - Refactored code for cargo clippy
 - Optimized prefomance
 - Simplified SPEC_LIMIT is nothing more than i64::MAX
 - Handle BUYGAS out-of-gas condition the same way as for SETGASLIMIT
 - Supported new cells (possibly without tag)
 - Some micro optimizations for hot spots
 - Make SaveList a vector instead of hashmap
 - Simplify StackItem::as_continuation_mut
 - Eliminate cloning of cmd_code's cell
 - Put log-related computations under an if
 - Improve perf of ContinuationData ctors
 - Do arith operations in-place
 - Get rid of swaps in step_while_loop()
 - Optimize transplanting of the topmost range of a stack
 - Optimize switching of loop iterations
 - Simplify SaveList::apply()
 - Improve move_stack_from_cc(): add a special case, remove unsafe code
 - Add a script tuning a linux machine for finer benchmarking
 - Add bigint benchmarks
 - Turn off pointless benchmarking of tests; improve profiling
 - Put tracing under a check to save a bunch of cycles
 - Specialize switch() for the case of switching to c0
 - Disable rug-bigint benchmark since CI can't build gmp-mpfr-sys
 - Make StackItem variants hold Rc instead of Arc
 - Streamline integer manipulations
 - Add load-boc benchmark
 - Make SaveList's storage an array
 - Remove unnecessary engine.cmd reset
 - Split Instruction struct
 - Remove Context struct
 - Move cc parts into loop cont instead of cloning
 - Move last_cmd field out of ContinuationData into Engine
 - Add SaveList::put_opt() an unchecked version of put()
 - Improve ContinuationData printing
 - Remove unused c6 from SaveList
 - Do addition in-place
 - Simplify raise_exception()
 - Add assertions
 - Add handlers printer
 - Add a script for estimating the tests coverage
 - Address feedback
 - Fix after rebase
 - Add deep stack switch test
 - Add a benchmark for deep stack switch
 - Minor improvements
 - Minor optimization

### Fixed
 - Fixed ZEROSWAP* and ZEROROT* promitives are fixed - check for bool instead of zero
 - Fixed empty AGAIN, REPEAT loops
 - Fixed GRAMTOGAS
 - Fixed BUYGAS

## Version: 1.8.38

### New

- Implemented behavior modifier mechanism
- Implemented behavior modifier for skipping check of signature for offline execution purposes

### Fixed
- Fixed tvm.tex and tvm.pdf
