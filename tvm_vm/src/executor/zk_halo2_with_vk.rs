// Copyright (C) 2026 Pruvendo (bridge integration team).
//
// `ZKHALO2VERIFYWITHVK` opcode handler — native Halo2 SHPLONK verification
// in TVM with a caller-supplied verifying key, public inputs, and proof.
//
// Designed for the Acki Nacki ↔ Ethereum bridge: each `TokenBridge`
// deploys with its own immutable Halo2 SHPLONK verifier key, so a single
// opcode covers every bridge / app circuit anyone ever deploys to AN
// without growing the node code for each new circuit variant.
//
// ## Stack ABI (frozen 2026-05-25 — Variant A)
//
// Three `Cell` operands, top → bottom:
//
// ```text
//   top      proof_cell           raw SHPLONK proof bytes (no header)
//   ↑        public_inputs_cell   raw Fr × N (strict 32-byte LE, no header)
//   bottom   vk_cell              VkBlob (magic + version + transcript
//                                  + config_json + vk_bytes)
// ```
//
// Assembly:
//
// ```text
//   PUSHREF vk_cell
//   PUSHREF public_inputs_cell
//   PUSHREF proof_cell
//   ZKHALO2VERIFYWITHVK
// ```
//
// The opcode pops three cells in order (proof, public_inputs, vk) and
// pushes a boolean (`true` = proof verifies, `false` = cryptographic
// rejection). Structural errors (bad magic / version / instances not a
// multiple of 32 / VK doesn't deserialise / public input ≥ Fr modulus)
// throw `FatalError`, mirroring `VERGRTH16`'s contract.
//
// See `super::zk_halo2_with_vk_bundle` for the `VkBlob` wire format and
// `docs/zkhalo2verifywithvk_design.md` for the frozen ABI memo.

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::OnceLock;

use axiom_eth::Field;
use axiom_eth::mpt::MPTChip;
use axiom_eth::rlc::circuit::builder::RlcCircuitBuilder;
use axiom_eth::utils::eth_circuit::EthCircuitImpl;
use axiom_eth::utils::eth_circuit::EthCircuitInstructions;
use axiom_eth::utils::eth_circuit::EthCircuitParams;
use halo2_base::gates::circuit::builder::BaseCircuitBuilder;
use halo2_base::halo2_proofs::SerdeFormat;
use halo2_base::halo2_proofs::halo2curves::bn256::Bn256;
use halo2_base::halo2_proofs::halo2curves::bn256::Fr;
use halo2_base::halo2_proofs::halo2curves::bn256::G1Affine;
use halo2_base::halo2_proofs::halo2curves::ff::PrimeField;
use halo2_base::halo2_proofs::plonk::VerifyingKey;
use halo2_base::halo2_proofs::plonk::verify_proof;
use halo2_base::halo2_proofs::poly::commitment::ParamsProver;
use halo2_base::halo2_proofs::poly::kzg::commitment::KZGCommitmentScheme;
use halo2_base::halo2_proofs::poly::kzg::commitment::ParamsKZG;
use halo2_base::halo2_proofs::poly::kzg::multiopen::VerifierSHPLONK;
use halo2_base::halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_base::halo2_proofs::transcript::Blake2bRead;
use halo2_base::halo2_proofs::transcript::Challenge255;
use halo2_base::halo2_proofs::transcript::TranscriptReadBuffer;
use tvm_types::SliceData;
use tvm_types::fail;

use crate::executor::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::gas::gas_state::Gas;
use crate::executor::zk_halo2_with_vk_bundle::VkBlob;
use crate::executor::zk_halo2_with_vk_bundle::VkBlobConfig;
use crate::executor::zk_halo2_with_vk_bundle::validate_proof_size;
use crate::executor::zk_halo2_with_vk_bundle::validate_public_inputs_size;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Status;
use crate::utils::unpack_data_from_cell;

/// Placeholder gas price. Halo2 SHPLONK verification with a warm VK cache
/// is in the millisecond range; the cold path additionally pays for VK
/// curve-checking and `EvaluationDomain` construction (which scales with
/// `k` — at `k = 20` it is on the order of seconds).
///
/// **MUST re-benchmark before mainnet.** Treat the current value as a
/// structural placeholder modelled on `VERGRTH16_GAS_PRICE`, scaled up
/// for the bigger VK (kilobytes vs 192 B). Operators are expected to
/// pre-warm the per-VK cache at node startup so the hot path dominates
/// production load; the cold path is intentionally *not* scaled out to
/// its true wall-clock cost. See `docs/zkhalo2verifywithvk_design.md`
/// Q-GAS-1 for the open re-benchmark task.
pub const ZKHALO2_VERIFY_WITH_VK_GAS_PRICE: i64 = 5_000;

