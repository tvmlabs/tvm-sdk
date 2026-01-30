---
description: RootPrivateNote Contract Interface Documentation
---

# RootPN

{% file src="../../.gitbook/assets/RootPN.abi (1).json" %}

## Overview

**RootPN** is the system root contract responsible for deploying and managing `PrivateNote` contracts.\
It also stores and manages the canonical contract code for related system components, including prediction markets, oracles, and nullifiers.

The contract acts as a trusted entry point for:

* zero-knowledge–verified deposits
* deterministic deployment of `PrivateNote`
* system-wide code upgrades

***

## Events

### PrivateNoteDeployed

Emitted when a new `PrivateNote` contract is deployed.

```solidity
event PrivateNoteDeployed(
    uint256 depositIdentifierHash,
    address noteAddress,
    uint128 initialBalance
);
```

**Meaning:**

* A `PrivateNote` was successfully deployed
* The address is deterministically derived from the deposit identifier

***

### NullifierDeployed

Emitted when a `Nullifier` contract is deployed.

```solidity
event NullifierDeployed(
    address nullifierAddress,
    uint64 value
);
```

**Meaning:**

* A nullifier was created to prevent double-spending
* Funds were transferred to the associated `PrivateNote`

***

## Public & External Interface

### **`sendEccShellToPrivateNote`**

Sends ECC Shell tokens to a `PrivateNote` after zero-knowledge proof verification.

```solidity
function sendEccShellToPrivateNote(
    bytes proof,
    uint256 nullifier_hash,
    uint256 deposit_identifier_hash,
    uint64 value
) public;
```

**Parameters:**

* `proof` — zero-knowledge proof validating the deposit
* `nullifier_hash` — unique nullifier preventing double spend
* `deposit_identifier_hash` — deposit identifier hash
* `value` — amount of ECC Shell tokens to mint and transfer

**Behavior:**

* Verifies the proof using `zkhalo2verify`
* Mints ECC Shell tokens
* Deploys a `Nullifier` contract
* Forwards minted tokens to the corresponding `PrivateNote`
* Emits `NullifierDeployed`

***

### **`deployPrivateNote`**

Deploys a new `PrivateNote` contract after ZK verification.

```solidity
function deployPrivateNote(
    bytes zkproof,
    uint256 deposit_identifier_hash,
    uint256 ethemeral_pubkey,
    uint128 value,
    uint32 token_type
) public;
```

**Parameters:**

* `zkproof` — zero-knowledge proof of deposit validity
* `deposit_identifier_hash` — unique deposit identifier
* `ethemeral_pubkey` — public key for the note
* `value` — initial token balance
* `token_type` — token type identifier

**Behavior:**

* Verifies token type
* Validates the deposit via zero-knowledge proof
* Computes deterministic `PrivateNote` address
* Deploys the `PrivateNote` contract
* Emits `PrivateNoteDeployed`

***

### **`privateNoteDeployed`**

Records the deployment of a `PrivateNote`.

```solidity
function privateNoteDeployed(
    uint256 deposit_identifier_hash,
    uint32 token_type,
    uint128 deployed_value
) public;
```

**Access Control:**

* Callable only by the corresponding `PrivateNote` contract

**Behavior:**

* Updates internal accounting of deployed values by token type

***

## View Functions

### **`getPrivateNoteCode`**

Returns the salted `PrivateNote` contract code.

```solidity
function getPrivateNoteCode()
    external
    view
    returns (TvmCell privateNoteCode, uint256 privateNoteHash);
```

**Returns:**

* `privateNoteCode` — salted code cell
* `privateNoteHash` — hash of the salted code

***

### **`getDetails`**

Returns core RootPN state information.

```solidity
function getDetails()
    external
    view
    returns (
        uint256 pmpCodeHash,
        uint256 privateNoteCodeHash,
        uint256 ownerPubkey,
        uint128 balance
    );
```

**Returns:**

* hash of PMP code
* hash of PrivateNote code
* root owner public key
* current contract balance

***

### **`getVersion()`**

Returns version information for the RootPN contract.

```solidity
function getVersion() external pure returns (string, string);
```

**Returns:**

* semantic version string
* contract identifier: `"RootPN"`
