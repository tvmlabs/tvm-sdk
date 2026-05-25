//! Real round-trip test for `ZKHALO2VERIFYWITHVK` (opcode `0xC7 0x4A`).
//!
//! Builds a [`Halo2TvmBundle`](`tvm_vm::executor::zk_halo2_with_vk_bundle::Halo2TvmBundle`)
//! payload from the checked-in DarkDex W=8 L0 fixture
//! (`tvm_vm/halo2_test_data/dark_dex_w8_L0_{proof,instances}.bin` and the
//! embedded `DARK_DEX_W8_VK_BYTES` constant), pushes it as a cell onto the
//! VM stack, runs the handler, and asserts the boolean result.
//!
//! Covers:
//!
//! - **Positive path**: a real Halo2 SHPLONK proof for DarkDex W=8 L0
//!   round-trips through `Halo2TvmBundle` → `execute_zkhalo2_verify_with_vk` →
//!   `true`.
//! - **Negative paths**:
//!   - Flip a byte in the proof — handler returns `false` (cryptographic
//!     reject).
//!   - Tweak an instance Fr — handler returns `false`.
//!   - Bad bundle magic — handler returns `FatalError`.
//!
//! Encoding uses strict 32-byte LE `Fr` (Q-WIRE-3) — no u64 shortcuts.

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

const BUNDLE_MAGIC: &[u8; 8] = b"HALO2TVM";
const BUNDLE_VERSION: u8 = 1;
const TRANSCRIPT_BLAKE2B: u8 = 0;

fn build_bundle_bytes(config_json: &[u8], vk: &[u8], instances: &[u8], proof: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(BUNDLE_MAGIC);
    out.push(BUNDLE_VERSION);
    out.push(TRANSCRIPT_BLAKE2B);
    out.extend_from_slice(&[0u8; 6]);
    for chunk in [config_json, vk, instances, proof] {
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

fn run_with_bundle(bundle_bytes: &[u8]) -> tvm_types::Status {
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(bundle_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));
    execute_zkhalo2_verify_with_vk(&mut engine)?;
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    if res { Ok(()) } else { Err(tvm_types::error!("verifier returned false")) }
}

#[test]
fn round_trip_dark_dex_w8_l0_valid_proof_returns_true() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).expect("config_params.json must exist");
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).expect("instances file must exist");
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).expect("proof file must exist");
    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);

    run_with_bundle(&bundle).expect("valid DarkDex W=8 L0 bundle must verify");
}

#[test]
fn flipped_proof_byte_rejected_as_false() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let mut proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();

    // Flip a byte in the middle of the proof. The handler collapses every
    // `halo2_proofs::plonk::verify_proof` `Result::Err` (whether structural
    // or cryptographic) into `false` via `.is_ok()` — the contract for a
    // well-formed *bundle* containing a tampered *proof* is `Ok(false)`,
    // never a `FatalError`. (Bundle-level structural errors — bad magic,
    // bad version, etc. — are tested separately as `FatalError`.)
    let mid = proof.len() / 2;
    proof[mid] ^= 0xFF;

    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

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

    // Flip the LOW byte of the first instance (it's a real Fr — flipping the
    // low byte stays inside the modulus and stays a valid Fr, but no longer
    // equals what the proof was computed against).
    instances[0] ^= 0x01;

    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    execute_zkhalo2_verify_with_vk(&mut engine).expect("handler must not fatal on valid Fr");
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "tweaked instance Fr must NOT verify as true");
}

#[test]
fn bad_magic_returns_fatal_error() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let mut bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);

    // Corrupt the first byte of the magic.
    bundle[0] = b'X';

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    let err = execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("bad bundle magic must trigger FatalError");
    assert!(
        err.to_string().contains("magic mismatch"),
        "expected magic mismatch error, got: {err}"
    );
}

