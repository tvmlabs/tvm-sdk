# Gosh VM Instructions



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

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/a58e859e68e14a17a8acd2f142d260127a0a3f2d/tvm_assembler/src/simple.rs#L842)

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

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/a58e859e68e14a17a8acd2f142d260127a0a3f2d/tvm_assembler/src/simple.rs#L843)

## CALCMINSTAKE (С730)

Calculate minstake for BK epoch start.

```
Input: params of bkroot state:
    uint128 epochDuration,
    uint128 timenetwork,
    uint128 numberOfActiveBlockKeepers,
    uint128 numberOfNeededActiveBlockKeeper
```

[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/a58e859e68e14a17a8acd2f142d260127a0a3f2d/tvm_assembler/src/simple.rs#L844)

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
[link to the opcode](https://github.com/tvmlabs/tvm-sdk/blob/vrgth_fixes_and_tests/tvm_assembler/src/simple.rs#L846)
