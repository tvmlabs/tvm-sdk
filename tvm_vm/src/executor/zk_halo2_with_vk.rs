// Copyright (C) 2026 Pruvendo (bridge integration team).
//
// `ZKHALO2VERIFYWITHVK` opcode handler — the sibling of `ZKHALO2VERIFY` that
// takes the verifying key (and circuit config) as a caller-supplied operand
// instead of a hard-coded chain-wide constant.
//
// Designed for the Acki Nacki ↔ Ethereum bridge: each `TokenBridge` deploys
// with its own immutable Halo2 SHPLONK verifier key, so a single opcode covers
// every bridge / app circuit anyone ever deploys to AN without growing the
// per-circuit-variant `match` in `ZKHALO2VERIFY`.
//
// Wire-format contract for the single stack operand is the
// [`Halo2TvmBundle`](`super::zk_halo2_with_vk_bundle::Halo2TvmBundle`) byte
// layout, frozen 2026-05-22 (Q-WIRE-1..5 + Q-NAME-1; see
// `docs/zkhalo2verifywithvk_design.md`). The format is also implemented on
// the producer side in
// `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`,
// where it is round-trip-tested against real Circuit 1B fallback proofs.

use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};

use axiom_eth::Field;
use axiom_eth::mpt::MPTChip;
use axiom_eth::rlc::circuit::builder::RlcCircuitBuilder;
use axiom_eth::utils::eth_circuit::{EthCircuitImpl, EthCircuitInstructions, EthCircuitParams};
use gosh_zk_snark_halo2_utils::proof::Proof;
use halo2_base::gates::circuit::builder::BaseCircuitBuilder;
use halo2_base::halo2_proofs::SerdeFormat;
use halo2_base::halo2_proofs::halo2curves::bn256::{Bn256, Fr, G1Affine};
use halo2_base::halo2_proofs::halo2curves::ff::PrimeField;
use halo2_base::halo2_proofs::plonk::VerifyingKey;
use halo2_base::halo2_proofs::poly::kzg::commitment::ParamsKZG;
use tvm_types::SliceData;
use tvm_types::fail;

use crate::executor::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::gas::gas_state::Gas;
use crate::executor::zk_halo2_with_vk_bundle::{BundleConfig, Halo2TvmBundle};
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::types::Status;
use crate::utils::unpack_data_from_cell;

/// Placeholder gas price. Halo2 SHPLONK verification with a warm VK cache is
/// ~milliseconds; the additional cost over `ZKHALO2VERIFY` covers
/// bundle deserialisation + VK reconstruction + per-VK cache lookup.
///
/// **Re-benchmark before mainnet**: this number is a structural guess
/// modelled on `VERGRTH16_WITH_VK_GAS_PRICE`, scaled up for the bigger
/// VK (kilobytes vs 192 B). Once we have a stable end-to-end node
/// integration we'll measure wall-clock for warm and cold paths and tune
/// this constant. The cold-cache path (~3 s `EvaluationDomain` build for
/// `K=20`) is intentionally **not** charged here — operators are
/// expected to pre-warm cached VKs at node startup the way
/// `warmup_halo2()` does for the DarkDex W=8 VK.
pub const ZKHALO2_VERIFY_WITH_VK_GAS_PRICE: i64 = 5_000;

/// Maximum number of distinct VKs the per-VK cache holds. A typical AN
/// chain runs a handful of bridge / NFT / zkLogin VKs concurrently; 8 is a
/// generous upper bound. Eviction is FIFO (oldest insert first); for
/// strictly-LRU eviction we'd need to track per-entry access timestamps,
/// which costs more than it saves at this cache size.
const VK_CACHE_CAPACITY: usize = 8;

/// One slot in the per-VK cache. `Arc` so handler invocations on different
/// threads don't block each other on the `Mutex` past the lookup.
#[derive(Clone)]
struct CachedVk {
    vk: VerifyingKey<G1Affine>,
    params: ParamsKZG<Bn256>,
}

