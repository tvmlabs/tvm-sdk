// Copyright (C) 2019-2022 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_types::Sha256;
use tvm_types::sha512_digest;

use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::integer::behavior::Signaling;
use crate::stack::integer::serialization::Encoding;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::stack::serialization::Deserializer;
use crate::types::Status;

// (x - )
pub(crate) fn execute_addrand(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("ADDRAND"))?;
    fetch_stack(engine, 1)?;
    let mut hasher = Sha256::new();
    hasher.update(engine.rand()?.as_builder::<UnsignedIntegerBigEndianEncoding>(256)?.data());
    hasher.update(
        engine.cmd.var(0).as_integer()?.as_builder::<UnsignedIntegerBigEndianEncoding>(256)?.data(),
    );
    let sha256 = hasher.finalize();
    engine.set_rand(UnsignedIntegerBigEndianEncoding::new(256).deserialize(&sha256))?;
    Ok(())
}

// (y - z)
pub(crate) fn execute_rand(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("RAND"))?;
    fetch_stack(engine, 1)?;
    let sha512 =
        sha512_digest(engine.rand()?.as_builder::<UnsignedIntegerBigEndianEncoding>(256)?.data());
    let value = IntegerData::from_unsigned_bytes_be(&sha512[32..]);
    let rand = value.mul_shr256::<Signaling>(engine.cmd.var(0).as_integer()?)?;
    engine.cc.stack.push(StackItem::integer(rand));
    engine.set_rand(UnsignedIntegerBigEndianEncoding::new(256).deserialize(&sha512[..32]))?;
    Ok(())
}

// ( - x)
pub(crate) fn execute_randu256(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(Instruction::new("RANDU256"))?;
    let sha512 =
        sha512_digest(engine.rand()?.as_builder::<UnsignedIntegerBigEndianEncoding>(256)?.data());
    engine.set_rand(UnsignedIntegerBigEndianEncoding::new(256).deserialize(&sha512[..32]))?;
    engine.cc.stack.push(StackItem::int(
        UnsignedIntegerBigEndianEncoding::new(256).deserialize(&sha512[32..]),
    ));
    Ok(())
}

// (x - )
pub(crate) fn execute_setrand(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("SETRAND"))?;
    fetch_stack(engine, 1)?;
    let rand = engine.cmd.var(0).as_integer()?.clone();
    engine.set_rand(rand)?;
    Ok(())
}
