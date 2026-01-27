---
description: PrivateNote Contract Interface Documentation
---

# PrivateNote

## Overview

**PrivateNote** is a non-custodial wallet contract.\
It manages user balances, deploys **PMP** **(Prediction Market Pool)** and **OracleUnion** contracts, places and manages stakes, and handles claims and withdrawals.

Each `PrivateNote` instance is **uniquely bound to a deposit identifier hash** and controlled by an **ephemeral public key**.

***

## Contract Metadata

* **Contract name:** `PrivateNote`
* **Role:** Wallet & interaction hub for PMP contracts
* **Deployed by:** `RootPN`
* **Authorization model:** Ephemeral public key (`_ethemeral_pubkey`)
* **Address derivation:** Deterministic via `_deposit_identifier_hash`

***

## Events

### `OwnerChanged`

```solidity
event OwnerChanged(uint256 oldPubkey, uint256 newPubkey);
```

Emitted when the owner (ephemeral public key) is changed.

**Parameters:**

* `oldPubkey` — Previous public key
* `newPubkey` — New public key

***

### `StakeConfirmed`

```solidity
event StakeConfirmed(address stakeController, uint32 outcome, uint128 amount);
```

Emitted when a stake is successfully accepted by a PMP contract.

**Parameters:**

* `stakeController` — PMP contract address
* `outcome` — Stake outcome
* `amount` — Confirmed stake amount

***

### `StakeCancelled`

```solidity
event StakeCancelled(address stakeController, uint32 outcome, uint128 amount);
```

Emitted when a stake is cancelled and funds are returned.

***

### `ClaimAccepted`

```solidity
event ClaimAccepted(
    address stakeController,
    optional(uint32) outcome,
    uint128 payout
);
```

Emitted after a claim is processed by PMP.

**Notes:**

* If `outcome` is empty, the PMP was not yet resolved.

***

### `PMPDeployed`

```solidity
event PMPDeployed(
    string name,
    uint32 token_type,
    address pmpAddress,
    uint128 oracleFee,
    address oracleUnion
);
```

Emitted after deploying a new PMP contract.

***

## Constructor

#### `constructor`

```solidity
constructor(
    uint128 value,
    uint256 ethemeral_pubkey,
    uint32 token_type,
    TvmCell pmpCode,
    TvmCell oracleUnionCode,
    TvmCell oracleCode
)
```

Initializes the wallet and registers the deployment in `RootPN`.

**Parameters:**

* `value` — Initial token balance
* `ethemeral_pubkey` — Authorization public key
* `token_type` — Token type of the initial balance
* `pmpCode` — PMP contract code
* `oracleUnionCode` — OracleUnion contract code
* `oracleCode` — Oracle contract code

**Requirements:**

* Must be deployed by `RootPN`

**Side Effects:**

* Initializes token balance
* Registers deployment in `RootPN`

***

## Ownership & Authorization

### `changeOwner`

```solidity
function changeOwner(uint256 new_pubkey) public;
```

Changes the wallet owner public key.

**Access Control:**

* Callable only by the current owner (`_ethemeral_pubkey`)

**Emits:** `OwnerChanged`

***

## Oracle & PMP Deployment

### `deployOracleUnion`

```solidity
function deployOracleUnion(string[] name)
    public view onlyOwnerPubkey accept;
```

Deploys a new `OracleUnion` contract.

**Parameters:**

* `name` — OracleUnion member names (must be unique, max length < 10)

***

### `deployPMP`

```solidity
function deployPMP(
    string name,
    uint128 oracleFee,
    uint32 token_type,
    string[] names
) public view onlyOwnerPubkey;
```

Deploys a new `PMP` contract.

**Parameters:**

* `name` — PMP name (must be non-empty)
* `oracleFee` — Oracle fee in shell tokens
* `token_type` — Token type used by PMP
* `names` — OracleUnion member names

**Emits:** `PMPDeployed`

***

## Staking

### `setStake`

```solidity
function setStake(
    string name,
    uint32 token_type,
    uint32 outcome,
    uint128 amount
) public onlyOwnerPubkey accept;
```

Places or updates a stake on a PMP contract.

**Requirements:**

* Sufficient balance
* Wallet must not be busy
* Previous candidate stake must be confirmed

***

### `cancelStake`

```solidity
function cancelStake(
    string name,
    uint32 token_type,
    uint32 outcome
) public onlyOwnerPubkey accept;
```

Cancels an active stake.

***

### `deleteStake`

```solidity
function deleteStake(
    string name,
    uint32 token_type,
    uint32 outcome
) public onlyOwnerPubkey accept;
```

Deletes a local stake record (no PMP interaction).

***

### `onStakeAccepted`

```solidity
function onStakeAccepted(
    string name,
    uint32 token_type,
    uint32 outcome
) public;
```

Callback from PMP after stake acceptance.

**Access Control:**

* Callable only by the active PMP (`_busy`)

**Emits:** `StakeConfirmed`

***

### `onStakeCancelled`

```solidity
function onStakeCancelled(
    string name,
    uint32 token_type,
    uint32 outcome
) public;
```

Callback from PMP after stake cancellation.

**Emits:** `StakeCancelled`

***

## Claims

### `claim`

```solidity
function claim(
    string name,
    uint32 token_type,
    uint32 outcome
) public onlyOwnerPubkey;
```

Claims winnings from a PMP contract.

***

### `onClaimAccepted`

```solidity
function onClaimAccepted(
    string name,
    uint32 token_type,
    optional(uint32) outcome,
    uint32 selfOutcome,
    uint128 payout
) public;
```

Callback from PMP after claim processing.

**Behavior:**

* If `outcome` is empty, PMP is unresolved
* Otherwise:
  * Adds payout to balance
  * Removes stake record

**Emits:** `ClaimAccepted`

***

## Withdrawals

### `withdrawTokens`

```solidity
function withdrawTokens(
    uint8 flags,
    address dest_wallet_addr,
    uint128 value,
    uint32 token_type
) public onlyOwnerPubkey accept;
```

Withdraws tokens to an external wallet via `Vault`.

***

### `revertWithdraw`

```solidity
function revertWithdraw(
    uint32 token_type,
    uint128 value
) public accept;
```

Reverts a withdrawal (called by `Vault` only).

***

## Native Transfers & Error Handling

### `receive()`

Receives native tokens and ensures minimum balance.

***

### `onBounce`

```solidity
onBounce(TvmSlice body) external;
```

Handles bounced PMP messages.

**Behavior:**

* Clears busy state
* Restores stake or candidate amount

***

## View Functions

### `getPMPCode`

```solidity
function getPMPCode()
    external view
    returns (TvmCell pmpCode, uint256 pmpCodeHash);
```

Returns salted PMP code and its hash.

***

### `getDetails`

```solidity
function getDetails()
    external view
    returns (
        uint256 depositIdentifierHash,
        uint256 etherealPubkey,
        mapping(uint32 => uint128) balance,
        uint256 pmpCodeHash,
        uint256 privateNoteCodeHash,
        optional(address) busyAddress
    );
```

Returns current wallet state and metadata.

***

### `getVersion`

```solidity
function getVersion() external pure returns (string);
```

Returns the contract identifier.

**Returns:** `"PrivateNote"`

***

## Access Control Summary

| Function Category | Access           |
| ----------------- | ---------------- |
| Owner operations  | Ephemeral pubkey |
| PMP callbacks     | Active PMP only  |
| Vault callbacks   | Vault only       |
| View functions    | Public           |

