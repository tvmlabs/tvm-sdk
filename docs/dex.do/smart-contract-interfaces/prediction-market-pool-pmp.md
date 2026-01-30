---
description: PMP Contract Interface Documentation
---

# Prediction Market Pool (PMP)

## Overview

{% file src="../../.gitbook/assets/PMP.abi.json" %}

**Prediction Market Pool** is a decentralized prediction market contract that aggregates user stakes on discrete outcomes of an event.\
A PMP **does not charge protocol fees**, but **requires approval from one or more oracles** before becoming active.

Each PMP is:

* Deployed by a **PrivateNote**
* Bound to a specific **event** and **oracle list**
* Governed by **oracle proposals** (multi-oracle voting)

***

## Contract Metadata

* **Contract name:** `PMP`
* **Role:** Prediction market pool
* **Deployed by:** `PrivateNote`
* **Authorization model:**
  * Users: `PrivateNote`
  * Oracles: oracle public keys
* **Lifecycle:**\
  `Deploy → Oracle Approval → Configuration → Staking → Resolution → Claims`

***

## Key Concepts

* **Outcomes** — Discrete result intervals (minimum 2)
* **Staking window** — Time interval for accepting stakes
* **Result window** — Time interval for resolving outcome
* **Oracle approval** — Required before activation
* **Oracle governance** — Proposals and voting for configuration & resolution

***

## Events

### `StakeAccepted`

Emitted when a stake is successfully accepted.

```solidity
event StakeAccepted(address note, uint32 outcomeId, uint128 amount);
```

* `note`: the user’s `PrivateNote` address
* `outcomeId`: the chosen outcome identifier
* `amount`: stake amount

***

### `ApprovedByOracle`

Emitted when all oracle event lists approve the PMP.

```solidity
event ApprovedByOracle(address oracleEventList, uint256 oraclePubkey);
```

* `oracleEventList`: the oracle event list contract address that sent the approval
* `oraclePubkey`: the oracle public key used as an oracle identifier

***

### `Resolved`

Emitted when the event outcome is resolved.

```solidity
event Resolved(uint32 outcomeId);
```

* `outcomeId`: the resolved outcome identifier

***

### `ClaimProcessed`

Emitted when a claim is processed.

```solidity
event ClaimProcessed(
    address note,
    uint32 outcomeId,
    uint128 payout,
    bool win
);
```

* `note`: the user’s `PrivateNote` address
* `outcomeId`: the outcome the user claimed for
* `payout`: computed payout amount (0 if unresolved or losing)
* `win`: `true` if the claim matches the resolved outcome

***

### TimingsSet

Emitted when staking and result windows are configured and the pool is marked approved.

```solidity
event TimingsSet(uint64 stakeStart, uint64 stakeEnd, uint64 resultStart, uint64 resultEnd);
```

* `stakeStart`: staking start timestamp
* `stakeEnd`: staking end timestamp
* `resultStart`: result window start timestamp
* `resultEnd`: result window end timestamp

### EventCancelled

Emitted when the event is cancelled.

```solidity
event EventCancelled();
```

### ProposalCreated

Emitted when an oracle governance proposal is created.

```solidity
event ProposalCreated(uint256 proposalId, uint32 functionType, TvmCell data);
```

* `proposalId`: deterministic proposal identifier (typically hash of `functionType` + `data`)
* `functionType`: action type identifier
* `data`: ABI-encoded payload (`TvmCell`)

### ProposalExecuted

Emitted when a proposal is executed.

```solidity
event ProposalExecuted(uint256 proposalId, uint32 functionType, TvmCell data);
```

* `proposalId`: executed proposal identifier
* `functionType`: executed action type identifier
* `data`: executed ABI payload (`TvmCell`)

***

## Oracle Approval Flow

### `approveEvent`

Oracle confirms/approves the event.

```solidity
function approveEvent(
    uint256 oracle_pubkey,
    mapping(uint32 => string) outcomeNames,
    string describe,
    optional(uint256) trustAddr
) public;
```

**Parameters:**

