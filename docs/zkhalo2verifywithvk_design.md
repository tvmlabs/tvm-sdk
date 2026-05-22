# `ZKHALO2VERIFYWITHVK` — Design Note + Frozen Wire-Format Contract

**Status**: design **CLOSED** as of 2026-05-22 (see "Decisions" section
below). Skeleton tracked as a Draft PR; real implementation lands as a
follow-up PR once `serhii/node-3406-vergrth16-with-vk` is on `main`.
**Authors**: bridge integration team @ Pruvendo.
**Original date**: 2026-05-17. **Frozen**: 2026-05-22.
**Parallel branch**: [`serhii/node-3406-vergrth16-with-vk`](https://github.com/tvmlabs/tvm-sdk/tree/serhii/node-3406-vergrth16-with-vk) — Serhii's current `ZKHALO2VERIFY` (hard-coded DarkDex W=8 VK) + `VERGRTH16WITHVK`.

## Decisions (2026-05-22)

Closed via Serhii's "Принимай решения на свой вкус, Сергей готов принять
любое наше решение" (2026-05-22). Bridge team locked the following
contract; canonical reference is `Pruvendo/acki-nacki-bridge`
`docs/zk_halo2_an_side_design.md` §4. Any future change requires a
bumped `Halo2TvmBundle::FORMAT_VERSION` byte and a paired Decision Log
entry in both repos.

| ID | Decision |
| -- | -------- |
| Q-WIRE-1 (transcript) | Blake2b SHPLONK. `transcript_kind` byte in bundle reserved (`0x01 = Blake2b`); `0x02` reserved for Keccak under a future format version. |
| Q-WIRE-2 (KZG SRS) | Globally shared `ParamsKZG<Bn256>` keyed by `k`, loaded once at VM startup from `$AN_DATA_DIR/halo2_srs/kzg_bn254_<k>.srs`. NOT in the bundle. |
| Q-WIRE-3 (pub inputs) | Strict 32-byte little-endian `Fr::to_repr()`. `Fr::from_repr` rejects ≥ modulus inputs structurally (FatalError). No u64 shortcut on this opcode (legacy `ZKHALO2VERIFY` keeps its shortcut for DarkDex backward compat). |
| Q-WIRE-4 (VK envelope) | Self-describing `Halo2TvmBundle`. Layout: `magic b"H2TVMBND"` (8 B) ‖ `format_version=0x01` (1 B) ‖ `transcript_kind=0x01` (1 B) ‖ LE-u32-len-prefixed `(config_json, vk_bytes, instances, proof)`. Reference impl: `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`. |
| Q-WIRE-5 (fork pinning) | Pin `gosh-sh/halo2-lib-zkevm-sha256-and-bls12-381` to `rev = "<sha>"` in `tvm_vm/Cargo.toml`. Bridge CI carries a fixture `Halo2TvmBundle` and verifies it through `verify_with_vk` on every SHA bump. |
| Q-NAME-1 (opcode name) | `ZKHALO2VERIFYWITHVK` at dispatch byte `0xC7 0x4A`. Rebases onto the byte adjacent to the final `ZKHALO2VERIFY` at follow-up PR time. Compiler builtin: `gosh.zkHalo2VerifyWithVK(proof, pub_inputs, vk_bundle)`. |

Empirical sizes (real Circuit 1B fallback fixture, k=20, 10 signers): bundle ≈ 21.2 KB (VK 6.1 KB + proof 14.8 KB + 4 × 32 B instances + headers). Round-trip-tested at `acki-nacki-bridge/crates/bridge-prover-orchestrator/tests/halo2_tvm_bundle_round_trip.rs`.

## TL;DR

`ZKHALO2VERIFY` (0xC7 0x49 on Serhii's branch) hard-codes the verifying key to the DarkDex W=8 circuit. The Acki Nacki ↔ Ethereum bridge needs to verify a **different** Halo2 SHPLONK proof — one produced by the `deposit-prover` Halo2 circuit in the bridge repo, with a different K, different `BaseCircuitParams`, and different VK. Generalising in place means a per-circuit branch in the VM, which doesn't scale.

We propose the same pivot that `VERGRTH16WITHVK` made for the Groth16 side: a sibling opcode that **takes the verifying key as a third stack operand**. This file is the design note; the companion code change on this branch is a thin skeleton (handler stub + mnemonic + gas constant) that compiles against `main`'s dependency set — no `halo2-base` wiring yet, so the skeleton can be reviewed independently of the merge order with respect to Serhii's branch.

The full design discussion (open questions Q-WIRE-1 … Q-WIRE-5, gas model, per-VK cache, public-input wire format, end-to-end roadmap) lives in the bridge repo: **`docs/zk_halo2_an_side_design.md`** in [`Pruvendo/acki-nacki-bridge`](https://github.com/Pruvendo/acki-nacki-bridge) at commit pointing to this design.

## Why a separate opcode and not "extend `ZKHALO2VERIFY`"

Same rationale as `VERGRTH16` / `VERGRTH16WITHVK`:

| | `ZKHALO2VERIFY` | `ZKHALO2VERIFYWITHVK` |
| --- | --- | --- |
| Stack | 2 cells: `[pub_inputs, proof]` | 3 cells: `[pub_inputs, proof, vk]` |
| VK | Hard-coded global (system circuit: zkLogin-style, DarkDex, …) | Caller-supplied (per-application: bridge, NFT, ZK-app) |
| Gas | Lower (no per-call VK deserialization) | Higher (~5000 placeholder, see §3.2 of bridge memo) |
| Use-cases | System-level circuits with one VK per network | App-level circuits with per-contract VKs |
| Cache | Single static `OnceLock<(VK, Params)>` | LRU keyed by `keccak256(vk_bytes)` |

Keeping both opcodes available means the bridge doesn't pay the `WithVK` gas premium on every system-circuit verification, and system circuits keep their compact 2-operand calling convention.

## Skeleton on this branch — what's here, what's stubbed

This branch is **deliberately minimal** so it can be reviewed without taking a dependency on the in-flight `halo2-base` / `gosh-zk-snark-halo2-utils` wiring from Serhii's parallel branch:

- `tvm_vm/src/executor/zk_halo2_with_vk_stub.rs` — handler stub. Loads instruction, pops 3 operands, charges placeholder gas, returns `FatalError("ZKHALO2VERIFYWITHVK is a design stub — implementation pending Phase A (transcript/VK envelope alignment)")`. Lets us pin down the **stack ABI, gas constant, and mnemonic dispatch byte** without locking in the verification machinery.
- `tvm_vm/src/executor/engine/handlers.rs` — registers `0xC7 0x4A` under `#[cfg(feature = "gosh")]`. **Note**: chose `0x4A` because Serhii's branch uses `0x49` for `ZKHALO2VERIFY` and `0x52` for `VERGRTH16WITHVK`; `0x4A` is the natural sibling. If Serhii's branch merges first and reshuffles bytes, this skeleton picks up whatever byte remains adjacent to `ZKHALO2VERIFY` at merge time.
- `tvm_assembler/src/simple.rs` — mnemonic entry.
- `tvm_vm/src/executor/gas/gas_state.rs` — `zkhalo2_verify_with_vk_price()` getter.
- A minimal `#[ignore]`d test fixture in `tvm_vm/src/tests/test_zkhalo2_verify_with_vk_stub.rs` that exercises the stack-pop + gas-charge path but expects `FatalError` (will be flipped to a real round-trip test once Phase A lands).

The skeleton does **NOT** include:

- The actual Halo2 SHPLONK verification call (deferred until `gosh-zk-snark-halo2-utils` dep tree lands via Serhii's PR).
- The per-VK LRU cache (deferred — will reuse `Lazy<Mutex<LruCache<…>>>` once the `lru` dep is approved or use `OnceLock<HashMap<...>>` if not).
- The VK envelope deserialization (deferred until Q-WIRE-4 is answered).

## Closed questions (rationale capture — read-only)

The following questions were posed against this skeleton on 2026-05-17;
all are now closed by the Decisions block above. Section kept for
review-trail purposes.

1. **Q-WIRE-1** — transcript flavour. Blake2b (current `gosh-zk-snark-halo2-utils`) vs Keccak. → **Blake2b**. Bridge already uses Blake2b in `crates/bridge-prover-orchestrator/src/prover.rs::generate_fallback_proof`; round-trip-tested.
2. **Q-WIRE-4** — VK serialization envelope. Self-describing vs caller-supplied `BaseCircuitParams` header. → **Self-describing `Halo2TvmBundle`**. Inline `config_json` removes any out-of-band schema dependency.
3. **Q-WIRE-2** — SRS sharing. Per-circuit `ParamsKZG<Bn256>` vs one shared SRS per `K`. → **Shared globally per `k`**. SRS depends only on `2^k`; per-circuit SRS would mean ~80 MB × N circuits on each TVM node.
4. **Q-WIRE-3** — public-input u64 shortcut. Strict 32-byte LE Fr (bridge preference) vs current dual-path. → **Strict LE Fr**. New opcode has no shortcut; legacy `ZKHALO2VERIFY` keeps its shortcut for DarkDex backward compat.
5. **Q-WIRE-5** — `halo2-lib` fork pinning. Branch vs commit-SHA. → **Commit SHA + format-stability CI gate** on the bridge side.

## Cooperation model

1. The AN team finishes the current `serhii/node-3406-vergrth16-with-vk` PR (VERGRTH16WITHVK + ZKHALO2VERIFY for DarkDex). Bridge team is reviewing.
2. Once that lands on `main`, the bridge team rebases this skeleton on top, replaces the `FatalError` stub with the real handler against the now-on-`main` `gosh-zk-snark-halo2-utils` import, adds the LRU cache + tests, and submits a follow-up PR. Branch tracking ID: `tvm-sdk:zkhalo2vk-real-impl`.
3. In parallel, the bridge team lands the producer-side companion changes in `Pruvendo/acki-nacki-bridge`:
   - `crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs` — wire format (already in tree, round-trip-tested).
   - `TVM-Solidity-Compiler` `gosh.zkHalo2VerifyWithVK` builtin — PR pending.
   - AN-side `TokenBridge.finalizeDeposit(...)` Solidity — PR pending.

Naming, dispatch byte, and gas number are open to last-mile adjustment at follow-up PR time (e.g. dispatch byte adjacency may shift after `node-3406` merge); the rest of the contract is frozen.
