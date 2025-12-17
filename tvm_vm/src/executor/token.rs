use std::sync::Arc;

use byte_slice_cast::AsByteSlice;
use tvm_abi::Param;
use tvm_abi::ParamType;
use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_2;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_block::ACTION_BURNECC;
use tvm_block::ACTION_CNVRTSHELLQ;
use tvm_block::ACTION_MINT_SHELL_TOKEN;
use tvm_block::ACTION_MINT_SHELLQ_TOKEN;
use tvm_block::ACTION_MINTECC;
use tvm_block::ACTION_SEND_TO_DAPP_CONFIG;
use tvm_block::ExtraCurrencyCollection;
use tvm_block::Serializable;
use tvm_block::VarUInteger32;
use tvm_types::BuilderData;
use tvm_types::ExceptionCode;
use tvm_types::SliceData;
use tvm_types::error;

use crate::error::TvmError;
use crate::executor::blockchain::add_action;
use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::executor::wasm::check_and_get_wasm_by_hash;
use crate::executor::wasm::run_wasm_core;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Exception;
use crate::types::Status;
use crate::utils::unpack_data_from_cell;

pub const ECC_NACKL_KEY: u32 = 1;
pub const ECC_SHELL_KEY: u32 = 2;
pub const INFINITY_CREDIT: i128 = -1;

// pub const ARFC: f64 = 1000_f64;
// pub const MINRC: f64 = 1_f64;
// pub const MAXRC: f64 = 3_f64;
pub const MAXRT: u128 = 157_766_400;
// pub const KF: f64 = 0.01_f64;
// pub const KS: f64 = 0.001_f64;
// pub const KM: f64 = 0.00001_f64;
// pub const KRMV: f64 = 0.225_f64;
// pub const MAX_FREE_FLOAT_FRAC: f64 = 1_f64 / 3_f64;

const DELTA_SBK_NUMENATOR: u128 = 10_000_000;
const DELTA_SBK_DENOMINATOR: u128 = 10_000_525;

const RC_ONE_Q32: i64 = 1i64 << 32;
const RC_POW2_COEFF: [i64; 6] = [
    4_294_967_296, // 1 * 2^32
    2_977_044_472, // ln2 * 2^32
    1_031_764_991, // (ln2)^2/2 * 2^32
    238_388_332,   // (ln2)^3/6  * 2^32
    41_309_550,    // (ln2)^4/24 * 2^32
    5_726_720,     // (ln2)^5/120 * 2^32
];
const RC_K1_Q32: i64 = 6_196_328_019; // 1 / ln 2 * 2^32
const RC_K2_Q32: i64 = 188; // ln(ARFC) / MAXRT * 2^32 = ln(1000) / 157766400 * 2^32
const RC_K3_Q32: i64 = 8_598_533_125; // (MAXRC - MINRC) / (1 - 1 / ARFC) * 2^32 = (3 - 1) / (1 − 1 / 1000) * 2^32
const RCSCALE: i128 = 1_000_000_000;

const ONE_Q32: i64 = 1i64 << 32;
const TOTALSUPPLY: u128 = 10_400_000_000_000_000_000;
const TTMT: u128 = 2_000_000_000;
const KM_Q32: i64 = 42_950; // KM * 2 ^ 32 = 1e-5 * 2 ^ 32
const ONE_PLUS_KM: i64 = ONE_Q32 + KM_Q32;
const KRBK_NUM: u128 = 675; // 0.675 = 675 / 1000
const KRBK_DEN: u128 = 1_000;
const KRBM_NUM: u128 = 1;
const KRBM_DEN: u128 = 10;
const KRMV_NUM: u128 = 225;
const KRMV_DEN: u128 = 1000;
const UM_Q64: i64 = 106_188_087_029; // -ln(KM / (KM + 1)) / TTMT * 2^64 = -ln(1e-5 / (1 + 1e-5)) / 2e9 * 2^64
const SBK_BASE_START: u128 = 14393409967783;
const BIG_MIN_STAKE: u128 = 10_000_000_000_000_000;

// e^(−n), n = 0...12 in Q‑32
const EXP_NEG_VAL_Q32: [i64; 13] = [
    4_294_967_296,
    1_580_030_169,
    581_260_615,
    213_833_830,
    78_665_070,
    28_939_262,
    10_646_160,
    3_916_503,
    1_440_801,
    530_041,
    194_991,
    71_733,
    26_389,
];

// Maclaurin coeffs
const MAC_EXP_NEG_COEFF_Q32: [i64; 9] = [
    4_294_967_296,  // 1 * 2^32
    -4_294_967_296, // -1 * 2^32
    2_147_483_648,  // 1/2!  * 2^32
    -715_827_883,   // -1/3! * 2^32
    178_956_971,    // 1/4! * 2^32
    -35_791_394,    // -1/5! 2^32
    5_965_232,      // 1/6! * 2^32
    -852_176,       // -1/7! * 2^32
    106_522,        // 1/8! * 2^32
];

const KF_Q32: i64 = 42_949_673; // KF * 2 ^ 32 = 1e-2 * 2 ^ 32
const ONE_PLUS_KF_Q32: i64 = ONE_Q32 + KF_Q32;
const MAX_FREE_FLOAT_FRAC_Q32: i64 = ONE_Q32 / 3;
const UF_Q64: i64 = 42_566_973_522; // -ln(KF / (KF + 1)) / TTMT * 2^64 = -ln(1e-2 / (1 + 1e-2)) / 2e9 * 2^64

const MV_FRAC_BITS: u32 = 32;
const ONE: i128 = 1i128 << MV_FRAC_BITS;

const MV_X1: i128 = 0;
const MV_X2: i128 = 1_288_490_189; // 0.3 * 2^32
const MV_X3: i128 = 3_006_477_107; // 0.7 * 2^32
const MV_X4: i128 = 4_294_967_296; // 1.0 * 2^32
const MV_Y1: i128 = 0;
const MV_Y2: i128 = 286_461_212; // 0.066696948409 * 2^32
const MV_Y3: i128 = 8_589_934_592; // 2 * 2^32
const MV_Y4: i128 = 34_359_738_368; // 8 * 2^32
const MV_K1: i128 = 42_949_672_960; // 10 * 2^32
const MV_K2: i128 = 8_135_370_769; // 1.894163612445 * 2^32
const MV_K3: i128 = 77_309_390_134; // 17.999995065464 * 2^32

