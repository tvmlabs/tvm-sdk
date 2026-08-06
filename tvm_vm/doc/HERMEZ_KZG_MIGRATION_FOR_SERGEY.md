# Hermez KZG Migration — Action List for Sergey Egorov

**Branch:** `feature/hermez-kzg-resurrection`
**Owner of this note:** Alina T.
**Written:** 2026-07-12

---

## TL;DR

The `KZG_S_G2_BYTES` in `tvm_vm/src/executor/zk_halo2_utils.rs` has been
restored to the correct **Hermez Perpetual Powers of Tau** value
(`powersOfTau28_hez_final_20.ptau`, halo2 raw SRS SHA-256
`80394564e2598883dbb5d7d61630287f34e29cdd806d7ef74f68acc6bffeb608`) and the
`DARK_DEX_W128_VK_BYTES` has been regenerated against that Hermez SRS.

Every proof / VK on the Acki Nacki side that was generated against the
previous (self-generated dark-dex-only, `gen_srs(K)` toxic-waste) SRS
**stops verifying** on this branch. That includes the deposit-prover
fixtures you own.

You need to regenerate three artefact families and update one Solidity
constant. Details below.

---

## 1. Why this is happening

An earlier commit (2ee96ca8, 2026-05-26) pasted the correct Hermez
constants into `zk_halo2_utils.rs`. A later merge (c249fb11, 2026-06-11)
silently clobbered them back to dark-dex `gen_srs(K)` toxic-waste bytes.
That is:

* **Insecure** — anyone with the `gen_srs(19)` RNG seed can forge
  proofs. Trapdoor `s` is knowable.
* **Non-portable** — the Ethereum-side verifier and the Acki Nacki–side
  verifier were supposed to share a ceremony. That contract is broken
  the moment the two sides drift.

The dark-dex prover has been re-keyed against Hermez in the
`dexdo-halo2-kit` repo. The tvm-sdk changes on this branch complete the
Acki Nacki side of the round-trip. Your bridge fixtures (deposit
proofs + deposit VK) are the only artefacts still on the old,
insecure SRS.

## 2. What has already been done on this branch

1. **`tvm_vm/src/executor/zk_halo2_utils.rs`** rewritten:
   * Single Hermez `KZG_G0_BYTES` / `KZG_G2_BYTES` / `KZG_S_G2_BYTES`.
   * Old `DARK_DEX_KZG_*` duplicate set removed.
   * `DARK_DEX_W128_VK_BYTES` replaced with the Hermez-anchored VK bytes
     emitted by `dex-halo2-circuit::gen_hermez_kzg_and_dark_dex_keys`
     (see `dexdo-halo2-kit/generated/dark_dex_w128_vk_bytes.rs`).
   * Local `build_kzg_verifier_params()` reimplementation deleted; the
     function now delegates to
     `gosh_zk_snark_halo2_utils::kzg_helper::build_kzg_verifier_params_from_points`
     — the same helper the bridge prover uses.

2. **`tvm_vm/src/executor/zk_halo2.rs`** — `build_kzg_verifier_params()`
   call updated to `build_kzg_verifier_params(19)` (new signature takes
   `k`).

3. **`tvm_vm/src/executor/zk_halo2_with_vk.rs`** — the local
   `build_shared_kzg_params(k)` was byte-for-byte identical to the shared
   helper; deleted and replaced with a delegated call to
   `crate::executor::zk_halo2_utils::build_kzg_verifier_params(k)`.

4. **`tvm_vm/halo2_test_data/dark_dex_w128_L{0,1,2,11}_{proof,instances}.bin`**
   regenerated against the Hermez SRS; L11 (MAX_CHAIN_LEN) added.

5. **`tvm_vm/src/tests/test_halo2.rs`** — new `test_verify_w128_l11_max_chain_steps`.

Everything in the `ZKHALO2VERIFY` (legacy DarkDex-VK path) is now
consistent end-to-end with the Hermez SRS.