/// Maximum number of distinct VKs the per-VK cache holds. A typical AN
/// chain runs a handful of bridge / NFT / zkLogin VKs concurrently; 8
/// is a generous upper bound. Eviction is FIFO (oldest insert first);
/// for strictly-LRU eviction we'd need to track per-entry access
/// timestamps, which costs more than it saves at this cache size.
const VK_CACHE_CAPACITY: usize = 8;

/// Best-effort extraction of a panic payload's message — pulls out a
/// `&str` / `String` payload as plain text, falling back to a generic
/// label so callers always get *something* to log. Used to turn a
/// halo2-base panic from a malicious VK bundle into a structured
/// `FatalError`.
fn panic_payload_to_str(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}

/// One slot in the per-VK cache. `Arc` so handler invocations on
/// different threads don't block each other on the `Mutex` past the
/// lookup.
#[derive(Clone)]
struct CachedVk {
    vk: VerifyingKey<G1Affine>,
    params: ParamsKZG<Bn256>,
}

/// Bounded FIFO map keyed by the `VkBlob`'s `vk_bytes` chunk.
struct VkCache {
    entries: HashMap<Vec<u8>, std::sync::Arc<CachedVk>>,
    insertion_order: VecDeque<Vec<u8>>,
    capacity: usize,
}

impl VkCache {
    fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
            insertion_order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn get(&self, key: &[u8]) -> Option<std::sync::Arc<CachedVk>> {
        self.entries.get(key).cloned()
    }

    fn insert(&mut self, key: Vec<u8>, value: std::sync::Arc<CachedVk>) {
        if self.entries.contains_key(&key) {
            return;
        }
        if self.insertion_order.len() == self.capacity {
            if let Some(evicted) = self.insertion_order.pop_front() {
                self.entries.remove(&evicted);
            }
        }
        self.insertion_order.push_back(key.clone());
        self.entries.insert(key, value);
    }
}

static VK_CACHE: OnceLock<Mutex<VkCache>> = OnceLock::new();

fn vk_cache() -> &'static Mutex<VkCache> {
    VK_CACHE.get_or_init(|| Mutex::new(VkCache::new(VK_CACHE_CAPACITY)))
}

/// Empty RLC circuit used purely as the `Circuit` *type* for deserialising
/// an `EthCircuitImpl`-shaped VK. `VerifyingKey::read` rebuilds the
/// constraint system via `Circuit::configure_with_params(EthCircuitParams)`,
/// which depends only on the carried params — never on this body — so the
/// instructions can be a no-op. The same family covers every RLC + keccak
/// coprocessor circuit (e.g. the deposit-prover's
/// `EthCircuitImpl<Fr, DepositEventCircuitV2>`); only the params differ.
#[derive(Clone)]
struct Noop;

impl<F: Field> EthCircuitInstructions<F> for Noop {
    type FirstPhasePayload = ();

    fn virtual_assign_phase0(&self, _builder: &mut RlcCircuitBuilder<F>, _mpt: &MPTChip<F>) {}
}

/// Read a `BaseCircuitBuilder`-shaped VK (v1 + v2 `circuit_shape = 0`).
fn read_base_vk(
    vk_bytes: &[u8],
    params: halo2_base::gates::circuit::BaseCircuitParams,
) -> tvm_types::Result<VerifyingKey<G1Affine>> {
    let config_for_read = params.clone();
    let mut vk_slice: &[u8] = vk_bytes;
    let read_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        VerifyingKey::<G1Affine>::read::<_, BaseCircuitBuilder<Fr>>(
            &mut vk_slice,
            SerdeFormat::RawBytes,
            config_for_read,
        )
    }));
    match read_result {
        Ok(Ok(vk)) => Ok(vk),
        Ok(Err(e)) => fail!(
            "ZKHALO2VERIFYWITHVK: Base VerifyingKey::read failed (config = {:?}): {}",
            params,
            e
        ),
        Err(payload) => {
            let panic_msg = panic_payload_to_str(&payload);
            fail!(
                "ZKHALO2VERIFYWITHVK: Base VK deserialisation panicked (likely adversarial \
                 config = {:?}): {}",
                params,
                panic_msg
            );
        }
    }
}

