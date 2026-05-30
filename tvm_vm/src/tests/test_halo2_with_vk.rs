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
//! - **Positive path**: a real Halo2 SHPLONK proof for DarkDex W=8 L0 round-trips
//!   through `Halo2TvmBundle` → `execute_zkhalo2_verify_with_vk` → `true`.
//! - **Negative paths**:
//!   - Flip a byte in the proof — handler returns `false` (cryptographic reject).
//!   - Tweak an instance Fr — handler returns `false`.
//!   - Bad bundle magic — handler returns `FatalError`.
//!
//! The fixture file is the SAME `instances.bin` used by `test_halo2.rs` for
//! the bare `ZKHALO2VERIFY` opcode, so the AN team can confirm parity at a
//! glance. The encoding in that file uses strict 32-byte LE `Fr` already
//! (Q-WIRE-3) — no u64 shortcuts — so it slots directly into the new
//! opcode's strict layout without conversion.

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

/// Real EVM->AN deposit proof: the deposit-prover's RLC (`EthCircuitImpl`)
/// SHPLONK proof for the Sepolia deposit tx (sender 0x967628..60Ce8e,
/// 0.002 ETH) round-trips through the v2 `circuit_shape = Rlc` path of
/// `ZKHALO2VERIFYWITHVK` and verifies `true`. The bundle was assembled from
/// the deposit-prover artefacts (VkBlob v2 RLC + 7 public inputs + Blake2b
/// proof) — the exact bytes `TokenBridge.finalizeDeposit` feeds the opcode.
#[test]
fn round_trip_deposit_rlc_real_proof_returns_true() {
    let bundle = std::fs::read("halo2_test_data/deposit_rlc_bundle.bin")
        .expect("deposit_rlc_bundle.bin must exist");
    run_with_bundle(&bundle).expect("real deposit RLC bundle must verify true");
}

#[test]
fn flipped_proof_byte_rejected_as_false() {
    let cfg = std::fs::read(DARK_DEX_W8_CONFIG_JSON).unwrap();
    let instances = std::fs::read(DARK_DEX_W8_L0_INSTANCES).unwrap();
    let mut proof = std::fs::read(DARK_DEX_W8_L0_PROOF).unwrap();

    // Flip a byte in the middle of the proof. Almost-any byte flip in a SHPLONK
    // proof results in either a structural deserialisation failure (panic-safe
    // path via Result inside verify_proof) OR a sound cryptographic reject.
    // Both surface here as `verifier returned false` because the handler
    // catches the structural error and returns `bool` via Proof::verify_with_vk.
    let mid = proof.len() / 2;
    proof[mid] ^= 0xFF;

    let bundle = build_bundle_bytes(&cfg, &DARK_DEX_W8_VK_BYTES, &instances, &proof);

    // Run twice: either FatalError or false-from-stack are acceptable rejections.
    // We assert that the result is NOT `Ok(true on stack)`.
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    match execute_zkhalo2_verify_with_vk(&mut engine) {
        Ok(()) => {
            let res = engine.cc.stack.get(0).as_bool().unwrap();
            assert!(!res, "flipped proof byte must NOT verify as true");
        }
        Err(_) => {
            // Structural reject inside verify (e.g. malformed proof bytes)
            // is also an acceptable rejection.
        }
    }
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
fn lru_cache_reused_across_two_invocations() {
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
