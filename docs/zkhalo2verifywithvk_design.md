# `ZKHALO2VERIFYWITHVK` — TVM Opcode Design Notes

**Status:** landed (this commit).
**Dispatch:** `0xC7 0x4A` (gosh-feature gated).
**Mnemonic:** `ZKHALO2VERIFYWITHVK`.
**ABI variant:** **A — three stack operands** (frozen 2026-05-25).

## 1. Purpose

Native verification of Halo2 SHPLONK proofs inside TVM where the
verifying key + circuit configuration are **caller-supplied** rather
than baked into the node binary. Each Acki Nacki ↔ Ethereum bridge
`TokenBridge` (and any other dApp using Halo2) carries its own VK;
this opcode covers all of them without growing per-circuit code in
the node.

## 2. Stack ABI

Three `Cell` operands, top → bottom:

```text
input : ...
        proof_cell           ← top
        public_inputs_cell
        vk_cell              ← bottom
output: ...
        ok: int (-1 = true, 0 = false)
```

Assembly:

```text
PUSHREF vk_cell
PUSHREF public_inputs_cell
PUSHREF proof_cell
ZKHALO2VERIFYWITHVK
```

Returns:

- **`Ok(int(-1))`** — proof verified.
- **`Ok(int(0))`** — proof rejected by Halo2 verifier (cryptographic
  reject; well-formed operands, just don't satisfy the relation).
- **`Err(FatalError(…))`** — structural input error: `vk_cell` bytes
  don't parse as a `VkBlob`, VK byte stream invalid, `BaseCircuitBuilder`
  panics on internally-inconsistent config, public input ≥ Fr modulus,
  `public_inputs_cell` length not a multiple of 32, `proof_cell`
  exceeds the hard cap, or `config.k` disagrees with the VK's domain
  `k`.

The `Ok(false)` vs `FatalError` distinction matches `VERGRTH16`'s
contract: caller bugs throw, adversarial-but-valid proofs are
silently rejected.

### Rationale for the 3-operand split

| Operand               | Why a separate cell                                                  |
|-----------------------|----------------------------------------------------------------------|
| `vk_cell`             | Long-lived. A `TokenBridge` contract stores it once in `c4` / storage at deploy time and re-uses it on every call. Magic + version + transcript_kind framing live here so a drifted producer / consumer is rejected loudly. |
| `public_inputs_cell`  | Computed per-call from the call arguments. Headerless so a contract can build it O(1) on the hot path — `BUILDER`, repeated `STZEROES`/`STIR`, `ENDC`. |
| `proof_cell`          | Comes straight from off-chain prover output. Headerless so it can be stored / forwarded verbatim through messages without re-encoding. |

## 3. `VkBlob` wire format (`vk_cell` payload)

```text
┌──────────────────────────────────────────────────────────────────┐
│ 0   "VKBLOB\x00\x00"           8 B   magic                       │
│ 8   0x01                       1 B   version                     │
│ 9   0x00 = Blake2b             1 B   transcript-kind (Q-WIRE-1)  │
│ 10  0x00 × 6                   6 B   reserved (zero)             │
│ 16  cfg_len                    4 B   u32 little-endian           │
│ 20  config_json                cfg_len B  serde_json(BaseCircuitParams)
│ ..  vk_len                     4 B   u32 little-endian           │
│ ..  vk_bytes                   vk_len B  VerifyingKey<G1Affine>  │
│                                            SerdeFormat::RawBytes │
└──────────────────────────────────────────────────────────────────┘
```

Constraints checked on parse:

- Magic matches `b"VKBLOB\x00\x00"`, version = `1`, transcript = `0`.
- Reserved bytes `[10..16]` are intentionally **not** validated by
  the parser — they exist for future-proofing (e.g. capability
  flags). Producers must emit them as zero, but consumers ignore the
  content.
- Each length prefix fits in the remaining buffer (no overflow /
  overrun).
- No trailing garbage after `vk_bytes`.
- Total `vk_cell` payload ≤ 1 MiB hard cap.

## 4. `public_inputs_cell` wire format

**No header.** Just a contiguous sequence of `N × 32` little-endian
`Fr::to_repr()` values. The number of public inputs `N` is implied
by `payload_len / 32` and is cross-checked against the VK on every
verify.

Constraints checked on read:

- `payload_len % 32 == 0`.
- `payload_len ≤ 256 KiB` (= 8192 distinct `Fr`).
- For each 32-byte chunk: `Fr::from_repr(chunk)` succeeds (i.e.
  the chunk represents a canonical field element strictly `<` BN254
  scalar modulus).

### How to assemble `public_inputs_cell` from contract code

Given `N` public inputs as TVM `int`s (each fitting in a 256-bit Fr),
the contract builds the cell payload in two steps:

1. Encode each `int` as a **32-byte little-endian** byte string and
   concatenate. TL-B notation:
   ```text
   public_inputs#_ {N:#} values:(N * fr_le)
       = PublicInputs N;
   fr_le#_ value:(## 256) = FrLittleEndian;
   ```
2. `STREF`-build the resulting bytes into a fresh `Cell`. Empty cell
   (N=0) is legal in principle but rejected by every realistic VK
   (every Halo2 circuit has at least one public input).

For instances that fit in `u64`, the contract can build the bottom
8 bytes from the integer and append 24 bytes of zeros — this is the
common case for amounts, block heights, addresses, hashes (where
the field element is exactly the 32-byte digest interpreted as
little-endian `Fr`).

**Worked example (3 inputs):**

```text
                   ┌──────────────────────────────────────────────┐
   instances[0]    │ block_seqno = 7  →  0x07 00..00 (32 B LE)    │
   instances[1]    │ nullifier   = 0xdead..beef (32 B LE)          │
   instances[2]    │ commitment  = 0xbabe..cafe (32 B LE)          │
                   └──────────────────────────────────────────────┘
   public_inputs_cell payload = bytes[0] ‖ bytes[1] ‖ bytes[2]
                              (96 B total, no header, no separator)
```

Out-of-range field elements (e.g. flipping a high bit so the chunk
exceeds the BN254 scalar modulus) are **always** structural errors
(`FatalError`), not silent verify-fails — this catches caller bugs
early instead of letting them masquerade as cryptographic rejects.

## 5. `proof_cell` wire format

**No header.** Just the SHPLONK proof bytes emitted by
`Blake2bWrite::<_, _, Challenge255<_>>::finalize()` on the producer
side. The handler hands these bytes directly to `Blake2bRead::init`
and runs `halo2_proofs::plonk::verify_proof`.

Constraints checked on read:

- `payload_len ≤ 1 MiB` hard cap.
- No further structural validation; any malformed proof is a
  cryptographic reject (`Ok(false)`), never a `FatalError` — this is
  necessary to match `VERGRTH16`'s contract (well-formed inputs,
  arbitrary content).

