---
description: RootPN Contract Interface Documentation
---

# RootPN

## Overview

**RootPN** is the root contract responsible for deploying and coordinating the **PrivateNote**, **Nullifier**, and **Oracle** contracts. It stores their code, manages upgrades, and aggregates deployment statistics.

***

## Events

### `PrivateNoteDeployed`

```solidity
event PrivateNoteDeployed(
    uint256 depositIdentifierHash,
    address noteAddress,
    uint128 initialBalance
);
```

Emitted after a successful deployment of a new `PrivateNote`.

**Parameters:**

* `depositIdentifierHash` — Unique hash identifying the deposit.
* `noteAddress` — Deterministic address of the deployed `PrivateNote`.
* `initialBalance` — Initial balance assigned to the note.

***

### `NullifierDeployed`

```solidity
event NullifierDeployed(
    address nullifierAddress,
    uint64 value
);
```

Emitted after deploying a `Nullifier` contract.

**Parameters:**

* `nullifierAddress` — _Note:_ the emitted value is the associated `PrivateNote` address, not the actual `Nullifier` address.
* `value` — Amount minted and transferred during deployment.

> ⚠️ **Important:**\
> Despite the event name, `nullifierAddress` contains the **PrivateNote address** passed during emission.

***

## Constructor

#### `constructor()`

Initializes the contract and accepts the deployment transaction.

***

## Public Functions

### `sendEccShellToPrivateNote`

```solidity
function sendEccShellToPrivateNote(
    bytes proof,
    uint256 nullifier_hash,
    uint256 deposit_identifier_hash,
    uint64 value
) public view accept;
```

Verifies a zero-knowledge proof, mints ECC Shell tokens, and deploys a `Nullifier` contract bound to a `PrivateNote`.

**Parameters:**

* `proof` — Zero-knowledge proof.
* `nullifier_hash` — Hash used to initialize the `Nullifier`.
* `deposit_identifier_hash` — Deposit identifier used to derive the `PrivateNote` address.
* `value` — Amount to mint and transfer.

**Requirements:**

* ZK proof must be valid for `CURRENCIES_ID_SHELL`.
* Contract balance must meet the minimum required level.

**Side Effects:**

* Mints ECC Shell tokens.
* Deploys a `Nullifier` contract.
* Emits `NullifierDeployed`.

***

### `deployPrivateNote`

```solidity
function deployPrivateNote(
    bytes zkproof,
    uint256 deposit_identifier_hash,
    uint256 ethemeral_pubkey,
    uint128 value,
    uint32 token_type
) public view accept;
```

Deploys a new `PrivateNote` contract after validating token type and ZK proof.

**Parameters:**

* `zkproof` — Zero-knowledge proof.
* `deposit_identifier_hash` — Unique deposit identifier.
* `ethemeral_pubkey` — Ephemeral public key for the note.
* `value` — Initial token balance.
* `token_type` — Token type (must equal `CURRENCIES_ID`).

**Requirements:**

* `token_type` must match `CURRENCIES_ID`.
* ZK proof must be valid.
* Contract must have sufficient balance.

**Side Effects:**

* Deploys a `PrivateNote` contract.
* Emits `PrivateNoteDeployed`.

***

### `privateNoteDeployed`

```solidity
function privateNoteDeployed(
    uint256 deposit_identifier_hash,
    uint32 token_type,
    uint128 deployed_value
) public accept;
```

Records a successful `PrivateNote` deployment and aggregates deployed values.

**Parameters:**

* `deposit_identifier_hash` — Deposit identifier.
* `token_type` — Token type being tracked.
* `deployed_value` — Amount to add to aggregated statistics.

**Access Control:**

* Callable **only** by the `PrivateNote` contract whose address is derived from `deposit_identifier_hash`.

**Side Effects:**

* Increments `_deployedValues[token_type]`.

***

### `deployOracle`

```solidity
function deployOracle(
    uint256 oraclePubkey,
    string oracleName
) public view accept;
```

Deploys a new `Oracle` contract.

**Parameters:**

* `oraclePubkey` — Oracle public key.
* `oracleName` — Human-readable oracle name.

**Access Control:**

* Owner only (`onlyOwnerPubkey`).

***

### `updateCode`

```solidity
function updateCode(
    TvmCell newcode,
    TvmCell cell
) public accept;
```

Upgrades the RootPN contract code.

**Parameters:**

* `newcode` — New contract code.
* `cell` — Encoded data for restoring state after upgrade.

**Access Control:**

* Owner only.

**Side Effects:**

* Replaces contract code.
* Resets storage and reinitializes state via `onCodeUpgrade`.

***

### `getPrivateNoteCode`

```solidity
function getPrivateNoteCode()
    external view
    returns (TvmCell privateNoteCode, uint256 privateNoteHash);
```

Returns the salted `PrivateNote` code and its hash.

**Returns:**

* `privateNoteCode` — Salted `TvmCell` of `PrivateNote`.
* `privateNoteHash` — Hash of the salted code.

***

### `getDetails`

```solidity
function getDetails()
    external view
    returns (
        uint256 pmpCodeHash,
        uint256 privateNoteCodeHash,
        uint256 ownerPubkey,
        uint128 balance
    );
```

Returns core contract metadata.

**Returns:**

* `pmpCodeHash` — Hash of PMP contract code.
* `privateNoteCodeHash` — Hash of `PrivateNote` code.
* `ownerPubkey` — Root owner public key.
* `balance` — Current contract balance.

***

### `getVersion`

```solidity
function getVersion() external pure returns (string);
```

Returns the contract version identifier.

**Returns:**

* `"RootPN"`

***

## Notes on Access Control and Roles

**Owner:**\
The `deployOracle` and `updateCode` functions are protected by `onlyOwnerPubkey(_ownerPubkey)`.

**PrivateNote-only:**\
The `privateNoteDeployed` function is protected by `senderIs(...)` and accepts calls only from the expected note address, which is calculated from `deposit_identifier_hash`
