// GETALLLAYERHASHES TVM instruction — returns the chain's full rolling-window
// snapshot of historical block hashes, used by the contract to assemble the
// public anonymity-set array for `gosh.zkhalo2verify` (layer-anonymized DEX
// circuit).
//
// Stack (no args):
//   -> bytes  (TVM cell-backed; concatenation of `num_layers * 128 * 32` bytes;
//              layer 1 first, then layer 2, …, then layer `num_layers`).
//   -> int    (num_layers, in [1, 10]).
//
// num_layers is the topmost stack item after the call; the bytes blob sits
// directly below. The receiving Solidity intrinsic should declare the return
// signature as `(uint8 numLayers, bytes data)` so the compiler emits a pop in
// the matching order.
//
// Host responsibility: when the callback is wired (see
// `Engine::set_get_all_layer_hashes`), it must return a `(num_layers, blob)`
// pair where the blob length equals `num_layers * 128 * 32`. Anything else is
// a host-side bug and will abort the transaction.

use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_types::fail;

use crate::executor::Engine;
use crate::executor::gas::gas_state::Gas;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Status;

pub const GETALLLAYERHASHES_GAS_PRICE: i64 = 200;

/// Number of layer-L hashes kept in the rolling window per layer.
/// Must match `HISTORY_PROOF_WINDOW_SIZE` in the chain config and
/// `HASHES_PER_LAYER` in the circuit.
pub const HASHES_PER_LAYER: usize = 128;
pub const HASH_BYTES: usize = 32;
pub const MAX_LAYERS: u8 = 10;

pub(super) fn execute_get_all_layer_hashes(engine: &mut Engine) -> Status {
    engine.mark_execution_as_block_related()?;
    engine
        .load_instruction(crate::executor::types::Instruction::new("GETALLLAYERHASHES"))?;
    engine.try_use_gas(Gas::getalllayerhashes_price())?;

    let (num_layers, blob): (u8, Vec<u8>) = match &engine.get_all_layer_hashes {
        Some(callback) => callback(),
        None => {
            eprintln!("GETALLLAYERHASHES: NO callback set, returning (0, [])");
            (0u8, Vec::new())
        }
    };

    // Validate the host-supplied snapshot. A misbehaving host is a fatal bug;
    // the circuit will reject the resulting proof anyway, but failing fast
    // here gives a clearer signal.
    if num_layers > MAX_LAYERS {
        fail!(
            "GETALLLAYERHASHES: host returned num_layers={} > MAX_LAYERS={}",
            num_layers,
            MAX_LAYERS
        );
    }
    let expected_len = (num_layers as usize) * HASHES_PER_LAYER * HASH_BYTES;
    if blob.len() != expected_len {
        fail!(
            "GETALLLAYERHASHES: host returned blob of {} bytes; expected num_layers ({}) * 128 * 32 = {}",
            blob.len(),
            num_layers,
            expected_len
        );
    }

    let bytes_cell =
        TokenValue::write_bytes(blob.as_slice(), &ABI_VERSION_2_4)?.into_cell()?;
    engine.cc.stack.push(StackItem::cell(bytes_cell));
    engine.cc.stack.push(int!(num_layers));

    Ok(())
}
