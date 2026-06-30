# Deposit opcode fixtures (consumer side)

These files exercise `ZKHALO2VERIFYWITHVK` in `tvm_vm` tests. They are the
**three-operand ABI** only:

```text
vk_cell              ← deposit_vk_blob.bin (VkBlob v2 RLC, 11 public inputs)
public_inputs_cell   ← proof_NN/public_inputs.bin (11 × 32 B LE Fr)
proof_cell           ← proof_NN/proof.bin (Blake2b SHPLONK)
```

## What does **not** belong here

Producer-side artefacts stay in `acki-nacki-bridge/deposit-prover/fixtures/`:

| Artefact | Why not in tvm-sdk |
|----------|-------------------|
| `proof_NN/input.json` | Full Ethereum witness (block header RLP, receipt MPT, …) — only needed to **generate** proofs |
| `deposit_eth_circuit_params.json` | Archive copy; the same JSON is embedded inside `deposit_vk_blob.bin` |
| `kzg_bn254_*.srs` | Chain ceremony SRS for keygen/prove; the opcode builds verifier params from 3 embedded KZG points |

## Refresh from deposit-prover

From `acki-nacki-bridge` (sibling repo):

```bash
scripts/sync_deposit_opcode_fixtures_to_tvm_sdk.sh
```

Or manually copy only:

- `deposit_vk_blob.bin`
- `proof_{00..09}/{public_inputs,proof}.bin`

Byte-identity with `deposit-prover/fixtures/deposit_10proofs/` is enforced by the
`test_zkhalo2_with_vk_deposit_10_real_proofs` suite.
