---
description: (Work in progress) RootPrivateNote Contract Interface Documentation
---

# RootPN

{% file src="../../.gitbook/assets/RootPN.abi (1).json" %}

## Overview

`RootPN` is the system root contract responsible for deploying and managing `PrivateNote` contracts.\
It also stores and manages the canonical contract code for related system components, including Pari Mutuel Pool, Oracles, and Nullifiers.

The contract acts as a trusted entry point for:

* zero-knowledge–verified deposits
* deterministic deployment of `PrivateNote`
* system-wide code upgrades

***

## Events

### vaucherGenerated

Emitted when a new voucher is generated.

```solidity
event vaucherGenerated(uint256 sk_u_commit, uint vaucher_nominal, uint32 token_type);
```

* `sk_u_commit` — Commitment of the user secret key
* `vaucher_nominal` — Voucher nominal value
* `token_type` — Token type associated with the voucher

### PrivateNoteDeployed

Emitted when a new `PrivateNote` contract is deployed.

```solidity
event PrivateNoteDeployed(
    uint256 depositIdentifierHash,
    address noteAddress,
    uint128 initialBalance
);
```

* `depositIdentifierHash` — Deposit identifier hash
* `noteAddress` — Deployed `PrivateNote` address
* `initialBalance` — Initial token balance

***

### NullifierDeployed

Emitted when a `Nullifier` contract is deployed.

```solidity
event NullifierDeployed(
    address nullifierAddress,
    uint64 value
)
```

* `nullifierAddress` — Address associated with the deployment
* `value` — Value linked to the nullifier

**Meaning:**

* A nullifier was created to prevent double-spending
* Funds were transferred to the associated `PrivateNote`

***

## Public & External Interface

### **`sendEccShellToPrivateNote`**

Verifies a zero-knowledge proof and deploys a `Nullifier` contract associated with a deterministic `PrivateNote` address.

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
    uint64 value,
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

* Ensures the root contract has sufficient native balance
* Builds ZK public inputs from:
  * fixed zero padding
  * `value` (encoded into 8 bytes)
  * more zero padding
  * `token_type` (encoded into 8 bytes)
  * `deposit_identifier_hash` (encoded into 32 bytes)
* Verifies the proof using `gosh.zkhalo2verify(pub_inputs, zkproof)`
* Deploys the `PrivateNote` contract deterministically from `deposit_identifier_hash`
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

### **`getPrivateNoteAddress`**

Returns the deterministic address of a `PrivateNote` contract for a given deposit identifier hash.

```solidity
function getPrivateNoteAddress(uint256 deposit_identifier_hash)
external
view
returns(address privateNoteAddress);
```

**Parameters:**

* `deposit_identifier_hash` — unique deposit identifier hash used to derive the PN address

**Returns:**

* `privateNoteAddress` — deterministic `PrivateNote` address for this deposit identifier

**Notes:**

* The address is computed deterministically and can be obtained even if the `PrivateNote` is not deployed yet.
* Intended for off-chain tooling, indexers, and integrations.

### `getPMPAddress`

Returns the deterministic address of a `PMP` contract for the given event, token type, and oracle name set.

```solidity
function getPMPAddress(
    uint256 event_id,
    string[] names,
    uint32 token_type
) external view
returns(address pmpAddress);
```

**Parameters:**

* `event_id` — event identifier used by the PMP (e.g., hash of the pool name)
* `names` — list of oracle names participating in the pool; used to build the oracle list hash
* `token_type` — token type used by the PMP

**Returns:**

* `pmpAddress` — deterministic PMP address computed from the provided inputs

**Notes:**

* The oracle list hash is computed from the set of oracle name hashes.
* The returned address matches the address used when deploying PMP from `PrivateNote`.

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

Returns version information for the `RootPN` contract.

```solidity
function getVersion() external pure returns (string, string);
```

**Returns:**

* semantic version string
* contract identifier: `"RootPN"`
