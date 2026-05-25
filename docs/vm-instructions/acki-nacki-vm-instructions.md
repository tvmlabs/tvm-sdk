# Acki Nacki VM Instructions

## MINTECC (C726)

Mint any ECC Token

```
Input: ECC KEY
```

Can be invoked only in special contracts. \
[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/a58e859e68e14a17a8acd2f142d260127a0a3f2d/tvm_assembler/src/simple.rs#L840)

## CNVRTSHELLQ (С727)

Converts SHELL to VMSHELL at a 1:1 ratio.

```
Input: amount of nanotokens to convert
```

Q in the end stands for ‘quiet’ which means that if there is not enough Shell, it will not throw an exception.

If the account balance does not have the required number of tokens, the exchange will be made for the entire available amount. That is, `MIN(available_tokens, want_cnt_to_convert)`. \
[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/a58e859e68e14a17a8acd2f142d260127a0a3f2d/tvm_assembler/src/simple.rs#L841)

## MINTSHELL (С728)

Mint some VMSHELL tokens, allowed by available credit in Dapp Config for this Dapp Id

```
Input: amount of nanoVMSHELL to mint
```

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_assembler/src/simple.rs#L842)

## CALCBKREWARD (С729)

Calculate reward for BK after epoch ended.

```
Input: params of bkroot state:
    uint128 numberOfActiveBlockKeepers,
    uint128 stake,
    uint128 totalStake,
    uint128 reputationTime,
    uint128 timenetwork,
    uint128 epochDuration
```

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_assembler/src/simple.rs#L843)

## CALCMINSTAKE (С730)

Calculate minstake for BK epoch start.

```
Input: params of bkroot state:
    uint128 epochDuration,
    uint128 timenetwork,
    uint128 numberOfActiveBlockKeepers,
    uint128 numberOfNeededActiveBlockKeeper
```

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_assembler/src/simple.rs#L844)

## VERGRTH16 (С731)

Verify Groth16 zero-knowledge proof prepared based on JWT token and extra salt password to prove that the user owns some OpenId account (Google, Facebook, Kakao accounts etc). Takes as input the proof, related public input Poseidon hash and index of verification key.

```
Input:
    uint32 vk_index,
    bytes public_inputs, // of length = 32 bytes
    bytes proof // of length = 128 bytes
```

```
Output:
    boolean value indicating if proof is valid or not.
```

Note: public\_inputs must be prepared using POSEIDON instruction.

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_assembler/src/simple.rs#L845)

## POSEIDON (С732)

Calculate POSEIDON hash function. This hash function is designed for now especially for ZkLogin protocol needs. It takes as input all public ZkLogin data related to OpenId authentication (i.e. some public fields of JWT token and extra public data).

```
Inputs:
    string zkaddr,
    uint256 ephimeral_pub_key,
    bytes modulus,
    uint64 max_epoch,
    string iss_base_64,
    uint8 index_mod_4,
    string header_base_64
```

```
Outputs:
    Poseidon hash (32 bytes array) of input data being sequentially concatenated.
```

Note: There is: zkaddr = Poseidon(JWT.stable\_id, JWT.iss, User salt password), where JWT.stable\_id and User salt password are secrets. ephimeral\_pub\_key is a temporary key that will be used sign transactions (i.e. the related secret key) till Unix timestamp max\_epoch (ephimeral\_pub\_key is embedded into JWT.nonce and JWT is a kind of TLS certificate for ephimeral\_pub\_key). modulus is RSA JWK public fresh modulus published by OpenId provider (the JWK is used to sign JWT tokens). iss\_base\_64, index\_mod\_4 is JWT public data describing OpenId provider. header\_base\_64 is JWT public data containing “kid” (key id) of JWK.\
[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_assembler/src/simple.rs#L846)

## ZKHALO2VERIFYWITHVK (С74A)

Native Halo2 SHPLONK proof verification where the verifying key and circuit configuration are supplied by the caller as a single self-describing operand, rather than baked into the node binary. Designed for the Acki Nacki ↔ Ethereum bridge: each deployed bridge contract carries its own immutable VK, so one opcode covers every circuit anyone deploys to AN without growing per-circuit code in the node.

```
Input:
    bundle_cell — Cell whose payload is a Halo2TvmBundle:
        magic            8 B   "HALO2TVM"
        version          1 B   0x01
        transcript_kind  1 B   0x00 = Blake2b SHPLONK (only flavour accepted today)
        reserved         6 B   zero (not validated; reserved for future flags)
        cfg_len          4 B   u32 LE
        config_json      cfg_len B   serde_json(BaseCircuitParams)
        vk_len           4 B   u32 LE
        vk_bytes         vk_len B    VerifyingKey<G1Affine>, SerdeFormat::RawBytes
                                     (curve-membership check ON every G1 element)
        inst_len         4 B   u32 LE
        instances_bytes  inst_len B  strict 32-byte little-endian Fr × N
                                     (no u64 shortcut; values ≥ Fr modulus rejected)
        prf_len          4 B   u32 LE
        proof_bytes      prf_len B   SHPLONK proof
```

```
Output:
    boolean value indicating if the proof is valid or not.
```

Notes:

- The opcode invokes `halo2_proofs::plonk::verify_proof::<KZGCommitmentScheme<Bn256>, VerifierSHPLONK, Challenge255, Blake2bRead, SingleStrategy>` directly. No `gosh-zk-snark-halo2-utils` runtime dependency.
- KZG SRS: only three points (`g[0]`, `g2`, `s_g2`) are embedded as constants in `zk_halo2_utils.rs`; `ParamsKZG<Bn256>` is reconstructed at runtime for any `k` via `from_parts`. No on-disk SRS blob is required for verification.
- Per-VK cache: bounded FIFO (8 entries) keyed by `vk_bytes`. `config.k` is cross-checked against `vk.get_domain().k()` on both the cold path (after VK::read) and the hot path (cache hit) as defence-in-depth against a malicious header that lies about `k`.
- DoS hardening: `VerifyingKey::read::<_, BaseCircuitBuilder<Fr>>` can panic deep inside `halo2-base` on adversarial `BaseCircuitParams` (e.g. `lookup_bits >= k`). The handler wraps the read in `std::panic::catch_unwind` and converts the unwind into a structural `FatalError`, so a single malicious bundle cannot abort the executor.
- Bundle parser rejects: bad magic, wrong version, wrong transcript-kind, length-prefix overrun, trailing garbage, instances length not a multiple of 32, and bundles over 16 MiB.
- Bytes-for-bytes the same wire format is emitted by the bridge producer at `acki-nacki-bridge/crates/bridge-prover-orchestrator/src/halo2_tvm_bundle.rs`.

Producer reference: `acki-nacki-bridge/docs/zkhalo2verifywithvk_reference.md`.
Design memo: [`docs/zkhalo2verifywithvk_design.md`](../zkhalo2verifywithvk_design.md).

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_assembler/src/simple.rs#L863)

## RUNWASM

Instruction allows arbitrary pre-compiled wasm code to be executed directly by the node.

```
Input:
    wasmHash,
    wasmArgs,
    wasmFunction,
    wasmModule,
    wasmBinary
```

\
You can find official documentation [here](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_vm/WASM.md) and example project [here](https://github.com/tvmlabs/tvm-sdk/blob/main/examples/wasm/WASM_EXAMPLES.md)\
[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/main/tvm_assembler/src/simple.rs#L853)
