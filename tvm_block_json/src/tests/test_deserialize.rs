// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.  You may obtain a copy
// of the License at:
//
// https://www.ton.dev/licenses
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_block::ConfigCopyleft;
use tvm_block::ConfigParam3;
use tvm_block::ConfigParam32;
use tvm_block::ConfigParam33;
use tvm_block::ConfigParam35;
use tvm_block::ConfigParam36;
use tvm_block::ConfigParam37;
use tvm_block::ConfigParam39;
use tvm_block::ConfigParam4;
use tvm_block::ConfigParam6;
use tvm_block::ConfigVotingSetup;
use tvm_block::DelectorParams;
use tvm_block::Number16;
use tvm_block::SigPubKey;
use tvm_block::VarUInteger32;
use tvm_types::BuilderData;
use tvm_types::IBitstring;

use super::*;
use crate::serialize_config;
use crate::serialize_config_param;
use crate::SerializationMode;

include!("./test_common.rs");

#[test]
fn test_parse_zerostate() {
    let ethalon = std::fs::read_to_string("src/tests/data/zerostate-ethalon.json").unwrap();
    let map = serde_json::from_str::<Map<String, Value>>(&ethalon).unwrap();
    let state = parse_state(&map).unwrap();
    let json = crate::debug_state_full(state).unwrap();
    assert_json_eq(&json, &ethalon, "zerostate");
}

fn check_err<T: std::fmt::Debug>(result: Result<T>, text: &str) {
    let len = text.len();
    assert_eq!(&result.expect_err("must generate error").to_string()[0..len], text)
}

#[test]
fn test_parse_errors() {
    let json = serde_json::json!({
        "obj": {
            "a1": "12345678901234567890",
            "a2": "qwe",
            "a3": 123.4567890
        },
        "array": [
            {
                "a1": "123"
            },
            {
                "a1": 123
            }
        ],
        "str": "qwe",
        "int": "-100",
        "uint": "-100",
    });

    let map = PathMap::new(&json.as_object().unwrap());
    check_err(map.get_obj("unknown"), "root must have the field `unknown`");
    check_err(map.get_vec("obj"), "root/obj must be the vector");
    let obj = map.get_obj("obj").unwrap();
    check_err(obj.get_obj("a1"), "root/obj/a1 must be the object");
    check_err(obj.get_num("a1"), "root/obj/a1 must be the integer or a string with the integer");
    check_err(obj.get_num("a2"), "root/obj/a2 must be the integer or a string with the integer");
    check_err(obj.get_num("a3"), "root/obj/a3 must be the integer or a string with the integer");
}

fn get_config_param0() -> ConfigParam0 {
    let mut c = ConfigParam0::new();
    c.config_addr = UInt256::from([1; 32]);
    c
}

fn get_config_param1() -> ConfigParam1 {
    let mut c = ConfigParam1::new();
    c.elector_addr = UInt256::from([1; 32]);
    c
}

fn get_config_param7() -> ConfigParam7 {
    let mut ecc = ExtraCurrencyCollection::default();
    for i in 1..100 {
        ecc.set(&(i as u32), &VarUInteger32::from_two_u128(i * 100, i * 205).unwrap()).unwrap();
    }
    ConfigParam7 { to_mint: ecc }
}

fn get_config_param16() -> ConfigParam16 {
    let mut c = ConfigParam16::new();
    c.max_validators = Number16::new(23424).unwrap();
    c.max_main_validators = Number16::new(35553).unwrap();
    c.min_validators = Number16::new(11).unwrap();
    c
}

fn get_config_param17() -> ConfigParam17 {
    let mut c = ConfigParam17::new();
    c.min_stake = Grams::zero();
    c.max_stake = Grams::one();
    c.max_stake_factor = 12121;
    c
}

fn get_storage_prices() -> StoragePrices {
    let mut st = StoragePrices::new();
    st.bit_price_ps = 10;
    st.cell_price_ps = 20;
    st.mc_bit_price_ps = 30;
    st.mc_cell_price_ps = 40;
    st.utime_since = 50;
    st
}

