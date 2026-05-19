---
description: (Work in progress) PMP Contract Interface Documentation
---

# PARI MUTUEL POOL (PMP)

{% file src="../../.gitbook/assets/PMP.abi (1).json" %}

## Overview

**Pari Mutuel Pool** is a decentralized contract that aggregates user stakes on discrete outcomes of an event.\
A PMP **does not charge protocol fees**, but **requires approval from one or more oracles** before becoming active.

Each PMP is:

* Deployed by a **PrivateNote**
* Bound to a specific **event** and **oracle list**
* Governed by **oracle proposals** (multi-oracle voting)

***

## Contract Metadata

* **Contract name:** `PMP`
* **Role:** Pari Mutuel Pool
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

### Outcomes

* The event has `_numOutcomes` possible outcomes.
* Outcome names are stored in `mapping(uint32 => string) _outcomeNames`.
* Stakes are tracked per outcome and per bet type.

### Bet Types

`bet_type`:

* `0` — clean bet
* `1` — debt bet
* `2` — coupon bet

### **Staking window**

Oracles set:

* `_stakeStart` / `_stakeEnd` — stake acceptance window
* `_resultStart` / `_resultEnd` — result/resolution window

## Events

### `StakeAccepted`

Emitted when a stake is accepted and accounted into the pool.

```solidity
event StakeAccepted(address indexed note, uint32 outcomeId, uint128 amount, uint8 bet_type);
```

* `note` (`address`) — PrivateNote address (wallet) that placed the stake.
* `outcomeId` (`uint32`) — Outcome identifier the stake is placed on.
* `amount` (`uint128`) — Stake amount added to the pool.
* `bet_type` (`uint8`) — 0 - clean bet, 1 - debt bet, 2 - coupon bet

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

If the event is not resolved or the user did not win, `payout` will be `0` and `win` will be `false`.

```solidity
event ClaimProcessed(
    address note,
    uint128 payout,
    bool win
);
```

* `note`  — `PrivateNote` address (wallet) that claimed.
* `payout`  — Calculated payout amount (0 if no payout).
* `win`  — True if the claim is winning and payout > 0.

***

### `NetworkFeeBurned`

Emitted when network fee was burned.

The shown code does not currently emit this event; it is reserved for future accounting.

```solidity
event NetworkFeeBurned(uint64 amount);
```

* `amount`  — Burned fee amount in native units.

***

### `TimingsSet`

Emitted when staking and result windows are configured and the pool is marked approved.

```solidity
event TimingsSet(uint64 stakeStart, uint64 stakeEnd, uint64 resultStart, uint64 resultEnd);
```

* `stakeStart`: staking start timestamp
* `stakeEnd`: staking end timestamp
* `resultStart`: result window start timestamp
* `resultEnd`: result window end timestamp

***

### `NumOutcomesSet`

Emitted when number of outcomes is set.

The contract currently derives `_numOutcomes` from `outcomeNames` and may not emit this event.

```solidity
event NumOutcomesSet(uint32 numOutcomes);
```

* `numOutcomes` (`uint32`) — Number of available outcomes

***

### `EventCancelled`

Emitted when the event is cancelled by oracle governance.

```solidity
event EventCancelled();
```

***

### `PMPCancelled`

Emitted when a PMP is cancelled by oracle.

```solidity
event EventCancelled();
```

***

### ProposalCreated

Emitted when an oracle governance proposal is created.

```solidity
event ProposalCreated(uint256 proposalId, uint32 functionType, TvmCell data);
```

* `proposalId`: deterministic proposal identifier (typically hash of `functionType` + `data`)
* `functionType`: action type identifier
* `data`: ABI-encoded payload (`TvmCell`)

***

### ProposalExecuted

Emitted when a proposal is executed.

```solidity
event ProposalExecuted(uint256 proposalId, uint32 functionType, TvmCell data);
```

* `proposalId`: executed proposal identifier
* `functionType`: executed action type identifier
* `data`: ABI-encoded payload used for execution.

***

## Oracle Approval Flow

### `approveEvent`

The function is called by an **OracleEventList** contract to confirm/approve the event initialization for this pool. It also optionally binds an internal sender address to the oracle pubkey for later governance actions.

```solidity
function approveEvent(
    uint256 oracle_pubkey,
    mapping(uint32 => string) outcomeNames,
    string describe,
    string name,
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
* `name`: pool name (used on every approval call)
* `trustAddr`: optional internal address binding to `oracle_pubkey` for internal voting flows

**Access Control:**

* Callable only by approved `OracleEventList` contracts
* The call is ignored (returns immediately) if:
  * this oracle event list address was already processed (implementation currently checks \
    or
  * the number of approvals already reached.

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
    uint8 bet_type
) public;
```

**Parameters:**

