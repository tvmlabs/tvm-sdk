use std::time::Instant;

use tvm_types::SliceData;

use crate::executor::engine::Engine;
use crate::executor::test_helper::*;
use crate::executor::zk_halo2::execute_halo2_proof_verification;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;

// W=128 (historical window size 128) test data paths.
// L0 = 0 chain steps, L1 = 1 chain step, L2 = 2 chain steps.
const W128_L0_PROOF_PATH: &str = "halo2_test_data/dark_dex_w128_L0_proof.bin";
const W128_L0_INSTANCES_PATH: &str = "halo2_test_data/dark_dex_w128_L0_instances.bin";
const W128_L1_PROOF_PATH: &str = "halo2_test_data/dark_dex_w128_L1_proof.bin";
const W128_L1_INSTANCES_PATH: &str = "halo2_test_data/dark_dex_w128_L1_instances.bin";
const W128_L2_PROOF_PATH: &str = "halo2_test_data/dark_dex_w128_L2_proof.bin";
const W128_L2_INSTANCES_PATH: &str = "halo2_test_data/dark_dex_w128_L2_instances.bin";

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

/// Run halo2 proof verification through the TVM engine and return (result,
/// elapsed_ms).
fn verify_proof(proof_path: &str, instances_path: &str) -> (bool, u128) {
    let mut engine = setup_engine();

    let pub_inputs_bytes = std::fs::read(instances_path).expect("Failed to read instances file");
    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(pub_inputs_cell));

    let proof_bytes = std::fs::read(proof_path).expect("Failed to read proof file");
    let proof_cell = pack_data_to_cell(&proof_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell));

    let start = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_millis();

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    (res, elapsed)
}

// ---------------------------------------------------------------------------
// W=128 positive tests: one per fixture (L0, L1, L2 chain steps)
// ---------------------------------------------------------------------------

#[test]
fn test_verify_w128_l0_zero_chain_steps() {
    let (res, elapsed) = verify_proof(W128_L0_PROOF_PATH, W128_L0_INSTANCES_PATH);
    println!("W128L0 (0 chain steps): result={}, elapsed={}ms", res, elapsed);
    assert!(res);
}

#[test]
fn test_verify_w128_l1_one_chain_step() {
    let (res, elapsed) = verify_proof(W128_L1_PROOF_PATH, W128_L1_INSTANCES_PATH);
    println!("W128L1 (1 chain step): result={}, elapsed={}ms", res, elapsed);
    assert!(res);
}

#[test]
fn test_verify_w128_l2_two_chain_steps() {
    let (res, elapsed) = verify_proof(W128_L2_PROOF_PATH, W128_L2_INSTANCES_PATH);
    println!("W128L2 (2 chain steps): result={}, elapsed={}ms", res, elapsed);
    assert!(res);
}

// ---------------------------------------------------------------------------
// Negative tests
// ---------------------------------------------------------------------------

#[test]
fn test_verify_w128_wrong_instances() {
    let mut engine = setup_engine();

    // Use valid L0 instances but flip one byte to make them wrong
    let mut pub_inputs_bytes =
        std::fs::read(W128_L0_INSTANCES_PATH).expect("Failed to read instances file");
    pub_inputs_bytes[0] ^= 0xFF;
    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(pub_inputs_cell));

    let proof_bytes = std::fs::read(W128_L0_PROOF_PATH).expect("Failed to read proof file");
    let proof_cell = pack_data_to_cell(&proof_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell));

    let start = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_millis();

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("W128 wrong instances: result={}, elapsed={}ms", res, elapsed);
    assert!(!res);
}

#[test]
fn test_verify_w128_bad_proof() {
    let mut engine = setup_engine();

    let pub_inputs_bytes =
        std::fs::read(W128_L0_INSTANCES_PATH).expect("Failed to read instances file");
    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(pub_inputs_cell));

    // Corrupt the proof bytes
    let mut proof_bytes = std::fs::read(W128_L0_PROOF_PATH).expect("Failed to read proof file");
    proof_bytes[10] ^= 0xFF;
    proof_bytes[20] ^= 0xFF;
    let proof_cell = pack_data_to_cell(&proof_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell));

    let start = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_millis();

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("W128 bad proof: result={}, elapsed={}ms", res, elapsed);
    assert!(!res);
}