// e^n for n = 0..18 in Q32.32
const E_INT: [i128; 19] = [
    4_294_967_296,
    11_674_931_555,
    31_735_754_293,
    86_266_724_208,
    234_497_268_814,
    637_429_664_642,
    1_732_713_474_316,
    4_710_003_551_159,
    12_803_117_065_094,
    34_802_480_465_680,
    94_602_950_235_157,
    257_157_480_542_844,
    699_026_506_411_923,
    1_900_151_049_990_741,
    5_165_146_070_517_207,
    14_040_322_704_823_566,
    38_165_554_074_222_850,
    103_744_732_113_031_053,
    282_007_420_101_203_878,
];

pub(super) fn execute_ecc_mint(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("MINTECC"))?;
    fetch_stack(engine, 2)?;
    let x: u32 = engine.cmd.var(0).as_integer()?.into(0..=255)?;
    let y: VarUInteger32 = VarUInteger32::from(engine.cmd.var(1).as_integer()?.into(0..=u64::MAX)?);
    let mut data = ExtraCurrencyCollection::new();
    data.set(&x, &y)?;
    let mut cell = BuilderData::new();
    data.write_to(&mut cell)?;
    add_action(engine, ACTION_MINTECC, None, cell)
}

pub(super) fn execute_run_wasm_concat_multiarg(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("RUNWASM"))?;
    fetch_stack(engine, 8)?;

    let (wasm_executable, wasm_hash) = check_and_get_wasm_by_hash(engine, 0, 7)?;

    // let s = engine.cmd.var(0).as_cell()?;
    // let wasm_executable = rejoin_chain_of_cells(s)?;

    // get exported instance name to call
    let s = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let wasm_instance_name = unpack_data_from_cell(s, engine)?;
    let wasm_instance_name = String::from_utf8(wasm_instance_name)?;

    // get exported func to call from within instance
    let s = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let wasm_func_name = unpack_data_from_cell(s, engine)?;
    let wasm_func_name = String::from_utf8(wasm_func_name)?;

    // execute wasm func
    // collect result
    // substract gas based on wasm fuel used
    let s = engine.cmd.var(3).as_cell()?;
    log::debug!("Loading WASM Args");
    let mut wasm_func_args =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(
                ExceptionCode::WasmCellUnpackError,
                "Failed to unpack wasm instruction {:?}",
                e
            )?,
        };
    let s = engine.cmd.var(4).as_cell()?;
    let mut wasm_args_tail =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(
                ExceptionCode::WasmCellUnpackError,
                "Failed to unpack wasm instruction {:?}",
                e
            )?,
        };
    wasm_func_args.append(&mut wasm_args_tail);
    let s = engine.cmd.var(5).as_cell()?;
    let mut wasm_args_tail =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(
                ExceptionCode::WasmCellUnpackError,
                "Failed to unpack wasm instruction {:?}",
                e
            )?,
        };
    wasm_func_args.append(&mut wasm_args_tail);
    let s = engine.cmd.var(6).as_cell()?;
    let mut wasm_args_tail =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(
                ExceptionCode::WasmCellUnpackError,
                "Failed to unpack wasm instruction {:?}",
                e
            )?,
        };
    wasm_func_args.append(&mut wasm_args_tail);
    log::debug!("WASM Args loaded {:?}", wasm_func_args);

    run_wasm_core(
        engine,
        wasm_executable,
        &wasm_func_name,
        &wasm_instance_name,
        wasm_func_args,
        wasm_hash,
    )
}

// execute wasm binary
pub(super) fn execute_run_wasm(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("RUNWASM"))?;
    fetch_stack(engine, 5)?;

    let (wasm_executable, wasm_hash) = check_and_get_wasm_by_hash(engine, 0, 4)?;
    // let s = engine.cmd.var(0).as_cell()?;
    // let wasm_executable = rejoin_chain_of_cells(s)?;

    // get exported instance name to call
    let s = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let wasm_instance_name = unpack_data_from_cell(s, engine)?;
    let wasm_instance_name = String::from_utf8(wasm_instance_name)?;

    // get exported func to call from within instance
    let s = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let wasm_func_name = unpack_data_from_cell(s, engine)?;
    let wasm_func_name = String::from_utf8(wasm_func_name)?;

    // execute wasm func
    // collect result
    // substract gas based on wasm fuel used
    let s = engine.cmd.var(3).as_cell()?;
    log::debug!("Loading WASM Args");
    let wasm_func_args =
        match TokenValue::read_bytes(SliceData::load_cell(s.clone())?, true, &ABI_VERSION_2_4)?.0 {
            TokenValue::Bytes(items) => items,
            e => err!(
                ExceptionCode::WasmCellUnpackError,
                "Failed to unpack wasm instruction {:?}",
                e
            )?,
        };
    log::debug!("WASM Args loaded {:?}", wasm_func_args);

    run_wasm_core(
        engine,
        wasm_executable,
        &wasm_func_name,
        &wasm_instance_name,
        wasm_func_args,
        wasm_hash,
    )
}

pub(super) fn execute_ecc_burn(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("BURNECC"))?;
    fetch_stack(engine, 2)?;
    let x: u32 = engine.cmd.var(0).as_integer()?.into(0..=255)?;
    let y = engine.cmd.var(1).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    y.write_to(&mut cell)?;
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_BURNECC, None, cell)
}

pub(super) fn execute_exchange_shell(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CNVRTSHELLQ"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_CNVRTSHELLQ, None, cell)
}

pub(super) fn execute_calculate_repcoef(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCREPCOEF"))?;
    fetch_stack(engine, 1)?;
    let bkrt = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as u128;
    let repcoef = repcoef_int(bkrt);
    engine.cc.stack.push(int!(repcoef));
    Ok(())
}

