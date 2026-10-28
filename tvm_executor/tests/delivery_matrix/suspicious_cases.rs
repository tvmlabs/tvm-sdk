//! Regressions from v2_suspicious_cases.html (SDK d53e12a5, node 65990e67f).
//! Network VM expectations are approximate: use integer transaction fees
//! instead. Flags 0/1 and 16/17 differ at the sender; destination tests receive
//! already emitted values and therefore vary only exchange bit 16. No network
//! is used.
use tvm_block::TrComputePhase;

use super::*;

// Keep gas and return-fee thresholds comparable, as in the report. This is
// test configuration, not a guessed VM result: assertions read charged fees.
fn config() -> BlockchainConfig {
    let base = super::config();
    let mut raw = base.raw_config().clone();
    let mut gas = base.get_gas_config(false).clone();
    gas.flat_gas_price =
        base.calc_fwd_fee(false, &tvm_types::Cell::default()).unwrap().as_u128() as u64;
    raw.set_config(ConfigParamEnum::ConfigParam21(gas)).unwrap();
    BlockchainConfig::with_config(raw).unwrap()
}

fn currencies(vm: u64, shell: u64, nackl: u64) -> CurrencyCollection {
    let mut value = CurrencyCollection::with_grams(vm);
    for (id, amount) in [(2, shell), (1, nackl)] {
        if amount != 0 {
            value.set_other(id, amount.into()).unwrap();
        }
    }
    value
}

fn existing(active: bool, vm: u64, fail: bool) -> Account {
    let mut balance = currencies(vm, 5 * UNIT, 7 * UNIT);
    balance.set_other(3, 123).unwrap();
    if active {
        let mut state = StateInit::default();
        state.set_code(
            tvm_assembler::compile_code_to_cell(if fail { "THROW 77" } else { "ACCEPT" }).unwrap(),
        );
        state.set_data(tvm_types::Cell::default());
        Account::active_by_init_code_hash(address(0x22), balance, 0, state, false).unwrap()
    } else {
        Account::uninit(address(0x22), 0, 0, balance)
    }
}

