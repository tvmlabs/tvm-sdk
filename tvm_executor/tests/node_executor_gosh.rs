mod common;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;

use common::*;
use tvm_block::AccStatusChange;
use tvm_block::OutAction;
use tvm_block::OutActions;
use tvm_block::TrComputePhase;
use tvm_types::UInt256;

#[test]
fn gosh_actions_full_executor_observable_effects() {
    let harness = ExecutionHarness::default();
    let last_tr_lt = Arc::new(AtomicU64::new(500));
    let vm_execution_is_block_related = Arc::new(Mutex::new(false));
    let block_collation_was_finished = Arc::new(Mutex::new(false));
    let dapp_id = UInt256::with_array([0x52; 32]);

    let mut params = NodeExecuteParamsFixture::default();
    params.block_unixtime = 321;
    params.block_lt = 654;
    params.seq_no = 9;
    params.last_tr_lt = last_tr_lt;
    params.seed_block = UInt256::with_array([0x71; 32]);
    params.vm_execution_is_block_related = vm_execution_is_block_related;
    params.block_collation_was_finished = block_collation_was_finished;
    params.dapp_id = Some(dapp_id.clone());
    params.available_credit = 13;

    let outbound_value = 123_000_000;
    let mut actions = OutActions::default();
    actions.push_back(OutAction::new_mint_shellq(20));
    actions.push_back(send_action(9, outbound_value));

    let code = compile_code(&format!(
        "NOW\nPUSHINT 321\nEQUAL\nTHROWIFNOT 301\n\
         BLOCKLT\nPUSHINT 654\nEQUAL\nTHROWIFNOT 302\n\
         SEQNO\nPUSHINT 9\nEQUAL\nTHROWIFNOT 303\n\
         {}POP c5\n",
        push_ref_cell_asm(&action_cell(actions)),
    ));
    let observation = harness.run(code, internal_message(1, 7, 1_000_000_000), params).unwrap();

    assert_eq!(observation.minted_shell, 13);
    assert_eq!(observation.transaction.account_id(), &address(7).address());
    assert_eq!(observation.transaction.logical_time(), 500);
    assert_eq!(observation.transaction.now(), 321);
    assert_eq!(observation.last_tr_lt, 502);
    assert!(observation.vm_execution_is_block_related);
    assert!(!observation.block_collation_was_finished);
    assert_eq!(observation.state_update.old_hash, observation.old_account_hash);
    assert_eq!(observation.state_update.new_hash, observation.account_root.repr_hash());
    assert_eq!(observation.new_account_hash, observation.account_root.repr_hash());
    assert_ne!(observation.state_update.old_hash, observation.state_update.new_hash);

    let description = observation.ordinary_description();
    assert!(!description.aborted);
    match description.compute_ph {
        TrComputePhase::Vm(phase) => {
            assert!(phase.success, "{phase:?}");
            assert_eq!(phase.exit_code, 0);
        }
        _ => panic!("unexpected compute phase"),
    }
    let action = description.action.expect("action phase");
    assert!(action.success, "{action:?}");
    assert_eq!(action.result_code, 0);
    assert_eq!(action.tot_actions, 2);
    assert_eq!(action.spec_actions, 1);
    assert_eq!(action.msgs_created, 1);

    assert_eq!(observation.out_messages.len(), 1);
    let out_msg = &observation.out_messages[0];
    assert_eq!(out_msg.src(), Some(address(7)));
    assert_eq!(out_msg.dst(), Some(masterchain_address(9)));
    assert_eq!(out_msg.at_and_lt(), Some((321, 501)));
    assert_eq!(out_msg.get_value().unwrap().grams.as_u128(), outbound_value as u128);
    assert_eq!(out_msg.int_header().unwrap().src_dapp_id(), &Some(dapp_id));
    assert_eq!(observation.account.last_paid(), 321);
    assert_eq!(observation.account.last_tr_time(), Some(502));
    assert!(observation.account.balance_checked().grams.as_u128() > 0);
}