pub(super) fn execute_calculate_adjustment_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBKREWARDADJ"))?;
    fetch_stack(engine, 4)?;
    let t = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?; //time from network start
    let rbkprev = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //previous value of rewardadjustment (not minimum)
    let mut drbkavg = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?;
    //_delta_reward = (_delta_reward * _calc_reward_num + (block.timestamp -
    //_delta_reward _reward_last_time)) / (_calc_reward_num + 1);
    //_delta_reward - average time between reward adj calculate
    //_calc_reward_num - number of calculate
    //_reward_last_time - time of last calculate
    let mbkt = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?; //sum of reward token (minted, include slash token)
    let rbkmin;
    if t <= TTMT - 1 {
        rbkmin = rbkprev / 3 * 2;
    } else {
        rbkmin = 0;
    }
    if drbkavg == 0 {
        drbkavg = 1;
    }
    let rbk =
        (((calc_mbk(t + drbkavg, KRBK_NUM, KRBK_DEN) - mbkt) / drbkavg).max(rbkmin)).min(rbkprev);
    engine.cc.stack.push(int!(rbk as u128));
    Ok(())
}

pub(super) fn execute_calculate_adjustment_reward_bmmv(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBMMVREWARDADJ"))?;
    fetch_stack(engine, 5)?;
    let is_bm = engine.cmd.var(0).as_bool()?;
    let t = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //time from network start
    let rbmprev = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?; //previous value of rewardadjustment (not minimum)
    let mut drbmavg = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?;
    let mbmt = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)?; //sum of reward token (minted, include slash token)
    let rbmmin;
    if t <= TTMT - 1 {
        rbmmin = rbmprev / 3 * 2;
    } else {
        rbmmin = 0;
    }
    let rbm;
    if drbmavg == 0 {
        drbmavg = 1;
    }
    if is_bm {
        rbm = (((calc_mbk(t + drbmavg, KRBM_NUM, KRBM_DEN) - mbmt) / drbmavg).max(rbmmin))
            .min(rbmprev);
    } else {
        rbm = (((calc_mbk(t + drbmavg, KRMV_NUM, KRMV_DEN) - mbmt) / drbmavg).max(rbmmin))
            .min(rbmprev);
    }
    engine.cc.stack.push(int!(rbm as u128));
    Ok(())
}

fn exp_neg_q32(v_q32: i64) -> i64 {
    let n = (v_q32 >> 32) as usize;
    if n >= EXP_NEG_VAL_Q32.len() {
        return 0;
    }
    let f = v_q32 & (ONE_Q32 - 1);
    let int_part = EXP_NEG_VAL_Q32[n];
    let frac_part = horner_q32(f, &MAC_EXP_NEG_COEFF_Q32);
    ((int_part as i128 * frac_part as i128) >> 32) as i64
}

fn calc_mbk(t: u128, krk_num: u128, krk_den: u128) -> u128 {
    let mbk: u128 = if t > TTMT {
        TOTALSUPPLY
    } else {
        let v_q32 = ((UM_Q64 as i128 * t as i128 + (1 << 31)) >> 32) as i64;
        let exp_q32 = exp_neg_q32(v_q32);
        let diff_q32 = ONE_Q32 - exp_q32;
        let prod_q32 = ((ONE_PLUS_KM as i128 * diff_q32 as i128) >> 32) as i64;
        ((TOTALSUPPLY as i128 * prod_q32 as i128) >> 32) as u128
    };
    (mbk * krk_num) / krk_den
}

pub(super) fn execute_calculate_mbk(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCMBK"))?;
    fetch_stack(engine, 1)?;
    let t = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?;
    engine.cc.stack.push(int!(calc_mbk(t, KRBK_NUM, KRBK_DEN)));
    Ok(())
}

pub(super) fn execute_calculate_block_manager_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBMREWARD"))?;
    fetch_stack(engine, 5)?;
    let radj = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?;
    let depoch = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?;
    let mbm = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?;
    let count_bm = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?;
    let _pubkey_cell = engine.cmd.var(4).as_cell()?;
    let reward;
    if mbm >= TOTALSUPPLY / 10 || count_bm == 0 {
        reward = 0;
    } else {
        reward = radj * depoch / count_bm;
    }
    engine.cc.stack.push(int!(reward as u128));
    Ok(())
}

fn calc_one_minus_fstk_q32_int(t: u128) -> u128 {
    let fstk_q32: i64 = if t > TTMT {
        MAX_FREE_FLOAT_FRAC_Q32
    } else {
        let v_q32 = ((UF_Q64 as i128 * t as i128 + (1 << 31)) >> 32) as i64;
        let diff_q32 = ONE_Q32 - exp_neg_q32(v_q32);
        let tmp_q32 = ((ONE_PLUS_KF_Q32 as i128 * diff_q32 as i128) >> 32) as i64;
        ((MAX_FREE_FLOAT_FRAC_Q32 as i128 * tmp_q32 as i128) >> 32) as i64
    };
    (ONE_Q32 - fstk_q32) as u128
}

pub(super) fn execute_calculate_min_stake(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKE"))?;
    fetch_stack(engine, 4)?;
    let _nbkreq = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?; // needNumberOfActiveBlockKeepers = 10000
    let mut nbk = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //numberOfActiveBlockKeepersAtBlockStart
    let tstk = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)?; //time from network start + uint128(_waitStep / 3) where waitStep - number of block duration of preEpoch
    let mbkav = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)?; //sum of reward token without slash tokens
    let sbkbase;
    let engine_version = engine.get_version();
    if engine_version >= "1.0.2".parse().unwrap() {
        engine.cc.stack.push(int!(BIG_MIN_STAKE as u128));
        return Ok(());
    }
    if mbkav != 0 {
        let one_minus_fstk_q32 = calc_one_minus_fstk_q32_int(tstk);
        if nbk == 0 {
            nbk = 1;
        }
        sbkbase = ((((mbkav as u128) * (one_minus_fstk_q32 as u128)) >> 32) * DELTA_SBK_NUMENATOR)
            / (2u128 * (nbk as u128) * DELTA_SBK_DENOMINATOR);
    } else {
        sbkbase = SBK_BASE_START;
    }
    engine.cc.stack.push(int!(sbkbase as u128));
    Ok(())
}

