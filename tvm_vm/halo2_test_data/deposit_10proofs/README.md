# Deposit opcode fixtures (consumer side)

These files exercise `ZKHALO2VERIFYWITHVK` in `tvm_vm` tests. They are the
**three-operand ABI** only:

```text
vk_cell              ← deposit_vk_blob.bin (VkBlob v2 RLC, 12 public inputs)
public_inputs_cell   ← proof_NN/public_inputs.bin (12 × 32 B LE Fr = 384 B)
proof_cell           ← proof_NN/proof.bin (Blake2b SHPLONK)
```

## KZG ceremony

Keyed on the **Hermez Perpetual Powers of Tau** SRS (`data/kzg_params_18.srs`,
`s_g2 = 928fafb3…`), matching `KZG_S_G2_BYTES` on
`feature/hermez-kzg-resurrection`. Regenerated 2026-07-29 via
`deposit-prover` `export_deposit_proof_set` after the circuit gained a
`chainId` public input at index 4 (bridge branch
`pruvendo/pause-chainid-pi-multil2`, commit `c0da8a3`). VkBlob sha256
`006cca5ddd457065bef8469fc10dd231ad8fd13c895877c9deff5c51191dec05` —
update this line whenever the fixtures are resynced.

Public-input layout (12 × 32 B LE Fr, in order):

```
0  depositId
1  sender
2  amount
3  contractAddress
4  chainId              ← added 2026-07-22 (bound via EIP-1559 tx MPT proof)
5  dappIdHigh
6  dappIdLow
7  anAccountHigh
8  anAccountLow
9  blockHashHigh
10 blockHashLow
11 promiseCommit        (auto-appended by axiom-eth EthCircuitImpl)
```

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
  --degree 18 --max-data-byte-len 256 --max-log-num 20 \
  --chain-id 11155111    # Sepolia; select source chain for the fetch layer
../scripts/sync_deposit_opcode_fixtures_to_tvm_sdk.sh
```

Byte-identity with `deposit-prover/fixtures/deposit_10proofs/` is enforced by the
`test_zkhalo2_with_vk_deposit_10_real_proofs` suite.
