# `ZKHALO2VERIFYWITHVK` — Design Note + Frozen Wire-Format Contract

**Status**: design **CLOSED**, implementation **LANDED** in this PR.
**Authors**: bridge integration team @ Pruvendo.
**Timeline**: design opened 2026-05-17, frozen 2026-05-22, real
implementation landed 2026-05-22.
**Parent branch**: [`serhii/node-3406-vergrth16-with-vk`](https://github.com/tvmlabs/tvm-sdk/tree/serhii/node-3406-vergrth16-with-vk) — Serhii's `ZKHALO2VERIFY` (hard-coded DarkDex W=8 VK) + `VERGRTH16WITHVK`.
**Companion**: bridge-side memo `docs/zk_halo2_an_side_design.md` in [`Pruvendo/acki-nacki-bridge`](https://github.com/Pruvendo/acki-nacki-bridge).

## TL;DR

`ZKHALO2VERIFY` (0xC7 0x49) hard-codes the verifying key to the DarkDex
W=8 circuit. The Acki Nacki ↔ Ethereum bridge needs to verify a
**different** Halo2 SHPLONK proof — produced by the
`bridge-prover-orchestrator` Halo2 Circuit 1B (Fallback BLS attestation),
with a different K, different `BaseCircuitParams`, and different VK.
Generalising in place would mean a per-circuit `match` in the VM, which
doesn't scale across the bridge / NFT / app-circuit roadmap.

We add the same pivot that `VERGRTH16WITHVK` made for the Groth16 side:
a sibling opcode whose **single stack operand carries the verifying key
(and config, and instances, and proof) bundled together in a
self-describing byte payload**. That payload is the `Halo2TvmBundle`
format, locked in §4 below.

| | `ZKHALO2VERIFY` (0xC7 0x49) | `ZKHALO2VERIFYWITHVK` (0xC7 0x4A) |
| --- | --- | --- |
| Stack | 2 cells: `[pub_inputs, proof]` | 1 cell: `[bundle]` |
| VK | Hard-coded DarkDex W=8 | Caller-supplied (from bundle) |
| Public inputs | Dual-path: u64 shortcut + LE Fr | Strict 32-byte LE Fr only |
| Gas | Lower (no per-call VK deserialisation) | Higher (~5000 placeholder, re-bench pending) |
| Cache | Single static `OnceLock<(VK, Params)>` | Bounded-FIFO map keyed by `vk_bytes` (capacity = 8) |

Keeping both opcodes means system circuits (DarkDex / zkLogin variants)
don't pay the WithVK gas premium, and app circuits (bridge / NFT / per-
contract VKs) don't need a per-circuit fork of the VM.

## Decisions — frozen 2026-05-22

Closed via Serhii's "Принимай решения на свой вкус, Сергей готов
принять любое наше решение" directive. Canonical reference is
`acki-nacki-bridge/docs/zk_halo2_an_side_design.md` §4. **Any future
change requires bumping `BUNDLE_VERSION` in both producer
(`bridge-prover-orchestrator`) and consumer (`tvm_vm`) and a paired
Decision Log entry in both repos.**

| ID | Decision |
| -- | -------- |
| Q-WIRE-1 (transcript) | **Blake2b SHPLONK**. `transcript_kind` byte in the bundle is `0x00 = Blake2b`. Values ≥ `0x01` reserved for a future Keccak variant under a bumped `BUNDLE_VERSION`. |
| Q-WIRE-2 (KZG SRS) | **Verifier-only `ParamsKZG<Bn256>`** built at runtime from 3 globally-embedded points (`g[0]`, `g2`, `s_g2`), parameterised by `k = vk.cs.degree`. NOT carried in the bundle and NOT loaded from disk — SHPLONK verification only needs these three points, so the 64-MB-per-`k` SRS file is unnecessary. Implementation: `zk_halo2_with_vk::build_shared_kzg_params(k)`, reusing the `KZG_*_BYTES` constants from `zk_halo2_utils.rs` (same constants `ZKHALO2VERIFY` uses for DarkDex W=8 at `k=19`). |
| Q-WIRE-3 (pub inputs) | **Strict 32-byte little-endian `Fr::to_repr()`**. `Fr::from_repr` rejects ≥ modulus inputs structurally (`FatalError`). No u64 shortcut on this opcode (legacy `ZKHALO2VERIFY` keeps its shortcut for DarkDex backward compat). |
| Q-WIRE-4 (VK envelope) | **Self-describing `Halo2TvmBundle`** (see §4 byte layout). Magic `b"HALO2TVM"` (8 bytes ASCII), `BUNDLE_VERSION = 0x01`, `transcript_kind = 0x00` (Blake2b), 6 reserved bytes, then LE-u32-length-prefixed `(config_json, vk_bytes, instances_bytes, proof_bytes)` in that order. |
| Q-WIRE-5 (fork pinning) | **Inherits Serhii's branch pin** of `gosh-sh/halo2-lib-zkevm-sha256-and-bls12-381` and `gosh-sh/gosh-zk-snark-halo2-utils`. Both deps are already in `tvm_vm/Cargo.toml` under `[dependencies]` with branch pins; promotion to commit-SHA + a format-stability CI gate is a follow-up. |
| Q-NAME-1 (opcode name) | **`ZKHALO2VERIFYWITHVK` at `0xC7 0x4A`**. Adjacent to Serhii's `ZKHALO2VERIFY` (0xC7 0x49) and below `POSEIDON` (0x50) / `CHKHISTPROOF` (0x51) / `VERGRTH16WITHVK` (0x52). Compiler builtin name pending: `gosh.zkHalo2VerifyWithVK(bundle_cell)`. |

Empirical sizes (real Circuit 1B fallback fixture, k=20, 10 signers):
bundle ≈ 21.2 KB (VK 6.1 KB + proof 14.8 KB + 4 × 32 B instances +
config_json ~120 B + 16 B header + 4 × 4 B length prefixes).
Round-trip-tested in
`acki-nacki-bridge/crates/bridge-prover-orchestrator/tests/halo2_tvm_bundle_round_trip.rs`.
DarkDex W=8 L0 fixture (k=19, used by this PR's unit tests) yields a
~9 KB bundle.

## 4. Wire format — `Halo2TvmBundle`

Frozen by the Decisions table above. Producer reference implementation:
`acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`.
Consumer reference implementation:
`tvm_vm/src/executor/zk_halo2_with_vk_bundle.rs`.

```text
  off  size  field
  ───  ────  ─────────────────────────────────────────────────────────
    0     8  magic           = b"HALO2TVM" (ASCII, no NUL)
    8     1  version         = 0x01 (BUNDLE_VERSION)
    9     1  transcript_kind = 0x00 (Blake2b)
   10     6  reserved        = 0 × 6
   16     4  config_len      (u32 LE)
   20  cl    config_json     (UTF-8 serde_json of `BaseCircuitParams`)
  ...     4  vk_len          (u32 LE)
  ...  vl    vk_bytes        (`VerifyingKey::write(SerdeFormat::RawBytes)`)
  ...     4  instances_len   (u32 LE; must be a multiple of 32)
  ...  il    instances_bytes (N × 32-byte LE `Fr::to_repr()`, strict)
  ...     4  proof_len       (u32 LE)
  ...  pl    proof_bytes     (SHPLONK proof with Blake2b transcript)
```

All length prefixes are `u32` LE. Producer-side cap: 16 MiB
(`zk_halo2_with_vk_bundle::MAX_BUNDLE_BYTES`); the consumer fails
structurally above that cap as a DoS guard.

### Safety of VK deserialisation

The consumer deserialises the VK using
`SerdeFormat::RawBytes` (NOT `RawBytesUnchecked`). `RawBytes` runs the
curve-membership check on every group element on read, which is
soundness-critical for caller-supplied VKs.
`gosh-zk-snark-halo2-utils::io::read_vk` uses `RawBytesUnchecked` and is
**deliberately not used** by this opcode — `RawBytesUnchecked` is only
safe for compile-time-pinned VKs (like Serhii's `DARK_DEX_W8_VK_BYTES`),
not for runtime-supplied ones.

### Per-VK cache

Bounded FIFO `HashMap<Vec<u8>, Arc<CachedVk>>` keyed by the bundle's
`vk_bytes`, capacity 8 (`VK_CACHE_CAPACITY`). Cold-cache verification
costs ~3 s on `k=20` because `EvaluationDomain::new` precomputes 2^20
FFT twiddle factors; warm-cache cost is dominated by the actual
SHPLONK pairing check (~50 ms on a 2024-era laptop). Operators are
expected to pre-warm cached VKs at node startup (mirroring
`warmup_halo2()` for the DarkDex W=8 VK in `zk_halo2.rs`).

## 5. Implementation layout

| File | Role |
| ---- | ---- |
| `tvm_vm/src/executor/zk_halo2_with_vk.rs` | Handler entry point + per-VK cache + shared KZG-params builder + `decode_instances_strict` + the actual `Proof::verify_with_vk` call. |
| `tvm_vm/src/executor/zk_halo2_with_vk_bundle.rs` | Pure wire-format decoder. No crypto; just byte-level validation of magic / version / transcript / chunk lengths / trailing-garbage / instances-multiple-of-32 / oversized bundle. 8 unit tests covering all rejection paths. |
| `tvm_vm/src/executor/zk_halo2_utils.rs` | Existing module. `KZG_G0_BYTES` / `KZG_G2_BYTES` / `KZG_S_G2_BYTES` promoted to `pub(crate)` so the new opcode can reuse them. |
| `tvm_vm/src/tests/test_halo2_with_vk.rs` | Real round-trip test. Builds a `Halo2TvmBundle` from the checked-in DarkDex W=8 L0 fixture, pushes it through `execute_zkhalo2_verify_with_vk`, asserts `true`. Negative cases: flipped proof byte (false), tweaked instance Fr (false), bad bundle magic (FatalError). |
| `tvm_assembler/src/simple.rs` | Mnemonic `ZKHALO2VERIFYWITHVK` → `0xC7 0x4A`. |
| `tvm_assembler/src/lib.rs` | `gosh_zk_opcode_tests::zk_opcode_bytes_round_trip` table-driven assembler dispatch test. |
| `tvm_vm/src/executor/engine/handlers.rs` | `c7_handlers.set(0x4A, execute_zkhalo2_verify_with_vk)` under `#[cfg(feature = "gosh")]`. |
| `tvm_vm/src/executor/gas/gas_state.rs` | `Gas::zkhalo2_verify_with_vk_price()` getter + placeholder `ZKHALO2_VERIFY_WITH_VK_GAS_PRICE = 5_000`. |

## 6. Drive-by fixes applied on this branch

The parent branch (`serhii/node-3406-vergrth16-with-vk`) does not compile
with `--features gosh` as it stands. Two pre-existing breaks were fixed
inline so this PR's CI can pass:

1. **Stray identifier in `gas_state.rs`** — `consume_chkhistproof`
   contained a bare `full_dex_test_with_final_halo2_circuit` token
   (looks like a branch-name accidentally pasted into source during a
   merge). Removed.
2. **Missing `execute_poseidon` in `zk.rs`** — the dispatch table
   references `execute_poseidon` at `0xC7 0x50` but the function was
   dropped during the merge from `full_dex_test_with_final_halo2_circuit`.
   Restored from `origin/main` verbatim.
3. **Missing `IntegerData::from_unsigned_bytes_le`** — same merge dropped
   this method. Restored from `origin/main` verbatim. Needed by
   `execute_poseidon`.

These are intentionally **separate, narrow** fixes and easy to drop into
Serhii's branch before merge if preferred. They are listed at the top of
the PR's commits.

## 7. Re-benchmark TODO before mainnet

`ZKHALO2_VERIFY_WITH_VK_GAS_PRICE = 5_000` is a placeholder modelled on
`VERGRTH16_WITH_VK_GAS_PRICE`. The numbers we want to nail down with a
proper benchmark:

- **Warm-cache verify** — VK already in the LRU, just bundle parse + 
  `verify_proof`. Expected: ~50 ms wall-clock at `k=20` → ~few thousand
  gas at typical AN gas-to-CPU ratio.
- **Cold-cache verify** — VK newly inserted, plus
  `EvaluationDomain::new(k)` (the slow step). Expected: ~3 s at `k=20`.
  Cold path is **NOT** charged extra in the current placeholder because
  operators are expected to pre-warm; if pre-warming proves
  operationally fragile we'll add a gas multiplier on the cold path.
- **Bundle parse + strict-LE instance decode** — pure bytewise work.
  Expected: <1 ms for typical bundle sizes.

## Cooperation model (going forward)

1. This PR rebases cleanly onto `serhii/node-3406-vergrth16-with-vk`.
   Once that branch is on `main`, this PR rebases onto `main` as a
   single follow-up commit.
2. In parallel, the bridge team lands the producer-side companion
   changes in `Pruvendo/acki-nacki-bridge`:
   - `crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs` — wire
     format (already in tree, round-trip-tested against real Circuit 1B
     fallback proofs).
   - `TVM-Solidity-Compiler` `gosh.zkHalo2VerifyWithVK` builtin — PR
     pending.
   - AN-side `TokenBridge.finalizeDeposit(...)` Solidity — PR pending.
