---
description: PrivateNote Contract Interface Documentation
---

# PrivateNote

{% file src="../../.gitbook/assets/PrivateNote.abi (1).json" %}

## Overview

**PrivateNote** is a non-custodial wallet contract bound to a single deposit identifier.\
It allows users to deploy and interact with **Prediction Market Pool (PMP)** contracts, manage stakes, claim rewards, and withdraw tokens — all while preserving privacy guarantees.

Each `PrivateNote`:

* is deterministically deployed by `RootPN`
* is controlled by an **public key**
* maintains internal token balances
* enforces single-operation-at-a-time semantics via a “busy” lock

***

## Events

### OwnerChanged

Emitted when the owner public key is updated.

```solidity
event OwnerChanged(uint256 oldPubkey, uint256 newPubkey);
```

* `oldPubkey` - Previous public key
* `newPubkey` - New public key

***

### StakeConfirmed

Emitted after a stake is successfully accepted by a PMP.

```solidity
event StakeConfirmed(address stakeController, uint32 outcome, uint128 amount);
```

* `stakeController` - Address of the PMP contract that accepted the stake
* `outcome` - Outcome identifier the stake was placed on
* `amount` - Amount of tokens added to the stake

***

### StakeCancelled

Emitted after a stake is cancelled and refunded.

```solidity
event StakeCancelled(address stakeController, uint32 outcome, uint128 amount);
```

* `stakeController` - Address of the PMP contract that cancelled the stake
* `outcome` - Outcome identifier of the cancelled stake
* `amount` - Amount of tokens returned

***

### ClaimAccepted

Emitted when a claim is resolved and payout is credited.

```solidity
event ClaimAccepted(address stakeController, optional(uint32) outcome, uint128 payout);
```

* `stakeController` - Address of the PMP contract
* `outcome` - Final resolved outcome. Empty if the event is not yet resolved
* `payout` - Amount of tokens paid to the wallet

***

### PMPDeployed

Emitted when a new PMP contract is deployed.

```solidity
event PMPDeployed(
    string name,
    uint32 token_type,
    address pmpAddress,
    address[] oracleEventLists,
    uint128[] oracleFee
);
```

* `name` - Name of the PMP event
* `token_type` - Token type used for staking
* `pmpAddress` - Address of the deployed PMP contract
* `oracleEventLists` - Oracle event list contract addresses
* `oracleFee` - Oracle fee values corresponding to each oracle

***

## Public & External Interface

### **`changeOwner`**

Changes the public key controlling the wallet.

```solidity
function changeOwner(uint256 new_pubkey) external;
```

**Parameters:**

* `new_pubkey`— Public key of the new owner

**Access Control:**

* Must be signed by the current public key

**Effects:**

* Updates owner key
* Emits `OwnerChanged`

***

### **`deployPMP`**

Deploys a new PMP contract associated with this wallet

```solidity
function deployPMP(
    string name,
    uint128[] oracleFee,
    uint32 token_type,
    string[] names,
    uint128[] index
)
```

**Parameters:**

* `name` — unique PMP name
* `oracleFee` — oracle fees per oracle
* `token_type` — token used for staking
* `names` — oracle names
* `index` — oracle event list indices

**Behavior:**

* Computes oracle list hash
* Calculates required oracle fees
* Deploys PMP deterministically
* Emits `PMPDeployed`

***

### **`setStake`**

Places or increases a stake on a PMP

```solidity
function setStake(
    string name,
    uint256 oracle_list_hash,
    uint32 token_type,
    uint32 outcome,
    uint128 amount
) public;
```

**Parameters:**

* `name` - Name of the `PMP` event.\
  Used to derive the unique event identifier and the target `PMP` contract address.
* `oracle_list_hash` - Hash of the oracle list configuration associated with the `PMP` event.\
  Must match the oracle list used when the `PMP` contract was deployed.
* `token_type` - Token type used for staking.\
  Must correspond to a token balance available in the wallet.
* `outcome` - Identifier of the outcome being staked on.\
  The meaning of the outcome is defined by the `PMP` contract logic.
* `amount` - Amount of tokens to stake.\
  Must be greater than zero and less than or equal to the available balance for the specified token type.

**Rules:**

* Wallet must not be busy
* Balance must be sufficient
* Stake is first stored as `candidate_amount`
* `PMP` confirmation is required

***

### **`cancelStake`**

Cancels an existing stake.

```solidity
function cancelStake(
string name,
uint256 oracle_list_hash,
uint32 token_type,
uint32 outcome
) public;
```

**Parameters**

* `name` - Name of the `PMP` event.\
  Used to derive the event identifier and the address of the target `PMP` contract.
* `oracle_list_hash` - Hash of the oracle list configuration associated with the `PMP` event.\
  Must match the oracle list hash of the existing stake.
* `token_type` - Token type used for the stake being cancelled.\
  Must correspond to the token type of the existing stake entry.
* `outcome` - Identifier of the outcome for which the stake was placed.\
  Specifies which stake position should be cancelled.

**Behavior:**

* Calls `PMP` to cancel stake
* Locks wallet until response

***

### **`deleteStake`**

Deletes a local stake record without interacting with PMP.

```solidity
function deleteStake(
string name,
uint256 oracle_list_hash,
uint32 token_type,
uint32 outcome
) public;
```

**Parameters:**

* `name` - Name of the `PMP` event.\
  Used to derive the internal stake identifier stored in the wallet.
* `oracle_list_hash` - Hash of the oracle list configuration associated with the stake.\
  Must match the oracle list hash used when the stake entry was created.
* `token_type` - Token type of the stake being deleted.\
  Used to identify the correct local stake record.
* `outcome` - Identifier of the outcome associated with the stake.\
  Specifies which stake record should be removed.

**Use case:**

* Cleanup after expiration or full resolution

***

### **`claim`**

Claims winnings from a resolved `PMP`.

```solidity
function claim(
    string name,
    uint256 oracle_list_hash,
    uint32 token_type,
    uint32 outcome
) public;
```

**Parameters**

* `name` - Name of the `PMP` event.\
  Used to derive the event identifier and the address of the target `PMP` contract.
* `oracle_list_hash` - Hash of the oracle list configuration associated with the `PMP` event.\
  Must match the oracle list hash of the existing stake.
* `token_type` - Token type used for the stake.\
  Determines which internal balance will receive the payout.
* `outcome` - Outcome identifier originally selected by the user when placing the stake.\
  Used to identify the stake position being claimed.

***

### **`withdrawTokens`**

Withdraws tokens from the wallet via the `Vault`.

```solidity
function withdrawTokens(
    uint8 flags,
    address dest_wallet_addr,
    uint32 token_type
) public;
```

**Parameters**

* `flags` - Transfer flags passed to the Vault contract.\
  Control message delivery behavior during the token transfer.
* `dest_wallet_addr` - Destination wallet address that will receive the withdrawn tokens.
* `token_type` - Type of token to withdraw.\
  All available balance for this token type will be withdrawn.

**Behavior:**

* Transfers full balance of the given token type
* Resets local balance to zero

***

### Bounce Handling

#### onBounce

Handles bounced PMP calls.

**Behavior:**

* Clears busy state
* Restores candidate stake amount
* Ensures balance consistency

***

## View Functions

### **`getPMPCode()`**

Returns salted PMP code and its hash.

***

### **`getDetails()`**

Returns current wallet state:

* deposit identifier hash
* public key
* token balances
* PMP code hash
* PrivateNote code hash
* busy PMP address (if any)

***

### **`getVersion()`**

Returns version and contract identifier.

```solidity
("1.0.0", "PrivateNote")
```

