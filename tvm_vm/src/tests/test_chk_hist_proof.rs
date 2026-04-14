use std::sync::Arc;

use tvm_types::SliceData;

use crate::executor::engine::Engine;
use crate::executor::test_helper::*;
use crate::executor::zk::execute_chk_hist_proof;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::savelist::SaveList;

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

fn make_hash_bytes() -> [u8; 32] {
    let mut hash = [0u8; 32];
    for i in 0..32 {
        hash[i] = (i + 1) as u8;
    }
    hash
}

fn push_chk_hist_proof_args(engine: &mut Engine, hash: &[u8; 32], block_height: u64, layer: i32) {
    // Push in order: hash (bottom), block_height, layer_number (top)
    engine
        .cc
        .stack
        .push(StackItem::integer(IntegerData::from_unsigned_bytes_be(hash)));
    engine.cc.stack.push(StackItem::int(block_height));
    engine.cc.stack.push(StackItem::int(layer));
}

#[test]
fn test_chk_hist_proof_callback_returns_true() {
    let mut engine = setup_engine();
    let hash = make_hash_bytes();
    let expected_hash = hash;

    engine.set_check_history_proof_hash(Arc::new(move |block_height, layer_number, h| {
        block_height == 42 && layer_number == 3 && h == expected_hash
    }));

    push_chk_hist_proof_args(&mut engine, &hash, 42, 3);
    execute_chk_hist_proof(&mut engine).unwrap();

    let result = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(result, "Expected true (-1) when callback matches");
}

#[test]
fn test_chk_hist_proof_callback_returns_false() {
    let mut engine = setup_engine();
    let hash = make_hash_bytes();

    engine.set_check_history_proof_hash(Arc::new(move |_block_height, _layer_number, _h| false));

    push_chk_hist_proof_args(&mut engine, &hash, 100, 5);
    execute_chk_hist_proof(&mut engine).unwrap();

    let result = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!result, "Expected false (0) when callback returns false");
}

#[test]
fn test_chk_hist_proof_no_callback() {
    let mut engine = setup_engine();
    let hash = make_hash_bytes();
    // No callback set — should return false

    push_chk_hist_proof_args(&mut engine, &hash, 42, 3);
    execute_chk_hist_proof(&mut engine).unwrap();

    let result = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!result, "Expected false (0) when no callback is set");
}

#[test]
fn test_chk_hist_proof_layer_zero_range_error() {
    let mut engine = setup_engine();
    let hash = make_hash_bytes();

    engine.set_check_history_proof_hash(Arc::new(move |_, _, _| true));

    push_chk_hist_proof_args(&mut engine, &hash, 42, 0);
    let result = execute_chk_hist_proof(&mut engine);
    assert!(result.is_err(), "layer_number=0 should cause RangeCheckError");
}

#[test]
fn test_chk_hist_proof_layer_eleven_range_error() {
    let mut engine = setup_engine();
    let hash = make_hash_bytes();

    engine.set_check_history_proof_hash(Arc::new(move |_, _, _| true));

    push_chk_hist_proof_args(&mut engine, &hash, 42, 11);
    let result = execute_chk_hist_proof(&mut engine);
    assert!(result.is_err(), "layer_number=11 should cause RangeCheckError");
}