* `oracle_pubkey`: oracle public key used as an oracle identifier
*   `outcomeNames`: mapping `outcomeId -> name` (used only on first approval)\
    It represents a **logical band of outcomes for a single event**.\
    **Where:**

    * **`outcomeId`** (key, e.g. `0, 1, 2, …`) — the identifier of a specific outcome
    * **`name`**  — a description of that outcome

    **Example:** \
    **Sports Event "**&#x52;eal vs Benfica" \
    **Outcome band:**

    * **0** — Real wins
    * **1** — Benfica wins
    * **2** — Draw
* `describe`: event description (used only on first approval)
* `trustAddr`: optional internal address binding to `oracle_pubkey` for internal voting flows

**Access Control:**

* Callable only by approved `OracleEventList` contracts

**Behavior:**

* prevents duplicate approvals (by pubkey / sender rules in implementation);
* on the first oracle approval, initializes:
  * the event description (`describe`);
  * outcome names mapping (`outcomeNames`);
  * number of outcomes (`numOutcomes = outcomeNames.keys().length`);
* once all required oracle confirmations are collected, emits `ApprovedByOracle(...)`.

***

### `rejectEvent`

Rejects the `PMP` and self-destructs the contract.

```solidity
function rejectEvent() public;
```

***

## Staking

### `acceptStake`

Accepts a stake from the user’s `PrivateNote` and records it in the pool.

```solidity
function acceptStake(
    uint32 outcomeId,
    uint128 stakeAmount,
    uint256 deposit_identifier_hash,
    bool isInc
) public;
```

**Parameters:**

* `outcomeId`: chosen outcome identifier
* `stakeAmount`: stake amount
* `deposit_identifier_hash`: hash used to compute the caller’s `PrivateNote` address
* `isInc`: whether this stake should increase the active stake count for the outcome

**Requirements:**

* PMP approved by all oracles
* Within staking time window
* Outcome ID valid

**Side Effects:**

* Updates pools and counters
* Notifies `PrivateNote` via callback

***

### `cancelStake`

Cancels a stake and triggers refund-side handling in `PrivateNote`.

```solidity
function cancelStake(
    uint32 outcomeId,
    uint128 stakeAmount,
    uint256 deposit_identifier_hash
) public;
```

**Parameters:**

* `outcomeId`: chosen outcome identifier
* `stakeAmount`: stake amount
* `deposit_identifier_hash`: hash used to compute the caller’s `PrivateNote` address

**Access:**

* only the computed `PrivateNote` address for `deposit_identifier_hash`.

**Requirements:**

* the event must be cancelled.

***

## Claims

### `claim`

Processes a claim for a user.

```solidity
function claim(
    uint128 stakeAmount,
    uint32 outcomeId,
    uint256 deposit_identifier_hash
) public
```

**Parameters:**

* `stakeAmount`: stake amount
* `outcomeId`: chosen outcome identifier
* `deposit_identifier_hash`: hash used to compute the caller’s `PrivateNote` address

**Access:**

* only the computed `PrivateNote` address for `deposit_identifier_hash`.

**Requirements:**

* the pool must be approved;
* `outcomeId < numOutcomes`.

**Behavior:**

* if the outcome is not resolved yet, payout is `0` and `win = false` (claim is acknowledged);
* if resolved and losing, payout is `0` and `win = false`;
* if resolved and winning, payout is proportional:

`payout = stakeAmount * totalPool / winningPool`

**Additionally:**

* emits `ClaimProcessed(...)`;
* calls `PrivateNote.onClaimAccepted(...)`;
* when the last active stake for the winning outcome is claimed, the contract self-destructs to `_deployer`.

***

## Oracle Governance

### `createProposal`

Creates a new oracle governance proposal.

```solidity
function createProposal(uint32 function_type, TvmCell data) public;
```

**Parameters:**

* `function_type`: action type identifier (implementation-specific)
* `data`: ABI-encoded payload (`TvmCell`) for the action

**Access**:

