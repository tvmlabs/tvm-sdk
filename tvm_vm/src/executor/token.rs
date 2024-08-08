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
use crate::types::Status;

pub(super) fn execute_ecc_mint(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("MINTECC"))?;
    fetch_stack(engine, 2)?;
    let x: u32 = engine.cmd.var(0).as_integer()?.into(0..=255)?;
    let y: VarUInteger32 =
        VarUInteger32::from(engine.cmd.var(1).as_integer()?.into(0..=2147483647)?);
    let mut data = ExtraCurrencyCollection::new();
    data.set(&x, &y)?;
    let mut cell = BuilderData::new();
    data.write_to(&mut cell)?;
    add_action(engine, ACTION_MINTECC, None, cell)
}

pub(super) fn execute_exchange_shell(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("CNVRTSHELLQ"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=2147483647)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_CNVRTSHELLQ, None, cell)
}