fn get_config_param18() -> ConfigParam18 {
    let mut cp18 = ConfigParam18::default();
    for _ in 0..10 {
        cp18.insert(&get_storage_prices()).unwrap();
    }
    cp18
}

fn get_gas_limit_prices() -> GasLimitsPrices {
    let mut glp = GasLimitsPrices {
        gas_price: 10,
        gas_limit: 20,
        gas_credit: 30,
        block_gas_limit: 40,
        freeze_due_limit: 50,
        delete_due_limit: 60,
        special_gas_limit: 70,
        flat_gas_limit: 80,
        flat_gas_price: 90,
        max_gas_threshold: 0,
    };
    glp.max_gas_threshold = glp.calc_max_gas_threshold();
    glp
}

fn get_msg_forward_prices() -> MsgForwardPrices {
    let mut mfp = MsgForwardPrices::new();
    mfp.lump_price = 10;
    mfp.bit_price = 20;
    mfp.cell_price = 30;
    mfp.ihr_price_factor = 40;
    mfp.first_frac = 50;
    mfp.next_frac = 60;
    mfp
}

fn get_cat_chain_config() -> CatchainConfig {
    let mut cc = CatchainConfig::new();
    cc.shuffle_mc_validators = true;
    cc.isolate_mc_validators = false;
    cc.mc_catchain_lifetime = 10;
    cc.shard_catchain_lifetime = 20;
    cc.shard_validators_lifetime = 30;
    cc.shard_validators_num = 40;
    cc
}

fn get_config_param31() -> ConfigParam31 {
    let mut cp31 = ConfigParam31::new();
    for _ in 1..10 {
        cp31.add_address(UInt256::rand());
    }
    cp31
}

fn get_workchain_desc() -> WorkchainDescr {
    let mut wc = WorkchainDescr::new();
    wc.enabled_since = 332;
    wc.accept_msgs = true;
    wc.active = false;
    wc.flags = 0x345;
    wc.version = 1;
    wc.zerostate_file_hash = UInt256::rand();
    wc.zerostate_root_hash = UInt256::rand();

    wc.format = WorkchainFormat::Basic(WorkchainFormat1::with_params(123, 453454));
    wc
}

fn get_config_param11() -> ConfigParam11 {
    let normal_params = ConfigProposalSetup {
        min_tot_rounds: 1,
        max_tot_rounds: 2,
        min_wins: 3,
        max_losses: 4,
        min_store_sec: 5,
        max_store_sec: 6,
        bit_price: 7,
        cell_price: 8,
    };
    let critical_params = ConfigProposalSetup {
        min_tot_rounds: 10,
        max_tot_rounds: 20,
        min_wins: 30,
        max_losses: 40,
        min_store_sec: 50000,
        max_store_sec: 60000,
        bit_price: 70000,
        cell_price: 80000,
    };
    ConfigVotingSetup::new(&normal_params, &critical_params).unwrap()
}

fn get_config_param12() -> ConfigParam12 {
    let mut cp12 = ConfigParam12::new();

    for i in 0..10 as i32 {
        let wc = get_workchain_desc();
        cp12.insert(i, &wc).unwrap();
    }
    cp12
}

fn get_config_param9() -> ConfigParam9 {
    let mut mp = MandatoryParams::default();
    for i in 1..100 {
        mp.set(&i, &()).unwrap();
    }
    ConfigParam9 { mandatory_params: mp }
}

fn get_config_param10() -> ConfigParam10 {
    let mut cp = MandatoryParams::default();
    for i in 1..100 {
        cp.set(&i, &()).unwrap();
    }
    ConfigParam10 { critical_params: cp }
}

fn get_config_param14() -> ConfigParam14 {
    ConfigParam14 {
        block_create_fees: BlockCreateFees {
            masterchain_block_fee: Grams::from(1458347523u64),
            basechain_block_fee: Grams::from(145800000000003u64),
        },
    }
}

fn get_config_param15() -> ConfigParam15 {
    ConfigParam15 {
        validators_elected_for: 10,
        elections_start_before: 20,
        elections_end_before: 30,
        stake_held_for: 40,
    }
}

