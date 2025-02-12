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
pub const FFFC: f64 = 0.01_f64;
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
    let bked = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as f64;
    let active_bk = engine.cmd.var(5).as_integer()?.into(0..=u128::MAX)? as f64;
    let repcoef = if bkrt < MAXRT {
        MINRC
            + (MAXRC - MINRC) / (1_f64 - 1_f64 / ARFC)
                * (1_f64 - (-1_f64 * ARFC.ln() * bkrt / MAXRT).exp())
    } else {
        MAXRC
    };
    let u = -1_f64 / TTMT * (TMTAFC / (1_f64 + TMTAFC)).ln();
    let grps = TOTALSUPPLY * (1_f64 + TMTAFC) * ((-u * t).exp() - (-u * (t + 1_f64)).exp());
    let tbbkrps = BKSFC * grps;

    let bkrpve = if totalbkstake != 0_f64 {
        let bkrps = tbbkrps * repcoef * bkstake / totalbkstake;
        bkrps * bked * TOKEN_DECIMALS
    } else {
        tbbkrps * repcoef * bked * TOKEN_DECIMALS / active_bk
    };
    engine.cc.stack.push(int!(bkrpve as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_min_stake(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKE"))?;
    fetch_stack(engine, 4)?;
    let need_bk_num = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let bk_num = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let t = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let bkpd = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let mut base_min_val_stake = 0_f64;
    if t >= 3_f64 / 2_f64 * bkpd {
        let u_free_flt = -1_f64 / TTMT * (FFFC / (FFFC + 1_f64)).ln();
        let u = -1_f64 / TTMT * (TMTAFC / (TMTAFC + 1_f64)).ln();
        let tmta = TOTALSUPPLY * (1_f64 + TMTAFC) * (1_f64 - (-1_f64 * t * u).exp());
        let free_flt_frac =
            MAX_FREE_FLOAT_FRAC * (1_f64 + FFFC) * (1_f64 - (-1_f64 * u_free_flt * t).exp());
        let tsta = tmta * (1_f64 - free_flt_frac);
        base_min_val_stake = BKSFC * tsta / 2_f64 / need_bk_num * TOKEN_DECIMALS;
    }
    let min_val_stake =
        if bk_num > need_bk_num { base_min_val_stake as u128 } else { base_min_val_stake as u128 };
    engine.cc.stack.push(int!(min_val_stake));
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
