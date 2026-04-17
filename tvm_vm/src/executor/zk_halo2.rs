use gosh_zk_snark_halo2_utils::proof::Proof;
use halo2_base::halo2_proofs::halo2curves::bn256::Fr;
use halo2_base::utils::ScalarField;
use tvm_types::ExceptionCode;
use tvm_types::SliceData;
use tvm_types::error;
use tvm_types::fail;

use crate::executor::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Status;
use crate::utils::unpack_data_from_cell;

pub fn consume_uint64(b: &[u8]) -> tvm_types::Result<u64> {
    if b.len() != 32 {
        fail!("Not enough bytes for u64");
    }
    let result = (b[31] as u64)
        | ((b[30] as u64) << 8)
        | ((b[29] as u64) << 16)
        | ((b[28] as u64) << 24)
        | ((b[27] as u64) << 32)
        | ((b[26] as u64) << 40)
        | ((b[25] as u64) << 48)
        | ((b[24] as u64) << 56);

    Ok(result)
}

pub fn check_is_u64(b: &[u8]) -> tvm_types::Result<bool> {
    if b.len() != 32 {
        fail!("Not enough bytes");
    }
    for i in 0..24 {
        if b[i] != 0 {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(crate) fn execute_halo2_proof_verification(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("ZKHALO2VERIFY"))?;

    fetch_stack(engine, 2)?;

    // Pop proof bytes from stack
    let proof_slice = SliceData::load_cell_ref(engine.cmd.var(0).as_cell()?)?;
    let proof_bytes = unpack_data_from_cell(proof_slice, engine)?;
    let proof = Proof::new(proof_bytes);

    // Pop public instances bytes (N × 32 bytes)
    let pub_inputs_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let pub_inputs_bytes = unpack_data_from_cell(pub_inputs_slice, engine)?;

    if pub_inputs_bytes.len() % 32 != 0 {
        fail!(ExceptionCode::FatalError);
    }

    // Dual-path instance parsing:
    //   - Small values (first 24 bytes zero): 24 zero bytes + 8-byte BE u64 → Fr::from(u64)
    //   - Full Fr elements: 32-byte LE Fr::to_repr() → Fr::from_bytes_le()
    let num_of_pub_inputs = pub_inputs_bytes.len() / 32;
    let mut pub_inputs: Vec<Fr> = Vec::new();
    for i in 0..num_of_pub_inputs {
        let pub_input_bytes: &[u8; 32] =
            &pub_inputs_bytes[i * 32..(i + 1) * 32].try_into().unwrap();
        let is_u64 = match check_is_u64(pub_input_bytes) {
            Ok(res) => res,
            Err(err) => {
                fail!("Invalid length {}", err)
            }
        };
        if is_u64 {
            let elem: u64 = match consume_uint64(pub_input_bytes) {
                Ok(el) => el,
                Err(err) => {
                    fail!("Invalid length {}", err)
                }
            };
            pub_inputs.push(Fr::from(elem));
        } else {
            pub_inputs.push(Fr::from_bytes_le(pub_input_bytes));
        }
    }

    // Deserialize VK and KZG params from embedded const bytes on each call.
    // No LazyLock — avoids poisoning / cdylib duplication issues in production.
    // Cost is sub-millisecond vs. the ~100ms of actual proof verification.
    let vk = crate::executor::zk_halo2_utils::build_dark_dex_w8_vk();
    let params = crate::executor::zk_halo2_utils::build_kzg_verifier_params();
    //let res = proof.verify_with_vk(&vk, &params, &[&pub_inputs]);
    let res = false;

    engine.cc.stack.push(boolean!(res));
    Ok(())
}
