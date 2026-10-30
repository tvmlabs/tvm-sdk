//! SDK-only delivery matrix: message header type and DApp boundary are
//! independent.
use tvm_block::Account;
use tvm_block::AccountStatus;
use tvm_block::AddSub;
use tvm_block::ConfigParam8;
use tvm_block::ConfigParam12;
use tvm_block::ConfigParam18;
use tvm_block::ConfigParam31;
use tvm_block::ConfigParamEnum;
use tvm_block::CrossDappMessageHeader;
use tvm_block::CurrencyCollection;
use tvm_block::Deserializable;
use tvm_block::GlobalCapabilities;
use tvm_block::GlobalVersion;
use tvm_block::InternalMessageHeader;
use tvm_block::Message;
use tvm_block::MsgAddressInt;
use tvm_block::MsgAddressIntOrNone;
use tvm_block::Serializable;
use tvm_block::StateInit;
use tvm_block::StoragePrices;
use tvm_block::TrBouncePhase;
use tvm_block::TransactionDescr;
use tvm_block::VarUInteger32;
use tvm_block::WorkchainDescr;
use tvm_executor::BlockchainConfig;
use tvm_executor::ExecuteParams;
use tvm_executor::OrdinaryTransactionExecutor;
use tvm_executor::TransactionExecutor;
use tvm_types::SliceData;
use tvm_types::UInt256;

const UNIT: u64 = 1_000_000_000;

fn address(seed: u8) -> MsgAddressInt {
    MsgAddressInt::with_standart(None, 0, SliceData::from_raw(vec![seed; 32], 256)).unwrap()
}

fn config() -> BlockchainConfig {
    let defaults = BlockchainConfig::default();
    let mut raw = defaults.raw_config().clone();
    let mut workchains = ConfigParam12::default();
    let mut workchain = WorkchainDescr::default();
    workchain.active = true;
    workchain.accept_msgs = true;
    workchains.insert(0, &workchain).unwrap();
    let mut storage = ConfigParam18::default();
    storage.insert(&StoragePrices::default()).unwrap();
    for param in [
        // Active contracts need nonzero c7 capabilities. The default raw config
        // omits parameter 8 even though BlockchainConfig caches capabilities.
        ConfigParamEnum::ConfigParam8(ConfigParam8 {
            global_version: GlobalVersion {
                version: 0,
                capabilities: GlobalCapabilities::CapInitCodeHash as u64,
            },
        }),
        ConfigParamEnum::ConfigParam12(workchains),
        ConfigParamEnum::ConfigParam18(storage),
        ConfigParamEnum::ConfigParam20(defaults.get_gas_config(true).clone()),
        ConfigParamEnum::ConfigParam21(defaults.get_gas_config(false).clone()),
        ConfigParamEnum::ConfigParam24(defaults.get_fwd_prices(true).clone()),
        ConfigParamEnum::ConfigParam25(defaults.get_fwd_prices(false).clone()),
        ConfigParamEnum::ConfigParam31(ConfigParam31::new()),
    ] {
        raw.set_config(param).unwrap();
    }
    BlockchainConfig::with_config(raw).unwrap()
}

fn message(
    cross_header: bool,
    same_dapp: bool,
    bounce: bool,
    exchange: bool,
    value: CurrencyCollection,
) -> (Message, UInt256) {
    let src_dapp = UInt256::from([1; 32]);
    let dst_dapp = if same_dapp { src_dapp.clone() } else { UInt256::from([2; 32]) };
    let message = if cross_header {
        Message::with_cross_dapp_header(CrossDappMessageHeader {
            src: MsgAddressIntOrNone::Some(address(0x11)),
            dst: address(0x22),
            src_dapp_id: src_dapp,
            dst_dapp_id: dst_dapp.clone(),
            value,
            bounce,
            is_exchange: exchange,
            ..Default::default()
        })
    } else {
        let mut header = InternalMessageHeader::with_addresses_and_bounce(
            address(0x11),
            address(0x22),
            value,
            bounce,
        );
        header.set_src_dapp_id(Some(src_dapp));
        header.set_exchange(exchange);
        Message::with_int_header(header)
    };
    (message, dst_dapp)
}

fn execute(message: &Message, dapp: UInt256) -> (tvm_block::Transaction, Account) {
    execute_versioned(message, dapp, tvm_executor::STATE_V2_ENGINE_VERSION)
}