* `outcomeId`-chosen outcome identifier
* `stakeAmount`- stake amount
* `deposit_identifier_hash`- hash used to compute the caller’s `PrivateNote` address
* `bet_type` — 0 - clean bet, 1 - debt bet, 2 - coupon bet

**Requirements:**

* PMP approved by all oracles
* Within staking time window
* Outcome ID valid

**Side Effects:**

* Updates pools and counters
* Notifies `PrivateNote` via callback

***

### `acceptFullSetStake`

Accepts a **full-set stake** from a `PrivateNote` wallet during the dedicated full-set staking window.

A full-set stake represents proportional participation across **all outcomes** of the event.

`acceptFullSetStake` allows a user to stake across **all outcomes simultaneously**, preserving the proportional distribution of existing pools.

This function:

* Validates that the event is approved and not cancelled.
* Ensures the event is not yet resolved.
* Restricts execution to the **full-set time window**.
* Verifies proportionality against current outcome pools.
* Updates internal pool accounting.
* Notifies the caller’s `PrivateNote` via `onFullSetStakeAccepted`.

```solidity
function acceptFullSetStake(
        uint128[] amount,
        uint256 deposit_identifier_hash
    ) public
```

**Parameters**

* **`amount`** - Array of stake amounts per outcome.
  * Must have length equal to `_numOutcomes`.
  * Must follow the required pool proportion rules (see below).
* **`deposit_identifier_hash`** - Deposit identifier hash used to deterministically compute the caller’s `PrivateNote` address.

**Effects**

* Increases total pool liquidity.
* Preserves pool balance proportions.
* Does not affect coupon or debt pools.
* Does not emit a specific event; confirmation occurs via callback.

**Notes**

* Full-set staking is only available during the final portion of the staking period.
* Designed to allow liquidity providers to enter proportionally without skewing odds.
* Cannot be executed after resolution or cancellation.

***

### `cancelStake`

Cancels a user’s stakes **after the event is cancelled** and returns refund information to the caller’s `PrivateNote` wallet.

```solidity
function cancelStake(
        uint128[] stakeAmount,
        uint128[] debtAmount,
        uint128[] couponsAmount,
        uint256 deposit_identifier_hash
    ) public;
```

#### Parameters

* `stakeAmount`  — Array of **clean** stake amounts per outcome.
* `debtAmount`  — Array of **debt** stake amounts per outcome.
* `couponsAmount`  — Array of **coupon** stake amounts per outcome.
* `deposit_identifier_hash`  — Deposit identifier hash used to deterministically compute the caller’s `PrivateNote` address.

#### **Access:**

* only the computed `PrivateNote` address for `deposit_identifier_hash`.

#### Requirements

* The event must be cancelled:
  * If `block.timestamp > _resultEnd` and the event is not resolved and not cancelled, the contract triggers `cancelEvent()` internally.
  * After that, the function requires `_isCancelled == true`.
* Caller must be the expected `PrivateNote` wallet address derived from `deposit_identifier_hash`.

#### External Calls

* Calls `PrivateNote(wallet).onStakeCancelled(...)` to notify the wallet about:
  * total refunded clean+debt stake (`totalStake`)
  * total refunded coupon amount (`totalCouponRefund`)

***

### `withdrawFullSet`

Cancels a previously placed **full-set stake** during the full-set window and updates pool accounting accordingly.

This function is used to **withdraw (undo) a proportional full-set stake** before the staking period ends

```solidity
function withdrawFullSet(
        uint128[] amount,
        uint256 deposit_identifier_hash
    ) public;
```

**Parameters**

* **`amount`** - Array of withdrawal amounts per outcome.
  * Must have length equal to `_numOutcomes`.
  * Must preserve proportionality relative to current outcome pools.
* **`deposit_identifier_hash`** - Deposit identifier hash used to deterministically compute the caller’s `PrivateNote` wallet address.

`withdrawFullSet` removes a proportional full-set stake across **all outcomes**, provided the withdrawal keeps the same proportionality constraints used for full-set staking.

The function:

* Ensures the event is approved, not cancelled, and not resolved.
* Restricts execution to the **full-set time window**.
* Validates proportionality of the provided `amount`.
* Decreases outcome clean pools and global totals.
* Notifies the caller’s `PrivateNote` wallet via `onFullSetStakeCancelled`.

**Proportionality Rule**

The withdrawal amounts must preserve the same proportional structure enforced for full-set operations:

* For outcomes with non-zero pools:
  * `amount[i]` must match the proportional ratio relative to pool sizes.
* For outcomes with zero pools:
  * `amount[i]` must be zero.

This is validated by `_checkFullSetProportion(amount)`.

**Notes**

