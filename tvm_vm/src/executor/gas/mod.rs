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
