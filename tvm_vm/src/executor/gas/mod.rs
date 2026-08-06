// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_block::GlobalCapabilities;
use tvm_types::Result;
use tvm_types::error;
use tvm_types::types::ExceptionCode;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::integer::behavior::Quiet;
use crate::stack::integer::conversion::FromInt;
use crate::stack::integer::math::Round;
use crate::types::Exception;
use crate::types::Status;

pub mod gas_state;

fn gramtogas(engine: &Engine, nanograms: &IntegerData) -> Result<i64> {
    let gas_price = IntegerData::from_i64(engine.get_gas().get_gas_price());
    let gas = nanograms.div::<Quiet>(&gas_price, Round::FloorToZero)?.0;
    let ret = gas.take_value_of(|x| i64::from_int(x).ok()).unwrap_or(i64::MAX);
    Ok(ret)
}
fn setgaslimit(engine: &mut Engine, gas_limit: i64) -> Status {
    if gas_limit < engine.gas_used() {
        return err!(ExceptionCode::OutOfGas);
    }
    engine.new_gas_limit(gas_limit);
    Ok(())
}

// Application-specific primitives - A.10; Gas-related primitives - A.10.2
// ACCEPT - F800
pub fn execute_accept(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("ACCEPT"))?;
    engine.new_gas_limit(i64::MAX);
    Ok(())
}
// Application-specific primitives - A.11; Gas-related primitives - A.11.2
// SETGASLIMIT - F801
pub fn execute_setgaslimit(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("SETGASLIMIT"))?;
    fetch_stack(engine, 1)?;
    let gas_limit = engine.cmd.var(0).as_integer()?.take_value_of(|x| i64::from_int(x).ok())?;
    setgaslimit(engine, gas_limit)
}
// Application-specific primitives - A.11; Gas-related primitives - A.11.2
// BUYGAS - F802
pub fn execute_buygas(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("BUYGAS"))?;
    fetch_stack(engine, 1)?;
    let nanograms = engine.cmd.var(0).as_integer()?;
    let gas_limit = gramtogas(engine, nanograms)?;
    setgaslimit(engine, gas_limit)
}
// Application-specific primitives - A.11; Gas-related primitives - A.11.2
// GRAMTOGAS - F804
pub fn execute_gramtogas(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("GRAMTOGAS"))?;
    fetch_stack(engine, 1)?;
    let nanograms_input = engine.cmd.var(0);
    let gas = if nanograms_input.as_integer()?.is_neg() {
        0
    } else {
        let nanograms = nanograms_input.as_integer()?;
        gramtogas(engine, nanograms)?
    };
    engine.cc.stack.push(int!(gas));
    Ok(())
}
// Application-specific primitives - A.10; Gas-related primitives - A.10.2
// GASTOGRAM - F805
pub fn execute_gastogram(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("GASTOGRAM"))?;
    fetch_stack(engine, 1)?;
    let gas = engine.cmd.var(0).as_integer()?;
    let gas_price = engine.get_gas().get_gas_price();
    let nanogram_output = gas.mul::<Quiet>(&IntegerData::from_i64(gas_price))?;
    engine.cc.stack.push(StackItem::int(nanogram_output));
    Ok(())
}

// Application-specific primitives - A.11; Gas-related primitives - A.11.2
// COMMIT - F80F
pub fn execute_commit(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("COMMIT"))?;
    engine.commit();
    Ok(())
}