* This function does not emit a dedicated event; confirmation is delivered via the callback to `PrivateNote`.
* Withdrawals are only possible before `_stakeEnd`, and only after `fullSetStart`.
* The proportionality rule prevents selective withdrawal that would skew the pool distribution.

***

## Claims

### `claim`

Claims winnings (or processes a zero-payout claim) for a `PrivateNote` wallet after event resolution.

```solidity
function claim(
        uint128[] stakeAmount,
        uint128[] debtAmount,
        uint128[] couponsAmount,
        uint256 deposit_identifier_hash
    ) public
```

`claim` allows a user to settle their position in the PMP after the event has been resolved.

Depending on the outcome and the user’s position, the function:

* Processes a **winning payout**, or
* Processes a **zero-payout claim** if:
  * The event is not resolved yet, or
  * The user has no stake on the winning outcome.

After calculation, the contract notifies the caller’s `PrivateNote` wallet via `onClaimAccepted` and emits `ClaimProcessed`.

**Effects**

* Finalizes user settlement.
* Distributes winnings according to pari-mutuel logic.
* Automatically terminates the contract when all winning claims are completed.

**Notes**

* Arrays are expected to correspond to `_numOutcomes`.
* Claim can be called multiple times by different participants until `_totalWinPool` is fully distributed.

***

## Oracle Governance

### `createProposal`

Creates a new oracle governance proposal for the PMP contract.

This function allows authorized oracle participants to propose administrative actions such as:

* Setting staking and result time windows
* Resolving the event
* Cancelling the event

```solidity
function createProposal(uint32 function_type, TvmCell data) public;
```

Parameters

* **`function_type`** - Identifier of the proposed action.\
  Must correspond to one of the supported `FUNCTION_TYPE_*` constants:
  * `FUNCTION_TYPE_SET_STAKE_DEADLINE`
  * `FUNCTION_TYPE_SET_RESOLVE`
  * `FUNCTION_TYPE_CANCEL_EVENT`
* **`data`** - ABI-encoded payload required for the specified function type.

**Behavior**:

`createProposal` initializes a new governance proposal and automatically casts the creator’s vote.

The proposal is identified deterministically by:

```
proposalId = hash(function_type, data)
```

If there is only **one oracle**, the proposal is executed immediately.

Otherwise, the proposal remains active until:

* It collects enough votes (based on threshold), or
* Its deadline expires (7 days from creation).

### Governance Flow

1. Oracle creates proposal → creator vote is automatically counted.
2. Other oracles call `vote(proposalId)`.
3. Once vote threshold is reached:
   * `executeProposal` is called.
4. Proposal is deleted after execution.
5. If deadline expires before threshold:
   * Proposal is discarded.

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

Returns the full current state of the PMP contract.

This function provides a comprehensive snapshot of the pool configuration, lifecycle state, oracle status, and pool balances.

```solidity
function getDetails() external view returns (
        string name,
        uint32 token_type,
        uint256 event_id,
        uint256 oracle_list_hash,
        address deployer,
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
        mapping(uint32 => mapping(uint8 => uint128)) typedOutcomePools,
        mapping(uint32 => string) outcomeNames
    )
```

**Returns** :

**General Information**

* **`name`** - Human-readable pool name.
* **`token_type`** - Static token type used for the pool.
* **`event_id`** - Identifier of the associated event.
* **`oracle_list_hash`** - Hash of the oracle list used during deployment.
* **`deployer`** - Address of the `PrivateNote` wallet that deployed the contract.
* **`privateNoteCodeHash`** - Hash of the `PrivateNote` contract code used for address derivation.

**Pool State**

* **`totalPool`**  - Total amount currently in the pool (after fees and redistributions).
* **`approved`** - Indicates whether the event is approved and staking is enabled.
* **`isCancelled`** - Indicates whether the event has been cancelled.
* **`resolvedOutcome`** - Final outcome identifier, if resolved.

**Outcomes**

* **`numOutcomes`** - Total number of outcomes.
* **`outcomeNames`** - Mapping of outcome identifiers to human-readable names.
* **`typedOutcomePools`** - Pool balances separated by:
  * Outcome ID
  * Bet type (0 = clean, 1 = debt, 2 = coupon)

**Time Windows**

* **`stakeStart`** - Stake acceptance start timestamp.
* **`stakeEnd`** - Stake acceptance end timestamp.
* **`resultStart`** - Result acceptance start timestamp.
* **`resultEnd`** - Result acceptance end timestamp.

**Oracle Governance**

* **`numberOfOracleEvents`** -Total number of required oracle confirmations.
* **`approvedOracleEvents`** -Number of oracle confirmations received.

***

### `getVersion`

Returns the implementation version and kind identifier.

```solidity
function getVersion() external pure returns (string, string);
```

**Returns**:

* `semver`: version string
* `kind`: contract kind string (expected `"PMP"`)
