use std::u32;
use std::u64;

use tvm_block::ExtraCurrencyCollection;
use tvm_block::Serializable;
use tvm_block::VarUInteger32;
use tvm_block::ACTION_CNVRTSHELLQ;
use tvm_block::ACTION_MINTECC;
use tvm_types::BuilderData;

use crate::executor::blockchain::add_action;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::engine::Engine;
use crate::executor::types::Instruction;
use crate::stack::integer::IntegerData;
use crate::stack::StackItem;
use crate::types::Status;

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

pub(super) fn execute_calculate_validator_reward(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CALCBKREWARD"))?;
    fetch_stack(engine, 7)?;
    let vrt = engine.cmd.var(0).as_integer()?.into(0..=u128::MAX)? as f64;
    let maxrt = engine.cmd.var(1).as_integer()?.into(0..=u128::MAX)? as f64;
    let valstake = engine.cmd.var(2).as_integer()?.into(0..=u128::MAX)? as f64;
    let totalstake = engine.cmd.var(3).as_integer()?.into(0..=u128::MAX)? as f64;
    let t = engine.cmd.var(4).as_integer()?.into(0..=u128::MAX)? as f64;
    let rac = engine.cmd.var(5).as_integer()?.into(0..=u128::MAX)? as f64;
    let vpd = engine.cmd.var(6).as_integer()?.into(0..=u128::MAX)? as f64;
    let repcoef;
    if vrt < maxrt {
        repcoef = 2999_f64 / 999_f64
            - (-1_f64 * 0.000000043784704975090621653621901520_f64 * vrt + 0.69414768089352884_f64)
                .exp();
    } else {
        repcoef = 3_f64;
    }
    let u = 0.000000005756467732460114376710395313_f64;
    let bkrps = (-1.0_f64 * u * t + 4.0921398489254479849893923389_f64).exp() - rac;
    let cbkrpv = ((valstake / totalstake) * repcoef * bkrps * vpd * (10e9 as f64)) as u128;
    log::debug!(target: "executor", "Calculate reward vrt: {}, maxrt: {}, valstake: {}, totalstake: {}, t: {}, rac: {}, vpd: {}", vrt, maxrt, valstake, totalstake, t, rac, vpd);
    log::debug!(target: "executor", "Calculate reward repcoef: {}, u: {}, bkrps: {}, cbkrpv: {}", repcoef, u, bkrps, cbkrpv);
    engine.cc.stack.push(int!(cbkrpv));
    Ok(())
}