pub(super) fn execute_calculate_min_stake_bm(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKEBM"))?;
    fetch_stack(engine, 2)?;
    let tstk = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)?; //time from network start 
    let mbkav = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)?; //sum of reward token without slash tokens
    let one_minus_fstk_q32 = calc_one_minus_fstk_q32_int(tstk);
    let sbkmin = ((mbkav as u128 * one_minus_fstk_q32 as u128) >> 32) as u128;
    engine.cc.stack.push(int!(sbkmin as u128));
    Ok(())
}

pub(super) fn execute_mint_shell(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("MINTSHELL"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_MINT_SHELL_TOKEN, None, cell)
}

fn to_umbnlst(weights: &Vec<u64>) -> Vec<u64> {
    let wsum: u128 = weights.iter().map(|&w| w as u128).sum();
    const M: u128 = 1u128 << 32;

    let mut acc: u128 = 0;
    let mut cur: u128 = 0;
    let mut out: Vec<u64> = Vec::with_capacity(weights.len() + 1);
    out.push(0);

    for &w in weights {
        if wsum == 0 {
            out.push(0);
            continue;
        }
        acc += M * (w as u128);
        let ticks = acc / wsum;
        acc -= ticks * wsum;
        cur += ticks;
        out.push(cur as u64);
    }
    out
}

fn build_bclst(umbnlst: &Vec<u64>) -> Vec<u64> {
    let len = umbnlst.len();
    let mut bclst = Vec::new();

    if len < 2 {
        return bclst;
    }

    for i in 0..(len - 1) {
        let dl = umbnlst[i] as i128;
        let dr = umbnlst[i + 1] as i128;
        let bc_i = boost_coef_fp(dl, dr);
        let mut bc: u64 = if bc_i <= 0 { 0 } else { bc_i as u64 };

        let width_i = dr - dl;
        if width_i <= 0 {
            bc = 0;
        } else {
            let width: u128 = width_i as u128;
            let one: u128 = 1u128 << 32;
            let num: u128 = (bc as u128) * one;
            bc = (num / width) as u64;
        }
        bclst.push(bc);
    }
    bclst
}

fn compute_rmv(rpc: i128, tap_num: i128, bclst: &Vec<u64>, mbi: u64, taplst: &Vec<u64>) -> i128 {
    let mut denom: i128 = 0;
    let len = bclst.len();
    let len_tap = taplst.len();
    if len == 0 {
        return 0;
    }
    if len_tap != len {
        return 0;
    }
    for j in 0..len {
        denom += taplst[j] as i128 * bclst[j] as i128;
    }

    if denom == 0 {
        return 0;
    }
    let new_mbi;
    if mbi >= len as u64 {
        new_mbi = len as u64 - 1;
    } else {
        new_mbi = mbi;
    }
    let numer = rpc * tap_num * bclst[new_mbi as usize] as i128;
    let rmv = numer / denom;
    rmv
}

fn params_from_types(types: Vec<ParamType>) -> Vec<Param> {
    let param_names = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    types
        .into_iter()
        .zip(param_names)
        .map(|(kind, name)| Param { name: name.to_owned(), kind })
        .collect()
}

pub(super) fn execute_calculate_mobile_verifiers_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCMVREWARD"))?;
    fetch_stack(engine, 5)?;
    let _rpc = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as u64;
    let _tap_num = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as u64;
    engine.cc.stack.push(int!(0 as u128));
    return Ok(());
    // let tap_lst_cell = engine.cmd.var(2).as_cell()?;
    // let tap_lst_slice =
    // SliceData::load_cell(tap_lst_cell.clone()).map_err(|e| {
    // exception!(ExceptionCode::CellUnpackError, "Failed to load cell tap:
    // {:?}", e) })?;
    // let params =
    // params_from_types(vec![ParamType::Array(Box::new(ParamType::Uint(64)))]);
    // let tokens = TokenValue::decode_params(&params, tap_lst_slice,
    // &ABI_VERSION_2_2, false) .map_err(|e| {
    // exception!(ExceptionCode::CellUnpackError, "Failed to decode tap_lst
    // array: {:?}", e) })?;
    //
    // let tap_lst = if let Some(token) = tokens.first() {
    // if let TokenValue::Array(_, items) = &token.value {
    // items
    // .iter()
    // .map(|item| {
    // if let TokenValue::Uint(uint) = item {
    // let bytes = uint.number.to_bytes_le();
    // if bytes.len() > 8 {
    // Err(exception!(
    // ExceptionCode::CellUnpackError,
    // "Value too large for u64: {}",
    // uint.number
    // ))
    // } else {
    // let mut array = [0u8; 8];
    // array[..bytes.len()].copy_from_slice(&bytes);
    // Ok(u64::from_le_bytes(array))
    // }
    // } else {
    // Err(exception!(
    // ExceptionCode::CellUnpackError,
    // "Expected Uint in array, got {:?}",
    // item
    // ))
    // }
    // })
    // .collect::<Result<Vec<u64>, _>>()?
    // } else {
    // return Err(exception!(
    // ExceptionCode::CellUnpackError,
    // "Expected array token, got {:?}",
    // token.value
    // ));
    // }
    // } else {
    // return Err(exception!(ExceptionCode::CellUnpackError, "No token found
    // after decoding")); };
    //
    // let mbn_lst_cell = engine.cmd.var(3).as_cell()?;
    // let mbn_lst_slice =
    // SliceData::load_cell(mbn_lst_cell.clone()).map_err(|e| {
    // exception!(ExceptionCode::CellUnpackError, "Failed to load cell mbn:
    // {:?}", e) })?;
    //
    // let tokens = TokenValue::decode_params(&params, mbn_lst_slice,
    // &ABI_VERSION_2_2, false) .map_err(|e| {
    // exception!(ExceptionCode::CellUnpackError, "Failed to decode mbn_lst
    // array: {:?}", e) })?;
    // let mbn_lst;
    // if engine_version <= "1.0.0".parse().unwrap() {
    // mbn_lst = if let Some(token) = tokens.first() {
    // if let TokenValue::Array(_, items) = &token.value {
    // items
    // .iter()
    // .map(|item| {
    // if let TokenValue::Uint(uint) = item {
    // let bytes = uint.number.to_bytes_le();
    // if bytes.len() > 8 {
    // Err(exception!(
    // ExceptionCode::CellUnpackError,
    // "Value too large for u64: {}",
    // uint.number
    // ))
    // } else {
    // let mut array = [0u8; 8];
    // array[..bytes.len()].copy_from_slice(&bytes);
    // Ok(u64::from_le_bytes(array))
    // }
    // } else {
    // Err(exception!(
    // ExceptionCode::CellUnpackError,
    // "Expected Uint in array, got {:?}",
    // item
    // ))
    // }
    // })
    // .collect::<Result<Vec<u64>, _>>()?
    // } else {
    // return Err(exception!(
    // ExceptionCode::CellUnpackError,
    // "Expected array token, got {:?}",
    // token.value
    // ));
    // }
    // } else {
    // return Err(exception!(
    // ExceptionCode::CellUnpackError,
    // "No token found after decoding"
    // ));
    // }
    // } else {
    // mbn_lst = engine.get_mv_config().mbn_lst_global;
    // }
    // let mbi = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as u64;
    // log::trace!(target: "executor", "mbn {:?}", mbn_lst.clone());
    // log::trace!(target: "executor", "tap {:?}", tap_lst.clone());
    //
    // let bclst = if engine_version <= "1.0.0".parse().unwrap() {
    // build_bclst_old(&to_umbnlst(&mbn_lst))
    // } else {
    // build_bclst(&to_umbnlst(&mbn_lst))
    // };
    // log::trace!(target: "executor", "bclst {:?}", bclst.clone());
    // log::trace!(target: "executor", "rpc {:?}", rpc.clone());
    // let rmv = compute_rmv(rpc as i128, tap_num as i128, &bclst, mbi,
    // &tap_lst); log::trace!(target: "executor", "rmv {:?}", rmv.clone());
    // engine.cc.stack.push(int!(rmv as u128));
    // Ok(())
}