#[test]
fn gosh_action_failure_rolls_back_observable_effects() {
    let harness = ExecutionHarness::default();
    let last_tr_lt = Arc::new(AtomicU64::new(600));
    let vm_execution_is_block_related = Arc::new(Mutex::new(false));

    let mut params = NodeExecuteParamsFixture::default();
    params.block_unixtime = 400;
    params.last_tr_lt = last_tr_lt;
    params.vm_execution_is_block_related = vm_execution_is_block_related;
    params.available_credit = 13;

    let mut actions = OutActions::default();
    actions.push_back(OutAction::new_mint_shellq(20));
    actions.push_back(send_action(9, 10_000_000_000_000));

    let code =
        compile_code(&format!("NOW\nDROP\n{}POP c5\n", push_ref_cell_asm(&action_cell(actions))));
    let observation = harness.run(code, internal_message(1, 7, 1_000_000_000), params).unwrap();

    assert_eq!(observation.minted_shell, 0);
    assert!(observation.out_messages.is_empty());
    assert!(observation.vm_execution_is_block_related);

    let description = observation.ordinary_description();
    assert!(description.aborted);
    let action = description.action.expect("failed action phase");
    assert!(!action.success, "{action:?}");
    assert_eq!(action.result_code, 37);
    assert_eq!(action.status_change, AccStatusChange::Unchanged);
    assert_eq!(observation.account.last_paid(), 400);
    assert_eq!(observation.account.last_tr_time(), Some(601));
    assert_eq!(observation.last_tr_lt, 601);
    assert!(observation.account.balance_checked().grams > observation.original_balance.grams);
    assert!(observation.account.balance_checked().grams.as_u128() > 0);
}

#[test]
fn block_related_execution_sets_or_rejects_flag() {
    let harness = ExecutionHarness::default();

    let mut params = NodeExecuteParamsFixture::default();
    params.block_unixtime = 111;
    params.block_lt = 222;
    params.seq_no = 3;
    params.last_tr_lt = Arc::new(AtomicU64::new(700));
    let code = compile_code(
        "NOW\nPUSHINT 111\nEQUAL\nTHROWIFNOT 401\n\
         BLOCKLT\nPUSHINT 222\nEQUAL\nTHROWIFNOT 402\n\
         SEQNO\nPUSHINT 3\nEQUAL\nTHROWIFNOT 403\n",
    );
    let observation =
        harness.run(code.clone(), internal_message(1, 7, 1_000_000_000), params).unwrap();
    assert!(observation.vm_execution_is_block_related);
    assert!(!observation.ordinary_description().aborted);

    let mut guarded = NodeExecuteParamsFixture::default();
    guarded.block_unixtime = 111;
    guarded.block_lt = 222;
    guarded.seq_no = 3;
    guarded.last_tr_lt = Arc::new(AtomicU64::new(800));
    guarded.block_collation_was_finished = Arc::new(Mutex::new(true));
    let guarded_observation = harness
        .run(code, internal_message(1, 7, 1_000_000_000), guarded)
        .expect("block-finalized guard should be a VM compute failure, not executor error");

    assert!(guarded_observation.vm_execution_is_block_related);
    let description = guarded_observation.ordinary_description();
    assert!(description.aborted);
    match description.compute_ph {
        TrComputePhase::Vm(phase) => assert!(!phase.success, "{phase:?}"),
        _ => panic!("unexpected compute phase"),
    }
}

