//! Real round-trip test for `ZKHALO2VERIFYWITHVK` (opcode `0xC7 0x4A`).
//!
//! Builds the three stack operands of the opcode from the checked-in
//! DarkDex W=8 L0 fixture
//! (`tvm_vm/halo2_test_data/dark_dex_w8_L0_{proof,instances}.bin` and the
//! embedded `DARK_DEX_W8_VK_BYTES` constant), pushes them as three
//! separate cells onto the VM stack, runs the handler, and asserts the
//! boolean result.
//!
//! **ABI** (frozen 2026-05-25 — Variant A):
//!
//! ```text
//!   PUSHREF vk_cell             ← VkBlob (magic "VKBLOB\0\0" + cfg + vk_bytes)
//!   PUSHREF public_inputs_cell  ← raw N × 32 LE Fr
//!   PUSHREF proof_cell          ← raw SHPLONK proof bytes
//!   ZKHALO2VERIFYWITHVK
//! ```
//!
//! Covers:
//! - **Positive path**: a real Halo2 SHPLONK proof for DarkDex W=8 L0
//!   round-trips through the three-cell ABI → `true`.
//! - **Negative paths**: byte-flipped proof, tweaked instance, bad VkBlob
//!   magic, corrupted VK, instance ≥ modulus, instance-count mismatch, empty
//!   proof, malformed config_json, config.k mismatch, cache reuse smoke-test.

use tvm_types::SliceData;

use crate::executor::Engine;
use crate::executor::test_helper::*;
use crate::executor::zk_halo2_utils::DARK_DEX_W8_VK_BYTES;
use crate::executor::zk_halo2_with_vk::execute_zkhalo2_verify_with_vk;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;

const DARK_DEX_W8_L0_INSTANCES: &str = "halo2_test_data/dark_dex_w8_L0_instances.bin";
const DARK_DEX_W8_L0_PROOF: &str = "halo2_test_data/dark_dex_w8_L0_proof.bin";
const DARK_DEX_W8_CONFIG_JSON: &str = "halo2_test_data/dark_dex_w8_config_params.json";

const VK_BLOB_MAGIC: &[u8; 8] = b"VKBLOB\x00\x00";
const VK_BLOB_VERSION: u8 = 1;
const TRANSCRIPT_BLAKE2B: u8 = 0;

/// Build the `vk_cell` payload (`VkBlob`).
fn build_vk_blob(config_json: &[u8], vk: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(VK_BLOB_MAGIC);
    out.push(VK_BLOB_VERSION);
    out.push(TRANSCRIPT_BLAKE2B);
    out.extend_from_slice(&[0u8; 6]);
    for chunk in [config_json, vk] {
        out.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out
}

fn setup_engine() -> Engine {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls),
        Some(stack),
        None,
        vec![],
    )
}

/// Push the three operands (vk_cell, public_inputs_cell, proof_cell) in
/// assembly order so the handler pops them top-down as
/// `proof, public_inputs, vk`.
fn push_three_operands(engine: &mut Engine, vk_blob: &[u8], public_inputs: &[u8], proof: &[u8]) {
    let vk_cell = pack_data_to_cell(vk_blob, &mut 0).unwrap();
    let pi_cell = pack_data_to_cell(public_inputs, &mut 0).unwrap();
    let pf_cell = pack_data_to_cell(proof, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(vk_cell));
    engine.cc.stack.push(StackItem::cell(pi_cell));
    engine.cc.stack.push(StackItem::cell(pf_cell));
}

fn run_with_operands(vk_blob: &[u8], public_inputs: &[u8], proof: &[u8]) -> tvm_types::Status {
    let mut engine = setup_engine();
    push_three_operands(&mut engine, vk_blob, public_inputs, proof);
    execute_zkhalo2_verify_with_vk(&mut engine)?;
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    if res { Ok(()) } else { Err(tvm_types::error!("verifier returned false")) }
}

#[test]
fn round_trip_dark_dex_w8_l0_valid_proof_returns_true() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).expect("config_params.json must exist");
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).expect("instances file must exist");
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).expect("proof file must exist");
    let vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);

    run_with_operands(&vk_blob, &instances, &proof)
        .expect("valid DarkDex W=8 L0 operands must verify");
}