#[test]
fn test_verify_w128_mismatched_proof_and_instances() {
    // L0 proof with L1 instances — should fail
    let (res, elapsed) = verify_proof(W128_L0_PROOF_PATH, W128_L1_INSTANCES_PATH);
    println!("W128 mismatched (L0 proof + L1 instances): result={}, elapsed={}ms", res, elapsed);
    assert!(!res);
}

// ---------------------------------------------------------------------------
// Security: malformed public-input bytes must NOT panic the executor.
//
// `Fr::from_bytes_le` is the default `ScalarField` impl, which calls
// `Fr::from_repr(repr).unwrap()`. For any 32-byte LE value >= the BN254
// scalar field modulus p (~2^254), `from_repr` returns `CtOption::None`
// and `.unwrap()` panics. `pub_input_bytes` originates from attacker-
// controllable cell data, so this is a DoS vector unless the executor
// rejects non-canonical encodings explicitly (matching the existing
// `% 32 != 0` length check).
// ---------------------------------------------------------------------------

/// BN254 Fr modulus p in little-endian (Fr::to_repr() encoding).
/// p = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
const BN254_FR_MODULUS_LE: [u8; 32] = [
    0x01, 0x00, 0x00, 0xf0, 0x93, 0xf5, 0xe1, 0x43, 0x91, 0x70, 0xb9, 0x79, 0x48, 0xe8, 0x33, 0x28,
    0x5d, 0x58, 0x81, 0x81, 0xb6, 0x45, 0x50, 0xb8, 0x29, 0xa0, 0x31, 0xe1, 0x72, 0x4e, 0x64, 0x30,
];

/// Run verification with custom pub_input bytes and return the executor's
/// `Status`. Used for negative tests that must NOT panic.
fn run_with_custom_pub_inputs(
    pub_inputs_bytes: &[u8],
    proof_path: &str,
) -> tvm_types::Result<bool> {
    let mut engine = setup_engine();
    let pub_inputs_cell = pack_data_to_cell(pub_inputs_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(pub_inputs_cell));

    let proof_bytes = std::fs::read(proof_path).expect("Failed to read proof file");
    let proof_cell = pack_data_to_cell(&proof_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell));

    execute_halo2_proof_verification(&mut engine)?;
    Ok(engine.cc.stack.get(0).as_bool().unwrap())
}

#[test]
fn test_verify_w128_pub_input_above_modulus_fails_cleanly() {
    // pub_input = 0xFF...FF (= 2^256 - 1) is far above p. Without the
    // canonicity check, this would panic the executor thread.
    let mut pub_inputs_bytes =
        std::fs::read(W128_L0_INSTANCES_PATH).expect("Failed to read instances file");
    pub_inputs_bytes[0..32].fill(0xFF);

    let result = run_with_custom_pub_inputs(&pub_inputs_bytes, W128_L0_PROOF_PATH);
    assert!(
        result.is_err(),
        "Expected FatalError for non-canonical Fr input (0xFF…FF > p), got {:?}",
        result
    );
    println!("W128 pub_input > p (0xFF…FF): rejected with err (as expected)");
}

#[test]
fn test_verify_w128_pub_input_equals_modulus_fails_cleanly() {
    // pub_input == p exactly: canonical Fr range is [0, p), so p must be
    // rejected (CtOption::None from from_repr).
    let mut pub_inputs_bytes =
        std::fs::read(W128_L0_INSTANCES_PATH).expect("Failed to read instances file");
    pub_inputs_bytes[0..32].copy_from_slice(&BN254_FR_MODULUS_LE);

    let result = run_with_custom_pub_inputs(&pub_inputs_bytes, W128_L0_PROOF_PATH);
    assert!(result.is_err(), "Expected FatalError for pub_input == p (modulus), got {:?}", result);
    println!("W128 pub_input == p: rejected with err (as expected)");
}

