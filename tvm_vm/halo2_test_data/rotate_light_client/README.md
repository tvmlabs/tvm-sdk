# ETH light-client recursive `rotate` opcode fixture (consumer side)

These files exercise `ZKHALO2VERIFYWITHVK` in `tvm_vm` tests with the Ethereum
beacon **light-client committee-rotation** proof
(`acki-nacki-bridge/eth-light-client-prover`, milestone M6). They are the
**three-operand ABI** only:

```text
vk_cell              ← rotate_vk_blob.bin (VkBlob v1 Base, Blake2b, 15 public inputs)
public_inputs_cell   ← rotate_public_inputs.bin (15 × 32 B LE Fr)
proof_cell           ← rotate_proof.bin (Blake2b SHPLONK)
```

The root is a **2-to-1 recursion tree** over 8 committee shards + a step snark
(`examples/rotate_tree_n8.rs`). It is a plain `BaseCircuitBuilder<Fr>` → VkBlob
**v1 Base** (`circuit_shape=0`, `transcript_kind=0` Blake2b), read by the opcode
via `read_base_vk` + `VerifyingKey::read::<_, BaseCircuitBuilder<Fr>>`.

## ⚠ Soundness caveat — opcode acceptance is NECESSARY but NOT sufficient

`ZKHALO2VERIFYWITHVK` performs a plain SHPLONK `verify_proof` (`SingleStrategy`,
Blake2b). It does **not** pair `instances[0..12]` (the KZG accumulator) against
the embedded `[s]·G2`, so it accepts the aggregation proof without checking that
the folded inner shard/step proofs are valid. These tests prove the opcode
**reads and verifies** the blob; full soundness of the recursion requires the
partner opcode **decider extension** (~20 lines, pair `instances[0..12]` vs the
Hermez `[s]·G2`). Validated reference:
`acki-nacki-bridge/eth-light-client-prover/examples/rotate_decider_check.rs`.

## SRS provenance

The proof is keyed on **Hermez** Perpetual Powers of Tau (outer k=21; shard
inners k=20). This matches the opcode's embedded `KZG_S_G2_BYTES`
(`92 8f af b3 …`, Hermez `[s]·G2`, valid for K ≤ 28).

## Public-input layout (15 × 32 B LE Fr)

```text
[0..12)  KZG accumulator limbs (lhs/rhs G1, 3×88-bit limbs per coord)
[12]     current_commit  (2-level Poseidon; snark-bound to step committee_commitment)
[13]     next_commit      (2-level Poseidon of the rotated-in committee)
[14]     period           (sync-committee period)
```

(Exact semantics: `eth-light-client-prover/src/rotate_aggregation.rs`.)

## Refresh from eth-light-client-prover

```bash
# regenerate the root proof + ROTATE_VK_BLOB on a big-RAM host (Hermez SRS):
cd eth-light-client-prover && \
  EMIT_VKBLOB=1 cargo run --release --features aggregation --example rotate_tree_n8
# then sync the three operands here:
scripts/sync_rotate_opcode_fixtures_to_tvm_sdk.sh
```

Byte-identity with `eth-light-client-prover/fixtures/rotate_vkblob/` is enforced
by the SHA-256 sidecars there:

| file | sha256 |
|------|--------|
| `rotate_vk_blob.bin` | `037cd274…4c57b81e` |
| `rotate_public_inputs.bin` | `569bee20…eab62a05e` |
| `rotate_proof.bin` | `d969978c…62841dd4` |

The VkBlob header carries `accumulator_limbs = 12` (byte 11): the opcode runs the
KZG accumulation decider `e(lhs, g2) == e(rhs, s_g2)` over `instances[0..12]`
after the SHPLONK verify.