## 3. What YOU need to do

### 3.1 Regenerate the tvm-sdk halo2 fixtures you added

The following files under `tvm_vm/halo2_test_data/` were added by you
(and are consumed by `test_halo2.rs` / `test_halo2_with_vk.rs` tests you
wired in). They were built against the OLD dark-dex `gen_srs(19)` SRS
and need to be rebuilt against `kzg_bn254_20.srs` (Hermez):

**Deposit circuit fixtures (your circuit, your fixtures):**
* `deposit_10proofs/*` — the 10-proof deposit-event fixture pack
  (`EthCircuitImpl<Fr, DepositEventCircuitV2>`, 11 public inputs).
* `deposit_rlc_*.bin` — any `deposit_rlc_proof.bin`,
  `deposit_rlc_instances.bin`, `deposit_rlc_vk.bin`, and the packed
  `deposit_vk_blob.bin` (the `VkBlob`-format wrapper carrying the VK
  plus the RLC `EthCircuitParams` JSON).

**Fallback (Circuit 1B) fixtures (my circuit, but *your* tvm-sdk
fixtures + tests):**
* `fallback_proof.bin`, `fallback_public_inputs.bin`, `fallback_vk.bin`,
  `fallback_vk_blob.bin`, `fallback_config_params.json` — regenerable
  from `acki-nacki-to-eth-bridge-halo2-circuits` (Circuit 1B Fallback
  BLS attestation verifier, K=20). The circuit code is mine and I can
  hand you a fresh keygen + prover run against Hermez on request; the
  packaging into tvm-sdk-shaped `fallback_vk_blob.bin` + repository
  commit is your side. Two positive-assertion fallback tests
  (`round_trip_fallback_circuit_valid_proof_returns_true`,
  `bridge_circuit_1b_fallback_real_proof_verifies`) plus one dependent
  test (`fifo_cache_reused_across_two_invocations`) are already
  `#[ignore]`'d in `test_halo2_with_vk.rs` from an earlier gosh
  halo2-lib bump — same root cause (KZG/fixture drift). Un-ignore them
  as part of this regeneration; the `#[ignore]` reason strings on those
  three predate this migration but resolve simultaneously.

Reproduction steps you already know — but for the record:

1. Bootstrap Hermez SRS locally:
   ```bash
   cd bridge/scripts
   ./bootstrap_hermez_srs.sh    # produces kzg_bn254_20.srs, sha256 80394564…
   ```
2. Downsize to whatever K your deposit circuit uses (e.g. `k=19`), then
   keygen and prove:
   ```bash
   cd bridge/deposit-prover
   cargo run --release --bin deposit_prover_keygen -- --srs …/kzg_bn254_20.srs
   cargo run --release --bin deposit_prover_batch -- --n 10 --out …/deposit_10proofs
   ```
   (Exact bin names / flags per your existing scripts. What matters is
   the SRS input.)
3. Repack the deposit VK into a v2 RLC `VkBlob` (magic `VKBLOB\x00\x00`,
   version `0x02`, `circuit_shape = 1`, `config_json = EthCircuitParams`
   JSON, `vk_bytes = VerifyingKey::to_bytes(SerdeFormat::RawBytes)`). Drop
   into `tvm_vm/halo2_test_data/deposit_vk_blob.bin` and equivalent
   locations under `deposit_10proofs/`.
4. Copy the regenerated files back into
   `tvm-sdk/tvm_vm/halo2_test_data/`.

### 3.2 Update `USDCBridge.sol`

`/Users/alinat/HALO2_TVM_EXPERIMENTS/acki-nacki/contracts/exchange/USDCBridge.sol`
carries the deposit `VkBlob` inline as
```solidity
bytes constant VK_BLOB = hex"564b424c4f4200000200010000000000ec0000…";
```
This value is a mirror of `deposit_vk_blob.bin`. After you regenerate
`deposit_vk_blob.bin` in step 3.1, replace `VK_BLOB` with the new hex
dump.

