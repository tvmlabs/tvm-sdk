# `ZKHALO2VERIFYWITHVK` — TVM Opcode Design Notes

**Status:** landed (this commit).
**Dispatch:** `0xC7 0x4A` (gosh-feature gated).
**Mnemonic:** `ZKHALO2VERIFYWITHVK`.

## 1. Purpose

Native verification of Halo2 SHPLONK proofs inside TVM where the verifying
key + circuit configuration are **caller-supplied** rather than baked into
the node binary. Each Acki Nacki ↔ Ethereum bridge `TokenBridge` (and any
other dApp using Halo2) carries its own VK; this opcode covers all of them
without growing per-circuit code in the node.

## 2. Stack ABI

```text
input : ...
        bundle_cell    ← top
output: ...
        ok: int (-1 = true, 0 = false)
```

A single `Cell` operand whose payload is a `Halo2TvmBundle` (see §3) is
consumed; a boolean is pushed.

Returns:

- **`Ok(int(-1))`** — proof verified.
- **`Ok(int(0))`** — proof rejected by Halo2 verifier (cryptographic
  reject; well-formed bundle, just doesn't satisfy the relation).
- **`Err(FatalError(…))`** — structural input error: bundle bytes don't
  parse, VK byte stream invalid, `BaseCircuitBuilder` panics on
  internally-inconsistent config, instance ≥ Fr modulus, or
  `config.k` disagrees with the VK's domain `k`.

The `Ok(false)` vs `FatalError` distinction matches `VERGRTH16`'s contract:
caller bugs throw, adversarial-but-valid proofs are silently rejected.

## 3. `Halo2TvmBundle` wire format (Q-WIRE-1…5, frozen 2026-05-22)

```text
┌────────────────────────────────────────────────────────────────────┐
│ 0   "HALO2TVM"                  8 B   magic                        │
│ 8   0x01                        1 B   version                      │
│ 9   0x00 = Blake2b              1 B   transcript-kind (Q-WIRE-1)   │
│ 10  0x00 × 6                    6 B   reserved (zero)              │
│ 16  cfg_len                     4 B   u32 little-endian            │
│ 20  config_json                cfg_len B   serde_json(BaseCircuitParams)
│ ..  vk_len                      4 B   u32 little-endian            │
│ ..  vk_bytes                    vk_len B   VerifyingKey<G1Affine>  │
│                                              SerdeFormat::RawBytes │
│ ..  inst_len                    4 B   u32 little-endian            │
│ ..  instances_bytes            inst_len B  strict 32-byte LE Fr*N  │
│ ..  prf_len                     4 B   u32 little-endian            │
│ ..  proof_bytes                prf_len B   SHPLONK proof           │
└────────────────────────────────────────────────────────────────────┘
```

Constraints checked on parse:

- Magic matches `b"HALO2TVM"`, version = `1`, transcript = `0`.
- Reserved bytes `[10..16]` are intentionally **not** validated by the parser — they exist for future-proofing (e.g. capability flags). Producers must emit them as zero, but consumers ignore the content.
- Each length prefix fits in remaining buffer (no overflow / overrun).
- No trailing garbage after `proof_bytes`.
- Sum of declared chunk sizes ≤ 16 MiB hard cap.
- `inst_len % 32 == 0`.

The producer side lives at
`acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`
and emits the exact same byte layout.

### Q-WIRE design pivots

| Q | Decision |
|---|----------|
| Q-WIRE-1 | Transcript kind byte is reserved for future hash agility; only `0x00` (Blake2b) is currently accepted. |
| Q-WIRE-2 | KZG SRS points (`g[0]`, `g2`, `s_g2`) are globally shared per `k` and embedded as constants in `zk_halo2_utils`. SHPLONK verification only needs those three points; the full multi-MB SRS blob is never loaded at runtime. |
| Q-WIRE-3 | Public inputs are **strict** 32-byte little-endian `Fr::from_repr`. No `u64` shortcut. Out-of-range bytes are a `FatalError`. |
| Q-WIRE-4 | Bundle is self-describing — magic + version + transcript-kind + reserved header, four `u32 LE` length-prefixed chunks. |
| Q-WIRE-5 | Halo2 stack (verifier transcript, multiopen, strategy) is fixed at `(Blake2b, SHPLONK, SingleStrategy)`. Adding GWC / Keccak transcript would require a new transcript kind byte. |

## 4. Implementation notes

- **VK cache:** bounded FIFO, 8 entries, keyed by `bundle.vk_bytes`. Eviction is oldest-insert-first. The cached value holds the deserialised `VerifyingKey<G1Affine>` and a verifier-only `ParamsKZG<Bn256>` rebuilt for the VK's domain `k` via `from_parts(k, vec![g0], Some(vec![]), g2, s_g2)`.
- **Config-`k` defence-in-depth:** both the cold path (after VK::read) and the hot path (cache hit) reject bundles whose `BaseCircuitParams.k` disagrees with `vk.get_domain().k()`. Catches a malicious bundle that reuses a benign VK byte-string but lies about `k` in the JSON header.
- **Panic safety:** `BaseCircuitBuilder::new(...)` (driven by `VerifyingKey::read::<_, BaseCircuitBuilder<Fr>>`) is not panic-safe for adversarial `BaseCircuitParams` — e.g. `lookup_bits >= k` triggers a `panic!` deep inside halo2-base. The handler wraps the read in `std::panic::catch_unwind` and converts the unwind into a structured `FatalError`, so a single malicious bundle cannot crash the executor thread.
- **VK curve checks:** `SerdeFormat::RawBytes` validates curve membership on every G1 element of the VK as it is read. BN254 G1 is prime-order (cofactor=1) so curve membership implies subgroup membership.
- **Verification call:** the handler invokes `halo2_proofs::plonk::verify_proof::<KZGCommitmentScheme<Bn256>, VerifierSHPLONK<'_, Bn256>, Challenge255<G1Affine>, Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>, SingleStrategy<'_, Bn256>>` directly — no `gosh-zk-snark-halo2-utils` dependency. The producer side uses the **same** generic parameters, so any well-formed proof from one is verifiable by the other.
- **Dependencies:** `tvm_vm/Cargo.toml` is unchanged from `origin/main`. The handler builds against public `halo2-base = "0.5.1"` from crates.io. No gosh-fork patch entries are required.

## 5. Gas model (Q-GAS-1, **must re-benchmark before mainnet**)

Current placeholder:

```rust
pub const ZKHALO2_VERIFY_WITH_VK_GAS_PRICE: i64 = 5_000;
```

This is a structural guess modelled on `VERGRTH16_GAS_PRICE = 2_380`, scaled
up for the larger VK (kilobytes vs 192 B) and SHPLONK pairing cost. The
cold path includes `EvaluationDomain` construction at `k = 20` which
benchmarks at ~3 seconds on commodity hardware — that wall-clock is
**not** charged here. The operational assumption is that operators
pre-warm the per-VK cache at node startup so the production hot path
dominates.

**Open task:** measure warm + cold path wall-clock against a realistic
bridge Circuit 1B bundle and either set the constant to the warm-path
cost (and document the cold-path DoS surface) or introduce a
`k`-scaled / cold-path surcharge.

## 6. Cross-references

- Wire-format producer: `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`
- Integration reference (consumer-side): `acki-nacki-bridge/docs/zkhalo2verifywithvk_reference.md`
- Assembler mnemonic: `tvm_assembler/src/simple.rs` (`ZKHALO2VERIFYWITHVK => 0xC7, 0x4A`)
- Round-trip test guarding mnemonic↔byte mapping: `tvm_assembler/src/lib.rs` → `gosh_zk_opcode_tests::zk_opcode_bytes_round_trip`
- Integration tests (positive + 10 negative): `tvm_vm/src/tests/test_halo2_with_vk.rs`
