use tvm_types::SliceData;

use crate::executor::Engine;
use crate::executor::gas::gas_state::Gas;

#[test]
fn test_gas_spending_simple() {
    let initial_gas = 1000;
    let mut engine = Engine::with_capabilities(0);
    engine.set_gas(Gas::test_with_limit(initial_gas));

    assert_eq!(engine.gas_remaining(), initial_gas);
    assert_eq!(engine.gas_used(), 0);

    // Create simple bytecode: just HALT instruction
    let code_slice = SliceData::new(vec![0x80]); // HALT
    engine = engine.setup_with_libraries(code_slice, None, None, None, vec![]);

    let initial_remaining = engine.gas_remaining();
    engine.execute().expect("Execution should succeed");

    let final_remaining = engine.gas_remaining();
    let gas_used = engine.gas_used();

    assert_eq!(gas_used, 5, "Gas used for simple run");
    assert!(final_remaining < initial_remaining, "Remaining gas should decrease");

    // The gas_used should be the difference between initial and final remaining
    let expected_gas_used = initial_remaining - final_remaining;
    assert_eq!(gas_used, expected_gas_used, "Gas accounting should be consistent");
}

#[test]
fn test_simple_out_of_gas() {
    let initial_gas = 3;
    let mut engine = Engine::with_capabilities(0);
    engine.set_gas(Gas::test_with_limit(initial_gas));

    assert_eq!(engine.gas_remaining(), initial_gas);
    assert_eq!(engine.gas_used(), 0);

    // Create simple bytecode: just HALT instruction
    let code_slice = SliceData::new(vec![0x80]); // HALT
    engine = engine.setup_with_libraries(
        code_slice,
        None,
        None,
        Some(Gas::test_with_limit(initial_gas)),
        vec![],
    );

    assert_eq!(engine.gas_remaining(), initial_gas);

    // engine should fail with OutOfGas
    if let Ok(_) = engine.execute() {
        eprintln!("Gas remaining {}", engine.gas_remaining());
        eprintln!("Gas used {}", engine.gas_used());
        assert!(false, "Should be OutOfGas");
    }
    assert!(engine.gas_remaining() < 0, "Expected out of gas");
}