/// Read an `EthCircuitImpl`-shaped (RLC) VK (v2 `circuit_shape = 1`). The
/// `config_json` chunk is the JSON of axiom-eth's `EthCircuitParams`.
fn read_rlc_vk(vk_bytes: &[u8], config_json: &[u8]) -> tvm_types::Result<VerifyingKey<G1Affine>> {
    let params: EthCircuitParams = match serde_json::from_slice(config_json) {
        Ok(p) => p,
        Err(e) => fail!(
            "ZKHALO2VERIFYWITHVK: malformed Rlc config_json (cannot parse as EthCircuitParams): {}",
            e
        ),
    };
    let mut slice: &[u8] = vk_bytes;
    match VerifyingKey::<G1Affine>::read::<_, EthCircuitImpl<Fr, Noop>>(
        &mut slice,
        SerdeFormat::RawBytes,
        params,
    ) {
        Ok(vk) => Ok(vk),
        Err(e) => fail!("ZKHALO2VERIFYWITHVK: Rlc VerifyingKey::read failed: {}", e),
    }
}

/// Look up `(vk, params)` for the given `VkBlob`, deserialising and
/// caching on miss. Returns an `Arc` so the lock can be released before
/// verification runs.
///
/// **Cache key:** `blob.vk_bytes` only — *not* `(vk_bytes, config)`.
/// This is safe because `VerifyingKey::read` fails deserialisation before
/// we cache on shape/config mismatch, and the on-disk VK byte representation
/// is fully determined by the circuit shape.
fn get_or_insert_vk(blob: &VkBlob) -> tvm_types::Result<std::sync::Arc<CachedVk>> {
    {
        let cache = vk_cache().lock().expect("VK cache mutex poisoned");
        if let Some(hit) = cache.get(&blob.vk_bytes) {
            if let VkBlobConfig::Base(params) = &blob.config {
                let cached_k = hit.vk.get_domain().k();
                if params.k as u32 != cached_k {
                    fail!(
                        "ZKHALO2VERIFYWITHVK: VkBlob config.k = {} but cached VK domain.k = {}",
                        params.k,
                        cached_k
                    );
                }
            }
            return Ok(hit);
        }
    }

    let vk = match &blob.config {
        VkBlobConfig::Base(params) => {
            let vk = read_base_vk(&blob.vk_bytes, params.clone())?;
            let k = vk.get_domain().k();
            if params.k as u32 != k {
                fail!(
                    "ZKHALO2VERIFYWITHVK: VkBlob config.k = {} disagrees with VK domain.k = {}",
                    params.k,
                    k
                );
            }
            vk
        }
        VkBlobConfig::Rlc(config_json) => read_rlc_vk(&blob.vk_bytes, config_json)?,
    };

    let k = vk.get_domain().k();
    let params = crate::executor::zk_halo2_utils::build_kzg_verifier_params(k);

    let entry = std::sync::Arc::new(CachedVk { vk, params });

    let mut cache = vk_cache().lock().expect("VK cache mutex poisoned");
    cache.insert(blob.vk_bytes.clone(), entry.clone());
    Ok(entry)
}

/// Decode the `public_inputs_cell` payload into `Vec<Fr>` using
/// **strict** 32-byte little-endian `Fr::from_repr` (Q-WIRE-3: no u64
/// shortcut). `Fr::from_repr` returns `None` for byte sequences that
/// are `>= modulus`; we surface that as a structural `FatalError`.
fn decode_instances_strict(instances_bytes: &[u8]) -> tvm_types::Result<Vec<Fr>> {
    if !instances_bytes.len().is_multiple_of(32) {
        fail!(
            "ZKHALO2VERIFYWITHVK: public_inputs_cell length {} is not a multiple of 32",
            instances_bytes.len()
        );
    }
    let mut out = Vec::with_capacity(instances_bytes.len() / 32);
    for (i, chunk) in instances_bytes.chunks_exact(32).enumerate() {
        let mut repr = <Fr as PrimeField>::Repr::default();
        repr.as_mut().copy_from_slice(chunk);
        let fr = Fr::from_repr(repr);
        if fr.is_none().into() {
            fail!(
                "ZKHALO2VERIFYWITHVK: public_inputs[{}] is >= modulus (Fr::from_repr \
                 rejected): {:02x?}",
                i,
                chunk
            );
        }
        out.push(fr.unwrap());
    }
    Ok(out)
}