#[test]
fn flipped_proof_byte_rejected_as_false() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let mut proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();

    // Flip a byte in the middle of the proof. The handler collapses
    // every `halo2_proofs::plonk::verify_proof` `Result::Err` (whether
    // structural or cryptographic) into `false` via `.is_ok()` — the
    // contract for a well-formed *vk_cell* with a tampered *proof_cell*
    // is `Ok(false)`, never a `FatalError`. (vk_cell-level structural
    // errors — bad magic, bad version, etc. — are tested separately as
    // `FatalError`.)
    let mid = proof.len() / 2;
    proof[mid] ^= 0xFF;

    let vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);

    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &proof);

    execute_zkhalo2_verify_with_vk(&mut engine)
        .expect("flipped proof byte must be a cryptographic reject (Ok(false)), not FatalError");
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "flipped proof byte must NOT verify as true");
}

#[test]
fn tweaked_instance_fr_rejected_as_false() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let mut instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();

    // Flip the LOW byte of the first instance (it's a real Fr —
    // flipping the low byte stays inside the modulus and stays a valid
    // Fr, but no longer equals what the proof was computed against).
    instances[0] ^= 0x01;

    let vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);

    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &proof);

    execute_zkhalo2_verify_with_vk(&mut engine).expect("handler must not fatal on valid Fr");
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "tweaked instance Fr must NOT verify as true");
}

#[test]
fn bad_vk_blob_magic_returns_fatal_error() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let mut vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);

    // Corrupt the first byte of the magic.
    vk_blob[0] = b'X';

    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &proof);

    let err = execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("bad VkBlob magic must trigger FatalError");
    assert!(
        err.to_string().contains("magic mismatch"),
        "expected magic mismatch error, got: {err}"
    );
}

#[test]
fn corrupt_vk_byte_never_verifies_as_true() {
    // Soundness contract: flipping a byte deep inside the VK must
    // NEVER result in `Ok(true)`. The acceptable outcomes are
    //   (a) `FatalError`            — VK deserialisation failed (curve
    //                                  membership or shape mismatch),
    //                                  or
    //   (b) `Ok(false)`              — VK deserialised to a
    //                                  structurally valid but
    //                                  cryptographically wrong key,
    //                                  and the proof failed to verify.
    // Either rejection is sound. We don't pin which one because
    // halo2-base VK serialisation has multiple layers (header, fixed
    // commitments, permutation commitments, transcript repr) and a
    // single byte flip hits different layers depending on offset / VK
    // layout.
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();

    let mut vk = DARK_DEX_W8_VK_BYTES;
    vk[64] ^= 0xFF;

    let vk_blob = build_vk_blob(&cfg, &vk);
    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &proof);

    match execute_zkhalo2_verify_with_vk(&mut engine) {
        Ok(()) => {
            let res = engine.cc.stack.get(0).as_bool().unwrap();
            assert!(!res, "corrupted VK must NOT verify as true");
        }
        Err(_) => {} // structural FatalError from VK::read — also sound.
    }
}

#[test]
fn instance_ge_modulus_returns_fatal_error() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let valid_instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    assert!(valid_instances.len() >= 32, "fixture must have ≥ 1 instance");

    // Overwrite the first instance chunk with all-`0xFF`, which is ≫
    // Fr modulus. Strict `Fr::from_repr` must reject and the handler
    // must surface that as `FatalError` (structural input error), not
    // `false`.
    let mut instances = valid_instances;
    for b in instances.iter_mut().take(32) {
        *b = 0xFF;
    }

    let vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);
    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &proof);

    let err = execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("out-of-range Fr must trigger FatalError, not false");
    assert!(err.to_string().contains("modulus"), "expected `>= modulus` error, got: {err}");
}

#[test]
fn instance_count_mismatch_rejected_as_false() {
    // The VK encodes the number of instance columns and per-column
    // lengths, so dropping the last instance Fr leaves a
    // structurally-valid public_inputs_cell (length still a multiple
    // of 32) but one whose instance vector disagrees with what the
    // proof was generated against. This is a cryptographic reject
    // (`Ok(false)`), not a structural error.
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let valid_instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    assert!(valid_instances.len() >= 64, "fixture must have ≥ 2 instances");

    let short_instances = valid_instances[..valid_instances.len() - 32].to_vec();
    let vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);

    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &short_instances, &proof);

    execute_zkhalo2_verify_with_vk(&mut engine).expect(
        "structurally-valid public_inputs_cell with wrong instance count must \
         verify-then-false",
    );
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "instance count mismatch must NOT verify as true");
}