pub(super) fn execute_mint_shellq(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("MINTSHELLQ"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_MINT_SHELLQ_TOKEN, None, cell)
}

pub(super) fn execute_send_to_dapp_config(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("SENDTODAPPCONFIG"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_SEND_TO_DAPP_CONFIG, None, cell)
}

pub(super) fn execute_get_available_balance(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("GETAVAILABLEBALANCE"))?;
    let mut balance = engine.get_available_credit();
    if balance < 0 {
        balance = 0;
    }
    engine.cc.stack.push(int!(balance as u128));
    Ok(())
}

pub(super) fn execute_my_dapp_id(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("MYDAPPID"))?;
    let dapp_id = match engine.self_dapp_id.as_ref() {
        Some(dapp_id) => dapp_id.clone(),
        None => err!(ExceptionCode::DAppIdNotSet)?,
    };
    engine.cc.stack.push(StackItem::Integer(Arc::new(IntegerData::from_unsigned_bytes_be(
        dapp_id.as_byte_slice(),
    ))));
    Ok(())
}

fn horner_q32<const N: usize>(f_q32: i64, coeffs: &[i64; N]) -> i64 {
    let mut acc = coeffs[N - 1];
    for &c in coeffs[..N - 1].iter().rev() {
        acc = (((acc as i128 * f_q32 as i128) >> 32) as i64) + c;
    }
    acc
}

fn rep_coef_pow2_horner_q32(f: i64) -> i64 {
    let mut acc = RC_POW2_COEFF[5];
    for &c in RC_POW2_COEFF[..5].iter().rev() {
        acc = (((acc as i128 * f as i128) >> 32) as i64) + c;
    }
    acc
}

fn rep_coef_exp_q32(x_q32: i64) -> i64 {
    let y_q64 = x_q32 as i128 * RC_K1_Q32 as i128;
    let y_q32 = (y_q64 >> 32) as i64;
    let i = (y_q32 >> 32) as i32;
    let f = y_q32 & (RC_ONE_Q32 - 1);
    let pow2_i_q32 = RC_ONE_Q32 >> (-i as u32);
    let pow2_f_q32 = rep_coef_pow2_horner_q32(f);
    ((pow2_i_q32 as i128 * pow2_f_q32 as i128) >> 32) as i64
}

fn repcoef_int(bkrt: u128) -> u128 {
    if bkrt == 0 {
        return 1_000_000_000;
    }
    if bkrt >= MAXRT {
        return 3_000_000_000;
    }
    let x_q32 = -(RC_K2_Q32 * bkrt as i64);
    let diff_q32 = RC_ONE_Q32 - rep_coef_exp_q32(x_q32);
    let rep_q32 = RC_ONE_Q32 + (((RC_K3_Q32 as i128 * diff_q32 as i128) >> 32) as i64);
    ((rep_q32 as i128 * RCSCALE) >> 32) as u128
}

fn mul_fp_128(a: i128, b: i128) -> i128 {
    (a * b) >> MV_FRAC_BITS
}
fn div_fp_128(a: i128, b: i128) -> i128 {
    (a << MV_FRAC_BITS) / b
}
fn add_fp_128(a: i128, b: i128) -> i128 {
    a + b
}

fn exp_fp(x_fp: i128) -> i128 {
    let n = (x_fp >> MV_FRAC_BITS) as usize;
    let f = x_fp & (ONE - 1);
    let en = E_INT[n];
    let mut term = ONE;
    let mut sum = ONE;
    for k in 1..=7 {
        term = mul_fp_128(term, f); // f^k
        term = div_fp_128(term, (k as i128) * ONE); // f^k / k!
        sum = add_fp_128(sum, term);
    }
    mul_fp_128(en, sum)
}

fn bc_integral_fp(bl: i128, br: i128, xl: i128, xr: i128, yd: i128, yu: i128, k: i128) -> i128 {
    let dx = xr - xl;
    let k_div = div_fp_128(k, dx);
    let expk = exp_fp(k);
    let term1 = mul_fp_128(
        mul_fp_128(yu - yd, dx),
        add_fp_128(exp_fp(mul_fp_128(k_div, br - xl)), -exp_fp(mul_fp_128(k_div, bl - xl))),
    );
    let term2 = mul_fp_128(mul_fp_128(k, br - bl), add_fp_128(mul_fp_128(yd, expk), -yu));
    div_fp_128(add_fp_128(term1, term2), mul_fp_128(k, add_fp_128(expk, -ONE)))
}