## 6. Q-WIRE design pivots

| Q | Decision |
|---|----------|
| Q-WIRE-1 | Transcript kind byte (in `VkBlob` header) is reserved for future hash agility; only `0x00` (Blake2b) is currently accepted. |
| Q-WIRE-2 | KZG SRS points (`g[0]`, `g2`, `s_g2`) are globally shared per `k` and embedded as constants in `zk_halo2_utils`. SHPLONK verification only needs those three points; the full multi-MB SRS blob is never loaded at runtime. |
| Q-WIRE-3 | Public inputs are **strict** 32-byte little-endian `Fr::from_repr`. No `u64` shortcut. Out-of-range bytes are a `FatalError`. |
| Q-WIRE-4 | VK is self-describing — magic + version + transcript-kind + reserved header + two `u32 LE` length-prefixed chunks (config_json, vk_bytes). Public inputs and proof are headerless because they are computed per-call and don't benefit from per-call framing. |
| Q-WIRE-5 | Halo2 stack (verifier transcript, multiopen, strategy) is fixed at `(Blake2b, SHPLONK, SingleStrategy)`. Adding GWC / Keccak transcript would require a new transcript kind byte. |

## 7. Implementation notes

- **VK cache:** bounded FIFO, 8 entries, keyed by `VkBlob.vk_bytes`.
  Eviction is oldest-insert-first. The cached value holds the
  deserialised `VerifyingKey<G1Affine>` and a verifier-only
  `ParamsKZG<Bn256>` rebuilt for the VK's domain `k` via
  `from_parts(k, vec![g0], Some(vec![]), g2, s_g2)`.
