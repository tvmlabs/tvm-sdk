---
description: (Work in progress) PrivateNote Contract Interface Documentation
---

# PrivateNote

{% file src="../../.gitbook/assets/PrivateNote.abi (1).json" %}

## Overview

**PrivateNote** is a non-custodial wallet contract that stores balances, manages stakes, and interacts with **Pari Mutuel Pool (PMP)** contracts (deployment, staking, cancellation, claiming).\
It also supports coupons and withdrawals via `RootPN`     contract — all while preserving privacy guarantees.<br>

Each `PrivateNote`:

* **Owner-authorized** actions are controlled by an **ephemeral public key** (`_ethemeral_pubkey`).
* The contract uses a **busy state** (`_busy`) to prevent concurrent PMP interactions.
* Stakes are processed in **two phases**: _candidate_ → confirmed/reverted via PMP callbacks.
* Some operations require **`debt == 0`** and **no active stakes**.

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
event StakeConfirmed(
    address stakeController,
    uint32 outcome,
    uint128 amount,
    uint8 bet_type
);
```

* `stakeController` - Address of the PMP contract that accepted the stake
* `outcome` - Outcome identifier the stake was placed on
* `amount` - Amount of tokens added to the stake
* `bet_type`  — bet type
  * `0` — clean bet (from balance)
  * `1` — debt bet
  * `2` — coupon bet

***

### StakeCancelled

Emitted after a stake is cancelled and refunded.

```solidity
event StakeCancelled(address stakeController, uint128 value);
```

* `stakeController` - Address of the PMP contract that cancelled the stake
* `amount` - Amount of tokens returned

***

### FullSetStakeConfirmed

Emitted when a **full-set stake** is confirmed by PMP

```solidity
event FullSetStakeConfirmed(address stakeController, uint128[] amount);
```

* `stakeController`  — PMP address
* `amount`  — Confirmed stake amounts per outcome

***

### FullSetStakeCancelled

Emitted when a full-set stake is cancelled and funds returned

```solidity
event FullSetStakeCancelled(address stakeController, uint128 value);
```

* `stakeController`  — PMP address
* `value`  — Total returned token value to balance

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
    uint256 event_id,
    uint32 token_type,
    address pmpAddress,
    address[] oracleEventLists,
    uint128[] oracleFee
);
```

* `event_id` - Identifier of the PMP event
* `token_type` - Token type used for staking
* `pmpAddress` - Address of the deployed PMP contract
* `oracleEventLists` - Oracle event list contract addresses
* `oracleFee` - Oracle fee values corresponding to each oracle

***

## Public & External Interface

### **`changeOwner`**

Changes the (ephemeral) public key controlling the wallet.

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
    uint256 event_id,
    uint128[] oracleFee,
    uint32 token_type,
    string[] names,
    uint128[] index
) public;
```

**Parameters:**

* `event_id` — Identifier of the PMP event
* `oracleFee` — Array of additional fees (in shell tokens) for each oracle. \
  &#xNAN;_&#x4D;ust match the length of `names` and `index`._
* `token_type` — Token type used by the PMP contract
* `names` — Array of oracle names used to compute oracle addresses.
* `index` — Array of oracle indexes used to compute OracleEventList addresses.

**Deployment flow:**&#x20;

1\. Validate input array lengths. \
2\. Compute oracle addresses from `names`. \
3\. Compute `OracleEventList` addresses using oracle code and indexes. \
4\. Aggregate oracle fees and include network fee. \
5\. Build PMP StateInit and compute deterministic PMP address. \
6\. Emit `PMPDeployed` event. \
7\. Deploy PMP contract with required currencies.

***

### **`setStake`**

Places a single-outcome stake on a PMP

```solidity
function setStake(
    uint256 event_id,
    uint256 oracle_list_hash,
    uint32 token_type,
    uint32 outcome,
    uint128 amount,
    bool use_coupon
) public;
```

**Parameters:**

* `event_id` - PMP event identifier
* `oracle_list_hash` - Hash of the oracle list configuration associated with the `PMP` event.\
  Must match the oracle list used when the `PMP` contract was deployed.
* `token_type` - Token type used for staking.\
  Must correspond to a token balance available in the wallet.
* `outcome` - Identifier of the outcome being staked on.\
  The meaning of the outcome is defined by the `PMP` contract logic.
* `amount` - Amount of tokens to stake.\
  Must be greater than zero and less than or equal to the available balance for the specified token type.
* `use_coupon` - Whether to use coupon for this stake \
  If true, amount will be taken from available coupons instead of balance

**Requirements**

* `amount > 0`
* If `use_coupon == true` → sufficient coupons
* Else → sufficient token balance
* Wallet is **not busy**

**Effects**

* Writes candidate stake
* Deducts balance or coupons
* Sets `_busy` and `_lastHash`
* Calls `PMP.acceptStake(...)`

***

### `setFullSetStake`

Places a full-set stake (amount per outcome)

```solidity
function setFullSetStake(
        uint256 event_id,
        uint256 oracle_list_hash,
        uint32 token_type,
        uint128[] amount
    ) public;
