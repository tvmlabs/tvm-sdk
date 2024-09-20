use tvm_block::ExtraCurrencyCollection;
use tvm_block::Serializable;
use tvm_block::VarUInteger32;
use tvm_block::ACTION_CNVRTSHELLQ;
use tvm_block::ACTION_MINTECC;
use tvm_block::ACTION_MINT_SHELL_TOKEN;
use tvm_types::BuilderData;

use crate::executor::blockchain::add_action;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::engine::Engine;
use crate::executor::types::Instruction;
use crate::stack::integer::IntegerData;
use crate::stack::StackItem;
use crate::types::Status;

pub const ECC_NACKL_KEY: u32 = 1;
pub const ECC_SHELL_KEY: u32 = 2;
pub const INFINITY_CREDIT: i128 = -1;

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
    fetch_stack(engine, 7)?;
    let bkrt = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let maxrt = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let bkstake = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let totalbkstake = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let t = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as f64;
    let bked = engine.cmd.var(5).as_integer()?.into(0..=u128::MAX)? as f64;
    let active_bk = engine.cmd.var(6).as_integer()?.into(0..=u128::MAX)? as f64;
    let arfc = 1000_f64;
    let minrc = 1_f64;
    let maxrc = 3_f64;
    let ttmt = 2000000000_f64;
    let tmtafc = 00001_f64;
    let totalsupply = 10400000000_f64;
    let token_decimals = 1e9_f64;
    let repcoef = if bkrt < maxrt {
        minrc
            + (maxrc - minrc) / (1_f64 - 1_f64 / arfc)
                * (1_f64 - (-1_f64 * arfc.ln() * bkrt / maxrt).exp())
    } else {
        3_f64
    };
    let u = -1_f64 / ttmt * (tmtafc / (1_f64 + tmtafc)).ln();
    let grps = totalsupply * (1_f64 + tmtafc) * ((-u * t).exp() - (-u * (t + 1_f64)).exp());
    let tbbkrps = 0.675 * grps;
    let bkrpve;
    if totalbkstake != 0_f64 {
        let bkrps = tbbkrps * repcoef * bkstake / totalbkstake;
        bkrpve = bkrps * bked * token_decimals;
    } else {
        bkrpve = tbbkrps * repcoef * bked * token_decimals / active_bk;
    }
    engine.cc.stack.push(int!(bkrpve as u128));
    Ok(())
}

#[allow(clippy::excessive_precision)]
pub(super) fn execute_calculate_min_stake(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("CALCMINSTAKE"))?;
    fetch_stack(engine, 4)?;
    let need_val_num = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let val_num = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let t = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let vpd = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let mut base_min_val_stake = 0_f64;
    if t >= 3_f64 / 2_f64 * vpd {
        let u_free_flt_pr = -1_f64 / 2000000000_f64 * (0.01_f64 / (0.01_f64 + 1_f64)).ln();
        let u_tmta = -1_f64 / 2000000000_f64 * (0.00001_f64 / (0.00001_f64 + 1_f64)).ln();
        let tmta = 10400000000_f64 * (1_f64 + 0.00001_f64) * (1_f64 - (-1_f64 * t * u_tmta).exp());
        let free_flt_pr = (1_f64 + 0.01_f64) * (1_f64 - (-1_f64 * u_free_flt_pr * t).exp()) / 3_f64;
        base_min_val_stake =
            (0.75_f64 * tmta * (1_f64 - free_flt_pr) / 2_f64 / need_val_num) * 1e9_f64;
    }
    let min_val_stake = if val_num > need_val_num {
        base_min_val_stake as u128
    } else {
        base_min_val_stake as u128
    };
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
