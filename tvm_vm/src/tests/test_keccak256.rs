use tvm_types::SliceData;

use crate::executor::engine::Engine;
use crate::executor::keccak256::execute_keccak256;
use crate::executor::test_helper::*;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::utils::pack_data_to_cell;

fn setup_engine() -> Engine {
    let code = SliceData::new_empty();
    Engine::with_capabilities(DEFAULT_CAPABILITIES)
        .setup_with_libraries(code, None, Some(Stack::new()), None, vec![])
}

fn keccak_of(data: &[u8]) -> IntegerData {
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(data, &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));
    execute_keccak256(&mut engine).unwrap();
    engine.cc.stack.get(0).as_integer().unwrap().clone()
}

fn digest(hex: &str) -> IntegerData {
    let bytes: Vec<u8> =
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
    IntegerData::from_unsigned_bytes_be(bytes)
}

#[test]
fn keccak256_matches_the_ethereum_vectors() {
    assert_eq!(
        keccak_of(b""),
        digest("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
    );
    assert_eq!(
        keccak_of(b"abc"),
        digest("4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45")
    );
}

#[test]
fn keccak256_spans_a_chain_of_cells() {
    let data: Vec<u8> = (0..642u32).map(|i| (i % 251) as u8).collect();
    let mut reference = [0u8; 32];
    let mut hasher = tiny_keccak::Keccak::v256();
    {
        use tiny_keccak::Hasher;
        hasher.update(&data);
        hasher.finalize(&mut reference);
    }
    assert_eq!(keccak_of(&data), IntegerData::from_unsigned_bytes_be(reference));
}

#[test]
fn keccak256_charges_its_gas() {
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(b"abc", &mut engine).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));
    let before = engine.gas_used();
    execute_keccak256(&mut engine).unwrap();
    assert!(
        engine.gas_used() - before >= crate::executor::keccak256::KECCAK256_GAS_PRICE,
        "KECCAK256 ran without charging its flat price"
    );
}

#[test]
fn keccak256_refuses_an_empty_stack() {
    let mut engine = setup_engine();
    assert!(execute_keccak256(&mut engine).is_err());
}