pub fn execute_gas_remaining(engine: &mut Engine) -> Status {
    engine.check_capability(GlobalCapabilities::CapsTvmBugfixes2022)?;
    engine.load_instruction(Instruction::new("GASREMAINING"))?;
    engine.cc.stack.push(StackItem::int(engine.gas_remaining()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use tvm_block::GlobalCapabilities;
    use tvm_types::BuilderData;
    use tvm_types::ExceptionCode;
    use tvm_types::SliceData;

    use super::*;
    use crate::error::tvm_exception_code;
    use crate::executor::gas::gas_state::Gas;
    use crate::stack::Stack;

    fn engine_with_stack(capabilities: u64, stack: Option<Stack>) -> Engine {
        Engine::with_capabilities(capabilities).setup(SliceData::default(), None, stack, None)
    }

    #[test]
    fn accept_and_commit_update_gas_and_committed_state() {
        let mut engine = engine_with_stack(0, None);
        engine.set_gas(Gas::new(20, 5, 100, 10));

        execute_accept(&mut engine).unwrap();

        assert_eq!(engine.get_gas().get_gas_limit(), 100);
        assert_eq!(engine.gas_remaining(), 90);

        let root = BuilderData::with_raw(vec![0xaa], 8).unwrap().into_cell().unwrap();
        let actions = BuilderData::with_raw(vec![0xbb], 8).unwrap().into_cell().unwrap();
        *engine.ctrl_mut(4).unwrap() = StackItem::cell(root.clone());
        *engine.ctrl_mut(5).unwrap() = StackItem::cell(actions.clone());

        execute_commit(&mut engine).unwrap();

        let committed = engine.get_committed_state();
        assert!(committed.is_committed());
        assert_eq!(committed.get_root(), &StackItem::cell(root));
        assert_eq!(committed.get_actions(), &StackItem::cell(actions));
    }

    #[test]
    fn setgaslimit_and_buygas_cover_success_and_out_of_gas_paths() {
        let mut stack = Stack::new();
        stack.push(StackItem::int(50));
        let mut engine = engine_with_stack(0, Some(stack));
        engine.set_gas(Gas::test());
        engine.use_gas(30);

        execute_setgaslimit(&mut engine).unwrap();

        assert_eq!(engine.get_gas().get_gas_limit(), 50);
        assert_eq!(engine.gas_remaining(), 10);

        let mut stack = Stack::new();
        stack.push(StackItem::int(20));
        let mut engine = engine_with_stack(0, Some(stack));
        engine.set_gas(Gas::test());
        engine.use_gas(30);
        let err = execute_setgaslimit(&mut engine).unwrap_err();
        assert_eq!(tvm_exception_code(&err), Some(ExceptionCode::OutOfGas));

        let mut stack = Stack::new();
        stack.push(StackItem::int(500));
        let mut engine = engine_with_stack(0, Some(stack));
        engine.set_gas(Gas::new(0, 50, 500, 10));

        execute_buygas(&mut engine).unwrap();

        assert_eq!(engine.get_gas().get_gas_limit(), 50);
        assert_eq!(engine.gas_remaining(), 40);
    }

    #[test]
    fn gramtogas_gastogram_and_gas_remaining_cover_conversion_paths() {
        let mut stack = Stack::new();
        stack.push(StackItem::int(-1));
        let mut engine = engine_with_stack(0, Some(stack));
        engine.set_gas(Gas::new(0, 0, 500, 10));

        execute_gramtogas(&mut engine).unwrap();

        assert_eq!(engine.stack().get(0), &StackItem::int(0));

        let mut stack = Stack::new();
        stack.push(StackItem::int(95));
        let mut engine = engine_with_stack(0, Some(stack));
        engine.set_gas(Gas::new(0, 0, 500, 10));

        execute_gramtogas(&mut engine).unwrap();

        assert_eq!(engine.stack().get(0), &StackItem::int(9));

        let mut stack = Stack::new();
        stack.push(StackItem::int(7));
        let mut engine = engine_with_stack(0, Some(stack));
        engine.set_gas(Gas::new(0, 0, 500, 10));

        execute_gastogram(&mut engine).unwrap();

        assert_eq!(engine.stack().get(0), &StackItem::int(70));

        let mut engine = engine_with_stack(0, None);
        engine.set_gas(Gas::new(17, 0, 500, 10));
        let err = execute_gas_remaining(&mut engine).unwrap_err();
        assert_eq!(tvm_exception_code(&err), Some(ExceptionCode::InvalidOpcode));

        let mut engine = engine_with_stack(GlobalCapabilities::CapsTvmBugfixes2022 as u64, None);
        engine.set_gas(Gas::new(17, 0, 500, 10));
        execute_gas_remaining(&mut engine).unwrap();
        assert_eq!(engine.stack().get(0), &StackItem::int(7));
    }
}
