// Copyright (C) 2026 Pruvendo (bridge integration team).
//
// Wire-format decoder for `VkBlob` — the payload of the *first* operand of
// the `ZKHALO2VERIFYWITHVK` opcode (the verification-key cell).
//
// The opcode takes THREE stack operands:
//
// ```text
//   top      proof_cell         raw SHPLONK proof bytes
//   ↑        public_inputs_cell raw Fr × N (strict 32-byte LE, no header)
//   bottom   vk_cell            VkBlob (this module — magic-tagged, versioned)
// ```
//
// Putting magic + version + transcript-kind on the `vk_cell` means the
// circuit's verification key is a single, immutable, content-addressable
// blob that a smart contract stores once in `c4`/storage and passes onto
// the stack on every call. The other two operands (`public_inputs` and
// `proof`) are bare bytes — contracts assemble them O(1) on the hot path
// without TLV headers.
//
// Wire format (frozen 2026-05-22, Q-WIRE-1..5 — see
// `docs/zkhalo2verifywithvk_design.md`):
//
// ```text
//   off  size  field
//   ───  ────  ─────────────────────────────────────────────────────────
//     0     8  magic           = b"VKBLOB\x00\x00" (ASCII + 2× NUL padding)
//     8     1  version         = 0x01
//     9     1  transcript_kind = 0x00 (Blake2b; 0x01 reserved for Keccak)
//    10     6  reserved        = 0 × 6 (ignored on read)
//    16     4  config_len      (u32 LE)
//    20  cl    config_json     (UTF-8 serde_json of `BaseCircuitParams`)
//   ...     4  vk_len          (u32 LE)
//   ...  vl    vk_bytes        (`VerifyingKey::write(SerdeFormat::RawBytes)`)
// ```
//
// `public_inputs_cell` payload — no header at all, just `N × 32` flat
// little-endian `Fr::to_repr()`. Length must be a multiple of 32; values
// `≥ Fr modulus` are rejected (strict, no u64 shortcut — Q-WIRE-3).
//
// `proof_cell` payload — no header at all, just the SHPLONK proof bytes
// produced by `Blake2bWrite::finalize()`.

use halo2_base::gates::circuit::BaseCircuitParams;
use tvm_types::Result;
use tvm_types::fail;

/// 8-byte ASCII magic at offset 0 of every `vk_cell` payload.
///
/// Pad to 8 bytes so the magic is a fixed-width word that disassemblers
/// and hex dumps can spot at a glance. `\x00\x00` after `VKBLOB` is
/// reserved padding — never a separator.
pub const VK_BLOB_MAGIC: &[u8; 8] = b"VKBLOB\x00\x00";

/// Current `VkBlob` layout version. Bump on any breaking change.
pub const VK_BLOB_VERSION: u8 = 1;

/// Transcript discriminator. Only `Blake2b = 0` is accepted by the opcode
/// today; `0x01 = Keccak` is reserved for a future format version.
pub const TRANSCRIPT_KIND_BLAKE2B: u8 = 0;

/// Header size in bytes: magic (8) + version (1) + transcript_kind (1) +
/// reserved (6).
const HEADER_LEN: usize = 16;

/// Maximum `vk_cell` payload size accepted by the consumer. Real circuits
/// have VKs in the low single-digit kilobytes; this cap is two orders of
/// magnitude above that and is the only guard against a DoS through a
/// pathologically-large length prefix.
const MAX_VK_BLOB_BYTES: usize = 1 * 1024 * 1024;

/// Maximum `public_inputs_cell` payload size. 256 KiB = 8192 distinct
/// 32-byte field elements, several orders of magnitude above any
/// realistic circuit's public-input count.
pub(crate) const MAX_PUBLIC_INPUTS_BYTES: usize = 256 * 1024;

/// Maximum `proof_cell` payload size. SHPLONK proofs for K≤22 circuits
/// are well under 100 KiB; 1 MiB cap is generous DoS guard.
pub(crate) const MAX_PROOF_BYTES: usize = 1 * 1024 * 1024;

