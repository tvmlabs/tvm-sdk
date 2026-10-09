# ETH light-client `step` opcode fixture (consumer side)

These files exercise `ZKHALO2VERIFYWITHVK` in `tvm_vm` tests with the Ethereum
beacon **light-client sync-step** circuit
(`acki-nacki-bridge/eth-light-client-prover`, milestone M5). They are the
**three-operand ABI** only:

```text
vk_cell              ← step_vk_blob.bin (VkBlob v1 Base, Blake2b, 10 public inputs)
public_inputs_cell   ← step_public_inputs.bin (10 × 32 B LE Fr)
proof_cell           ← step_proof.bin (Blake2b SHPLONK)
```

Unlike the deposit fixtures (VkBlob **v2 RLC**, `BaseCircuitBuilder`-over-`Eth`),
the step circuit is a plain `BaseCircuitBuilder<Fr>` → VkBlob **v1 Base**
(`circuit_shape=0`, `transcript_kind=0` Blake2b). The opcode reads it via
`read_base_vk` + `VerifyingKey::read::<_, BaseCircuitBuilder<Fr>>`.

## SRS provenance

The proof is keyed on **Hermez** Perpetual Powers of Tau (k=19, downsized from
the local Hermez k=20 SRS). This matches the opcode's embedded
`KZG_S_G2_BYTES` (`92 8f af b3 …`, Hermez `[s]·G2`, valid for K ≤ 28). See
`eth-light-client-prover/docs/m5_vkblob.md`.

## Public-input layout (10 × 32 B LE Fr)

```text
[0] attested_slot             [1] finalized_slot
[2] finalized_beacon_root_hi  [3] finalized_beacon_root_lo
[4] participation             [5] committee_commitment   (2-level Poseidon)
[6] execution_block_hash_hi   [7] execution_block_hash_lo
[8] attested_state_root_hi    [9] attested_state_root_lo
```

`committee_commitment` at `[5]` is the **2-level** scheme
(`commit_sync_committee_2level`), byte-identical to the value the recursive
`rotate` proof emits as `next_commit` — so an on-chain rotate advances
`_currentCommittee` into exactly what the next step recomputes. Indices `[8|9]`
are the attested `state_root` that anchors the rotate committee branch.

(Exact semantics: `eth-light-client-prover/src/step.rs::pack_step_instances`.)

## Refresh from eth-light-client-prover

Producer-side artefacts + regeneration recipe live in
`acki-nacki-bridge/eth-light-client-prover/`:

```bash
# regenerate against Hermez k=19 SRS + opcode-faithful self-check
cd eth-light-client-prover && cargo run --release --example export_step_vk_blob
# then sync the three operands here:
scripts/sync_step_opcode_fixtures_to_tvm_sdk.sh
```

Byte-identity with `eth-light-client-prover/fixtures/step_vkblob/` is enforced by
the SHA-256 sidecars there:

| file | sha256 |
|------|--------|
| `step_vk_blob.bin` | `bd108c08…7d21bab0` |
| `step_public_inputs.bin` | `493c95ce…5bf87703` |
| `step_proof.bin` | `54eec6d8…eddb1960` |