fn get_config_param29() -> ConfigParam29 {
    ConfigParam29 {
        consensus_config: ConsensusConfig {
            new_catchain_ids: true,
            round_candidates: 10 as u32 | 1,
            next_candidate_delay_ms: 20,
            consensus_timeout_ms: 30,
            fast_attempts: 40,
            attempt_duration: 50,
            catchain_max_deps: 60,
            max_block_bytes: 70,
            max_collated_bytes: 80,
        },
    }
}

fn get_config_param40() -> ConfigParam40 {
    ConfigParam40 {
        slashing_config: SlashingConfig {
            slashing_period_mc_blocks_count: 10,
            resend_mc_blocks_count: 20,
            min_samples_count: 30,
            collations_score_weight: 40,
            signing_score_weight: 50,
            min_slashing_protection_score: 60,
            z_param_numerator: 70,
            z_param_denominator: 80,
        },
    }
}

fn get_config_param42() -> ConfigCopyleft {
    let mut cfg =
        ConfigCopyleft { copyleft_reward_threshold: 100.into(), license_rates: Default::default() };
    for i in 0..10 {
        cfg.license_rates.set(&(i as u8), &(i * 10 as u8)).unwrap();
    }
    cfg
}

fn get_config_param44() -> SuspendedAddresses {
    let mut cfg = SuspendedAddresses::new();
    cfg.add_suspended_address(-1, UInt256::default()).unwrap();
    cfg.add_suspended_address(0, UInt256::max()).unwrap();
    cfg
}

fn get_block_limits(some_val: u32) -> BlockLimits {
    BlockLimits::with_limits(
        ParamLimits::with_limits(some_val + 1, some_val + 2, some_val + 3).unwrap(),
        ParamLimits::with_limits(some_val + 4, some_val + 5, some_val + 6).unwrap(),
        ParamLimits::with_limits(some_val + 7, some_val + 8, some_val + 9).unwrap(),
    )
}

fn get_config_param_39() -> ConfigParam39 {
    let mut cp = ConfigParam39::default();

    let vstk = ValidatorSignedTempKey::construct_from_base64("te6ccgEBAgEAlAABgwRQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgKAEAmgMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDA46BJ4rNcGUgnAwuWyQw+RaHbPRm6ub11xlYhzyXOQiirgIdZgABiJRdJFss").unwrap();
    cp.insert(&UInt256::from([1; 32]), &vstk).unwrap();

    let vstk = ValidatorSignedTempKey::construct_from_base64("te6ccgEBAgEAlAABgwRQYGBgYGBgYGBgYGBgYGBgYGBgYGBgYGBgYGBgYGBgYGBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBweAEAmgMICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICI6BJ4obim1cCeb6yST1ojiep2h7raTYuNUkJEbwlPIjN4qOLAAHoYRdJF8U").unwrap();
    cp.insert(&UInt256::from([1; 32]), &vstk).unwrap();

    cp
}

fn get_validator_set() -> ValidatorSet {
    let mut list = vec![];

    let key = SigPubKey::from_bytes(
        &*base64_decode("39MLqLIVrzLqPCHCFpbn1/jILSbfNMtnr/7zOkKE1Ds=").unwrap(),
    )
    .unwrap();
    let vd = ValidatorDescr::with_params(key, 4, None, None);
    list.push(vd);

    let key = SigPubKey::from_bytes(
        &*base64_decode("BIYYOFHTgVDIFzVLhuSZw2ne1J3zuv75zwYhAXb0+iY=").unwrap(),
    )
    .unwrap();
    let vd = ValidatorDescr::with_params(key, 5, None, None);
    list.push(vd);

    ValidatorSet::new(0, 100, 1, list).unwrap()
}

