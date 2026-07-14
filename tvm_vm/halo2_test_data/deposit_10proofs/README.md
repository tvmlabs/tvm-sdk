# Deposit opcode fixtures (consumer side)

These files exercise `ZKHALO2VERIFYWITHVK` in `tvm_vm` tests. They are the
**three-operand ABI** only:

```text
vk_cell              ← deposit_vk_blob.bin (VkBlob v2 RLC, 11 public inputs)
public_inputs_cell   ← proof_NN/public_inputs.bin (11 × 32 B LE Fr)
proof_cell           ← proof_NN/proof.bin (Blake2b SHPLONK)
```

## KZG ceremony

Keyed on the **Hermez Perpetual Powers of Tau** SRS (`data/kzg_params_18.srs`,
`s_g2 = 928fafb3…`), matching `KZG_S_G2_BYTES` on
`feature/hermez-kzg-resurrection`. Regenerated 2026-07-13 via
`deposit-prover` `export_deposit_proof_set` (VkBlob sha256
`304c1c4ed1e4cf09a00fb1d83a0ae2ba42db2afead035ae089a4faa85346251a`).

## What does **not** belong here

Producer-side artefacts stay in `acki-nacki-bridge/deposit-prover/fixtures/`:

| Artefact | Why not in tvm-sdk |
|----------|-------------------|
| `proof_NN/input.json` | Full Ethereum witness — only needed to **generate** proofs |
| `deposit_eth_circuit_params.json` | Archive copy; same JSON is embedded inside `deposit_vk_blob.bin` |
| `kzg_params_*.srs` / `kzg_bn254_*.srs` | Full SRS for keygen/prove; the opcode builds verifier params from 3 embedded KZG points |

## Refresh from deposit-prover

From `acki-nacki-bridge` (sibling repo):

```bash
# Force Hermez (default after hermez-kzg-resurrection); do NOT set DEPOSIT_USE_CHAIN_SRS
cd deposit-prover
cargo run --release --example export_deposit_proof_set -- \
  --set-dir fixtures/deposit_10proofs --count 10 \
  --degree 18 --max-data-byte-len 256 --max-log-num 20
../scripts/sync_deposit_opcode_fixtures_to_tvm_sdk.sh
```

Byte-identity with `deposit-prover/fixtures/deposit_10proofs/` is enforced by the
`test_zkhalo2_with_vk_deposit_10_real_proofs` suite.
