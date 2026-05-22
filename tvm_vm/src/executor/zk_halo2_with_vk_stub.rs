// Copyright (C) 2026 Pruvendo (bridge integration team).
//
// Skeleton / discussion artifact for the proposed `ZKHALO2VERIFYWITHVK` opcode.
// See `docs/zkhalo2verifywithvk_design.md` at the repo root for the full design
// note and `Pruvendo/acki-nacki-bridge` `docs/zk_halo2_an_side_design.md` for
// the bridge-side context (Phase 4.3 of the EVM ↔ Acki Nacki bridge
// integration plan).
//
// This file is **deliberately not wired to any halo2 backend**. Its only job
// at this stage is to lock in the stack ABI, gas constant placeholder and
// mnemonic dispatch byte so partner review can converge. The real
// implementation will land on top of Serhii's `serhii/node-3406-vergrth16-with-vk`
// branch once that PR (which brings in `halo2-base` and
// `gosh-zk-snark-halo2-utils` as `gosh`-feature deps) is merged.
//
// Design questions Q-WIRE-1 … Q-WIRE-5 are CLOSED as of 2026-05-22. See
// `docs/zkhalo2verifywithvk_design.md` §"Decisions (2026-05-22)" for the
// frozen wire-format contract. The remaining gate is purely dependency
// availability on `tvm-sdk`'s `main` (i.e., merge of
// `serhii/node-3406-vergrth16-with-vk`).

use tvm_types::ExceptionCode;
use tvm_types::error;

use crate::error::TvmError;
use crate::executor::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::gas::gas_state::Gas;
use crate::types::Exception;
use crate::types::Status;

/// Placeholder gas price for the proposed `ZKHALO2VERIFYWITHVK` opcode.
///
/// **Status**: tentative. Real benchmarking pending Phase B of the bridge
/// integration roadmap (see `docs/zkhalo2verifywithvk_design.md` §3.2).
///
/// Rationale for the placeholder magnitude:
/// - Halo2 SHPLONK verification at K=19 (the deposit-prover's circuit size)
///   is on the order of single-digit milliseconds on a warm VK/SRS cache —
///   structurally similar in wall-clock cost to a Groth16 pairing check, so
///   we start from `VERGRTH16_WITH_VK_GAS_PRICE` (2600 on Serhii's branch)
///   and add a margin for the larger MSM in the public-input contribution
///   computation and for the deserialization of the (much larger) Halo2 VK.
/// - Cold-cache calls (fresh VK, requiring `EvaluationDomain` reconstruction
///   for 2^K twiddle factors) are *not* covered by this number; they must
///   either be served from the `warmup_halo2()` background loader path or
///   carry a separate one-time setup gas charge.
///
/// **Re-benchmark before mainnet.**
pub const ZKHALO2_VERIFY_WITH_VK_GAS_PRICE: i64 = 5_000;

/// Skeleton handler for the proposed `ZKHALO2VERIFYWITHVK` opcode.
///
/// **Stack** (top → bottom) — frozen as of 2026-05-22:
/// - `vk_cell` — `Cell` carrying a `Halo2TvmBundle` (magic `b"H2TVMBND"`,
///   `format_version = 0x01`, `transcript_kind = 0x01 = Blake2b`,
///   length-prefixed `(config_json, vk_bytes, instances_in_vk_position,
///   /* proof slot empty for VK-only path */)`). Self-describing —
///   consumer reconstructs `BaseCircuitParams` from `config_json` before
///   calling `VerifyingKey::<G1Affine>::read(...)`. Format defined in
///   `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`.
///   Q-WIRE-4 — DECIDED (Option B / self-describing).
/// - `public_inputs_cell` — `Cell` holding `N × 32` bytes, each a little-endian
///   `Fr::to_repr()`. Strict — no u64 shortcut, no preprocessing. ≥ modulus
///   bytes raise `FatalError`. Q-WIRE-3 — DECIDED (strict LE Fr).
/// - `proof_cell` — `Cell` holding the SHPLONK proof bytes. Transcript:
///   Blake2b (matches `gosh-zk-snark-halo2-utils::Proof`).
///   Q-WIRE-1 — DECIDED (Blake2b). The bundle's `transcript_kind` byte
///   discriminates if the AN team later wants to add Keccak under a
///   bumped `format_version`.
///
/// **KZG SRS** is **NOT** on the stack. The opcode looks up a shared
/// `ParamsKZG<Bn256>` keyed by `vk.cs.k()`, loaded once at VM startup from
/// `$AN_DATA_DIR/halo2_srs/kzg_bn254_<k>.srs`. Q-WIRE-2 — DECIDED
/// (shared globally per `k`).
///
/// **Pushes** `Boolean`:
/// - `true`  — pairing/MSM checks succeed.
/// - `false` — well-formed but invalid proof.
///
/// **Throws** `FatalError` on structural errors (malformed VK / proof bytes;
/// public-input length not a multiple of 32; transcript mismatch *once*
/// transcript discrimination lands).
///
/// **Current implementation** (this file): always throws
/// `ExceptionCode::FatalError`. The stack-pop and gas-charge are exercised so
/// upstream callers and the assembler-side dispatch table get tested today;
/// the actual cryptographic verification call will replace the explicit error
/// raise in the follow-up PR (Phase B of the roadmap).
#[cfg(feature = "gosh")]
pub(crate) fn execute_zkhalo2_verify_with_vk_stub(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("ZKHALO2VERIFYWITHVK"))?;
    engine.try_use_gas(Gas::zkhalo2_verify_with_vk_price())?;
    fetch_stack(engine, 3)?;

    // Pop the three operands so the failure mode in this stub matches the
    // shape of the final implementation (no spurious stack-shape difference
    // in callers' error paths).
    let _vk_cell = engine.cmd.var(0).as_cell()?;
    let _public_inputs_cell = engine.cmd.var(1).as_cell()?;
    let _proof_cell = engine.cmd.var(2).as_cell()?;

    err!(
        ExceptionCode::FatalError,
        "ZKHALO2VERIFYWITHVK skeleton: design CLOSED 2026-05-22 (Q-WIRE-1..5 \
         frozen in docs/zkhalo2verifywithvk_design.md), wiring blocked on \
         serhii/node-3406-vergrth16-with-vk landing the gosh-zk-snark-halo2-utils \
         dependency tree on tvm-sdk main. Follow-up PR will replace this err! \
         with Proof::verify_with_vk against an LRU-cached (VerifyingKey, ParamsKZG) \
         pair."
    )
}