* only authorized oracles (either via trusted internal sender binding or external sender pubkey membership, depending on implementation rules).

**Behavior**:

* the creator’s vote is counted immediately;
* if there is only one oracle, the proposal executes immediately;
* otherwise emits `ProposalCreated(...)`.

***

### `vote`

Votes on an existing proposal.

```solidity
function vote(uint256 proposalId) public;
```

**Parameter**:

* `proposalId`: identifier of the proposal to vote on

**Access**:

* only authorized oracles.

**Behavior**:

* rejects if proposal does not exist;
* ignores/removes proposal if it is past deadline;
* prevents double voting by the same oracle pubkey;
* executes proposal automatically once the threshold is reached (implementation-specific);
* successful execution leads to `ProposalExecuted(...)`.

***

## View Functions

### `getDetails`

Returns the complete contract state in a single call.

```solidity
function getDetails() external view returns (
    string name,
    uint32 token_type,
    uint256 event_id,
    uint256 oracle_list_hash,
    address deployer,
    address root,
    uint256 privateNoteCodeHash,
    uint128 totalPool,
    bool approved,
    uint32 numOutcomes,
    optional(uint32) resolvedOutcome,
    uint64 stakeStart,
    uint64 stakeEnd,
    uint64 resultStart,
    uint64 resultEnd,
    bool isCancelled,
    uint128 numberOfOracleEvents,
    uint128 approvedOracleEvents,
    mapping(uint32 => uint128) outcomePoolAmounts,
    mapping(uint32 => uint128) outcomeStakeCounts,
    mapping(uint32 => string) outcomeNames
);
```

**Returns** :

* **`name`**\
  Human-readable pool name. Acts as a static identifier of the prediction market and is passed to `PrivateNote` callbacks.
* **`token_type`**\
  Identifier of the token type used by this pool. Fixed at deployment time.
* **`event_id`**\
  Unique identifier of the event associated with this prediction market. Used in oracle interactions.
* **`oracle_list_hash`**\
  Hash of the oracle list configuration for this event, ensuring all oracle confirmations refer to the same oracle set.
* **`deployer`**\
  Address of the contract deployer. Receives remaining funds when the contract self-destructs after the final winning claim.
* **`root`**\
  Address of the **RootPN** contract. Used for deterministic `PrivateNote` address computation.
* **`privateNoteCodeHash`**\
  Hash of the `PrivateNote` contract code used by the pool. Allows off-chain verification of `PrivateNote` addresses.
* **`totalPool`**\
  Total amount staked across all outcomes. Used in payout calculation.
* **`approved`**\
  Indicates whether the pool has been fully approved by oracles and is ready to accept stakes.
* **`numOutcomes`**\
  Number of possible outcomes for the event. Set once during the first oracle approval.
* **`resolvedOutcome`**\
  Optional final outcome identifier.\
  If empty, the event is not resolved yet; if set, contains the winning `outcomeId`.
* **`stakeStart`**\
  Unix timestamp when staking becomes available.
* **`stakeEnd`**\
  Unix timestamp when staking ends.
* **`resultStart`**\
  Unix timestamp when the result resolution window starts.
* **`resultEnd`**\
  Unix timestamp when the result resolution window ends.
* **`isCancelled`**\
  Indicates whether the event has been cancelled. When `true`, staking is disabled and stake cancellation is allowed.
* **`numberOfOracleEvents`**\
  Total number of oracle event list contracts configured at deployment.
* **`approvedOracleEvents`**\
  Number of oracle event lists that have already approved the event.
* **`outcomePoolAmounts`**\
  Mapping of `outcomeId` to the total amount staked on that outcome.
* **`outcomeStakeCounts`**\
  Mapping of `outcomeId` to the number of active stakes for that outcome.
* **`outcomeNames`**\
  Mapping of `outcomeId` to a name of the outcome.

***

### `getVersion`

Returns the implementation version and kind identifier.

```solidity
function getVersion() external pure returns (string);
```

**Returns**:

* `semver`: version string
* `kind`: contract kind string (expected `"PMP"`)
