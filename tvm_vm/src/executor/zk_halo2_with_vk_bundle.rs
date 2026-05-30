// Copyright (C) 2026 Pruvendo (bridge integration team).
//
// Wire-format decoder for `Halo2TvmBundle` — the byte payload of the
// `ZKHALO2VERIFYWITHVK` opcode's single stack operand.
//
// The format is defined and frozen by the bridge integration team in
// `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`
// (commit on `main` at the time of writing). This module is the **consumer
// side** of that format: deserialise the bundle bytes back into
// `(BaseCircuitParams, vk_bytes, instances_bytes, proof_bytes)` so the
// opcode handler can run `verify_proof::<KZG, SHPLONK, Blake2bRead,
// SingleStrategy>` against it.
//
// Frozen contract (2026-05-22, Q-WIRE-1..5 — see
// `docs/zkhalo2verifywithvk_design.md`; `circuit_shape` byte + v2 added
// 2026-05-29 for the deposit / RLC integration):
//
// ```text
//   off  size  field
//   ───  ────  ─────────────────────────────────────────────────────────
//     0     8  magic           = b"HALO2TVM" (ASCII, no NUL)
//     8     1  version         = 0x01 (Base only) | 0x02 (shape-tagged)
//     9     1  transcript_kind = 0x00 (Blake2b; 0x01 reserved for Keccak)
//    10     1  circuit_shape   = 0x00 Base | 0x01 Rlc (v1 pins this to 0x00)
//    11     5  reserved        = 0 × 5
//    16     4  config_len      (u32 LE)
//    20  cl    config_json     Base: serde_json(`BaseCircuitParams`)
//                              Rlc : serde_json(`EthCircuitParams`)
//   ...     4  vk_len          (u32 LE)
//   ...  vl    vk_bytes        (`VerifyingKey::write(SerdeFormat::RawBytes)`)
//                              Base VK ← BaseCircuitBuilder<Fr>
//                              Rlc  VK ← EthCircuitImpl<Fr, Noop>
//   ...     4  instances_len   (u32 LE; must be a multiple of 32)
//   ...  il    instances_bytes (N × 32-byte LE `Fr::to_repr()`, strict)
//   ...     4  proof_len       (u32 LE)
//   ...  pl    proof_bytes     (SHPLONK proof with Blake2b transcript)
// ```
//
// All length prefixes are `u32` LE. The consumer reads them as `usize` after
// bounds-checking against the remaining slice length so malformed bundles
// fail with a `FatalError` rather than panicking on out-of-bounds slice
// access.

use halo2_base::gates::circuit::BaseCircuitParams;
use tvm_types::Result;
use tvm_types::fail;

/// 8-byte ASCII magic at offset 0 of every bundle.
pub const BUNDLE_MAGIC: &[u8; 8] = b"HALO2TVM";

/// Legacy bundle layout version: Base-shape only (`BaseCircuitParams` in
/// `config_json`). The `circuit_shape` byte (offset 10) MUST be `0`.
pub const BUNDLE_VERSION: u8 = 1;

/// Shape-tagged bundle layout version (added 2026-05-29 for the deposit /
/// `TokenBridge.finalizeDeposit` integration). The `circuit_shape` byte at
/// offset 10 selects how `config_json` and `vk_bytes` are interpreted:
///   - `CIRCUIT_SHAPE_BASE` → `config_json` is `BaseCircuitParams`, VK read
///     with `BaseCircuitBuilder<Fr>` (identical to v1).
///   - `CIRCUIT_SHAPE_RLC`  → `config_json` is `EthCircuitParams`, VK read with
///     `EthCircuitImpl<Fr, Noop>` (RLC + keccak-coprocessor circuits, e.g. the
///     deposit-prover's `EthCircuitImpl<Fr, DepositEventCircuitV2>`).
/// Mirrors the producer-side `VK_BLOB_VERSION_V2` / `CircuitShape` in
/// `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.
/// rs`.
pub const BUNDLE_VERSION_V2: u8 = 2;

/// `circuit_shape` byte values (offset 10). Only meaningful in v2 bundles; a
/// v1 bundle pins this to `CIRCUIT_SHAPE_BASE`.
pub const CIRCUIT_SHAPE_BASE: u8 = 0;
pub const CIRCUIT_SHAPE_RLC: u8 = 1;

/// Transcript discriminator. Only `Blake2b = 0` is accepted by the opcode
/// today; `0x01 = Keccak` is reserved for a future format version.
pub const TRANSCRIPT_KIND_BLAKE2B: u8 = 0;

/// Header size in bytes: magic (8) + version (1) + transcript_kind (1) +
/// reserved (6).
const HEADER_LEN: usize = 16;