fn boost_coef_fp(dl: i128, dr: i128) -> i128 {
    let mut bc = 0i128;
    if MV_X1 <= dl && dl <= MV_X2 {
        if MV_X1 <= dr && dr <= MV_X2 {
            bc = bc_integral_fp(dl, dr, MV_X1, MV_X2, MV_Y1, MV_Y2, MV_K1);
        } else if MV_X2 < dr && dr <= MV_X3 {
            bc = bc_integral_fp(dl, MV_X2, MV_X1, MV_X2, MV_Y1, MV_Y2, MV_K1)
                + bc_integral_fp(MV_X2, dr, MV_X2, MV_X3, MV_Y2, MV_Y3, MV_K2);
        } else if MV_X3 < dr && dr <= MV_X4 {
            bc = bc_integral_fp(dl, MV_X2, MV_X1, MV_X2, MV_Y1, MV_Y2, MV_K1)
                + bc_integral_fp(MV_X2, MV_X3, MV_X2, MV_X3, MV_Y2, MV_Y3, MV_K2)
                + bc_integral_fp(MV_X3, dr, MV_X3, MV_X4, MV_Y3, MV_Y4, MV_K3);
        }
    } else if MV_X2 < dl && dl <= MV_X3 {
        if MV_X2 < dr && dr <= MV_X3 {
            bc = bc_integral_fp(dl, dr, MV_X2, MV_X3, MV_Y2, MV_Y3, MV_K2);
        } else if MV_X3 < dr && dr <= MV_X4 {
            bc = bc_integral_fp(dl, MV_X3, MV_X2, MV_X3, MV_Y2, MV_Y3, MV_K2)
                + bc_integral_fp(MV_X3, dr, MV_X3, MV_X4, MV_Y3, MV_Y4, MV_K3);
        }
    } else if MV_X3 < dl && dl <= MV_X4 && MV_X3 < dr && dr <= MV_X4 {
        bc = bc_integral_fp(dl, dr, MV_X3, MV_X4, MV_Y3, MV_Y4, MV_K3);
    }
    bc
}

const SHIFT_Q40: u32 = 40;
const SHIFT_Q80: u32 = 80;

const ONE_Q40: i128 = 1i128 << SHIFT_Q40; // 2^40
const HALF_Q80: i128 = 1i128 << (SHIFT_Q80 - 1); // 2^79

const LN2_Q40: i128 = 762_123_384_785; // floor(ln(2) * 2^40)
const INV_LN2_Q40: i128 = 1_586_259_972_792; // floor(1/ln(2) * 2^40)

const T_B: i64 = 6000;
const K_B: u64 = 100;
const L: i128 = 200;

// k = 0.00054687998269748
// K_Q40 = floor(k * 2^40)
const K_Q40: i128 = 601_300_899;

// Polynomial coefficients for exp(r) on r in [-0.35, 0.35] in Q40
const C1_2_Q40: i128 = ONE_Q40 / 2; //  1/2
const C1_6_Q40: i128 = ONE_Q40 / 6; //  1/6
const C1_24_Q40: i128 = ONE_Q40 / 24; //  1/24
const C1_120_Q40: i128 = ONE_Q40 / 120; //  1/120

// Coefficients for ln series: ln(m) via z = (m-1)/(m+1)
const C1_3_Q40: i128 = ONE_Q40 / 3; // 1/3
const C1_5_Q40: i128 = ONE_Q40 / 5; // 1/5

// exp(x_real) where x_real = x_q40 / 2^40
// Accurate for |x_real| <= 10
fn fixed_exp_q40(mut x_q40: i128) -> i128 {
    // Clamp the argument range to [-10, 10] in real units
    let max_x_q40: i128 = 10i128 << SHIFT_Q40;
    if x_q40 > max_x_q40 {
        x_q40 = max_x_q40;
    } else if x_q40 < -max_x_q40 {
        x_q40 = -max_x_q40;
    }
    // x = n * ln(2) + r,  r in [-ln(2)/2, ln(2)/2]
    // n = round(x / ln(2))
    //
    // x_q40 and LN2_Q40 are Q40:
    // z_q80 = x_q40 * INV_LN2_Q40 is Q80
    // n = round(z_q80 / 2^40)
    let z_q80: i128 = x_q40 * INV_LN2_Q40; // Q40 * Q40 = Q80
    let n: i32 = ((z_q80 + HALF_Q80) >> SHIFT_Q80) as i32;
    let r_q40: i128 = x_q40 - (n as i128) * LN2_Q40; // r in Q40
    // exp(r) ≈ 1 + r + r^2/2 + r^3/6 + r^4/24 + r^5/120 (Horner)
    let mut t_q40: i128 = C1_120_Q40;
    t_q40 = C1_24_Q40 + ((r_q40 * t_q40) >> SHIFT_Q40);
    t_q40 = C1_6_Q40 + ((r_q40 * t_q40) >> SHIFT_Q40);
    t_q40 = C1_2_Q40 + ((r_q40 * t_q40) >> SHIFT_Q40);
    t_q40 = ONE_Q40 + ((r_q40 * t_q40) >> SHIFT_Q40);
    let mut res_q40: i128 = ONE_Q40 + ((r_q40 * t_q40) >> SHIFT_Q40); // Q40
    // exp(x) = exp(r) * 2^n
    if n > 0 {
        let sh: u32 = n as u32;
        res_q40 <<= sh;
    } else if n < 0 {
        let sh: u32 = (-n) as u32;
        res_q40 >>= sh;
    }
    res_q40
}

