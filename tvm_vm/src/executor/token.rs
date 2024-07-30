use tvm_block::Serializable;
use tvm_block::ACTION_MINT_SHELL_TOKEN;
use tvm_types::BuilderData;

use crate::executor::blockchain::add_action;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::engine::Engine;
use crate::executor::types::Instruction;
use crate::types::Status;

pub(super) fn execute_mint_shell(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("MINTSHELL"))?;
    fetch_stack(engine, 1)?;
    let x: u64 = engine.cmd.var(0).as_integer()?.into(0..=2147483647)?;
    let mut cell = BuilderData::new();
    x.write_to(&mut cell)?;
    add_action(engine, ACTION_MINT_SHELL_TOKEN, None, cell)
}