#[test]
fn empty_proof_rejected_as_false() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);

    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &[]);

    execute_zkhalo2_verify_with_vk(&mut engine)
        .expect("zero-length proof must be cryptographic reject, not FatalError");
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "empty proof must NOT verify as true");
}

#[test]
fn malformed_config_json_returns_fatal_error() {
    // Replace the JSON config with non-JSON garbage. The VkBlob parser
    // attempts `serde_json::from_slice::<BaseCircuitParams>` and must
    // surface the deserialisation failure as `FatalError`.
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let bad_cfg = b"this is not json";

    let vk_blob = build_vk_blob(bad_cfg, &DARK_DEX_W8_VK_BYTES);
    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &proof);

    execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("malformed config_json must trigger FatalError");
}

#[test]
fn config_k_mismatch_returns_fatal_error() {
    // Read the real config, mutate its `k`, re-serialise. The handler
    // verifies the JSON `k` agrees with `vk.get_domain().k()` and must
    // reject loudly (FatalError) when it doesn't — even though VK
    // deserialisation under the wrong `BaseCircuitParams` may still
    // succeed.
    let cfg_text = std::fs::read_to_string(DARK_DEX_W8_CONFIG_JSON).unwrap();
    // Real fixture has `"k":19` (compact, no spaces) — swap it for
    // `"k":18`.
    let mutated = cfg_text.replacen("\"k\":19", "\"k\":18", 1);
    assert_ne!(mutated, cfg_text, "config fixture must contain `\"k\":19` to mutate");

    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let vk_blob = build_vk_blob(mutated.as_bytes(), &DARK_DEX_W8_VK_BYTES);

    let mut engine = setup_engine();
    push_three_operands(&mut engine, &vk_blob, &instances, &proof);

    let err = execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("config.k != VK domain.k must trigger FatalError");
    let s = err.to_string();
    // Three acceptable failure modes:
    //   (a) our post-read `config.k != vk.domain().k()` guard fires
    //   (b) VK byte-stream rejects under the wrong config
    //       (`VerifyingKey::read`)
    //   (c) `BaseCircuitBuilder::new(config)` panics on an internally
    //       inconsistent config (e.g. `lookup_bits >= k`) and our
    //       `catch_unwind` wrapper converts that to a structural
    //       FatalError.
    assert!(
        s.contains("config.k")
            || s.contains("domain.k")
            || s.contains("VerifyingKey")
            || s.contains("panicked"),
        "expected k-mismatch error (got: {s})"
    );
}

#[test]
fn bridge_circuit_1b_fallback_real_proof_verifies() {
    // End-to-end with the real Acki Nacki ↔ Ethereum bridge **Circuit
    // 1B** (Fallback BLS attestation verifier) fixture produced by
    // `bridge-prover-orchestrator::halo2_tvm_bundle` and dumped via
    // `EXPORT_HALO2_FIXTURE_DIR=… cargo test --test
    // halo2_tvm_bundle_round_trip`. The fixture exercises a real
    // K=20 SHPLONK proof (~14.8 KiB) with a real K=20 VK (~6 KiB) and
    // 4 public inputs.
    //
    // The three operand byte streams are stored exactly as they
    // appear on the TVM stack — no extra framing. The handler must
    // accept them and return `Ok(true)`.
    let vk_blob = std::fs::read("halo2_test_data/fallback_vk_blob.bin")
        .expect("real bridge Circuit 1B VkBlob fixture must exist");
    let public_inputs = std::fs::read("halo2_test_data/fallback_public_inputs.bin")
        .expect("real bridge Circuit 1B public_inputs fixture must exist");
    let proof = std::fs::read("halo2_test_data/fallback_proof.bin")
        .expect("real bridge Circuit 1B proof fixture must exist");

    assert_eq!(
        public_inputs.len(),
        4 * 32,
        "Circuit 1B fixture has exactly 4 public inputs (= 128 B), got {}",
        public_inputs.len()
    );

    run_with_operands(&vk_blob, &public_inputs, &proof)
        .expect("real bridge Circuit 1B fallback proof must verify");
}

#[test]
fn fifo_cache_reused_across_two_invocations() {
    // Same VK twice in a row → second invocation should hit the
    // per-VK cache. We can't directly observe cache state, but we can
    // sanity-check that two sequential proves with the same VK both
    // verify and complete quickly.
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let vk_blob = build_vk_blob(&cfg, &DARK_DEX_W8_VK_BYTES);

    for _ in 0..2 {
        run_with_operands(&vk_blob, &instances, &proof)
            .expect("identical operands must verify on every call");
    }
}