// Computes ln(u_real) where u_real = u_q40 / 2^40, u_real > 0
// u = m * 2^n, m in [0.5, 1.5)
// ln(u) = n * ln(2) + ln(m), and ln(m) is approximated via
// z = (m-1)/(m+1)
// ln(m) = 2 * ( z + z^3/3 + z^5/5 )
fn fixed_ln_q40(mut u_q40: i128) -> i128 {
    if u_q40 <= 0 {
        return 0;
    }
    // Normalize u_q40 to m_q40 in [0.5, 1.5) in real units
    // m_q40 in [0.5*2^40, 1.5*2^40)
    let lower_q40: i128 = ONE_Q40 >> 1; // 0.5 * 2^40
    let upper_q40: i128 = ONE_Q40 + (ONE_Q40 >> 1); // 1.5 * 2^40
    let mut n: i32 = 0;
    // Bring u_q40 into [0.5, 1.5)
    while u_q40 < lower_q40 {
        u_q40 <<= 1;
        n -= 1;
    }
    while u_q40 >= upper_q40 {
        u_q40 >>= 1;
        n += 1;
    }
    // u_q40 represents m in [0.5, 1.5) in Q40
    // Compute z_q40 = (m - 1) / (m + 1) in Q40
    let num_q40: i128 = u_q40 - ONE_Q40; // (m - 1) in Q40
    let denom_q40: i128 = u_q40 + ONE_Q40; // (m + 1) in Q40
    let z_q40: i128 = (num_q40 << SHIFT_Q40) / denom_q40; // Q40
    // Powers of z: z^2, z^3, z^5 in Q40
    let z2_q40: i128 = (z_q40 * z_q40) >> SHIFT_Q40; // z^2
    let z3_q40: i128 = (z2_q40 * z_q40) >> SHIFT_Q40; // z^3
    let z5_q40: i128 = (z3_q40 * z2_q40) >> SHIFT_Q40; // z^5
    // ln(m) = 2 * ( z + z^3/3 + z^5/5 )
    let term1_q40: i128 = z_q40; // z
    let term2_q40: i128 = (z3_q40 * C1_3_Q40) >> SHIFT_Q40; // z^3 / 3
    let term3_q40: i128 = (z5_q40 * C1_5_Q40) >> SHIFT_Q40; // z^5 / 5
    let series_q40: i128 = term1_q40 + term2_q40 + term3_q40; // Q40
    let ln_m_q40: i128 = series_q40 << 1; // *2
    // ln(u) = n * ln(2) + ln(m)
    let ln_u_q40: i128 = (n as i128) * LN2_Q40 + ln_m_q40; // Q40
    ln_u_q40
}

// Integral of f(x) on [a, b) in Q40 (without dividing by K_B)
//
// f(x) = L / (1 + exp(k * (x - T_B)))
//
// Integral_a^b f(x) dx = L * [ (b - a)
//   - (1/k) * ( ln(1 + e^{k(b-T_B)}) - ln(1 + e^{k(a-T_B)}) ) ]
fn calc_interval_integral_q40(a_u: u64, b_u: u64) -> i128 {
    if b_u <= a_u {
        return 0;
    }
    let a: i64 = a_u as i64;
    let b: i64 = b_u as i64;
    let dx_i64: i64 = b - a;
    let dx: i128 = dx_i64 as i128;
    // (b - a) in Q40
    let dx_q40: i128 = dx << SHIFT_Q40; // Q40
    // a0_q40 = k * (a - T_B), a1_q40 = k * (b - T_B) in Q40
    let diff_a: i128 = (a as i128) - (T_B as i128);
    let diff_b: i128 = (b as i128) - (T_B as i128);
    let a0_q40: i128 = K_Q40 * diff_a; // Q40
    let a1_q40: i128 = K_Q40 * diff_b; // Q40
    // exp(k * (x - T_B)) in Q40
    let e0_q40: i128 = fixed_exp_q40(a0_q40); // Q40
    let e1_q40: i128 = fixed_exp_q40(a1_q40); // Q40
    // u_q40 = 1 + exp(...)
    let u0_q40: i128 = ONE_Q40 + e0_q40; // Q40
    let u1_q40: i128 = ONE_Q40 + e1_q40; // Q40
    // ln(1 + exp(...)) in Q40
    let ln0_q40: i128 = fixed_ln_q40(u0_q40); // Q40
    let ln1_q40: i128 = fixed_ln_q40(u1_q40); // Q40
    // dln in Q40
    let dln_q40: i128 = ln1_q40 - ln0_q40; // Q40
    // W_q40 = (1/k) * dln in Q40
    // (dln / k) = (dln_q40 / 2^40) / (K_Q40 / 2^40) = dln_q40 / K_Q40
    let w_q40: i128 = (dln_q40 << SHIFT_Q40) / K_Q40; // Q40
    // bracket_q40 = (b - a) - (1/k)*dln, in Q40
    let bracket_q40: i128 = dx_q40 - w_q40; // Q40
    if bracket_q40 <= 0 {
        return 0;
    }
    // Integral in Q40 (without / K_B)
    let integral_q40: i128 = L * bracket_q40; // Q40
    integral_q40
}