- **Config-`k` defence-in-depth:** both the cold path (after
  `VK::read`) and the hot path (cache hit) reject `VkBlob` payloads
  whose `BaseCircuitParams.k` disagrees with `vk.get_domain().k()`.
  Catches a malicious blob that reuses a benign VK byte-string but
  lies about `k` in the JSON header.
- **Panic safety:** `BaseCircuitBuilder::new(...)` (driven by
  `VerifyingKey::read::<_, BaseCircuitBuilder<Fr>>`) is not
  panic-safe for adversarial `BaseCircuitParams` — e.g.
  `lookup_bits >= k` triggers a `panic!` deep inside halo2-base.
  The handler wraps the read in `std::panic::catch_unwind` and
  converts the unwind into a structured `FatalError`, so a single
  malicious blob cannot crash the executor thread.
- **VK curve checks:** `SerdeFormat::RawBytes` validates curve
  membership on every G1 element of the VK as it is read. BN254 G1
  is prime-order (cofactor=1) so curve membership implies subgroup
  membership.
- **Verification call:** the handler invokes
  `halo2_proofs::plonk::verify_proof::<KZGCommitmentScheme<Bn256>,
  VerifierSHPLONK<'_, Bn256>, Challenge255<G1Affine>, Blake2bRead<&[u8],
  G1Affine, Challenge255<G1Affine>>, SingleStrategy<'_, Bn256>>`
  directly — no `gosh-zk-snark-halo2-utils` dependency. The producer
  side uses the **same** generic parameters, so any well-formed
  proof from one is verifiable by the other.
- **Dependencies:** `tvm_vm/Cargo.toml` is unchanged from
  `origin/main`. The handler builds against public
  `halo2-base = "0.5.1"` from crates.io. No gosh-fork patch entries
  are required.

## 8. Gas model (Q-GAS-1, **must re-benchmark before mainnet**)

Current placeholder:

```rust
pub const ZKHALO2_VERIFY_WITH_VK_GAS_PRICE: i64 = 5_000;
```

This is a structural guess modelled on
`VERGRTH16_GAS_PRICE = 2_380`, scaled up for the larger VK
(kilobytes vs 192 B) and SHPLONK pairing cost. The cold path
includes `EvaluationDomain` construction at `k = 20` which
benchmarks at ~3 seconds on commodity hardware — that wall-clock
is **not** charged here. The operational assumption is that
operators pre-warm the per-VK cache at node startup so the
production hot path dominates.

**Open task:** measure warm + cold path wall-clock against a
realistic bridge Circuit 1B `(vk_cell, public_inputs_cell, proof_cell)`
triple (already checked in as `tvm_vm/halo2_test_data/fallback_*.bin`)
and either set the constant to the warm-path cost (and document the
cold-path DoS surface) or introduce a `k`-scaled / cold-path
surcharge.

## 9. Cross-references

- Wire-format producer: `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`
- Integration reference (consumer-side): `acki-nacki-bridge/docs/zkhalo2verifywithvk_reference.md`
- Assembler mnemonic: `tvm_assembler/src/simple.rs` (`ZKHALO2VERIFYWITHVK => 0xC7, 0x4A`)
- Round-trip test guarding mnemonic↔byte mapping: `tvm_assembler/src/lib.rs` → `gosh_zk_opcode_tests::zk_opcode_bytes_round_trip`
- Integration tests (12 — positive + 10 negative + real bridge Circuit 1B):
  `tvm_vm/src/tests/test_halo2_with_vk.rs`
- Real bridge Circuit 1B fixture (produced by
  `bridge-prover-orchestrator` round-trip test): 
  `tvm_vm/halo2_test_data/fallback_{vk_blob,public_inputs,proof}.bin`