fn prepare_config_params() -> ConfigParams {
    let mut cp = ConfigParams::new();

    let c0 = ConfigParamEnum::ConfigParam0(get_config_param0());
    cp.set_config(c0).unwrap();

    let c1 = ConfigParamEnum::ConfigParam1(get_config_param1());
    cp.set_config(c1).unwrap();

    let c2 = ConfigParamEnum::ConfigParam2(ConfigParam2 { minter_addr: UInt256::from([123; 32]) });
    cp.set_config(c2).unwrap();

    let c3 = ConfigParamEnum::ConfigParam3(ConfigParam3 {
        fee_collector_addr: UInt256::from([133; 32]),
    });
    cp.set_config(c3).unwrap();

    let c4 =
        ConfigParamEnum::ConfigParam4(ConfigParam4 { dns_root_addr: UInt256::from([144; 32]) });
    cp.set_config(c4).unwrap();

    let c5 = ConfigParamEnum::ConfigParam5(ConfigParam5 { owner_addr: UInt256::from([200; 32]) });
    cp.set_config(c5).unwrap();

    let c6 = ConfigParamEnum::ConfigParam6(ConfigParam6 {
        mint_new_price: Grams::from(123u64),
        mint_add_price: Grams::from(1458347523u64),
    });
    cp.set_config(c6).unwrap();

    let c7 = ConfigParamEnum::ConfigParam7(get_config_param7());
    cp.set_config(c7).unwrap();

    let c8 = ConfigParamEnum::ConfigParam8(ConfigParam8 {
        global_version: GlobalVersion { version: 123, capabilities: 4567890 },
    });
    cp.set_config(c8).unwrap();

    let c9 = ConfigParamEnum::ConfigParam9(get_config_param9());
    cp.set_config(c9).unwrap();

    let c10 = ConfigParamEnum::ConfigParam10(get_config_param10());
    cp.set_config(c10).unwrap();

    let c11 = ConfigParamEnum::ConfigParam11(get_config_param11());
    cp.set_config(c11).unwrap();

    let c12 = ConfigParamEnum::ConfigParam12(get_config_param12());
    cp.set_config(c12).unwrap();

    let mut builder = BuilderData::new();
    builder.append_u32(100).unwrap();
    let c13 = ConfigParamEnum::ConfigParam13(ConfigParam13 { cell: builder.into_cell().unwrap() });
    cp.set_config(c13).unwrap();

    let c14 = ConfigParamEnum::ConfigParam14(get_config_param14());
    cp.set_config(c14).unwrap();

    let c15 = ConfigParamEnum::ConfigParam15(get_config_param15());
    cp.set_config(c15).unwrap();

    let c16 = ConfigParamEnum::ConfigParam16(get_config_param16());
    cp.set_config(c16).unwrap();

    let c17 = ConfigParamEnum::ConfigParam17(get_config_param17());
    cp.set_config(c17).unwrap();

    let c18 = ConfigParamEnum::ConfigParam18(get_config_param18());
    cp.set_config(c18).unwrap();

    let c20 = ConfigParamEnum::ConfigParam20(get_gas_limit_prices());
    cp.set_config(c20).unwrap();

    let c21 = ConfigParamEnum::ConfigParam21(get_gas_limit_prices());
    cp.set_config(c21).unwrap();

    let cp22 = get_block_limits(22);
    let c22 = ConfigParamEnum::ConfigParam22(cp22);
    cp.set_config(c22).unwrap();

    let cp23 = get_block_limits(23);
    let c23 = ConfigParamEnum::ConfigParam23(cp23);
    cp.set_config(c23).unwrap();

    let c24 = ConfigParamEnum::ConfigParam24(get_msg_forward_prices());
    cp.set_config(c24).unwrap();

    let c25 = ConfigParamEnum::ConfigParam25(get_msg_forward_prices());
    cp.set_config(c25).unwrap();

    let c28 = ConfigParamEnum::ConfigParam28(get_cat_chain_config());
    cp.set_config(c28).unwrap();

    let c29 = ConfigParamEnum::ConfigParam29(get_config_param29());
    cp.set_config(c29).unwrap();

    let c30 = ConfigParamEnum::ConfigParam30(DelectorParams {
        delections_step: 10,
        validator_init_code_hash: UInt256::rand(),
        staker_init_code_hash: UInt256::rand(),
    });
    cp.set_config(c30).unwrap();

    let c31 = ConfigParamEnum::ConfigParam31(get_config_param31());
    cp.set_config(c31).unwrap();

    let mut cp32 = ConfigParam32::new();
    cp32.prev_validators = get_validator_set();
    cp.set_config(ConfigParamEnum::ConfigParam32(cp32)).unwrap();

    let mut cp33 = ConfigParam33::new();
    cp33.prev_temp_validators = get_validator_set();
    cp.set_config(ConfigParamEnum::ConfigParam33(cp33)).unwrap();

    let mut cp34 = ConfigParam34::new();
    cp34.cur_validators = get_validator_set();
    cp.set_config(ConfigParamEnum::ConfigParam34(cp34)).unwrap();

    let mut cp35 = ConfigParam35::new();
    cp35.cur_temp_validators = get_validator_set();
    cp.set_config(ConfigParamEnum::ConfigParam35(cp35)).unwrap();

    let mut cp36 = ConfigParam36::new();
    cp36.next_validators = get_validator_set();
    cp.set_config(ConfigParamEnum::ConfigParam36(cp36)).unwrap();

    let mut cp37 = ConfigParam37::new();
    cp37.next_temp_validators = get_validator_set();
    cp.set_config(ConfigParamEnum::ConfigParam37(cp37)).unwrap();

    let cp39 = get_config_param_39();
    let c39 = ConfigParamEnum::ConfigParam39(cp39);
    cp.set_config(c39).unwrap();

    let c40 = ConfigParamEnum::ConfigParam40(get_config_param40());
    cp.set_config(c40).unwrap();

    let c42 = get_config_param42();
    cp.set_config(ConfigParamEnum::ConfigParam42(c42)).unwrap();

    let c44 = get_config_param44();
    cp.set_config(ConfigParamEnum::ConfigParam44(c44)).unwrap();

    cp
}