#[test]
fn non_block_related_execution_keeps_flag_false() {
    let harness = ExecutionHarness::default();
    let mut params = NodeExecuteParamsFixture::default();
    params.block_unixtime = 222;
    params.block_lt = 333;
    params.seq_no = 4;
    params.last_tr_lt = Arc::new(AtomicU64::new(900));

    let mut actions = OutActions::default();
    actions.push_back(send_action(9, 100_000_000));
    let code = compile_code(&format!("{}POP c5\n", push_ref_cell_asm(&action_cell(actions))));
    let observation = harness.run(code, internal_message(1, 7, 1_000_000_000), params).unwrap();

    assert_eq!(observation.minted_shell, 0);
    assert_eq!(observation.transaction.logical_time(), 900);
    assert_eq!(observation.last_tr_lt, 902);
    assert!(!observation.vm_execution_is_block_related);
    assert_eq!(observation.out_messages.len(), 1);
    let description = observation.ordinary_description();
    assert!(!description.aborted);
    assert_eq!(description.action.unwrap().tot_actions, 1);
    assert_eq!(observation.account.last_paid(), 222);
    assert_eq!(observation.account.last_tr_time(), Some(902));
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
#[test]
fn wasm_hash_fixture_runs_through_full_executor() {
    let harness = ExecutionHarness::default();
    let wasm = NodeWasmFixtures::default();
    let call = wasm.adder_call(1, 2);

    let mut params = NodeExecuteParamsFixture::default();
    params.block_unixtime = 1000;
    params.last_tr_lt = Arc::new(AtomicU64::new(1000));
    params.wasm_fixtures = wasm;

    let mut actions = OutActions::default();
    actions.push_back(send_action(9, 100_000_000));
    let code = compile_code(&wasm_validation_code(&call, actions));
    let observation = harness.run(code, internal_message(1, 7, 1_000_000_000), params).unwrap();

    let description = observation.ordinary_description();
    assert!(!description.aborted, "{description:?}");
    assert_eq!(observation.out_messages.len(), 1);
    assert_eq!(observation.out_messages[0].dst(), Some(masterchain_address(9)));
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
#[test]
fn wasm_whitelist_blocks_unlisted_hash() {
    let harness = ExecutionHarness::default();
    let wasm = NodeWasmFixtures::default();
    let call = wasm.adder_call(1, 2);

    let mut params = NodeExecuteParamsFixture::default();
    params.block_unixtime = 1000;
    params.last_tr_lt = Arc::new(AtomicU64::new(1100));
    params.wasm_fixtures = NodeWasmFixtures::without_hash(call.hash);

    let mut actions = OutActions::default();
    actions.push_back(send_action(9, 100_000_000));
    let code = compile_code(&wasm_validation_code(&call, actions));
    let observation = harness.run(code, internal_message(1, 7, 1_000_000_000), params).unwrap();

    assert!(observation.out_messages.is_empty());
    let description = observation.ordinary_description();
    assert!(description.aborted);
    match description.compute_ph {
        TrComputePhase::Vm(phase) => assert!(!phase.success, "{phase:?}"),
        _ => panic!("unexpected compute phase"),
    }
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
#[test]
fn wasm_block_time_comes_from_execute_params() {
    let harness = ExecutionHarness::default();
    let wasm = NodeWasmFixtures::default();
    let expected_time = 3_005_792_924;
    let call = wasm.clock_call(expected_time);

    let mut params = NodeExecuteParamsFixture::default();
    params.block_unixtime = expected_time;
    params.last_tr_lt = Arc::new(AtomicU64::new(1200));
    params.wasm_fixtures = wasm.clone();

    let mut actions = OutActions::default();
    actions.push_back(send_action(9, 100_000_000));
    let code = compile_code(&wasm_validation_code(&call, actions));
    let observation =
        harness.run(code.clone(), internal_message(1, 7, 1_000_000_000), params).unwrap();

    let description = observation.ordinary_description();
    assert!(!description.aborted, "{description:?}");
    assert_eq!(observation.out_messages.len(), 1);

    let mut mismatched = NodeExecuteParamsFixture::default();
    mismatched.block_unixtime = expected_time - 1;
    mismatched.last_tr_lt = Arc::new(AtomicU64::new(1300));
    mismatched.wasm_fixtures = wasm;
    let mismatched_observation =
        harness.run(code, internal_message(1, 7, 1_000_000_000), mismatched).unwrap();

    assert!(mismatched_observation.out_messages.is_empty());
    let description = mismatched_observation.ordinary_description();
    assert!(description.aborted);
    match description.compute_ph {
        TrComputePhase::Vm(phase) => assert!(!phase.success, "{phase:?}"),
        _ => panic!("unexpected compute phase"),
    }
}