/// Length prefix size in bytes.
const LEN_PREFIX_BYTES: usize = 4;

/// Owned, validated view into a `vk_cell` payload. Holds the parsed
/// `BaseCircuitParams` plus the raw VK byte stream (which gets handed to
/// `VerifyingKey::<G1Affine>::read::<_, BaseCircuitBuilder<Fr>>` inside
/// the opcode handler).
#[derive(Clone, Debug)]
pub struct VkBlob {
    /// Circuit shape that `BaseCircuitBuilder` needs at VK-deserialisation
    /// time. Carried inline because halo2-base's `VerifyingKey::read` API
    /// cannot deserialise without it.
    pub config: BaseCircuitParams,
    /// `VerifyingKey<G1Affine>` serialised with `SerdeFormat::RawBytes`.
    pub vk_bytes: Vec<u8>,
}

impl VkBlob {
    /// Parse a `VkBlob` from the byte payload of the `vk_cell` operand.
    ///
    /// Validates the magic, version, transcript discriminator, that each
    /// length prefix fits in the remaining slice, that `MAX_VK_BLOB_BYTES`
    /// isn't exceeded, and that there is no trailing garbage. The VK
    /// bytes are NOT validated cryptographically here — that happens
    /// inside the opcode handler when `VerifyingKey::read` deserialises
    /// them against `BaseCircuitParams`.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_VK_BLOB_BYTES {
            fail!(
                "VkBlob: vk_cell payload length {} exceeds MAX_VK_BLOB_BYTES ({})",
                bytes.len(),
                MAX_VK_BLOB_BYTES
            );
        }
        if bytes.len() < HEADER_LEN {
            fail!(
                "VkBlob: vk_cell payload length {} is shorter than the 16-byte header",
                bytes.len()
            );
        }
        if &bytes[0..8] != VK_BLOB_MAGIC {
            fail!(
                "VkBlob: magic mismatch (expected b\"VKBLOB\\x00\\x00\", got {:02x?})",
                &bytes[0..8]
            );
        }
        if bytes[8] != VK_BLOB_VERSION {
            fail!("VkBlob: version mismatch (expected {VK_BLOB_VERSION}, got {})", bytes[8]);
        }
        if bytes[9] != TRANSCRIPT_KIND_BLAKE2B {
            fail!(
                "VkBlob: unsupported transcript_kind byte {} (only 0 = Blake2b is currently \
                 defined)",
                bytes[9]
            );
        }
        // bytes[10..16] reserved; ignored.

        let mut cursor = HEADER_LEN;
        let config_bytes = read_chunk(bytes, &mut cursor, "config_json")?;
        let vk_bytes = read_chunk(bytes, &mut cursor, "vk_bytes")?;

        if cursor != bytes.len() {
            fail!(
                "VkBlob: trailing garbage after vk_bytes chunk (cursor = {}, len = {})",
                cursor,
                bytes.len()
            );
        }

        let config: BaseCircuitParams = match serde_json::from_slice(&config_bytes) {
            Ok(c) => c,
            Err(e) => {
                fail!("VkBlob: malformed config_json (cannot parse as BaseCircuitParams): {}", e)
            }
        };

        Ok(Self { config, vk_bytes })
    }
}

/// Validate the size of the `public_inputs_cell` payload BEFORE handing
/// it to `decode_instances_strict`. Catches oversized bundles cheaply
/// (without iterating the chunks).
pub(crate) fn validate_public_inputs_size(bytes: &[u8]) -> Result<()> {
    if bytes.len() > MAX_PUBLIC_INPUTS_BYTES {
        fail!(
            "ZKHALO2VERIFYWITHVK: public_inputs_cell payload length {} exceeds \
             MAX_PUBLIC_INPUTS_BYTES ({})",
            bytes.len(),
            MAX_PUBLIC_INPUTS_BYTES
        );
    }
    Ok(())
}