Sanity check: the first 8 bytes must stay `564b424c4f420000` (`VKBLOB\0\0`
magic); the next byte is version `0x02`; the following byte is
`circuit_shape = 0x01` (RLC).

Once updated, coordinate with **Sergey Gorelyshev** for on-chain
deployment.

### 3.3 Un-ignore the deposit + fallback tests

Nine tests total are currently `#[ignore]`'d because they consume the
old-SRS fixtures. Six were added in this branch (deposit, all with the
new "…regenerate against Hermez per doc/…" reason string); three were
already ignored on prior branches for the same root cause. Delete the
`#[ignore]` attribute on all nine once the corresponding fixtures are
regenerated:

**Deposit fixture set (`deposit_10proofs/*`) — added on this branch:**

| File | Test |
|---|---|
| `tvm_vm/src/tests/test_halo2.rs` | `test_zkhalo2_with_vk_deposit_10_real_proofs` |
| `tvm_vm/src/tests/test_halo2.rs` | `test_zkhalo2_with_vk_corrupt_proof_rejects` |
| `tvm_vm/src/tests/test_halo2.rs` | `test_zkhalo2_with_vk_mismatched_public_inputs_reject` |
| `tvm_vm/src/tests/test_halo2_with_vk.rs` | `round_trip_deposit_rlc_real_proof_returns_true` |
| `tvm_vm/src/tests/test_halo2_with_vk.rs` | `deposit_rlc_flipped_proof_byte_rejected_as_false` |
| `tvm_vm/src/tests/test_halo2_with_vk.rs` | `deposit_rlc_cache_reused_across_two_invocations` |

**Fallback fixture set (`fallback_*.bin`) — pre-existing ignores, same root cause:**

| File | Test |
|---|---|
| `tvm_vm/src/tests/test_halo2_with_vk.rs` | `round_trip_fallback_circuit_valid_proof_returns_true` |
| `tvm_vm/src/tests/test_halo2_with_vk.rs` | `bridge_circuit_1b_fallback_real_proof_verifies` |
| `tvm_vm/src/tests/test_halo2_with_vk.rs` | `fifo_cache_reused_across_two_invocations` |

### 3.4 Run the tests

Once fixtures + VK are regenerated and the six tests un-ignored:

```bash
cd tvm-sdk
cargo test -p tvm_vm --lib --features gosh test_halo2 -- --nocapture
cargo test -p tvm_vm --lib --features gosh test_halo2_with_vk -- --nocapture
```

Everything on this branch should pass end-to-end at that point.

## 4. What is NOT your responsibility

* `dark_dex_w128_L{0,1,2,11}_*.bin` — already regenerated on this branch
  (mine).
* `dark_dex_w128_config_params.json` — already byte-for-byte matches the
  regeneration; no change needed.
* `zk_halo2_utils.rs` DarkDex VK / KZG bytes — already updated on this
  branch (mine).
* Circuit 1B Fallback **Rust circuit code** in
  `acki-nacki-to-eth-bridge-halo2-circuits` — mine. I will hand over a
  fresh Hermez-anchored keygen + prover artefact set on request; the
  `fallback_*.bin` file layout in `tvm_vm/halo2_test_data/` and the
  tvm-sdk-side tests that consume it are your side.

## 5. Reference

* Hermez ceremony provenance: `bridge/scripts/bootstrap_hermez_srs.sh`
* Reference regeneration tool for the DarkDex side:
  `dexdo-halo2-kit/dex-halo2-circuit/src/bin/gen_hermez_kzg_and_dark_dex_keys.rs`
* Shared KZG helper (single source of truth for verifier param
  reconstruction): `gosh-zk-snark-halo2-utils/src/kzg_helper.rs` →
  `build_kzg_verifier_params_from_points`.

If anything above is unclear, please ping me before regenerating — the
old-vs-new SRS drift is subtle and I would rather answer questions than
have to debug a re-drift.
