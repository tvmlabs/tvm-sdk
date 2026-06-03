use std::sync::OnceLock;

use gosh_zk_snark_halo2_utils::proof::Proof;
use halo2_base::halo2_proofs::halo2curves::bn256::Bn256;
use halo2_base::halo2_proofs::halo2curves::bn256::Fr;
use halo2_base::halo2_proofs::halo2curves::bn256::G1Affine;
use halo2_base::halo2_proofs::halo2curves::ff::PrimeField;
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
        let vk = crate::executor::zk_halo2_utils::build_dark_dex_w128_vk();
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
    //   - Small values (first 24 bytes zero): 24 zero bytes + 8-byte BE u64 →
    //     Fr::from(u64)
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
            // SECURITY: `Fr::from_bytes_le` calls `Fr::from_repr(repr).unwrap()`,
            // which panics for any 32-byte LE value >= the BN254 scalar field
            // modulus p (~2^254). `pub_input_bytes` comes from attacker-controlled
            // cell data, so a crafted input (e.g. 0xFF…FF = 2^256-1) would crash
            // the executor thread. Use the checked `from_repr` and abort the
            // instruction cleanly on non-canonical encodings, matching the
            // existing `% 32 != 0` length check above.
            let mut repr = <Fr as PrimeField>::Repr::default();
            repr.as_mut().copy_from_slice(pub_input_bytes);
            let Some(fr) = Option::<Fr>::from(Fr::from_repr(repr)) else {
                fail!(ExceptionCode::FatalError);
            };
            pub_inputs.push(fr);
        }
    }

    let (vk, params) = get_vk_and_params();
    let res = proof.verify_with_vk(vk, params, &[&pub_inputs]);

    engine.cc.stack.push(boolean!(res));
    Ok(())
}

// ---------------------------------------------------------------------------
// ZKHALO2VERIFYWITHVK (0xC7 0x4A) — RLC deposit-proof verification.
//
// Variant B (process isolation): the RLC verifier lives on the axiom-eth halo2
// backend, which conflicts with the gosh-halo2 backend used by Dark DEX (0x49)
// over a single `halo2_proofs` source. To keep BOTH opcodes in one node binary,
// this handler does NOT verify in-process. It collects the three on-wire blobs
// (vk_blob, public_inputs, proof) from the stack, writes them to temp files,
// and spawns the standalone `an_rlc_verify` binary (separate Cargo.lock,
// axiom-eth). The spawned process performs the exact golden-reference
// verification (VkBlob::read RLC -> EthCircuitImpl -> 7 PI -> Blake2bRead
// SHPLONK verify_proof).
//
// Exit codes from `an_rlc_verify`:
//   0 = ACCEPT  -> push true
//   2 = REJECT  -> push false (proof structurally valid but verify_proof
// failed)   3 = ERROR   -> fail! (malformed input / IO / SRS error)
// ---------------------------------------------------------------------------
pub(crate) fn execute_zkhalo2_verify_with_vk(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("ZKHALO2VERIFYWITHVK"))?;

    // Canonical 3-operand ABI (matches TokenBridge.sol gosh.zkhalo2VerifyWithVK
    // and TVM-Solidity-Compiler Types.cpp: "VK blob, public-inputs, proof,
    // top-of-stack last"). fetch_stack binds var(0) to the TOP of the stack
    // (the last value pushed), so the index order is REVERSED vs the source
    // argument order:
    //   var(0) = proof, var(1) = public_inputs, var(2) = vk_blob
    fetch_stack(engine, 3)?;

    let proof_slice = SliceData::load_cell_ref(engine.cmd.var(0).as_cell()?)?;
    let proof_bytes = unpack_data_from_cell(proof_slice, engine)?;

    let pub_inputs_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let pub_inputs_bytes = unpack_data_from_cell(pub_inputs_slice, engine)?;

    if pub_inputs_bytes.len() % 32 != 0 {
        fail!(ExceptionCode::FatalError);
    }

    let vk_blob_slice = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let vk_blob_bytes = unpack_data_from_cell(vk_blob_slice, engine)?;

    let res = match run_isolated_rlc_verify(&vk_blob_bytes, &pub_inputs_bytes, &proof_bytes) {
        Ok(verdict) => verdict,
        Err(e) => {
            log::error!("ZKHALO2VERIFYWITHVK isolated verifier error: {}", e);
            fail!(ExceptionCode::FatalError);
        }
    };

    engine.cc.stack.push(boolean!(res));
    Ok(())
}

/// Spawn the standalone `an_rlc_verify` binary to verify an RLC deposit-proof
/// triple out-of-process. Returns Ok(true)=accept, Ok(false)=reject,
/// Err(..) on any technical/malformed error (exit 3 or spawn failure).
fn run_isolated_rlc_verify(
    vk_blob: &[u8],
    public_inputs: &[u8],
    proof: &[u8],
) -> Result<bool, String> {
    use std::io::Write;
    use std::process::Command;

    // Binary path: env override, else machine-local default for E2E.
    let bin = std::env::var("AN_RLC_VERIFY_BIN").unwrap_or_else(|_| {
        "/home/sergey/Pruvendo/gosh/acki-nacki-bridge/deposit-prover/target/release/an_rlc_verify"
            .to_string()
    });
    let srs_dir = std::env::var("AN_RLC_SRS_DIR").unwrap_or_else(|_| {
        "/home/sergey/Pruvendo/gosh/acki-nacki-bridge/deposit-prover/data".to_string()
    });

    // Unique temp dir so concurrent verifications don't collide.
    let nonce = format!(
        "{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let dir = std::env::temp_dir().join(format!("an_rlc_verify_{}", nonce));
    std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir tmp: {}", e))?;

    let vk_path = dir.join("vk_blob.bin");
    let pi_path = dir.join("public_inputs.bin");
    let proof_path = dir.join("proof.bin");

    let write_blob = |p: &std::path::Path, data: &[u8]| -> Result<(), String> {
        let mut f = std::fs::File::create(p).map_err(|e| format!("create {:?}: {}", p, e))?;
        f.write_all(data).map_err(|e| format!("write {:?}: {}", p, e))?;
        Ok(())
    };
    write_blob(&vk_path, vk_blob)?;
    write_blob(&pi_path, public_inputs)?;
    write_blob(&proof_path, proof)?;

    let num_pi = public_inputs.len() / 32;

    let output = Command::new(&bin)
        .arg("--vk-blob")
        .arg(&vk_path)
        .arg("--proof")
        .arg(&proof_path)
        .arg("--pubin")
        .arg(&pi_path)
        .arg("--degree")
        .arg("18")
        .arg("--srs-dir")
        .arg(&srs_dir)
        .arg("--expect-pi")
        .arg(num_pi.to_string())
        .arg("--quiet")
        .output();

    // Best-effort cleanup regardless of outcome.
    let _ = std::fs::remove_dir_all(&dir);

    let output = output.map_err(|e| format!("spawn {}: {}", bin, e))?;
    match output.status.code() {
        Some(0) => Ok(true),
        Some(2) => Ok(false),
        Some(3) => {
            Err(format!("verifier ERROR (exit 3): {}", String::from_utf8_lossy(&output.stderr)))
        }
        other => Err(format!(
            "verifier unexpected exit {:?}: {}",
            other,
            String::from_utf8_lossy(&output.stderr)
        )),
    }
}
