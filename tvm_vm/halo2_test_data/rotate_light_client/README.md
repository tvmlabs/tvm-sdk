# ETH light-client committee `rotate` opcode fixture (consumer side)

These files exercise `ZKHALO2VERIFYWITHVK` in `tvm_vm` tests with the Ethereum
beacon **light-client committee-rotation** proof
(`acki-nacki-bridge/eth-light-client-prover`). They are the
**three-operand ABI** only:

```text
vk_cell              ← rotate_vk_blob.bin (VkBlob v1 Base, Blake2b, 17 public inputs)
public_inputs_cell   ← rotate_public_inputs.bin (17 × 32 B LE Fr)
proof_cell           ← rotate_proof.bin (Blake2b SHPLONK)
```

The proof is one aggregation (k=21) over the committee circuit
(`src/committee_table.rs`, k=18 on the zkevm SHA-256 table), produced by
`examples/rotate_committee.rs`. It is a plain `BaseCircuitBuilder<Fr>` → VkBlob
**v1 Base** (`circuit_shape=0`, `transcript_kind=0` Blake2b), read by the opcode
via `read_base_vk` + `VerifyingKey::read::<_, BaseCircuitBuilder<Fr>>`.

The current committee is not in this proof: the light client's `submitRotate`
verifies an ordinary step proof of the hand-off update in the same transaction
and requires both proofs to name the same attested slot, state root and Gloas
fork slot.

## Soundness

The VkBlob header carries `accumulator_limbs = 12` (byte 11), so after the
SHPLONK verify the opcode runs the KZG accumulation decider
`e(lhs, g2) == e(rhs, s_g2)` over `instances[0..12]` against the embedded
Hermez `[s]·G2`. That pairing is what checks the folded committee proof; the
decider is unit-tested in `zk_halo2_with_vk::decider_tests`.

## SRS provenance

The proof is keyed on **Hermez** Perpetual Powers of Tau (outer k=21, inner
k=18 cut from the same file). This matches the opcode's embedded
`KZG_S_G2_BYTES` (`92 8f af b3 …`, Hermez `[s]·G2`, valid for K ≤ 28).

## Public-input layout (17 × 32 B LE Fr)

```text
[0..12)  KZG accumulator limbs (lhs/rhs G1, 3×88-bit limbs per coord)
[12]     next_commit      (2-level Poseidon of the rotated-in committee)
[13]     state_root_hi    (attested beacon state root, high 128 bits)
[14]     state_root_lo    (attested beacon state root, low 128 bits)
[15]     attested_slot
[16]     gloas_fork_slot
```

The fixture is the live Sepolia hand-off of period 1403 → 1404
(`attested_slot` 11493440, `gloas_fork_slot` 11296768).

## Refresh from eth-light-client-prover

```bash
# regenerate the proof + ROTATE_VK_BLOB (about 6 min and 16 GB RAM, Hermez SRS):
cd eth-light-client-prover && ROTATE_OUT=fixtures/rotate_vkblob \
  OUTER_SRS=../params/kzg_bn254_21.srs ROTATE_UPDATE_PATH=<update.json> \
  cargo run --release --features aggregation --example rotate_committee
# then sync the three operands here:
scripts/sync_rotate_opcode_fixtures_to_tvm_sdk.sh
```

Byte-identity with `eth-light-client-prover/fixtures/rotate_vkblob/` is enforced
by the SHA-256 sidecars there:

| file | sha256 |
|------|--------|
| `rotate_vk_blob.bin` | `6fd3b8b4…7bdb6ad86` |
| `rotate_public_inputs.bin` | `393308ee…f401763424` |
| `rotate_proof.bin` | `be5d4b78…d1504169` |
