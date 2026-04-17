use std::sync::OnceLock;

use gosh_zk_snark_halo2_utils::proof::Proof;
use halo2_base::halo2_proofs::halo2curves::bn256::{Bn256, Fr, G1Affine};
use halo2_base::halo2_proofs::plonk::VerifyingKey;
use halo2_base::halo2_proofs::poly::kzg::commitment::ParamsKZG;
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

/// Cached VK + KZG verifier params.  Built once on first access.
///
/// `VerifyingKey::read` internally constructs an `EvaluationDomain` for K=19
/// which precomputes FFT twiddle factors for 2^19 elements — this takes
/// several seconds, so it must be cached across calls.
static VK_AND_PARAMS: OnceLock<(VerifyingKey<G1Affine>, ParamsKZG<Bn256>)> = OnceLock::new();

fn get_vk_and_params() -> &'static (VerifyingKey<G1Affine>, ParamsKZG<Bn256>) {
    VK_AND_PARAMS.get_or_init(|| {
        let vk = crate::executor::zk_halo2_utils::build_dark_dex_w8_vk();
        let params = crate::executor::zk_halo2_utils::build_kzg_verifier_params();
        (vk, params)
    })
}

/// Force-initialize the cached VK and KZG params in a background thread.
/// Call early during node startup so that by the time the first
/// ZKHALO2VERIFY transaction arrives the heavy computation is already done.
pub fn warmup_halo2() {
    if VK_AND_PARAMS.get().is_some() {
        return;
    }
    std::thread::Builder::new()
        .name("halo2-warmup".into())
        .spawn(|| {
            let _ = get_vk_and_params();
        })
        .ok();
}

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

    let (vk, params) = get_vk_and_params();
    let res = proof.verify_with_vk(vk, params, &[&pub_inputs]);

    engine.cc.stack.push(boolean!(res));
    Ok(())
}
