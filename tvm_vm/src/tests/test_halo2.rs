use std::time::Instant;

use tvm_types::SliceData;

use crate::executor::engine::Engine;
use crate::executor::test_helper::*;
use crate::executor::zk_halo2::execute_halo2_proof_verification;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;

// W=8 (historical window size 8) test data paths.
// L0 = 0 chain steps, L1 = 1 chain step, L2 = 2 chain steps.
const W8_L0_PROOF_PATH: &str = "halo2_test_data/dark_dex_w8_L0_proof.bin";
const W8_L0_INSTANCES_PATH: &str = "halo2_test_data/dark_dex_w8_L0_instances.bin";
const W8_L1_PROOF_PATH: &str = "halo2_test_data/dark_dex_w8_L1_proof.bin";
const W8_L1_INSTANCES_PATH: &str = "halo2_test_data/dark_dex_w8_L1_instances.bin";
const W8_L2_PROOF_PATH: &str = "halo2_test_data/dark_dex_w8_L2_proof.bin";
const W8_L2_INSTANCES_PATH: &str = "halo2_test_data/dark_dex_w8_L2_instances.bin";

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

/// Run halo2 proof verification through the TVM engine and return (result, elapsed_ms).
fn verify_proof(proof_path: &str, instances_path: &str) -> (bool, u128) {
    let mut engine = setup_engine();

    let pub_inputs_bytes =
        std::fs::read(instances_path).expect("Failed to read instances file");
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
// W=8 positive tests: one per fixture (L0, L1, L2 chain steps)
// ---------------------------------------------------------------------------

#[test]
fn test_verify_w8_l0_zero_chain_steps() {
    let (res, elapsed) = verify_proof(W8_L0_PROOF_PATH, W8_L0_INSTANCES_PATH);
    println!("W8 L0 (0 chain steps): result={}, elapsed={}ms", res, elapsed);
    assert!(res);
}

#[test]
fn test_verify_w8_l1_one_chain_step() {
    let (res, elapsed) = verify_proof(W8_L1_PROOF_PATH, W8_L1_INSTANCES_PATH);
    println!("W8 L1 (1 chain step): result={}, elapsed={}ms", res, elapsed);
    assert!(res);
}

#[test]
fn test_verify_w8_l2_two_chain_steps() {
    let (res, elapsed) = verify_proof(W8_L2_PROOF_PATH, W8_L2_INSTANCES_PATH);
    println!("W8 L2 (2 chain steps): result={}, elapsed={}ms", res, elapsed);
    assert!(res);
}

// ---------------------------------------------------------------------------
// Negative tests
// ---------------------------------------------------------------------------

#[test]
fn test_verify_w8_wrong_instances() {
    let mut engine = setup_engine();

    // Use valid L0 instances but flip one byte to make them wrong
    let mut pub_inputs_bytes =
        std::fs::read(W8_L0_INSTANCES_PATH).expect("Failed to read instances file");
    pub_inputs_bytes[0] ^= 0xFF;
    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(pub_inputs_cell));

    let proof_bytes = std::fs::read(W8_L0_PROOF_PATH).expect("Failed to read proof file");
    let proof_cell = pack_data_to_cell(&proof_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell));

    let start = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_millis();

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("W8 wrong instances: result={}, elapsed={}ms", res, elapsed);
    assert!(!res);
}

#[test]
fn test_verify_w8_bad_proof() {
    let mut engine = setup_engine();

    let pub_inputs_bytes =
        std::fs::read(W8_L0_INSTANCES_PATH).expect("Failed to read instances file");
    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(pub_inputs_cell));

    // Corrupt the proof bytes
    let mut proof_bytes = std::fs::read(W8_L0_PROOF_PATH).expect("Failed to read proof file");
    proof_bytes[10] ^= 0xFF;
    proof_bytes[20] ^= 0xFF;
    let proof_cell = pack_data_to_cell(&proof_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell));

    let start = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_millis();

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("W8 bad proof: result={}, elapsed={}ms", res, elapsed);
    assert!(!res);
}

#[test]
fn test_verify_w8_mismatched_proof_and_instances() {
    // L0 proof with L1 instances — should fail
    let (res, elapsed) = verify_proof(W8_L0_PROOF_PATH, W8_L1_INSTANCES_PATH);
    println!("W8 mismatched (L0 proof + L1 instances): result={}, elapsed={}ms", res, elapsed);
    assert!(!res);
}