#[test]
fn test_config_params() {
    let cp = prepare_config_params();

    let check_params = |old: &ConfigParams, new: &ConfigParams| {
        for i in 0..45 {
            println!("Iteration {}", i);
            if old.config_present(i).unwrap() {
                let old_conf = old.config(i).unwrap().unwrap();
                let new_conf = new.config(i).unwrap().unwrap();
                assert_eq!(old_conf, new_conf);
            } else {
                assert!(!new.config_present(i).unwrap());
            }
        }
    };

    let mut json = serde_json::Map::<String, Value>::new();
    serialize_config(&mut json, &cp, SerializationMode::QServer).unwrap();
    let parsed_config = parse_config(&json.get("config").unwrap().as_object().unwrap()).unwrap();
    check_params(&cp, &parsed_config);

    let mut json = serde_json::Map::<String, Value>::new();
    serialize_config(&mut json, &cp, SerializationMode::Debug).unwrap();
    let parsed_config = parse_config(&json.get("config").unwrap().as_object().unwrap()).unwrap();
    check_params(&cp, &parsed_config);
}

#[test]
fn test_parse_config_params() {
    let cp = prepare_config_params();

    for index in 0..45 {
        if let Ok(param) = serialize_config_param(&cp, index) {
            println!("{}: {}", index, param);
            let config = serde_json::from_str(&param).unwrap();
            let cp_new = parse_config_with_mandatory_params(&config, &[index]).unwrap();
            assert_eq!(cp.config(index).unwrap(), cp_new.config(index).unwrap());
        }
    }
}

#[test]
fn test_parse_block_proof() {
    let boc = include_bytes!("data/block_proof");
    let ethalon_proof = tvm_block::BlockProof::construct_from_bytes(boc).unwrap();
    let json = serde_json::from_str(include_str!("data/proof-ethalon.json")).unwrap();

    let parsed_proof = parse_block_proof(&json, ethalon_proof.proof_for.file_hash.clone()).unwrap();

    assert_eq!(ethalon_proof, parsed_proof);
    assert_eq!(boc.as_slice(), &parsed_proof.write_to_bytes().unwrap());
}