#[test]
fn test_verify_w128_pub_input_just_below_modulus_parses() {
    // pub_input == p - 1 is the largest canonical Fr — must parse OK.
    // Verification then fails (instances no longer match the proof), so
    // the executor must return Ok(false), not Err.
    let mut pub_inputs_bytes =
        std::fs::read(W128_L0_INSTANCES_PATH).expect("Failed to read instances file");
    let mut p_minus_one = BN254_FR_MODULUS_LE;
    p_minus_one[0] -= 1; // LSB: 0x01 -> 0x00
    pub_inputs_bytes[0..32].copy_from_slice(&p_minus_one);

    let result = run_with_custom_pub_inputs(&pub_inputs_bytes, W128_L0_PROOF_PATH);
    let res = result.expect("p-1 (largest canonical Fr) must parse without error");
    assert!(!res, "Verification must fail for mutated instances");
    println!("W128 pub_input == p-1: parsed OK, verify=false (as expected)");
}

// ---------------------------------------------------------------------------
// ZKHALO2VERIFYWITHVK (0xC7 0x4A) — caller-supplied-VK opcode, variant B
// (isolated process verifier). Drives the opcode through the real TVM engine
// over a committed set of **10 real deposit proofs** and asserts ACCEPT for
// each, plus negative (corrupt-proof / mismatched-public-input) cases.
//
// Each proof uses the **11-public-input** deposit layout: [depositId, sender,
// amount, contractAddress, dappIdHigh, dappIdLow, anAccountHigh, anAccountLow,
// blockHashHigh, blockHashLow, promiseCommit]. `dappId` (a config-supplied
// UInt256 AN dApp tag) replaced the single `anWorkchain` slot on 2026-06-02;
// `anAccount{High,Low}` remain the event-bound AN recipient account. The opcode
// is generic over the input count (it forwards num_pi = public_inputs.len()/32
// to the isolated verifier), so no opcode change was needed when the layout
// grew 7 -> 10 -> 11.
//
// Stack ABI (top-of-stack last): push vk_blob, then public_inputs, then proof.
// fetch_stack then binds var(0)=proof, var(1)=public_inputs, var(2)=vk_blob.
//
// Requires the isolated verifier binary; its path is taken from the
// AN_RLC_VERIFY_BIN env var (the handler reads the same var). The on-wire
// proofs are version-controlled in the acki-nacki-bridge repo at
// `deposit-prover/fixtures/deposit_10proofs/` (a shared `deposit_vk_blob.bin`
// plus `proof_00`..`proof_09`; override the dir with DEPOSIT_10PROOFS_DIR).
// Ignored by default so CI without the binary/proofs stays green; run with:
//   AN_RLC_VERIFY_BIN=.../an_rlc_verify cargo test -p tvm_vm \
//     test_zkhalo2_with_vk_deposit_10_real_proofs -- --ignored --nocapture
// ---------------------------------------------------------------------------

/// Directory holding the committed 10-real-proof set (11-PI deposit layout).
/// Override with the `DEPOSIT_10PROOFS_DIR` env var.
fn deposit_set_dir() -> String {
    std::env::var("DEPOSIT_10PROOFS_DIR").unwrap_or_else(|_| {
        "/home/sergey/Pruvendo/gosh/acki-nacki-bridge/deposit-prover/fixtures/deposit_10proofs"
            .to_string()
    })
}

/// Number of real proofs in the committed set.
const DEPOSIT_PROOF_COUNT: usize = 10;

