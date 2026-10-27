//! SDK-only delivery matrix: message header type and DApp boundary are independent.
use tvm_block::{
    Account, AccountStatus, AddSub, ConfigParam12, ConfigParam18, ConfigParam31, ConfigParamEnum,
    CrossDappMessageHeader, CurrencyCollection, Deserializable, InternalMessageHeader, Message,
    MsgAddressInt, MsgAddressIntOrNone, Serializable, StateInit, StoragePrices, TrBouncePhase,
    TransactionDescr, VarUInteger32, WorkchainDescr,
};
use tvm_executor::{
    BlockchainConfig, ExecuteParams, OrdinaryTransactionExecutor, TransactionExecutor,
};
use tvm_types::{SliceData, UInt256};

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
    let executor = OrdinaryTransactionExecutor::new(config());
    let mut root = Account::default().serialize().unwrap();
    let (tx, minted) = executor
        .execute_with_libs_and_params(
            Some(message),
            &mut root,
            ExecuteParams {
                dapp_id: Some(dapp),
                engine_version: semver::Version::new(1, 0, 8),
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
fn failed_state_init_does_not_use_plain_funding_fallback() {
    let mut value = CurrencyCollection::default();
    value.set_other(1, 10 * UNIT as u128).unwrap();
    let (mut msg, dapp) = message(false, true, true, false, value);
    msg.set_state_init(StateInit::default());
    let (_, account) = execute(&msg, dapp);
    assert_eq!(account.status(), AccountStatus::AccStateNonexist);
}
