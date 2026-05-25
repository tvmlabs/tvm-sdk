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

/// Build `ParamsKZG<Bn256>` for an arbitrary `k` using the chain-wide
/// shared trusted-setup G1/G2 points (Q-WIRE-2: "globally shared per `k`").
///
/// SHPLONK verification only needs `g[0]`, `g2`, and `s_g2` from the SRS,
/// so we don't need the full 64 MB blob — three embedded points
/// (`KZG_G0_BYTES` + `KZG_G2_BYTES` + `KZG_S_G2_BYTES`, ~320 bytes) are
/// enough. We bootstrap a `ParamsKZG` at `k=0` from a synthetic blob
/// containing those three points and then re-base it via `from_parts`
/// to the actual `k` the VK was generated for.
fn build_shared_kzg_params(k: u32) -> ParamsKZG<Bn256> {
    use halo2_base::halo2_proofs::halo2curves::serde::SerdeObject;

    let mut blob = Vec::with_capacity(388);
    blob.extend_from_slice(&0u32.to_le_bytes());
    blob.extend_from_slice(&crate::executor::zk_halo2_utils::KZG_G0_BYTES);
    blob.extend_from_slice(&crate::executor::zk_halo2_utils::KZG_G0_BYTES);
    blob.extend_from_slice(&crate::executor::zk_halo2_utils::KZG_G2_BYTES);
    blob.extend_from_slice(&crate::executor::zk_halo2_utils::KZG_S_G2_BYTES);
    let mut cursor: &[u8] = &blob;
    let dummy = ParamsKZG::<Bn256>::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked)
        .expect("Parsing embedded KZG verifier blob should not fail");

    let g0 = G1Affine::from_raw_bytes_unchecked(&crate::executor::zk_halo2_utils::KZG_G0_BYTES);
    dummy.from_parts(k, vec![g0], Some(vec![]), dummy.g2(), dummy.s_g2())
}

/// Look up `(vk, params)` for the given `VkBlob`, deserialising and
/// caching on miss. Returns an `Arc` so the lock can be released before
/// verification runs.
///
/// **Cache key:** `blob.vk_bytes` only — *not* `(vk_bytes, config)`.
/// This is safe because (a) `VerifyingKey::read::<_, BaseCircuitBuilder<Fr>,
/// _>` uses `config` only to size internal allocations, and any mismatch
/// between `config` and the VK byte payload fails the *deserialisation*
/// before we cache; (b) the on-disk VK byte representation is fully
/// determined by the circuit shape, so a single byte sequence cannot
/// have been produced under two different `BaseCircuitParams`. As an
/// extra belt-and-braces measure we still verify `config.k ==
/// vk.get_domain().k()` below — drifted JSON tries to mislead the
/// verifier into the wrong `ParamsKZG` size and we want it rejected
/// loudly.
fn get_or_insert_vk(blob: &VkBlob) -> tvm_types::Result<std::sync::Arc<CachedVk>> {
    {
        let cache = vk_cache().lock().expect("VK cache mutex poisoned");
        if let Some(hit) = cache.get(&blob.vk_bytes) {
            let cached_k = hit.vk.get_domain().k();
            if blob.config.k as u32 != cached_k {
                fail!(
                    "ZKHALO2VERIFYWITHVK: VkBlob config.k = {} but cached VK domain.k = {}",
                    blob.config.k,
                    cached_k
                );
            }
            return Ok(hit);
        }
    }

    // Cold path: deserialise outside the lock.
    //
    // `BaseCircuitBuilder::new(...)` (driven by `VerifyingKey::read`) is
    // not panic-safe for adversarial `BaseCircuitParams` — e.g. a
    // config where `lookup_bits >= k - blinding_factors` triggers a
    // `panic!` inside `halo2-base`. Catch the panic and surface it as
    // a structural `FatalError` so a single malicious VkBlob cannot
    // crash the executor thread. `AssertUnwindSafe` is safe here
    // because no state we hold across the call is mutated (we own
    // `vk_slice` and `blob.config`).
    let config_for_read = blob.config.clone();
    let mut vk_slice: &[u8] = blob.vk_bytes.as_slice();
    let read_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // RawBytes — DOES validate curve membership on every group
        // element on read. This is the soundness-critical setting for
        // caller-supplied VKs (vs RawBytesUnchecked which skips the
        // check and is only safe for trusted, pre-vetted VKs at
        // compile/build time).
        VerifyingKey::<G1Affine>::read::<_, BaseCircuitBuilder<Fr>>(
            &mut vk_slice,
            SerdeFormat::RawBytes,
            config_for_read,
        )
    }));
    let vk = match read_result {
        Ok(Ok(vk)) => vk,
        Ok(Err(e)) => fail!(
            "ZKHALO2VERIFYWITHVK: VerifyingKey<G1Affine>::read failed (config = {:?}): {}",
            blob.config,
            e
        ),
        Err(payload) => {
            let panic_msg = panic_payload_to_str(&payload);
            fail!(
                "ZKHALO2VERIFYWITHVK: VK deserialisation panicked (likely adversarial config \
                 = {:?}): {}",
                blob.config,
                panic_msg
            );
        }
    };

    let k = vk.get_domain().k();
    if blob.config.k as u32 != k {
        fail!(
            "ZKHALO2VERIFYWITHVK: VkBlob config.k = {} disagrees with VK domain.k = {}",
            blob.config.k,
            k
        );
    }
    let params = build_shared_kzg_params(k);

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
///    transcript discriminator + `BaseCircuitParams` JSON +
///    `VerifyingKey<G1Affine>` bytes). See `super::zk_halo2_with_vk_bundle` for
///    the format and `docs/zkhalo2verifywithvk_design.md` for the frozen
///    wire-format contract.
///
/// **Pushes** a boolean: `true` on a cryptographically valid proof,
/// `false` on a well-formed-but-invalid proof.
///
/// **Throws `FatalError`** on structural input errors only:
/// - `vk_cell` doesn't parse as `VkBlob` (bad magic / version / transcript /
///   chunk-length overrun / trailing garbage / malformed `BaseCircuitParams`).
/// - `public_inputs_cell` length is not a multiple of 32, exceeds
///   `MAX_PUBLIC_INPUTS_BYTES`, or any 32-byte chunk is `≥ Fr modulus`.
/// - `proof_cell` exceeds `MAX_PROOF_BYTES`.
/// - `VerifyingKey<G1Affine>::read` rejects the VK bytes (curve membership
///   failure with `SerdeFormat::RawBytes`, or shape doesn't match the inline
///   `BaseCircuitParams`).
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