fn execute_versioned(
    message: &Message,
    dapp: UInt256,
    engine_version: semver::Version,
) -> (tvm_block::Transaction, Account) {
    let executor = OrdinaryTransactionExecutor::new(config());
    let mut root = Account::default().serialize().unwrap();
    let (tx, minted) = executor
        .execute_with_libs_and_params(
            Some(message),
            &mut root,
            ExecuteParams {
                dapp_id: Some(dapp),
                engine_version,
                block_unixtime: 1234,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(minted, 0);
    (tx, Account::construct_from_cell(root).unwrap())
}

fn run_matrix(cross_header: bool, same_dapp: bool) {
    let fee = config().calc_fwd_fee(false, &tvm_types::Cell::default()).unwrap().as_u128() as u64;
    for bounce in [false, true] {
        for exchange in [false, true] {
            for vm in [0, 10 * UNIT] {
                for shell in [0, 10 * UNIT] {
                    for nackl in [0, 10 * UNIT] {
                        let label = format!(
                            "cross_header={cross_header}, same={same_dapp}, bounce={bounce}, exchange={exchange}, value=({vm},{shell},{nackl})"
                        );
                        let mut value = CurrencyCollection::with_grams(vm);
                        if shell != 0 {
                            value.set_other(2, shell.into()).unwrap();
                        }
                        if nackl != 0 {
                            value.set_other(1, nackl.into()).unwrap();
                        }
                        let (msg, dapp) = message(cross_header, same_dapp, bounce, exchange, value);
                        let (tx, account) = execute(&msg, dapp);
                        let returns = bounce && vm >= fee;
                        let converted = if exchange { shell } else { 0 };
                        let expected_vm = if returns {
                            0
                        } else {
                            (if !bounce && same_dapp { vm } else { 0 }) + converted
                        };
                        let expected_shell = if returns { 0 } else { shell - converted };
                        let expected_nackl = if returns { 0 } else { nackl };
                        let expected_status =
                            if expected_vm != 0 || expected_shell != 0 || expected_nackl != 0 {
                                AccountStatus::AccStateUninit
                            } else {
                                AccountStatus::AccStateNonexist
                            };
                        assert_eq!(account.status(), expected_status, "{label}");
                        assert_eq!(tx.end_status, expected_status, "{label}");
                        if expected_status == AccountStatus::AccStateUninit {
                            assert_eq!(account.last_paid(), 1234, "{label}; last_paid");
                        }
                        let balance = account.balance().cloned().unwrap_or_default();
                        assert_eq!(balance.grams.as_u128(), expected_vm as u128, "{label}");
                        for (id, expected) in [(2, expected_shell), (1, expected_nackl)] {
                            assert_eq!(
                                balance.get_other(id).unwrap().unwrap_or_default(),
                                VarUInteger32::from(expected),
                                "{label}; ECC {id}"
                            );
                        }
                        assert_eq!(tx.msg_count(), i16::from(returns), "{label}");
                        let TransactionDescr::Ordinary(desc) = tx.read_description().unwrap()
                        else {
                            panic!("ordinary transaction")
                        };
                        if returns {
                            assert!(matches!(desc.bounce, Some(TrBouncePhase::Ok(_))), "{label}");
                            let returned = tx.get_out_msg(0).unwrap().unwrap();
                            let returned_value = returned.get_value().unwrap();
                            assert_eq!(
                                returned_value.grams.as_u128(),
                                (vm - fee) as u128,
                                "{label}"
                            );
                            for (id, expected) in [(2, shell), (1, nackl)] {
                                assert_eq!(
                                    returned_value.get_other(id).unwrap().unwrap_or_default(),
                                    VarUInteger32::from(expected),
                                    "{label}; returned ECC {id}"
                                );
                            }
                        } else if bounce {
                            assert!(
                                matches!(desc.bounce, Some(TrBouncePhase::Nofunds(_))),
                                "{label}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn internal_same_dapp_delivery_matrix() {
    run_matrix(false, true);
}
#[test]
fn internal_other_dapp_delivery_matrix() {
    run_matrix(false, false);
}
#[test]
fn cross_header_same_dapp_delivery_matrix() {
    run_matrix(true, true);
}
#[test]
fn cross_header_other_dapp_delivery_matrix() {
    run_matrix(true, false);
}

#[test]
fn no_funds_exchange_preserves_large_shell_remainder_and_other_ecc() {
    let shell = VarUInteger32::from_two_u128(1, 123).unwrap();
    let mut value = CurrencyCollection::default();
    value.set_other_ex(2, &shell).unwrap();
    value.set_other(1, 10 * UNIT as u128).unwrap();
    value.set_other(3, 777).unwrap();
    for cross_header in [false, true] {
        for same_dapp in [false, true] {
            let (msg, dapp) = message(cross_header, same_dapp, true, true, value.clone());
            let (tx, account) = execute(&msg, dapp);
            assert_eq!(account.status(), AccountStatus::AccStateUninit);
            let balance = account.balance().unwrap();
            assert_eq!(balance.grams.as_u128(), u64::MAX as u128);
            let mut remainder = shell.clone();
            remainder.sub(&VarUInteger32::from(u64::MAX)).unwrap();
            assert_eq!(balance.get_other(2).unwrap().unwrap(), remainder);
            assert_eq!(balance.get_other(1).unwrap().unwrap(), VarUInteger32::from(10 * UNIT));
            assert_eq!(balance.get_other(3).unwrap().unwrap(), VarUInteger32::from(777u64));
            assert_eq!(tx.msg_count(), 0);
        }
    }
}

#[test]
fn failed_state_init_retains_ecc_when_return_is_unfunded() {
    let mut value = CurrencyCollection::default();
    value.set_other(1, 10 * UNIT as u128).unwrap();
    let (mut msg, dapp) = message(false, true, true, false, value);
    msg.set_state_init(StateInit::default());
    let (_, account) = execute(&msg, dapp);
    assert_eq!(account.status(), AccountStatus::AccStateUninit);
    assert_eq!(
        account.balance().unwrap().get_other(1).unwrap().unwrap(),
        VarUInteger32::from(10 * UNIT)
    );
}

#[test]
fn legacy_no_funds_does_not_apply_state_v2_ecc_settlement() {
    let mut value = CurrencyCollection::default();
    value.set_other(1, 10 * UNIT as u128).unwrap();
    let (msg, dapp) = message(false, true, true, false, value);
    let (tx, account) = execute_versioned(&msg, dapp, semver::Version::new(1, 0, 7));
    assert_eq!(account.status(), AccountStatus::AccStateNonexist);
    assert_eq!(tx.end_status, AccountStatus::AccStateNonexist);
    assert_eq!(tx.msg_count(), 0);
}

// Table 6 of v2_fix_tables_4_5_6.html (node d107c6d2f, SDK dd7c0c90).
// Expectations describe retained funds / returns after an unsuccessful deploy.
// Sender execution and return receipt require the separate network tests.
fn assert_bad_state_init_case(
    row: u8,
    cross_header: bool,
    destination_dapp: u8,
    bounce: bool,
    exchange: bool,
    input: [u64; 3],
    expected: [u64; 3],
    returns: bool,
) {
    let mut value = CurrencyCollection::with_grams(input[0]);
    for (id, amount) in [(2, input[1]), (1, input[2])] {
        if amount != 0 {
            value.set_other(id, amount.into()).unwrap();
        }
    }
    let (mut msg, _) = message(cross_header, destination_dapp == 1, bounce, exchange, value);
    let dapp = UInt256::from([destination_dapp; 32]);
    let mut state = StateInit::default();
    state.set_code(tvm_types::Cell::default());
    state.set_data(SliceData::from_raw(vec![row; 32], 256).into_cell());
    let dst = MsgAddressInt::with_standart(
        None,
        0,
        SliceData::from_raw(state.serialize().unwrap().repr_hash().as_slice().to_vec(), 256),
    )
    .unwrap();
    if let Some(header) = msg.int_header_mut() {
        header.set_dst(dst.clone());
    } else {
        let header = msg.cross_dapp_header_mut().unwrap();
        header.dst = dst.clone();
        header.dst_dapp_id = dapp.clone();
    }
    msg.set_state_init(state);
    let (tx, account) = execute(&msg, dapp);
    let expected_status = if expected == [0; 3] {
        AccountStatus::AccStateNonexist
    } else {
        AccountStatus::AccStateUninit
    };
    let balance = account.balance().cloned().unwrap_or_default();
    let mut actual_out = Vec::new();
    tx.iterate_out_msgs(|message| {
        actual_out.push(message);
        Ok(true)
    })
    .unwrap();
    // Check outgoing data even when the destination assertion will fail.
    assert_eq!(actual_out.len(), usize::from(returns), "table 6 row {row}: returns");
    if returns {
        let fee = config().calc_fwd_fee(false, &tvm_types::Cell::default()).unwrap().as_u128();
        let returned = &actual_out[0];
        assert_eq!(returned.src_ref(), Some(&dst));
        assert_eq!(returned.dst_ref(), Some(&address(0x11)));
        assert_eq!(returned.get_value().unwrap().grams.as_u128(), input[0] as u128 - fee);
        for (id, amount) in [(2, input[1]), (1, input[2])] {
            assert_eq!(
                returned.get_value().unwrap().get_other(id).unwrap().unwrap_or_default(),
                VarUInteger32::from(amount)
            );
        }
    }
    assert_eq!(account.status(), expected_status, "table 6 row {row}: destination");
    assert_eq!(tx.end_status, expected_status, "table 6 row {row}: transaction end status");
    assert_eq!(balance.grams.as_u128(), expected[0] as u128, "table 6 row {row}: VM");
    for (id, amount) in [(2, expected[1]), (1, expected[2])] {
        assert_eq!(
            balance.get_other(id).unwrap().unwrap_or_default(),
            VarUInteger32::from(amount),
            "table 6 row {row}: ECC {id}"
        );
    }
}

macro_rules! bad_state_init_case {
    (
        $name:ident,
        $row:expr,
        $cross:expr,
        $dapp:expr,
        $bounce:expr,
        $exchange:expr,
        $input:expr,
        $expected:expr,
        $returns:expr
    ) => {
        #[test]
        fn $name() {
            assert_bad_state_init_case(
                $row, $cross, $dapp, $bounce, $exchange, $input, $expected, $returns,
            );
        }
    };
}
bad_state_init_case!(
    html_bad_state_init_001,
    1,
    false,
    1,
    true,
    false,
    [0, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_002,
    2,
    false,
    1,
    true,
    true,
    [0, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_003,
    3,
    false,
    1,
    true,
    false,
    [0, 10000000000, 0],
    [0, 10000000000, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_004,
    4,
    false,
    1,
    true,
    true,
    [0, 10000000000, 0],
    [10000000000, 0, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_005,
    5,
    false,
    1,
    true,
    false,
    [0, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_006,
    6,
    false,
    1,
    true,
    true,
    [0, 10000000000, 10000000000],
    [10000000000, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_007,
    7,
    false,
    1,
    true,
    false,
    [500000, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_008,
    8,
    false,
    1,
    true,
    true,
    [500000, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_009,
    9,
    false,
    1,
    true,
    false,
    [500000, 10000000000, 0],
    [0, 10000000000, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_010,
    10,
    false,
    1,
    true,
    true,
    [500000, 10000000000, 0],
    [10000000000, 0, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_011,
    11,
    false,
    1,
    true,
    false,
    [500000, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_012,
    12,
    false,
    1,
    true,
    true,
    [500000, 10000000000, 10000000000],
    [10000000000, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_013,
    13,
    false,
    1,
    false,
    false,
    [0, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_014,
    14,
    false,
    1,
    false,
    true,
    [0, 10000000000, 10000000000],
    [10000000000, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_015,
    15,
    false,
    1,
    true,
    false,
    [10000000000, 10000000000, 10000000000],
    [0, 0, 0],
    true
);
bad_state_init_case!(
    html_bad_state_init_017,
    17,
    true,
    2,
    true,
    false,
    [0, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_018,
    18,
    true,
    2,
    true,
    true,
    [0, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_019,
    19,
    true,
    2,
    true,
    false,
    [0, 10000000000, 0],
    [0, 10000000000, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_020,
    20,
    true,
    2,
    true,
    true,
    [0, 10000000000, 0],
    [10000000000, 0, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_021,
    21,
    true,
    2,
    true,
    false,
    [0, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_022,
    22,
    true,
    2,
    true,
    true,
    [0, 10000000000, 10000000000],
    [10000000000, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_023,
    23,
    true,
    2,
    true,
    false,
    [500000, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_024,
    24,
    true,
    2,
    true,
    true,
    [500000, 0, 10000000000],
    [0, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_025,
    25,
    true,
    2,
    true,
    false,
    [500000, 10000000000, 0],
    [0, 10000000000, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_026,
    26,
    true,
    2,
    true,
    true,
    [500000, 10000000000, 0],
    [10000000000, 0, 0],
    false
);
bad_state_init_case!(
    html_bad_state_init_027,
    27,
    true,
    2,
    true,
    false,
    [500000, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_028,
    28,
    true,
    2,
    true,
    true,
    [500000, 10000000000, 10000000000],
    [10000000000, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_029,
    29,
    true,
    2,
    false,
    false,
    [0, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_030,
    30,
    true,
    2,
    false,
    true,
    [0, 10000000000, 10000000000],
    [10000000000, 0, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_031,
    31,
    true,
    2,
    true,
    false,
    [10000000000, 10000000000, 10000000000],
    [0, 0, 0],
    true
);
bad_state_init_case!(
    html_bad_state_init_069,
    69,
    true,
    3,
    true,
    false,
    [0, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);
bad_state_init_case!(
    html_bad_state_init_070,
    70,
    true,
    3,
    false,
    false,
    [0, 10000000000, 10000000000],
    [0, 10000000000, 10000000000],
    false
);

fn constructor_message(
    cross_header: bool,
    same_dapp: bool,
    bounce: bool,
    exchange: bool,
) -> (Message, UInt256) {
    let mut value = CurrencyCollection::with_grams(10 * UNIT);
    value.set_other(2, (10 * UNIT).into()).unwrap();
    value.set_other(1, (10 * UNIT).into()).unwrap();
    let (mut msg, dapp) = message(cross_header, same_dapp, bounce, exchange, value);
    let mut state = StateInit::default();
    state.set_code(tvm_assembler::compile_code_to_cell("ACCEPT").unwrap());
    state.set_data(tvm_types::Cell::default());
    let dst = MsgAddressInt::with_standart(None, 0, state.serialize().unwrap().repr_hash().into())
        .unwrap();
    if let Some(header) = msg.int_header_mut() {
        header.set_dst(dst);
    } else {
        msg.cross_dapp_header_mut().unwrap().dst = dst;
    }
    msg.set_state_init(state);
    msg.set_body(SliceData::from_raw(1u32.to_be_bytes().to_vec(), 32));
    (msg, dapp)
}

#[test]
fn successful_constructor_keeps_active_account_and_charges_gas_once() {
    for exchange in [false, true] {
        let (msg, dapp) = constructor_message(false, true, true, exchange);
        let (tx, account) = execute(&msg, dapp);
        let TransactionDescr::Ordinary(desc) = tx.read_description().unwrap() else {
            panic!("ordinary")
        };
        assert!(!desc.aborted);
        assert!(matches!(desc.compute_ph, tvm_block::TrComputePhase::Vm(ref vm) if vm.success));
        assert_eq!(account.status(), AccountStatus::AccStateActive);
        assert_eq!(tx.msg_count(), 0);
        let balance = account.balance().unwrap();
        let initial_vm = if exchange { 20 * UNIT } else { 10 * UNIT };
        assert_eq!(balance.grams.as_u128() + tx.total_fees().grams.as_u128(), initial_vm as u128);
        assert_eq!(
            balance.get_other(2).unwrap().unwrap_or_default(),
            VarUInteger32::from(if exchange { 0 } else { 10 * UNIT })
        );
        assert_eq!(balance.get_other(1).unwrap().unwrap(), VarUInteger32::from(10 * UNIT));
    }
}

#[test]
fn cross_header_constructor_is_still_rejected_when_funds_are_settled() {
    for same_dapp in [false, true] {
        for bounce in [false, true] {
            let (msg, dapp) = constructor_message(true, same_dapp, bounce, true);
            let (tx, account) = execute(&msg, dapp);
            let TransactionDescr::Ordinary(desc) = tx.read_description().unwrap() else {
                panic!("ordinary")
            };
            assert!(desc.aborted);
            assert!(matches!(desc.compute_ph, tvm_block::TrComputePhase::Skipped(_)));
            if bounce {
                assert_eq!(account.status(), AccountStatus::AccStateNonexist);
                assert_eq!(tx.msg_count(), 1);
            } else {
                assert_eq!(account.status(), AccountStatus::AccStateUninit);
                assert_eq!(tx.msg_count(), 0);
                assert_eq!(
                    account.balance().unwrap().grams.as_u128(),
                    (if same_dapp { 20 * UNIT } else { 10 * UNIT }) as u128
                );
            }
        }
    }
}

#[test]
fn legacy_failed_constructor_keeps_pre_fix_exchange_behavior() {
    let mut value = CurrencyCollection::default();
    value.set_other(2, (10 * UNIT).into()).unwrap();
    let (mut msg, dapp) = message(false, true, true, true, value);
    let mut state = StateInit::default();
    state.set_code(tvm_types::Cell::default());
    state.set_data(SliceData::from_raw(vec![0x42; 32], 256).into_cell());
    msg.int_header_mut().unwrap().set_dst(
        MsgAddressInt::with_standart(None, 0, state.serialize().unwrap().repr_hash().into())
            .unwrap(),
    );
    msg.set_state_init(state);
    let (tx, account) = execute_versioned(&msg, dapp, semver::Version::new(1, 0, 7));
    assert_eq!(account.status(), AccountStatus::AccStateUninit);
    assert_eq!(account.balance().unwrap().grams.as_u128(), 0);
    assert_eq!(
        account.balance().unwrap().get_other(2).unwrap().unwrap(),
        VarUInteger32::from(10 * UNIT)
    );
    assert_eq!(tx.msg_count(), 0);
}

#[path = "delivery_matrix/suspicious_cases.rs"]
mod suspicious_cases;