/// Maximum bundle size accepted by the consumer. Empirical fixture is
/// ~21 KB; this cap is two orders of magnitude above that and is the only
/// guard against a DoS through pathologically-large length prefixes.
const MAX_BUNDLE_BYTES: usize = 16 * 1024 * 1024;

/// Length prefix size in bytes.
const LEN_PREFIX_BYTES: usize = 4;

/// Circuit configuration carried inline in the bundle (Q-WIRE-4 / Option B —
/// self-describing bundle). The variant is selected by the `circuit_shape`
/// byte and tells the opcode handler which `Circuit` type to rebuild the
/// constraint system with at VK-deserialisation time.
#[derive(Clone, Debug)]
pub enum BundleConfig {
    /// `BaseCircuitParams` — VK read with `BaseCircuitBuilder<Fr>`.
    Base(BaseCircuitParams),
    /// Raw `EthCircuitParams` JSON bytes — VK read with
    /// `EthCircuitImpl<Fr, Noop>`. Kept as raw bytes here so this parser
    /// stays free of an `axiom-eth` dependency; the handler does the
    /// `serde_json::from_slice::<EthCircuitParams>` itself.
    Rlc(Vec<u8>),
}

/// Owned, validated view into a `Halo2TvmBundle`. Holds owned buffers for
/// the four payload chunks so the caller can drop the original input
/// `Vec<u8>` without invalidating these references.
#[derive(Clone, Debug)]
pub struct Halo2TvmBundle {
    /// Circuit shape + params that the VK deserialiser needs. `Base` for v1
    /// bundles and v2 bundles with `circuit_shape = 0`; `Rlc` for v2 bundles
    /// with `circuit_shape = 1`.
    pub config: BundleConfig,
    /// `VerifyingKey<G1Affine>` serialised with `SerdeFormat::RawBytes`.
    pub vk_bytes: Vec<u8>,
    /// `N × 32` flat little-endian `Fr::to_repr()`. Strict — no u64
    /// shortcut (Q-WIRE-3).
    pub instances_bytes: Vec<u8>,
    /// SHPLONK proof bytes from `Blake2bWrite` (Q-WIRE-1).
    pub proof_bytes: Vec<u8>,
}

impl Halo2TvmBundle {
    /// Parse a `Halo2TvmBundle` from a byte slice produced by
    /// `bridge_prover_orchestrator::Halo2TvmBundle::write` (or any
    /// format-conforming producer).
    ///
    /// Validates the magic, version, transcript discriminator, that every
    /// length prefix fits in the remaining slice, that `MAX_BUNDLE_BYTES`
    /// isn't exceeded, and that `instances_len` is a multiple of 32. The
    /// VK bytes are NOT validated cryptographically here — that happens
    /// inside the opcode handler when `VerifyingKey::read` deserialises
    /// the bytes against `BaseCircuitParams`.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_BUNDLE_BYTES {
            fail!(
                "Halo2TvmBundle: bundle length {} exceeds MAX_BUNDLE_BYTES ({})",
                bytes.len(),
                MAX_BUNDLE_BYTES
            );
        }
        if bytes.len() < HEADER_LEN {
            fail!(
                "Halo2TvmBundle: bundle length {} is shorter than the 16-byte header",
                bytes.len()
            );
        }
        if &bytes[0..8] != BUNDLE_MAGIC {
            fail!(
                "Halo2TvmBundle: bundle magic mismatch (expected b\"HALO2TVM\", got {:02x?})",
                &bytes[0..8]
            );
        }
        let version = bytes[8];
        if version != BUNDLE_VERSION && version != BUNDLE_VERSION_V2 {
            fail!(
                "Halo2TvmBundle: bundle version mismatch (expected {BUNDLE_VERSION} or \
                 {BUNDLE_VERSION_V2}, got {version})"
            );
        }
        if bytes[9] != TRANSCRIPT_KIND_BLAKE2B {
            fail!(
                "Halo2TvmBundle: unsupported transcript_kind byte {} (only 0 = Blake2b is \
                 currently defined)",
                bytes[9]
            );
        }

        // Offset 10: circuit_shape. In a v1 bundle this is part of the
        // reserved zero region and MUST be 0 (Base). In a v2 bundle it
        // selects Base (0) vs Rlc (1).
        let circuit_shape = bytes[10];
        if version == BUNDLE_VERSION && circuit_shape != CIRCUIT_SHAPE_BASE {
            fail!(
                "Halo2TvmBundle: v1 bundle carries non-zero circuit_shape byte {circuit_shape} \
                 (a shape-tagged bundle must set version = {BUNDLE_VERSION_V2})"
            );
        }
        if circuit_shape != CIRCUIT_SHAPE_BASE && circuit_shape != CIRCUIT_SHAPE_RLC {
            fail!(
                "Halo2TvmBundle: unsupported circuit_shape byte {circuit_shape} (only 0 = Base \
                 and 1 = Rlc are currently defined)"
            );
        }
        // bytes[11..16] reserved; ignored.

        let mut cursor = HEADER_LEN;
        let config_bytes = read_chunk(bytes, &mut cursor, "config_json")?;
        let vk_bytes = read_chunk(bytes, &mut cursor, "vk_bytes")?;
        let instances_bytes = read_chunk(bytes, &mut cursor, "instances_bytes")?;
        let proof_bytes = read_chunk(bytes, &mut cursor, "proof_bytes")?;

        if cursor != bytes.len() {
            fail!(
                "Halo2TvmBundle: trailing garbage after proof chunk (cursor = {}, len = {})",
                cursor,
                bytes.len()
            );
        }
        if !instances_bytes.len().is_multiple_of(32) {
            fail!(
                "Halo2TvmBundle: instances_bytes length {} is not a multiple of 32 (each public \
                 input must be a 32-byte LE Fr)",
                instances_bytes.len()
            );
        }

        let config = match circuit_shape {
            CIRCUIT_SHAPE_BASE => {
                let params: BaseCircuitParams = match serde_json::from_slice(&config_bytes) {
                    Ok(c) => c,
                    Err(e) => fail!(
                        "Halo2TvmBundle: malformed config_json (cannot parse as \
                         BaseCircuitParams): {}",
                        e
                    ),
                };
                BundleConfig::Base(params)
            }
            // Rlc: keep the EthCircuitParams JSON opaque here; the handler
            // parses it against axiom-eth's EthCircuitParams type. We still
            // sanity-check it is non-empty so a malformed bundle fails at
            // parse time rather than deep inside VK reconstruction.
            CIRCUIT_SHAPE_RLC => {
                if config_bytes.is_empty() {
                    fail!("Halo2TvmBundle: Rlc bundle carries an empty config_json chunk");
                }
                BundleConfig::Rlc(config_bytes)
            }
            _ => unreachable!("circuit_shape validated above"),
        };

        Ok(Self { config, vk_bytes, instances_bytes, proof_bytes })
    }

    /// Number of public inputs encoded in the bundle.
    #[allow(dead_code)]
    pub fn num_instances(&self) -> usize {
        self.instances_bytes.len() / 32
    }
}