fn execute_account(
    msg: &Message,
    dapp: UInt256,
    initial: &Account,
) -> (tvm_block::Transaction, Account) {
    let mut root = initial.serialize().unwrap();
    let (tx, minted) = OrdinaryTransactionExecutor::new(config())
        .execute_with_libs_and_params(
            Some(msg),
            &mut root,
            ExecuteParams {
                dapp_id: Some(dapp),
                engine_version: tvm_executor::STATE_V2_ENGINE_VERSION,
                block_unixtime: 1234,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(minted, 0);
    (tx, Account::construct_from_cell(root).unwrap())
}

fn description(tx: &tvm_block::Transaction) -> tvm_block::TransactionDescrOrdinary {
    let TransactionDescr::Ordinary(desc) = tx.read_description().unwrap() else {
        panic!("ordinary transaction")
    };
    desc
}

fn no_return(tx: &tvm_block::Transaction, failed: bool) {
    assert_eq!(tx.msg_count(), 0, "scenario must retain currencies rather than return them");
    let desc = description(tx);
    if failed {
        assert!(desc.aborted);
        assert!(
            matches!(desc.bounce, Some(TrBouncePhase::Nofunds(_))),
            "expected an unfunded return, got {:?}",
            desc.bounce
        );
    }
}

fn settled(
    tx: &tvm_block::Transaction,
    account: &Account,
    vm_range: (u128, u128),
    shell: u64,
    nackl: u64,
    unrelated: u64,
) {
    let value = account.balance().cloned().unwrap_or_default();
    let extra = |id| value.get_other(id).unwrap().unwrap_or_default();
    assert!(
        (vm_range.0..=vm_range.1).contains(&value.grams.as_u128())
            && extra(2) == VarUInteger32::from(shell)
            && extra(1) == VarUInteger32::from(nackl)
            && extra(3) == VarUInteger32::from(unrelated),
        "VM={}, expected {}..={}; SHELL={:?}, expected {}; NACKL={:?}, expected {}; currency 3={:?}, expected {}; transaction fees={}",
        value.grams,
        vm_range.0,
        vm_range.1,
        extra(2),
        shell,
        extra(1),
        nackl,
        extra(3),
        unrelated,
        tx.total_fees().grams,
    );
}

// Finding 1 (#164) and finding 4 (#85). Original cross-DApp VM must not
// increase the receiver balance. A fee may be paid from the attached VM or
// receiver-local funds; bound that debit by actual transaction fees.
fn cross_success(same_dapp: bool, exchange: bool, shell: u64) {
    let initial = existing(true, 100 * UNIT, false);
    let (msg, dapp) =
        message(true, same_dapp, true, exchange, currencies(10 * UNIT, shell, 10 * UNIT));
    let (tx, account) = execute_account(&msg, dapp, &initial);
    let desc = description(&tx);
    assert!(!desc.aborted);
    assert!(matches!(desc.compute_ph, TrComputePhase::Vm(ref vm) if vm.success));
    assert_eq!(account.status(), AccountStatus::AccStateActive);
    no_return(&tx, false);
    let converted = if exchange { shell } else { 0 };
    let gross = (100 * UNIT + if same_dapp { 10 * UNIT } else { 0 } + converted) as u128;
    let fees = tx.total_fees().grams.as_u128();
    let range =
        if same_dapp { (gross - fees, gross - fees) } else { (gross.saturating_sub(fees), gross) };
    settled(&tx, &account, range, 5 * UNIT + shell - converted, 17 * UNIT, 123);
}

// Finding 2 (#1450), including mismatched init hashes (#2999) and absent
// constructor body. Old receiver currencies must not be exchanged.
fn bad_init_existing(exchange: bool, vm: u64, shell: u64, wrong_hash: bool) {
    let initial = existing(false, 0, false);
    let (mut msg, dapp) = message(false, true, false, exchange, currencies(vm, shell, 10 * UNIT));
    let mut state = StateInit::default();
    state.set_code(tvm_assembler::compile_code_to_cell("ACCEPT").unwrap());
    state.set_data(tvm_types::Cell::default());
    if !wrong_hash {
        msg.int_header_mut().unwrap().set_dst(
            MsgAddressInt::with_standart(None, 0, state.serialize().unwrap().repr_hash().into())
                .unwrap(),
        );
    } else {
        msg.set_body(SliceData::from_raw(1u32.to_be_bytes().to_vec(), 32));
    }
    msg.set_state_init(state);
    // Rebuild the initial account at the destination; only wrong_hash tests
    // actually mismatch StateInit and destination account addresses.
    let initial =
        Account::uninit(msg.dst_ref().unwrap().clone(), 0, 0, initial.balance().unwrap().clone());
    let (tx, account) = execute_account(&msg, dapp, &initial);
    let desc = description(&tx);
    assert!(desc.aborted);
    assert!(matches!(desc.compute_ph, TrComputePhase::Skipped(_)));
    assert_eq!(account.status(), AccountStatus::AccStateUninit);
    no_return(&tx, false);
    let converted = if exchange { shell } else { 0 };
    let expected = (vm + converted) as u128 - tx.total_fees().grams.as_u128();
    settled(&tx, &account, (expected, expected), 5 * UNIT + shell - converted, 17 * UNIT, 123);
}

// Finding 3 (#1606, #1607, #1611, #1613). Gas really executes and exits 77;
// this must not silently turn into a skipped-compute regression.
fn failed_constructor(exchange: bool, vm: u64, shell: u64, returns: bool) {
    let (mut msg, dapp) = message(false, true, true, exchange, currencies(vm, shell, 10 * UNIT));
    let mut state = StateInit::default();
    state.set_code(tvm_assembler::compile_code_to_cell("THROW 77").unwrap());
    state.set_data(tvm_types::Cell::default());
    let dst = MsgAddressInt::with_standart(None, 0, state.serialize().unwrap().repr_hash().into())
        .unwrap();
    msg.int_header_mut().unwrap().set_dst(dst);
    msg.set_state_init(state);
    msg.set_body(SliceData::from_raw(1u32.to_be_bytes().to_vec(), 32));
    let (tx, account) = execute_account(&msg, dapp, &Account::default());
    let desc = description(&tx);
    assert!(desc.aborted);
    assert!(
        matches!(desc.compute_ph, TrComputePhase::Vm(ref phase) if !phase.success && phase.exit_code == 77),
        "constructor must actually execute and fail: {:?}",
        desc.compute_ph
    );
    if returns {
        assert!(matches!(desc.bounce, Some(TrBouncePhase::Ok(_))));
        assert_eq!(tx.msg_count(), 1);
        let returned = tx.get_out_msg(0).unwrap().unwrap();
        let value = returned.get_value().unwrap();
        let gas = match desc.compute_ph {
            TrComputePhase::Vm(vm) => vm.gas_fees.as_u128(),
            _ => unreachable!(),
        };
        let fee = config().calc_fwd_fee(false, &tvm_types::Cell::default()).unwrap().as_u128();
        assert_eq!(value.grams.as_u128(), vm as u128 - gas - fee);
        assert_eq!(value.get_other(2).unwrap().unwrap_or_default(), VarUInteger32::from(shell));
        assert_eq!(value.get_other(1).unwrap().unwrap(), VarUInteger32::from(10 * UNIT));
        assert!(!returned.int_header().unwrap().is_exchange);
        assert!(
            account.balance().cloned().unwrap_or_default().is_zero().unwrap(),
            "a completed return must leave no incoming currencies behind"
        );
    } else {
        no_return(&tx, true);
        assert_eq!(account.status(), AccountStatus::AccStateActive);
        let converted = if exchange { shell } else { 0 };
        // Unreturned raw VM below the return threshold is not retained.
        // Charge gas from raw VM first, then from converted incoming SHELL.
        let fees = tx.total_fees().grams.as_u128();
        let expected = (converted as u128).saturating_sub(fees.saturating_sub(vm as u128));
        settled(&tx, &account, (expected, expected), shell - converted, 10 * UNIT, 0);
    }
}

// Findings 4 and 5: retain and exchange only incoming SHELL on an existing
// Uninit when the bounce cannot be funded. VM sent below F is not retained.
fn uninit_no_funds(cross_header: bool, same_dapp: bool, exchange: bool, old_vm: u64, vm: u64) {
    let initial = existing(false, old_vm, false);
    let (msg, dapp) =
        message(cross_header, same_dapp, true, exchange, currencies(vm, 10 * UNIT, 10 * UNIT));
    let (tx, account) = execute_account(&msg, dapp, &initial);
    assert!(matches!(description(&tx).compute_ph, TrComputePhase::Skipped(_)));
    assert_eq!(account.status(), AccountStatus::AccStateUninit);
    no_return(&tx, true);
    let converted = if exchange { 10 * UNIT } else { 0 };
    let expected = (old_vm + converted) as u128 - tx.total_fees().grams.as_u128();
    settled(&tx, &account, (expected, expected), 15 * UNIT - converted, 17 * UNIT, 123);
}

// Findings 4 and 5 (#54, #55): active contract fails, and actual gas leaves
// too little original VM for a return. Conversion must survive Nofunds.
fn active_no_funds(cross_header: bool, same_dapp: bool, vm: u64) {
    let initial = existing(true, 100 * UNIT, true);
    let (msg, dapp) =
        message(cross_header, same_dapp, true, true, currencies(vm, 10 * UNIT, 10 * UNIT));
    let (tx, account) = execute_account(&msg, dapp, &initial);
    let desc = description(&tx);
    assert!(
        matches!(desc.compute_ph, TrComputePhase::Vm(ref phase) if !phase.success && phase.exit_code == 77),
        "expected VM exit 77, got {:?}",
        desc.compute_ph
    );
    no_return(&tx, true);
    assert_eq!(account.status(), AccountStatus::AccStateActive);
    let gross = (110 * UNIT + if same_dapp { vm } else { 0 }) as u128;
    let fees = tx.total_fees().grams.as_u128();
    let range =
        if same_dapp { (gross - fees, gross - fees) } else { (gross.saturating_sub(fees), gross) };
    settled(&tx, &account, range, 5 * UNIT, 17 * UNIT, 123);
}

fn half_return_fee() -> u64 {
    config().calc_fwd_fee(false, &tvm_types::Cell::default()).unwrap().as_u128() as u64 / 2
}

macro_rules! suspicious_case {
    ($name:ident, $body:expr) => {
        #[test]
        fn $name() {
            $body
        }
    };
}
suspicious_case!(
    suspicious_cross_success_other_flag_1_shell_0,
    cross_success(false, false, 0 * UNIT)
);
suspicious_case!(
    suspicious_cross_success_other_flag_1_shell_10,
    cross_success(false, false, 10 * UNIT)
);
suspicious_case!(
    suspicious_cross_success_other_flag_17_shell_0,
    cross_success(false, true, 0 * UNIT)
);
suspicious_case!(
    suspicious_cross_success_other_flag_17_shell_10,
    cross_success(false, true, 10 * UNIT)
);
suspicious_case!(
    suspicious_cross_success_same_flag_1_shell_0,
    cross_success(true, false, 0 * UNIT)
);
suspicious_case!(
    suspicious_cross_success_same_flag_1_shell_10,
    cross_success(true, false, 10 * UNIT)
);
suspicious_case!(
    suspicious_cross_success_same_flag_17_shell_0,
    cross_success(true, true, 0 * UNIT)
);
suspicious_case!(
    suspicious_cross_success_same_flag_17_shell_10,
    cross_success(true, true, 10 * UNIT)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_body_flag_1_vm_10_shell_0,
    bad_init_existing(false, 10 * UNIT, 0 * UNIT, false)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_body_flag_1_vm_0_shell_10,
    bad_init_existing(false, 0 * UNIT, 10 * UNIT, false)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_body_flag_1_vm_10_shell_10,
    bad_init_existing(false, 10 * UNIT, 10 * UNIT, false)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_body_flag_17_vm_10_shell_0,
    bad_init_existing(true, 10 * UNIT, 0 * UNIT, false)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_body_flag_17_vm_0_shell_10,
    bad_init_existing(true, 0 * UNIT, 10 * UNIT, false)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_body_flag_17_vm_10_shell_10,
    bad_init_existing(true, 10 * UNIT, 10 * UNIT, false)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_hash_flag_1_vm_10_shell_0,
    bad_init_existing(false, 10 * UNIT, 0 * UNIT, true)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_hash_flag_1_vm_0_shell_10,
    bad_init_existing(false, 0 * UNIT, 10 * UNIT, true)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_hash_flag_1_vm_10_shell_10,
    bad_init_existing(false, 10 * UNIT, 10 * UNIT, true)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_hash_flag_17_vm_10_shell_0,
    bad_init_existing(true, 10 * UNIT, 0 * UNIT, true)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_hash_flag_17_vm_0_shell_10,
    bad_init_existing(true, 0 * UNIT, 10 * UNIT, true)
);
suspicious_case!(
    suspicious_existing_uninit_bad_init_hash_flag_17_vm_10_shell_10,
    bad_init_existing(true, 10 * UNIT, 10 * UNIT, true)
);
suspicious_case!(
    suspicious_failed_constructor_report_1606,
    failed_constructor(false, 3 * half_return_fee(), 0 * UNIT, false)
);
suspicious_case!(
    suspicious_failed_constructor_report_1607,
    failed_constructor(false, 3 * half_return_fee(), 10 * UNIT, false)
);
suspicious_case!(
    suspicious_failed_constructor_report_1611,
    failed_constructor(true, 3 * half_return_fee(), 0 * UNIT, false)
);
suspicious_case!(
    suspicious_failed_constructor_report_1613,
    failed_constructor(true, 0, 2 * UNIT, false)
);
suspicious_case!(
    suspicious_failed_constructor_successful_return_flag_1,
    failed_constructor(false, 10 * UNIT, 10 * UNIT, true)
);
suspicious_case!(
    suspicious_failed_constructor_successful_return_flag_17,
    failed_constructor(true, 10 * UNIT, 10 * UNIT, true)
);
suspicious_case!(
    suspicious_uninit_no_funds_internal_same_old_vm_0_incoming_zero,
    uninit_no_funds(false, true, true, 0 * UNIT, 0)
);
suspicious_case!(
    suspicious_uninit_no_funds_internal_same_old_vm_0_incoming_half_fee,
    uninit_no_funds(false, true, true, 0 * UNIT, half_return_fee())
);
suspicious_case!(
    suspicious_uninit_no_funds_internal_same_old_vm_10_incoming_zero,
    uninit_no_funds(false, true, true, 10 * UNIT, 0)
);
suspicious_case!(
    suspicious_uninit_no_funds_internal_same_old_vm_10_incoming_half_fee,
    uninit_no_funds(false, true, true, 10 * UNIT, half_return_fee())
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_same_old_vm_0_incoming_zero,
    uninit_no_funds(true, true, true, 0 * UNIT, 0)
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_same_old_vm_0_incoming_half_fee,
    uninit_no_funds(true, true, true, 0 * UNIT, half_return_fee())
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_same_old_vm_10_incoming_zero,
    uninit_no_funds(true, true, true, 10 * UNIT, 0)
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_same_old_vm_10_incoming_half_fee,
    uninit_no_funds(true, true, true, 10 * UNIT, half_return_fee())
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_other_old_vm_0_incoming_zero,
    uninit_no_funds(true, false, true, 0 * UNIT, 0)
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_other_old_vm_0_incoming_half_fee,
    uninit_no_funds(true, false, true, 0 * UNIT, half_return_fee())
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_other_old_vm_10_incoming_zero,
    uninit_no_funds(true, false, true, 10 * UNIT, 0)
);
suspicious_case!(
    suspicious_uninit_no_funds_cross_other_old_vm_10_incoming_half_fee,
    uninit_no_funds(true, false, true, 10 * UNIT, half_return_fee())
);
suspicious_case!(
    suspicious_uninit_no_exchange_internal_same,
    uninit_no_funds(false, true, false, 0, 0)
);
suspicious_case!(
    suspicious_uninit_no_exchange_cross_same,
    uninit_no_funds(true, true, false, 0, 0)
);
suspicious_case!(
    suspicious_uninit_no_exchange_cross_other,
    uninit_no_funds(true, false, false, 0, 0)
);
suspicious_case!(suspicious_active_internal_failed_no_funds_zero, active_no_funds(false, true, 0));
suspicious_case!(
    suspicious_active_internal_failed_no_funds_half_fee,
    active_no_funds(false, true, half_return_fee())
);
suspicious_case!(
    suspicious_active_internal_failed_no_funds_fee,
    active_no_funds(false, true, 2 * half_return_fee())
);
suspicious_case!(
    suspicious_active_cross_failed_no_funds_other,
    active_no_funds(true, false, 3 * half_return_fee())
);
suspicious_case!(
    suspicious_active_cross_failed_no_funds_same,
    active_no_funds(true, true, 3 * half_return_fee())
);

fn cell_asm(cell: &tvm_types::Cell, directive: &str) -> String {
    let mut source = format!("{directive} {{\n");
    if cell.bit_length() != 0 {
        source.push_str(&format!(".BLOB x{}\n", cell.to_hex_string(true)));
    }
    for index in 0..cell.references_count() {
        source.push_str(&cell_asm(&cell.reference(index).unwrap(), ".CELL"));
    }
    source.push_str("}\n");
    source
}

fn cannot_spend_foreign_vm(same_dapp: bool) {
    let mut actions = tvm_block::OutActions::default();
    actions.push_back(tvm_block::OutAction::new_send(
        tvm_block::SENDMSG_PAY_FEE_SEPARATELY,
        Message::with_int_header(InternalMessageHeader::with_addresses(
            address(0x22),
            address(0x33),
            CurrencyCollection::with_grams(105 * UNIT),
        )),
    ));
    let code = tvm_assembler::compile_code_to_cell(&format!(
        "{}POP c5\n",
        cell_asm(&actions.serialize().unwrap(), "PUSHREF"),
    ))
    .unwrap();
    let mut initial = existing(true, 100 * UNIT, false);
    assert!(initial.set_code(code));
    let (msg, dapp) = message(true, same_dapp, true, false, currencies(10 * UNIT, 0, 0));
    let (tx, account) = execute_account(&msg, dapp, &initial);
    let desc = description(&tx);
    assert!(matches!(desc.compute_ph, TrComputePhase::Vm(ref phase) if phase.success));
    let action = desc.action.unwrap();
    if same_dapp {
        assert!(action.success, "local VM should fund the action: {action:?}");
        assert_eq!(tx.msg_count(), 1);
        let out = tx.get_out_msg(0).unwrap().unwrap();
        assert_eq!(out.get_value().unwrap().grams.as_u128(), (105 * UNIT) as u128);
    } else {
        assert!(!action.success, "foreign VM must not fund a local send");
        tx.iterate_out_msgs(|out| {
            assert_ne!(
                out.dst_ref(),
                Some(&address(0x33)),
                "foreign VM escaped through the action phase"
            );
            Ok(true)
        })
        .unwrap();
        assert!(account.balance().unwrap().grams.as_u128() <= (100 * UNIT) as u128);
    }
}

suspicious_case!(suspicious_action_cannot_forward_foreign_vm, cannot_spend_foreign_vm(false));
suspicious_case!(suspicious_action_can_forward_same_dapp_vm, cannot_spend_foreign_vm(true));