#[test]
fn corrupt_vk_byte_never_verifies_as_true() {
    // Soundness contract: flipping a byte deep inside the VK must NEVER
    // result in `Ok(true)`. The acceptable outcomes are
    //   (a) `FatalError`            — VK deserialisation failed (curve
    //                                  membership or shape mismatch), or
    //   (b) `Ok(false)`              — VK deserialised to a structurally
    //                                  valid but cryptographically wrong
    //                                  key, and the proof failed to verify.
    // Either rejection is sound. We don't pin which one because halo2-base
    // VK serialisation has multiple layers (header, fixed commitments,
    // permutation commitments, transcript repr) and a single byte flip
    // hits different layers depending on offset / VK layout.
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();

    let mut vk = DARK_DEX_W8_VK_BYTES;
    vk[64] ^= 0xFF;

    let bundle = build_bundle_bytes(&cfg, &vk, &instances, &proof);
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

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

    // Overwrite the first instance chunk with all-`0xFF`, which is ≫ Fr
    // modulus. Strict `Fr::from_repr` must reject and the handler must
    // surface that as `FatalError` (structural input error), not `false`.
    let mut instances = valid_instances;
    for b in instances.iter_mut().take(32) {
        *b = 0xFF;
    }

    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    let err = execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("out-of-range Fr must trigger FatalError, not false");
    assert!(err.to_string().contains("modulus"), "expected `>= modulus` error, got: {err}");
}

#[test]
fn instance_count_mismatch_rejected_as_false() {
    // The VK encodes the number of instance columns and per-column lengths,
    // so dropping the last instance Fr leaves a structurally-valid bundle
    // (length still a multiple of 32) but one whose instance vector
    // disagrees with what the proof was generated against. This is a
    // cryptographic reject (`Ok(false)`), not a structural error.
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let valid_instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    assert!(valid_instances.len() >= 64, "fixture must have ≥ 2 instances");

    let short_instances = valid_instances[..valid_instances.len() - 32].to_vec();
    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &short_instances, &proof);

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    execute_zkhalo2_verify_with_vk(&mut engine)
        .expect("structurally-valid bundle with wrong instance count must verify-then-false");
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "instance count mismatch must NOT verify as true");
}

#[test]
fn empty_proof_rejected_as_false() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &[]);

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    execute_zkhalo2_verify_with_vk(&mut engine)
        .expect("zero-length proof must be cryptographic reject, not FatalError");
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "empty proof must NOT verify as true");
}

#[test]
fn malformed_config_json_returns_fatal_error() {
    // Replace the JSON config with non-JSON garbage. The bundle parser
    // attempts `serde_json::from_slice::<BaseCircuitParams>` and must
    // surface the deserialisation failure as `FatalError`.
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let bad_cfg = b"this is not json";

    let bundle = build_bundle_bytes(bad_cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

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
    // Real fixture has `"k":19` (compact, no spaces) — swap it for `"k":18`.
    let mutated = cfg_text.replacen("\"k\":19", "\"k\":18", 1);
    assert_ne!(mutated, cfg_text, "config fixture must contain `\"k\":19` to mutate");

    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let bundle = build_bundle_bytes(mutated.as_bytes(), &DARK_DEX_W8_VK_BYTES, &instances, &proof);

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    let err = execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("config.k != VK domain.k must trigger FatalError");
    let s = err.to_string();
    // Three acceptable failure modes:
    //   (a) our post-read `config.k != vk.domain().k()` guard fires
    //   (b) VK byte-stream rejects under the wrong config (`VerifyingKey::read`)
    //   (c) `BaseCircuitBuilder::new(config)` panics on an internally
    //       inconsistent config (e.g. `lookup_bits >= k`) and our
    //       `catch_unwind` wrapper converts that to a structural FatalError.
    assert!(
        s.contains("config.k")
            || s.contains("domain.k")
            || s.contains("VerifyingKey")
            || s.contains("panicked"),
        "expected k-mismatch error (got: {s})"
    );
}

#[test]
fn fifo_cache_reused_across_two_invocations() {
    // Same VK twice in a row → second invocation should hit the per-VK cache.
    // We can't directly observe cache state, but we can sanity-check that two
    // sequential proves with the same VK both verify and complete quickly.
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();
    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);

    for _ in 0..2 {
        run_with_bundle(&bundle).expect("identical bundle must verify on every call");
    }
}