fn read_chunk(bytes: &[u8], cursor: &mut usize, field: &'static str) -> Result<Vec<u8>> {
    if bytes.len() < *cursor + LEN_PREFIX_BYTES {
        fail!(
            "Halo2TvmBundle: ran out of bytes reading length prefix for chunk `{}` (cursor = {}, \
             len = {})",
            field,
            *cursor,
            bytes.len()
        );
    }
    let len_bytes: [u8; 4] = bytes[*cursor..*cursor + LEN_PREFIX_BYTES]
        .try_into()
        .expect("4-byte slice always converts to [u8; 4]");
    let len = u32::from_le_bytes(len_bytes) as usize;
    *cursor += LEN_PREFIX_BYTES;
    if bytes.len() < *cursor + len {
        fail!(
            "Halo2TvmBundle: chunk `{}` length {} overruns bundle (cursor = {}, len = {})",
            field,
            len,
            *cursor,
            bytes.len()
        );
    }
    let chunk = bytes[*cursor..*cursor + len].to_vec();
    *cursor += len;
    Ok(chunk)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dark_dex_config_json() -> Vec<u8> {
        // Mirrors `dark_dex_w8_config_params()` in `zk_halo2_utils.rs`,
        // serialised with `serde_json::to_vec(&BaseCircuitParams)`.
        br#"{"k":19,"num_advice_per_phase":[4],"num_fixed":1,"num_lookup_advice_per_phase":[1],"lookup_bits":18,"num_instance_columns":1}"#
            .to_vec()
    }

    fn make_minimal_bundle(
        config_json: &[u8],
        vk: &[u8],
        instances: &[u8],
        proof: &[u8],
    ) -> Vec<u8> {
        make_bundle(BUNDLE_VERSION, CIRCUIT_SHAPE_BASE, config_json, vk, instances, proof)
    }

    fn make_bundle(
        version: u8,
        circuit_shape: u8,
        config_json: &[u8],
        vk: &[u8],
        instances: &[u8],
        proof: &[u8],
    ) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(BUNDLE_MAGIC);
        out.push(version);
        out.push(TRANSCRIPT_KIND_BLAKE2B);
        out.push(circuit_shape);
        out.extend_from_slice(&[0u8; 5]);
        for chunk in [config_json, vk, instances, proof] {
            out.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
            out.extend_from_slice(chunk);
        }
        out
    }

    /// A small but structurally valid `EthCircuitParams` JSON, mirroring what
    /// `EthCircuitParams::default()` serialises to (with `k`/`lookup_bits`
    /// set).
    fn rlc_config_json() -> Vec<u8> {
        br#"{"rlc":{"base":{"k":14,"num_advice_per_phase":[1,1],"num_fixed":1,"num_lookup_advice_per_phase":[1,0],"lookup_bits":13,"num_instance_columns":1},"num_rlc_columns":1},"keccak_rows_per_round":20}"#
            .to_vec()
    }

    #[test]
    fn parse_round_trips_minimal_bundle() {
        let cfg = dark_dex_config_json();
        let bytes = make_minimal_bundle(&cfg, b"vk", &[0u8; 32], b"proof");
        let bundle = Halo2TvmBundle::parse(&bytes).expect("minimal bundle must parse");
        assert!(matches!(bundle.config, BundleConfig::Base(_)), "v1 bundle must be Base shape");
        assert_eq!(bundle.vk_bytes, b"vk");
        assert_eq!(bundle.instances_bytes, vec![0u8; 32]);
        assert_eq!(bundle.proof_bytes, b"proof");
        assert_eq!(bundle.num_instances(), 1);
    }

    #[test]
    fn parse_v2_base_bundle_is_base_shape() {
        let cfg = dark_dex_config_json();
        let bytes =
            make_bundle(BUNDLE_VERSION_V2, CIRCUIT_SHAPE_BASE, &cfg, b"vk", &[0u8; 32], b"p");
        let bundle = Halo2TvmBundle::parse(&bytes).expect("v2 Base bundle must parse");
        assert!(matches!(bundle.config, BundleConfig::Base(_)));
    }

    #[test]
    fn parse_v2_rlc_bundle_carries_opaque_eth_params() {
        let cfg = rlc_config_json();
        let bytes =
            make_bundle(BUNDLE_VERSION_V2, CIRCUIT_SHAPE_RLC, &cfg, b"vk", &[0u8; 64], b"p");
        let bundle = Halo2TvmBundle::parse(&bytes).expect("v2 Rlc bundle must parse");
        match &bundle.config {
            BundleConfig::Rlc(json) => {
                assert_eq!(json, &cfg, "Rlc config bytes must round-trip verbatim")
            }
            BundleConfig::Base(_) => panic!("expected Rlc shape"),
        }
        assert_eq!(bundle.num_instances(), 2);
    }

    #[test]
    fn parse_rejects_v1_with_nonzero_shape() {
        let cfg = dark_dex_config_json();
        let bytes = make_bundle(BUNDLE_VERSION, CIRCUIT_SHAPE_RLC, &cfg, b"vk", &[0u8; 32], b"p");
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("non-zero circuit_shape"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_unknown_shape() {
        let cfg = rlc_config_json();
        let bytes = make_bundle(BUNDLE_VERSION_V2, 7, &cfg, b"vk", &[0u8; 32], b"p");
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("unsupported circuit_shape"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_empty_rlc_config() {
        let bytes = make_bundle(BUNDLE_VERSION_V2, CIRCUIT_SHAPE_RLC, b"", b"vk", &[0u8; 32], b"p");
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("empty config_json"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_bad_magic() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_bundle(&cfg, b"vk", &[0u8; 32], b"proof");
        bytes[0] = b'X';
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("magic mismatch"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_bad_version() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_bundle(&cfg, b"vk", &[0u8; 32], b"proof");
        bytes[8] = 99;
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("version mismatch"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_unknown_transcript() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_bundle(&cfg, b"vk", &[0u8; 32], b"proof");
        bytes[9] = 7;
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("transcript_kind"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_instances_not_multiple_of_32() {
        let cfg = dark_dex_config_json();
        let bytes = make_minimal_bundle(&cfg, b"vk", &[0u8; 31], b"proof");
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("multiple of 32"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_chunk_overrun() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_bundle(&cfg, b"vk", &[0u8; 32], b"proof");
        // Bump the vk_len prefix past end-of-buffer.
        let cfg_len = cfg.len();
        let vk_len_offset = HEADER_LEN + LEN_PREFIX_BYTES + cfg_len;
        bytes[vk_len_offset..vk_len_offset + 4].copy_from_slice(&(u32::MAX - 100).to_le_bytes());
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("overruns bundle"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_trailing_garbage() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_bundle(&cfg, b"vk", &[0u8; 32], b"proof");
        bytes.push(0xFF);
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("trailing garbage"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_oversized_bundle() {
        let mut bytes = vec![0u8; MAX_BUNDLE_BYTES + 1];
        bytes[0..8].copy_from_slice(BUNDLE_MAGIC);
        bytes[8] = BUNDLE_VERSION;
        let err = Halo2TvmBundle::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("MAX_BUNDLE_BYTES"), "actual: {err}");
    }
}