/// Shared VkBlob for the deposit circuit (identical across all proofs — same
/// circuit shape, only the witness/public inputs differ per proof).
fn deposit_vk_blob_path() -> String {
    format!("{}/deposit_vk_blob.bin", deposit_set_dir())
}
fn deposit_pubin_path(i: usize) -> String {
    format!("{}/proof_{:02}/public_inputs.bin", deposit_set_dir(), i)
}
fn deposit_proof_path(i: usize) -> String {
    format!("{}/proof_{:02}/proof.bin", deposit_set_dir(), i)
}

fn run_zkhalo2_with_vk(vk_path: &str, pubin_path: &str, proof_path: &str) -> bool {
    use crate::executor::zk_halo2::execute_zkhalo2_verify_with_vk;
    let mut engine = setup_engine();

    // Push in source-argument order: vk_blob, public_inputs, proof (top last).
    let vk_bytes = std::fs::read(vk_path).expect("read vk_blob");
    let vk_cell = pack_data_to_cell(&vk_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(vk_cell));

    let pubin_bytes = std::fs::read(pubin_path).expect("read public_inputs");
    let pubin_cell = pack_data_to_cell(&pubin_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(pubin_cell));

    let proof_bytes = std::fs::read(proof_path).expect("read proof");
    let proof_cell = pack_data_to_cell(&proof_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell));

    execute_zkhalo2_verify_with_vk(&mut engine).unwrap();
    engine.cc.stack.get(0).as_bool().unwrap()
}

#[test]
#[ignore]
fn test_zkhalo2_with_vk_deposit_10_real_proofs() {
    for i in 0..DEPOSIT_PROOF_COUNT {
        let res = run_zkhalo2_with_vk(
            &deposit_vk_blob_path(),
            &deposit_pubin_path(i),
            &deposit_proof_path(i),
        );
        println!("ZKHALO2VERIFYWITHVK 11-PI deposit proof_{:02}: result={}", i, res);
        assert!(res, "expected ACCEPT on real deposit proof_{:02}", i);
    }
    println!("All {} real deposit proofs ACCEPTED.", DEPOSIT_PROOF_COUNT);
}

#[test]
#[ignore]
fn test_zkhalo2_with_vk_corrupt_proof_rejects() {
    use crate::executor::zk_halo2::execute_zkhalo2_verify_with_vk;
    let mut engine = setup_engine();

    let vk_bytes = std::fs::read(deposit_vk_blob_path()).expect("read vk_blob");
    engine.cc.stack.push(StackItem::cell(pack_data_to_cell(&vk_bytes, &mut 0).unwrap()));

    let pubin_bytes = std::fs::read(deposit_pubin_path(0)).expect("read public_inputs");
    engine.cc.stack.push(StackItem::cell(pack_data_to_cell(&pubin_bytes, &mut 0).unwrap()));

    // Flip a byte deep in the proof body: structurally valid, fails verification.
    let mut proof_bytes = std::fs::read(deposit_proof_path(0)).expect("read proof");
    let idx = proof_bytes.len() / 2;
    proof_bytes[idx] ^= 0xFF;
    engine.cc.stack.push(StackItem::cell(pack_data_to_cell(&proof_bytes, &mut 0).unwrap()));

    execute_zkhalo2_verify_with_vk(&mut engine).unwrap();
    let verdict = engine.cc.stack.get(0).as_bool().unwrap();
    println!("ZKHALO2VERIFYWITHVK corrupt proof: verdict={}", verdict);
    assert!(!verdict, "expected REJECT (false) on a corrupted proof");
}

#[test]
#[ignore]
fn test_zkhalo2_with_vk_mismatched_public_inputs_reject() {
    // proof_00's proof paired with proof_01's public inputs must REJECT: each
    // proof is bound to its own public inputs, so a cross-proof pairing fails.
    let res = run_zkhalo2_with_vk(
        &deposit_vk_blob_path(),
        &deposit_pubin_path(1),
        &deposit_proof_path(0),
    );
    println!("ZKHALO2VERIFYWITHVK mismatched pubin: result={}", res);
    assert!(!res, "expected REJECT when pairing proof_00 with proof_01 public inputs");
}