/// `ZKHALO2VERIFYWITHVK` handler.
///
/// **Stack** (top → bottom):
/// 1. `proof_cell` — `Cell` whose payload is the raw SHPLONK proof byte stream
///    produced by `Blake2bWrite::finalize()`. No header, no framing.
/// 2. `public_inputs_cell` — `Cell` whose payload is `N × 32` flat
///    little-endian `Fr::to_repr()` (strict). No header, length must be a
///    multiple of 32.
/// 3. `vk_cell` — `Cell` whose payload is a [`VkBlob`] (magic + version +
///    transcript discriminator + circuit shape + `config_json` +
///    `VerifyingKey<G1Affine>` bytes). See `super::zk_halo2_with_vk_bundle` for
///    the format and `docs/zkhalo2verifywithvk_design.md` for the frozen
///    wire-format contract.
///
/// **Pushes** a boolean: `true` on a cryptographically valid proof,
/// `false` on a well-formed-but-invalid proof.
///
/// **Throws `FatalError`** on structural input errors only:
/// - `vk_cell` doesn't parse as `VkBlob` (bad magic / version / transcript /
///   chunk-length overrun / trailing garbage / malformed config JSON).
/// - `public_inputs_cell` length is not a multiple of 32, exceeds
///   `MAX_PUBLIC_INPUTS_BYTES`, or any 32-byte chunk is `≥ Fr modulus`.
/// - `proof_cell` exceeds `MAX_PROOF_BYTES`.
/// - `VerifyingKey<G1Affine>::read` rejects the VK bytes (curve membership
///   failure with `SerdeFormat::RawBytes`, or shape doesn't match the inline
///   config).
///
/// Cryptographic rejection (well-formed proof that just doesn't
/// satisfy the relation) is a normal `false` return, not an exception,
/// matching `VERGRTH16`'s contract.
pub(crate) fn execute_zkhalo2_verify_with_vk(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("ZKHALO2VERIFYWITHVK"))?;
    engine.try_use_gas(Gas::zkhalo2_verify_with_vk_price())?;
    fetch_stack(engine, 3)?;

    // Top of stack = var(0) = proof_cell (most recently pushed).
    let proof_cell = engine.cmd.var(0).as_cell()?;
    let proof_slice = SliceData::load_cell_ref(proof_cell)?;
    let proof_bytes = unpack_data_from_cell(proof_slice, engine)?;
    validate_proof_size(&proof_bytes)?;

    // var(1) = public_inputs_cell.
    let public_inputs_cell = engine.cmd.var(1).as_cell()?;
    let public_inputs_slice = SliceData::load_cell_ref(public_inputs_cell)?;
    let public_inputs_bytes = unpack_data_from_cell(public_inputs_slice, engine)?;
    validate_public_inputs_size(&public_inputs_bytes)?;

    // Bottom = var(2) = vk_cell (pushed first).
    let vk_cell = engine.cmd.var(2).as_cell()?;
    let vk_slice = SliceData::load_cell_ref(vk_cell)?;
    let vk_payload = unpack_data_from_cell(vk_slice, engine)?;
    let vk_blob = VkBlob::parse(&vk_payload)?;

    let cached = get_or_insert_vk(&vk_blob)?;
    let instances = decode_instances_strict(&public_inputs_bytes)?;

    let verifier_params = cached.params.verifier_params();
    let strategy = SingleStrategy::new(&cached.params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(proof_bytes.as_slice());
    let instance_refs: &[&[Fr]] = &[&instances];
    let res = verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        SingleStrategy<'_, Bn256>,
    >(verifier_params, &cached.vk, strategy, &[instance_refs], &mut transcript)
    .is_ok();

    engine.cc.stack.push(boolean!(res));
    Ok(())
}

#[cfg(test)]
mod rlc_branch_tests {
    //! Consumer-side coverage for the VkBlob-v2 `Rlc` circuit shape (the
    //! deposit-prover / `EthCircuitImpl` family). Keygens a real RLC-shaped VK
    //! on the SAME backend the opcode uses, serialises it, and checks the two
    //! VK-read branches: the `Rlc` branch reconstructs it; the legacy `Base`
    //! branch rejects it. This is the unit-level proof that the shape
    //! discriminator routes to the right `Circuit` type.
    use axiom_eth::mpt::MPTChip;
    use axiom_eth::rlc::circuit::builder::RlcCircuitBuilder;
    use axiom_eth::utils::eth_circuit::EthCircuitInstructions;
    use axiom_eth::utils::eth_circuit::EthCircuitParams;
    use halo2_base::gates::circuit::BaseCircuitParams;
    use halo2_base::gates::circuit::CircuitBuilderStage;
    use halo2_base::halo2_proofs::plonk::keygen_vk;

