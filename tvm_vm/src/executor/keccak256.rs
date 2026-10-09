// Copyright (C) 2019-2026 EverX. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tiny_keccak::Hasher;
use tiny_keccak::Keccak;
use tvm_types::SliceData;

use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::gas::gas_state::Gas;
use crate::executor::types::Instruction;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Status;
use crate::utils::unpack_data_from_cell;

pub const KECCAK256_GAS_PRICE: i64 = 1_000;

pub(crate) fn execute_keccak256(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("KECCAK256"))?;
    engine.try_use_gas(Gas::keccak256_price())?;
    fetch_stack(engine, 1)?;

    let cell = engine.cmd.var(0).as_cell()?.clone();
    let slice = SliceData::load_cell(cell)?;
    let data = unpack_data_from_cell(slice, engine)?;

    let mut digest = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&data);
    hasher.finalize(&mut digest);

    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(digest)));
    Ok(())
}