/// Validate the size of the `proof_cell` payload.
pub(crate) fn validate_proof_size(bytes: &[u8]) -> Result<()> {
    if bytes.len() > MAX_PROOF_BYTES {
        fail!(
            "ZKHALO2VERIFYWITHVK: proof_cell payload length {} exceeds MAX_PROOF_BYTES ({})",
            bytes.len(),
            MAX_PROOF_BYTES
        );
    }
    Ok(())
}

fn read_chunk(bytes: &[u8], cursor: &mut usize, field: &'static str) -> Result<Vec<u8>> {
    if bytes.len() < *cursor + LEN_PREFIX_BYTES {
        fail!(
            "VkBlob: ran out of bytes reading length prefix for chunk `{}` (cursor = {}, len = {})",
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
            "VkBlob: chunk `{}` length {} overruns payload (cursor = {}, len = {})",
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
        // Mirrors `dark_dex_w8_config_params()` shape, serialised with
        // `serde_json::to_vec(&BaseCircuitParams)`.
        br#"{"k":19,"num_advice_per_phase":[4],"num_fixed":1,"num_lookup_advice_per_phase":[1],"lookup_bits":18,"num_instance_columns":1}"#
            .to_vec()
    }

    fn make_minimal_vk_blob(config_json: &[u8], vk: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(VK_BLOB_MAGIC);
        out.push(VK_BLOB_VERSION);
        out.push(TRANSCRIPT_KIND_BLAKE2B);
        out.extend_from_slice(&[0u8; 6]);
        for chunk in [config_json, vk] {
            out.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
            out.extend_from_slice(chunk);
        }
        out
    }

    #[test]
    fn parse_round_trips_minimal_blob() {
        let cfg = dark_dex_config_json();
        let bytes = make_minimal_vk_blob(&cfg, b"vk");
        let blob = VkBlob::parse(&bytes).expect("minimal blob must parse");
        assert_eq!(blob.vk_bytes, b"vk");
        assert_eq!(blob.config.k, 19);
    }

    #[test]
    fn parse_rejects_bad_magic() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_vk_blob(&cfg, b"vk");
        bytes[0] = b'X';
        let err = VkBlob::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("magic mismatch"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_bad_version() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_vk_blob(&cfg, b"vk");
        bytes[8] = 99;
        let err = VkBlob::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("version mismatch"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_unknown_transcript() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_vk_blob(&cfg, b"vk");
        bytes[9] = 7;
        let err = VkBlob::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("transcript_kind"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_chunk_overrun() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_vk_blob(&cfg, b"vk");
        // Bump the vk_len prefix past end-of-buffer.
        let cfg_len = cfg.len();
        let vk_len_offset = HEADER_LEN + LEN_PREFIX_BYTES + cfg_len;
        bytes[vk_len_offset..vk_len_offset + 4].copy_from_slice(&(u32::MAX - 100).to_le_bytes());
        let err = VkBlob::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("overruns payload"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_trailing_garbage() {
        let cfg = dark_dex_config_json();
        let mut bytes = make_minimal_vk_blob(&cfg, b"vk");
        bytes.push(0xFF);
        let err = VkBlob::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("trailing garbage"), "actual: {err}");
    }

    #[test]
    fn parse_rejects_oversized_blob() {
        let mut bytes = vec![0u8; MAX_VK_BLOB_BYTES + 1];
        bytes[0..8].copy_from_slice(VK_BLOB_MAGIC);
        bytes[8] = VK_BLOB_VERSION;
        let err = VkBlob::parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("MAX_VK_BLOB_BYTES"), "actual: {err}");
    }

    #[test]
    fn validate_oversized_public_inputs() {
        let bytes = vec![0u8; MAX_PUBLIC_INPUTS_BYTES + 1];
        let err = validate_public_inputs_size(&bytes).unwrap_err();
        assert!(err.to_string().contains("MAX_PUBLIC_INPUTS_BYTES"), "actual: {err}");
    }

    #[test]
    fn validate_oversized_proof() {
        let bytes = vec![0u8; MAX_PROOF_BYTES + 1];
        let err = validate_proof_size(&bytes).unwrap_err();
        assert!(err.to_string().contains("MAX_PROOF_BYTES"), "actual: {err}");
    }
}