    use super::*;

    /// Keygen probe: makes ONE fixed-length keccak call so
    /// `mock_fulfill_keccak_promises` has a populated promise map (the real
    /// deposit circuit always hashes). The instructions body only matters at
    /// keygen — the produced VK's constraint-system shape depends solely on
    /// `EthCircuitParams`, so it reads back fine with the handler's empty
    /// `Noop`.
    #[derive(Clone)]
    struct KeygenProbe;

    impl<F: Field> EthCircuitInstructions<F> for KeygenProbe {
        type FirstPhasePayload = ();

        fn virtual_assign_phase0(&self, builder: &mut RlcCircuitBuilder<F>, mpt: &MPTChip<F>) {
            let keccak = mpt.keccak();
            let ctx = builder.base.main(0);
            let b = ctx.load_witness(F::ZERO);
            let _ = keccak.keccak_fixed_len(ctx, vec![b]);
        }
    }

    /// Keygen a real `EthCircuitImpl`-shaped VK + its derived
    /// `EthCircuitParams` JSON, exactly as a deposit-prover-class producer
    /// would emit into a VkBlob-v2 `Rlc` bundle.
    fn keygen_rlc_vk() -> (Vec<u8>, Vec<u8>) {
        let mut init = EthCircuitParams::default();
        init.rlc.base.k = 14;
        init.rlc.base.lookup_bits = Some(13);
        let mut circuit = EthCircuitImpl::<Fr, KeygenProbe>::new_impl(
            CircuitBuilderStage::Keygen,
            KeygenProbe,
            init.rlc.clone(),
            init.keccak.clone(),
        );
        circuit.mock_fulfill_keccak_promises(None);
        let calc = circuit.calculate_params();
        let k = calc.rlc.base.k as u32;
        let srs = ParamsKZG::<Bn256>::setup(k, rand::rngs::OsRng);
        let vk = keygen_vk(&srs, &circuit).expect("keygen_vk EthCircuitImpl<KeygenProbe>");
        let vk_bytes = vk.to_bytes(SerdeFormat::RawBytes);
        let cfg_json = serde_json::to_vec(&calc).expect("serialise EthCircuitParams");
        (vk_bytes, cfg_json)
    }

    #[test]
    fn rlc_branch_reconstructs_vk() {
        let (vk_bytes, cfg_json) = keygen_rlc_vk();
        let vk = read_rlc_vk(&vk_bytes, &cfg_json)
            .expect("Rlc branch must reconstruct an EthCircuitImpl VK from its own bytes");
        let _params =
            crate::executor::zk_halo2_utils::build_kzg_verifier_params(vk.get_domain().k());
        let reser = vk.to_bytes(SerdeFormat::RawBytes);
        assert_eq!(reser, vk_bytes, "Rlc VK must round-trip byte-for-byte");
    }

    #[test]
    fn base_branch_reads_fewer_points_than_rlc_vk() {
        let (vk_bytes, _cfg_json) = keygen_rlc_vk();
        let base = BaseCircuitParams {
            k: 14,
            num_advice_per_phase: vec![1],
            num_fixed: 1,
            num_lookup_advice_per_phase: vec![0],
            lookup_bits: None,
            num_instance_columns: 1,
        };
        match read_base_vk(&vk_bytes, base) {
            Ok(vk) => {
                let reser = vk.to_bytes(SerdeFormat::RawBytes);
                assert_ne!(
                    reser, vk_bytes,
                    "Base reader must not faithfully round-trip an RLC-shaped VK"
                );
            }
            Err(_) => {}
        }
    }

    #[test]
    fn rlc_branch_rejects_malformed_config_json() {
        let (vk_bytes, _cfg) = keygen_rlc_vk();
        let err = read_rlc_vk(&vk_bytes, b"{ not valid eth params }").unwrap_err();
        assert!(
            err.to_string().contains("EthCircuitParams"),
            "expected EthCircuitParams parse error, got: {err}"
        );
    }
}
