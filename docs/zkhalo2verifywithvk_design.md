# `ZKHALO2VERIFYWITHVK` — Design Note (Discussion Artifact)

**Status**: discussion artifact, **not a merge-ready PR**.
**Authors**: bridge integration team @ Pruvendo.
**Date**: 2026-05-17.
**Parallel branch**: [`serhii/node-3406-vergrth16-with-vk`](https://github.com/tvmlabs/tvm-sdk/tree/serhii/node-3406-vergrth16-with-vk) — Serhii's current `ZKHALO2VERIFY` (hard-coded DarkDex W=8 VK) + `VERGRTH16WITHVK`.

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

## Open questions (mirror of the bridge memo)

See bridge repo `docs/zk_halo2_an_side_design.md` §4 for the full text. Headline questions, in order of expected impact on this opcode:

1. **Q-WIRE-1** — transcript flavour. Blake2b (current `gosh-zk-snark-halo2-utils`) vs Keccak (bridge's `deposit-prover` historical default). Bridge team prefers Blake2b; one-line switch on producer side.
2. **Q-WIRE-4** — VK serialization envelope. Self-describing (preferred, requires PR to `gosh-zk-snark-halo2-utils`) vs caller-supplied `BaseCircuitParams` header in the cell.
3. **Q-WIRE-2** — SRS sharing. Per-circuit `ParamsKZG<Bn256>` vs one shared SRS per `K`.
4. **Q-WIRE-3** — public-input u64 shortcut. Strict 32-byte LE Fr (bridge preference) vs current dual-path.
5. **Q-WIRE-5** — `halo2-lib` fork pinning. Branch vs commit-SHA.

## Cooperation model

We propose:

1. The AN team finishes the current `serhii/node-3406-vergrth16-with-vk` PR (VERGRTH16WITHVK + ZKHALO2VERIFY for DarkDex). We're already reviewing it.
2. Once that's merged, the bridge team rebases this skeleton on top, fleshes out the handler against the now-on-`main` `gosh-zk-snark-halo2-utils` import, adds the LRU cache + tests, and sends a follow-up PR.
3. In parallel, we land the producer-side companion changes in `Pruvendo/acki-nacki-bridge` (`deposit-prover` transcript switch if needed; `TVM-Solidity-Compiler` `gosh.zkHalo2VerifyWithVK` builtin; AN-side `TokenBridge.finalizeDeposit(...)`).

Naming, dispatch byte, and gas number are all open to change based on partner feedback — this skeleton exists to make the conversation concrete.