fn calc_tap_coef_with_params(
    total_tap_num: u64,
    tap_num: u64,
    mining_dur: u64,
    modified_tap_rem_q40: u64,
    total_mining_dur_5min: u64,
    total_modified_tap_num_5min: u64,
    total_tap_num_5min: u64,
) -> (u64, u64, u64, u64, u64, u64) {
    let denom_q40_i: i128 = (K_B as i128) * ONE_Q40;
    let modified_tap_rem_q40: u64 = modified_tap_rem_q40 % (denom_q40_i as u64);
    if total_tap_num >= 12_000
        || total_tap_num_5min >= 70
        || total_mining_dur_5min >= 330
        || total_modified_tap_num_5min >= 200
    {
        return (
            0,
            modified_tap_rem_q40,
            total_mining_dur_5min,
            total_modified_tap_num_5min,
            total_tap_num_5min,
            total_tap_num,
        );
    }
    let a_u: u64 = total_tap_num;
    // max_creditable = 12_000 - a_u  (a_u <= 12_000 here)
    let max_creditable: u64 = 12_000 - a_u;
    let remaining_5min: u64 = 70 - total_tap_num_5min;
    let mut used_taps: u64 = tap_num;
    if used_taps > max_creditable {
        used_taps = max_creditable;
    }
    if used_taps > remaining_5min {
        used_taps = remaining_5min;
    }
    if used_taps == 0 {
        return (
            0,
            modified_tap_rem_q40,
            total_mining_dur_5min,
            total_modified_tap_num_5min,
            total_tap_num_5min,
            total_tap_num,
        );
    }
    if mining_dur == 0 {
        return (
            0,
            modified_tap_rem_q40,
            total_mining_dur_5min,
            total_modified_tap_num_5min,
            total_tap_num_5min,
            total_tap_num,
        );
    }
    // Integration interval [a, b), with b = a + used_taps <= 12_000
    let b_u: u64 = a_u + used_taps;
    // Integral of f(x) on [a, b) in Q40 (without dividing by K_B)
    let integral_q40: i128 = calc_interval_integral_q40(a_u, b_u);
    if integral_q40 <= 0 {
        return (
            0,
            modified_tap_rem_q40,
            total_mining_dur_5min,
            total_modified_tap_num_5min,
            total_tap_num_5min,
            total_tap_num,
        );
    }
    let sum_q40: i128 = (modified_tap_rem_q40 as i128) + integral_q40;
    let modified_taps_q: i128 = sum_q40 / denom_q40_i;
    let rem_q40_i: i128 = sum_q40 % denom_q40_i;
    let new_modified_tap_rem_q40: u64 = rem_q40_i as u64;

    let modified_taps: u64 = if modified_taps_q <= 0 { 0 } else { modified_taps_q as u64 };

    let remaining_dur: u64 = 330 - total_mining_dur_5min;
    let effective_mining_dur: u64 =
        if mining_dur > remaining_dur { remaining_dur } else { mining_dur };

    let tap_coef: u64 = modified_taps * total_mining_dur_5min
        + effective_mining_dur * total_modified_tap_num_5min
        + modified_taps * effective_mining_dur;

    let new_total_modified_tap_num_5min: u64 = total_modified_tap_num_5min + modified_taps;
    let new_total_mining_dur_5min: u64 = total_mining_dur_5min + effective_mining_dur;
    let new_total_tap_num_5min: u64 = total_tap_num_5min + used_taps;
    let new_total_tap_num: u64 = total_tap_num + used_taps;
    (
        tap_coef,
        new_modified_tap_rem_q40,
        new_total_mining_dur_5min,
        new_total_modified_tap_num_5min,
        new_total_tap_num_5min,
        new_total_tap_num,
    )
}

pub(super) fn execute_calculate_miner_tap_coef(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCMINERTAPCOEF"))?;
    fetch_stack(engine, 7)?;
    let total_tap_num_5min = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as u64;
    let total_modified_tap_num_5min = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as u64;
    let total_mining_dur_5min = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as u64;
    let modified_tap_rem_q40 = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as u64;
    let mining_dur: u64 = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as u64;
    let tap_num = engine.cmd.var(5).as_integer()?.into(0..=u128::MAX)? as u64;
    let total_tap_num = engine.cmd.var(6).as_integer()?.into(0..=u128::MAX)? as u64;
    let (
        tap_coef,
        new_modified_tap_rem_q40,
        new_total_mining_dur_5min,
        new_total_modified_tap_num_5min,
        new_total_tap_num_5min,
        new_total_tap_num,
    ) = calc_tap_coef_with_params(
        total_tap_num,
        tap_num,
        mining_dur,
        modified_tap_rem_q40,
        total_mining_dur_5min,
        total_modified_tap_num_5min,
        total_tap_num_5min,
    );
    engine.cc.stack.push(int!(new_total_tap_num as u128));
    engine.cc.stack.push(int!(new_total_tap_num_5min as u128));
    engine.cc.stack.push(int!(new_total_modified_tap_num_5min as u128));
    engine.cc.stack.push(int!(new_total_mining_dur_5min as u128));
    engine.cc.stack.push(int!(new_modified_tap_rem_q40 as u128));
    engine.cc.stack.push(int!(tap_coef as u128));
    return Ok(());
}

pub(super) fn execute_calculate_miner_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCMINERREWARD"))?;
    fetch_stack(engine, 4)?;
    let rpc = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as u64;
    let tap_num = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as u64;
    let tap_lst_cell = engine.cmd.var(2).as_cell()?;
    let tap_lst_slice = SliceData::load_cell(tap_lst_cell.clone()).map_err(|e| {
        exception!(ExceptionCode::CellUnpackError, "Failed to load cell tap: {:?}", e)
    })?;
    let params = params_from_types(vec![ParamType::Array(Box::new(ParamType::Uint(64)))]);
    let tokens = TokenValue::decode_params(&params, tap_lst_slice, &ABI_VERSION_2_2, false)
        .map_err(|e| {
            exception!(ExceptionCode::CellUnpackError, "Failed to decode tap_lst array: {:?}", e)
        })?;

    let tap_lst = if let Some(token) = tokens.first() {
        if let TokenValue::Array(_, items) = &token.value {
            items
                .iter()
                .map(|item| {
                    if let TokenValue::Uint(uint) = item {
                        let bytes = uint.number.to_bytes_le();
                        if bytes.len() > 8 {
                            Err(exception!(
                                ExceptionCode::CellUnpackError,
                                "Value too large for u64: {}",
                                uint.number
                            ))
                        } else {
                            let mut array = [0u8; 8];
                            array[..bytes.len()].copy_from_slice(&bytes);
                            Ok(u64::from_le_bytes(array))
                        }
                    } else {
                        Err(exception!(
                            ExceptionCode::CellUnpackError,
                            "Expected Uint in array, got {:?}",
                            item
                        ))
                    }
                })
                .collect::<Result<Vec<u64>, _>>()?
        } else {
            return Err(exception!(
                ExceptionCode::CellUnpackError,
                "Expected array token, got {:?}",
                token.value
            ));
        }
    } else {
        return Err(exception!(ExceptionCode::CellUnpackError, "No token found after decoding"));
    };
    let mbn_lst = engine.get_mv_config().mbn_lst_global;
    let mbi = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as u64;
    log::trace!(target: "executor", "mbn {:?}", mbn_lst.clone());
    log::trace!(target: "executor", "tap {:?}", tap_lst.clone());

    let bclst = build_bclst(&to_umbnlst(&mbn_lst));
    log::trace!(target: "executor", "bclst {:?}", bclst.clone());
    log::trace!(target: "executor", "rpc {:?}", rpc.clone());
    let rmv = compute_rmv(rpc as i128, tap_num as i128, &bclst, mbi, &tap_lst);
    log::trace!(target: "executor", "rmv {:?}", rmv.clone());
    engine.cc.stack.push(int!(rmv as u128));
    Ok(())
}
