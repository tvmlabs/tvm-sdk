// CHKHISTPROOF TVM instruction — verifies that a key block's common section
// at a specified layer stores a particular layer hash value.
//
// Stack: hash (256-bit) | block_height (u64) | layer_number (u8, 1..=10)
// Result: pushes boolean (-1 true, 0 false)

use crate::executor::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::gas::gas_state::Gas;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::types::Status;

pub const CHKHISTPROOF_GAS_PRICE: i64 = 100;

pub(super) fn execute_chk_hist_proof(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine.load_instruction(crate::executor::types::Instruction::new("CHKHISTPROOF"))?;
    engine.try_use_gas(Gas::chkhistproof_price())?;
    fetch_stack(engine, 3)?;

    // Stack order: var(0)=top=layer_number, var(1)=block_height, var(2)=hash
    let layer_number: u8 = engine.cmd.var(0).as_integer()?.into(1..=10)?;
    let block_height: u64 = engine.cmd.var(1).as_integer()?.into(0..=u64::MAX)?;
    let hash_builder = engine
        .cmd
        .var(2)
        .as_integer()?
        .as_builder::<UnsignedIntegerBigEndianEncoding>(256)?;
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(hash_builder.data());

    let result = match &engine.check_history_proof_hash {
        Some(callback) => {
            eprintln!("CHKHISTPROOF: callback present, calling with height={}, layer={}, hash={}", block_height, layer_number, hex::encode(hash_bytes));
            let r = callback(block_height, layer_number, hash_bytes);
            eprintln!("CHKHISTPROOF: callback returned {}", r);
            r
        }
        None => {
            eprintln!("CHKHISTPROOF: NO callback set, returning false");
            false
        }
    };

    engine.cc.stack.push(boolean!(result));
    Ok(())
}