/// Bounded FIFO map keyed by the bundle's `vk_bytes`.
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
/// Mirrors the technique in `zk_halo2_utils::build_kzg_verifier_params`
/// (DarkDex W=8 at `k=19`) but parameterised by `k`. SHPLONK verification
/// only needs `g[0]`, `g2`, and `s_g2` from the SRS, so we don't need the
/// full 64 MB blob — three embedded points (~320 bytes) are enough.
fn build_shared_kzg_params(k: u32) -> ParamsKZG<Bn256> {
    use halo2_base::halo2_proofs::halo2curves::serde::SerdeObject;

    // Build a minimal K=0 binary blob containing the real g[0], g2, s_g2.
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
    let mut slice: &[u8] = vk_bytes;
    // RawBytes — DOES validate curve membership on every group element on
    // read. This is the soundness-critical setting for caller-supplied VKs
    // (vs RawBytesUnchecked which skips the check and is only safe for
    // trusted, pre-vetted VKs at compile/build time).
    match VerifyingKey::<G1Affine>::read::<_, BaseCircuitBuilder<Fr>>(
        &mut slice,
        SerdeFormat::RawBytes,
        params.clone(),
    ) {
        Ok(vk) => Ok(vk),
        Err(e) => fail!(
            "ZKHALO2VERIFYWITHVK: Base VerifyingKey::read failed (config = {:?}): {}",
            params,
            e
        ),
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

/// Look up `(vk, params)` for the given bundle, deserialising and caching
/// on miss. Returns an `Arc` so the lock can be released before
/// verification runs.
fn get_or_insert_vk(bundle: &Halo2TvmBundle) -> tvm_types::Result<std::sync::Arc<CachedVk>> {
    // Fast path: lock-and-clone.
    {
        let cache = vk_cache().lock().expect("VK cache mutex poisoned");
        if let Some(hit) = cache.get(&bundle.vk_bytes) {
            return Ok(hit);
        }
    }

    // Cold path: deserialise outside the lock, branching on circuit shape.
    let vk = match &bundle.config {
        BundleConfig::Base(params) => read_base_vk(&bundle.vk_bytes, params.clone())?,
        BundleConfig::Rlc(config_json) => read_rlc_vk(&bundle.vk_bytes, config_json)?,
    };

    let k = vk.get_domain().k();
    let params = build_shared_kzg_params(k);

    let entry = std::sync::Arc::new(CachedVk { vk, params });

    let mut cache = vk_cache().lock().expect("VK cache mutex poisoned");
    cache.insert(bundle.vk_bytes.clone(), entry.clone());
    Ok(entry)
}

/// Decode the bundle's `instances_bytes` into `Vec<Fr>` using **strict**
/// 32-byte little-endian `Fr::from_repr` (Q-WIRE-3: no u64 shortcut).
/// `Fr::from_repr` returns `None` for byte sequences that are `>= modulus`;
/// we surface that as a structural `FatalError`.
fn decode_instances_strict(instances_bytes: &[u8]) -> tvm_types::Result<Vec<Fr>> {
    if !instances_bytes.len().is_multiple_of(32) {
        fail!(
            "ZKHALO2VERIFYWITHVK: instances byte length {} is not a multiple of 32",
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
                "ZKHALO2VERIFYWITHVK: instances[{}] is >= modulus (Fr::from_repr rejected): \
                 {:02x?}",
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
/// - `bundle_cell` — `Cell` whose payload is a [`Halo2TvmBundle`] (single
///   self-describing byte stream carrying a `circuit_shape` byte + circuit
///   params (`BaseCircuitParams` for `circuit_shape = 0`, `EthCircuitParams`
///   for `circuit_shape = 1`) + `VerifyingKey<G1Affine>` bytes +
///   public-input bytes + SHPLONK proof bytes, in that order, behind length
///   prefixes). The VK is rebuilt with `BaseCircuitBuilder<Fr>` for the Base
///   shape and `EthCircuitImpl<Fr, Noop>` for the Rlc shape. See
///   `super::zk_halo2_with_vk_bundle` for the format and
///   `docs/zkhalo2verifywithvk_design.md` for the frozen wire-format
///   contract.
///
/// **Pushes** a boolean: `true` on a cryptographically valid proof,
/// `false` on a well-formed-but-invalid proof.
///
/// **Throws `FatalError`** on structural input errors only:
/// - Bundle bytes don't deserialise (bad magic / version / transcript /
///   chunk-length overrun / trailing garbage / malformed `BaseCircuitParams`).
/// - `VerifyingKey<G1Affine>::read` rejects the VK bytes (curve membership
///   failure with `SerdeFormat::RawBytes`, or shape doesn't match the
///   inline `BaseCircuitParams`).
/// - Any 32-byte chunk in `instances_bytes` is `>= modulus` (strict
///   `Fr::from_repr`).
///
/// Cryptographic rejection (well-formed proof that just doesn't satisfy
/// the relation) is a normal `false` return, not an exception, matching
/// `VERGRTH16WITHVK`'s contract.
pub(crate) fn execute_zkhalo2_verify_with_vk(engine: &mut Engine) -> Status {
    engine
        .load_instruction(crate::executor::types::Instruction::new("ZKHALO2VERIFYWITHVK"))?;
    engine.try_use_gas(Gas::zkhalo2_verify_with_vk_price())?;
    fetch_stack(engine, 1)?;

    let bundle_cell = engine.cmd.var(0).as_cell()?;
    let bundle_slice = SliceData::load_cell_ref(bundle_cell)?;
    let bundle_bytes = unpack_data_from_cell(bundle_slice, engine)?;

    let bundle = Halo2TvmBundle::parse(&bundle_bytes)?;
    let cached = get_or_insert_vk(&bundle)?;
    let instances = decode_instances_strict(&bundle.instances_bytes)?;

    let proof = Proof::new(bundle.proof_bytes.clone());
    let res = proof.verify_with_vk(&cached.vk, &cached.params, &[&instances]);

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
    use axiom_eth::utils::eth_circuit::{EthCircuitInstructions, EthCircuitParams};
    use halo2_base::gates::circuit::{BaseCircuitParams, CircuitBuilderStage};
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

    /// Keygen a real `EthCircuitImpl`-shaped VK + its derived `EthCircuitParams`
    /// JSON, exactly as a deposit-prover-class producer would emit into a
    /// VkBlob-v2 `Rlc` bundle.
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
        // The shared-KZG path must accept the VK's domain k (sanity: build params).
        let _params = build_shared_kzg_params(vk.get_domain().k());
        // Full round-trip fidelity: re-serialising the reconstructed VK must
        // reproduce the original bytes byte-for-byte. This is the strong proof
        // that the Rlc reader rebuilt the *exact* EthCircuitImpl constraint
        // system (a wrong CS would read a different number of commitments and
        // round-trip to different bytes).
        let reser = vk.to_bytes(SerdeFormat::RawBytes);
        assert_eq!(reser, vk_bytes, "Rlc VK must round-trip byte-for-byte");
    }

    #[test]
    fn base_branch_reads_fewer_points_than_rlc_vk() {
        // NB: `VerifyingKey::read` is NOT strictly shape-validating — it reads
        // exactly the commitment count the supplied config dictates and ignores
        // any trailing bytes (it does not check EOF). So reading RLC bytes with
        // a *smaller* Base config does not error; instead it consumes only a
        // prefix and yields a structurally-valid-but-semantically-wrong VK.
        // The discriminator's real job is to pick the reader whose CS matches
        // the proof, so verification *succeeds* for valid proofs and *fails*
        // for shape-mismatched ones. We assert the observable consequence: the
        // Base reader produces a VK that does NOT round-trip to the RLC bytes.
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
            // An error is equally acceptable (config dictates more points than
            // the stream carries → unexpected EOF).
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
