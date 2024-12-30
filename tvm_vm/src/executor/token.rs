use tvm_block::ACTION_CNVRTSHELLQ;
use tvm_block::ACTION_MINT_SHELL_TOKEN;
use tvm_block::ACTION_MINTECC;
use tvm_block::ExtraCurrencyCollection;
use tvm_block::Serializable;
use tvm_block::VarUInteger32;
use tvm_types::BuilderData;

use crate::executor::blockchain::add_action;
use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Status;

pub const ECC_NACKL_KEY: u32 = 1;
pub const ECC_SHELL_KEY: u32 = 2;
pub const INFINITY_CREDIT: i128 = -1;

pub const ARFC: f64 = 1000_f64;
pub const MINRC: f64 = 1_f64;
pub const MAXRC: f64 = 3_f64;
pub const TTMT: f64 = 2000000000_f64;
pub const TMTAFC: f64 = 0.00001_f64;
pub const TOTALSUPPLY: f64 = 10400000000_f64;
pub const TOKEN_DECIMALS: f64 = 1e9_f64;
pub const MAXRT: f64 = 157766400_f64;
pub const KF: f64 = 0.01_f64;
pub const KS: f64 = 0.001_f64;
pub const MAX_FREE_FLOAT_FRAC: f64 = 1_f64 / 3_f64;
pub const BKSFC: f64 = 0.675_f64;

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

pub(super) fn execute_exchange_shell(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CNVRTSHELLQ"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=u64::MAX)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_CNVRTSHELLQ, None, cell)
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_validator_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBKREWARD"))?;
    fetch_stack(engine, 6)?;
    let bkrt = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let bkstake = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let totalbkstake = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let t = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let mbk = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as f64;
    let active_bk = engine.cmd.var(5).as_integer()?.into(0..=u128::MAX)? as f64;
    let repcoef = if bkrt < MAXRT {
        MINRC
            + (MAXRC - MINRC) / (1_f64 - 1_f64 / ARFC)
                * (1_f64 - (-1_f64 * ARFC.ln() * bkrt / MAXRT).exp())
    } else {
        MAXRC
    };
    let reward;
    if mbk == 0_f64 {
        reward = 1_f64;
    } else
    if mbk < TOTALSUPPLY {
        reward = 1_f64;
    } else {
        reward = 0_f64;
    } 
    engine.cc.stack.push(int!(reward as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_min_stake(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKE"))?;
    fetch_stack(engine, 4)?;
    let nbkreq = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let nbk = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let tstk = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let mbkav = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let sbkbase;
    if mbkav != 0_f64 {
        let fstk;
        if tstk > TTMT {
            fstk = MAX_FREE_FLOAT_FRAC;
        } else {
            let uf = (-1_f64 / TTMT) * (KF / (1_f64 + KF)).ln();
            fstk = MAX_FREE_FLOAT_FRAC * (1_f64 + KF) * (1_f64 - (-1_f64 * tstk * uf).exp());
        }
        sbkbase = (mbkav * (1_f64 - fstk) / 2_f64) / nbkreq; 
    } else {
        sbkbase = 0_f64;
    }
    let sbkmin;        
    let us = -1_f64 * (KS / (KS + 1_f64)).ln() / nbkreq;
    if (nbk >= 0_f64) && (nbk <= nbkreq) {
        sbkmin = sbkbase * (1_f64 + KS) * (1_f64 - (-1_f64 * us * nbk).exp());
    } else {
        let unbk = 2_f64 * nbkreq - nbk;
        sbkmin = sbkbase * (2_f64 - (1_f64 + KS) * (1_f64 - (-1_f64 * us * unbk).exp()));
    }
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