```

**Parameters:**

* `event_id` - PMP event identifier
* `oracle_list_hash` - Hash of the oracle list configuration associated with the `PMP` event.\
  Must match the oracle list used when the `PMP` contract was deployed
* `token_type` - Token type used for staking.\
  Must correspond to a token balance available in the wallet
* `amount` - Array of stake amounts per outcome

**Requirements**

* Wallet is **not busy**
* `amount.length > 0`
* `debt == 0`
* Sufficient balance
* If stake exists → lengths must match
* Owner authorization

**Effects**

* Sets `candidate_amount = sum(amount)`
* Deducts balance
* Sets `_busy` and `_lastHash`
* Calls `PMP.acceptFullSetStake(...)`

***

### **`cancelStake`**

Cancels an existing stake in `PMP`.

```solidity
function cancelStake(
    uint256 event_id,
    uint256 oracle_list_hash,
    uint32 token_type,
) public;
```

**Parameters**

* `event_id` — Identifier of the PMP event
* `oracle_list_hash` - Hash of the oracle list configuration associated with the `PMP` event.\
  Must match the oracle list hash of the existing stake.
* `token_type` - Token type used for the stake being cancelled.\
  Must correspond to the token type of the existing stake entry.

**Requirements**

* Wallet is **not busy**
* Stake record exists
* Owner authorization

**Flow**

1. Computes stake hash and PMP address
2. Sets `_busy` and `_lastHash`
3. Calls `PMP.cancelStake(...)`

***

### **`deleteStake`**

Deletes a local stake record (does not call PMP)

```solidity
function deleteStake(
    uint256 event_id,
    uint256 oracle_list_hash,
    uint32 token_type,
) public;
```

**Parameters:**

* `event_id` - PMP event identifier
* `oracle_list_hash` - Hash of the oracle list configuration associated with the stake.\
  Must match the oracle list hash used when the stake entry was created.
* `token_type` - Token type of the stake being deleted.\
  Used to identify the correct local stake record.

**Requirements**

* Wallet is **not busy**
* Owner authorization

**Effects**

* Removes the local stake record



***

### **`claim`**

Claims winnings from a resolved `PMP`.

```solidity
function claim(
    uint256 event_id,
    uint256 oracle_list_hash,
    uint32 token_type,
) public;
```

**Parameters**

* `event_id` - PMP event identifier.\
  Used to derive the event identifier and the address of the target `PMP` contract.
* `oracle_list_hash` - Hash of the oracle list configuration associated with the `PMP` event.\
  Must match the oracle list hash of the existing stake.
* `token_type` - Token type used for the stake.\
  Determines which internal balance will receive the payout.

**Requirements**

* Wallet is **not busy**
* No pending candidate amount
* Owner authorization

**Effects**

* Sets `_busy` and `_lastHash`
* Calls `PMP.claim(...)`

***

### `generateCoupon`

Generates a free coupon for a specified token type.

The function allows the wallet owner to receive a one-time free coupon.\
The coupon value depends on the provided `token_type` and is internally determined by the contract.

Coupons can later be used to place stakes instead of using wallet balance.

```solidity
function generateCoupon(uint32 token_type) public;
```

**Parameters**

* `token_type` - Token type for which the coupon should be generated

**Requirements**

* `debt == 0`
* No active stakes
* All balances are zero
* No existing coupon

***

### `withdrawFullSet`

Withdraws or cancels part of a full-set stake.

```solidity
function withdrawFullSet(
        uint256 event_id,
        uint256 oracle_list_hash,
        uint32 token_type,
        uint128[] amount
    ) public
```

**Parameters**

* `event_id` - PMP event ID
* `oracle_list_hash` - Hash of Oracles
* `token_type` - Token type
* `amount` - Array of amounts per outcome to withdraw

**Requirements**

* Wallet is **not busy**
* Debt must be zero.
* Stake record must exist
* Array length must match existing stake

**Effects**

* Sets `_busy` and `_lastHash`
* Calls `PMP.withdrawFullSet(...)`

### **`withdrawTokens`**

Withdraws tokens from the wallet via the `RootPN`.

```solidity
function withdrawTokens(
    uint8 flags,
    address dest_wallet_addr,
    uint32 token_type
) public;
```

**Parameters**

* `flags` - Transfer flags passed to the `RootPN` contract.\
  Control message delivery behavior during the token transfer.
* `dest_wallet_addr` - Destination wallet address that will receive the withdrawn tokens.
* `token_type` - Type of token to withdraw.\
  All available balance for this token type will be withdrawn.

**Requirements**

* No active stakes
* `debt == 0`
* Owner authorization

**Behavior:**

* Transfers full balance of the given token type
* Resets local balance to zero

***

## View Functions

### **`getPMPCode`**

Returns salted PMP code and its hash.

```solidity
function getPMPCode() external view returns(TvmCell pmpCode, uint256 pmpCodeHash)
```

***

### **`getDetails`**

Returns current wallet state:

```solidity
function getDetails() external view returns (
        uint256 depositIdentifierHash,
        uint256 etherealPubkey,
        mapping(uint32 => uint128) balance,
        uint256 pmpCodeHash,
        uint256 privateNoteCodeHash,
        optional(address) busyAddress
    )
```

**Returns:**

* `depositIdentifierHash` - deposit identifier hash
* `etherealPubkey` - public key
* `balance` - token balances&#x20;
* `pmpCodeHash` - `PMP` code hash
* `privateNoteCodeHash` - `PrivateNote` code hash
* `busyAddress` - busy PMP address (if any)

***

### **`getVersion`**

Returns version and contract identifier.

```solidity
function getVersion() external pure returns (string, string)
```

**Returns:**

* semantic version string
* Contract name: `"PrivetNote"`
