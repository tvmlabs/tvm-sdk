---
description: PMP Contract Interface Documentation
---

# Prediction Market Pool (PMP)

## Overview

**PMP (Prediction Market Pool)** is a decentralized prediction market contract that aggregates user stakes on discrete outcomes of an event.\
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

```solidity
event StakeAccepted(address note, uint32 outcomeId, uint128 amount);
```

Emitted when a stake is successfully accepted.

***

### `ApprovedByOracle`

```solidity
event ApprovedByOracle(address oracleEventList, uint256 oraclePubkey);
```

Emitted when all oracle event lists approve the PMP.

***

### `Resolved`

```solidity
event Resolved(uint32 outcomeId);
```

Emitted when the event outcome is resolved.

***

### `ClaimProcessed`

```solidity
event ClaimProcessed(
    address note,
    uint32 outcomeId,
    uint128 payout,
    bool win
);
```

Emitted when a claim is processed.

***

### Governance & Lifecycle Events

```solidity
event NetworkFeeBurned(uint64 amount);
event TimingsSet(uint64 stakeStart, uint64 stakeEnd, uint64 resultStart, uint64 resultEnd);
event NumOutcomesSet(uint32 numOutcomes);
event EventCancelled();
event ProposalCreated(uint256 proposalId, uint32 functionType, TvmCell data);
event ProposalExecuted(uint256 proposalId, uint32 functionType, TvmCell data);
```

***

## Constructor

#### `constructor`

```solidity
constructor(
    uint256 deposit_identifier_hash,
    string name,
    uint32 token_type,
    address[] oracle_event_lists,
    uint128[] oracle_fees
)
```

Deploys a new PMP and requests approval from all listed oracle event lists.

**Parameters:**

* `deposit_identifier_hash` — Identifies the deploying `PrivateNote`
* `name` — Unique PMP name
* `token_type` — Token used for staking
* `oracle_event_lists` — OracleEventList contracts
* `oracle_fees` — Fees paid to each oracle

**Behavior:**

* Verifies deployer is the expected `PrivateNote`
* Requests confirmation from each oracle
* PMP becomes inactive until all approvals are received

***

## Oracle Approval Flow

### `approveEvent`

```solidity
function approveEvent(uint256 oracle_pubkey) public;
```

Confirms oracle approval for the PMP.

**Access Control:**

* Callable only by approved `OracleEventList` contracts

**Notes:**

* PMP activates only when **all** oracle event lists approve

***

### `rejectEvent`

```solidity
function rejectEvent() public;
```

Rejects the PMP and self-destructs the contract.

***

## Staking

### `acceptStake`

```solidity
function acceptStake(
    uint32 outcomeId,
    uint128 stakeAmount,
    uint256 deposit_identifier_hash
) public;
```

Accepts a stake from a `PrivateNote`.

**Requirements:**

* PMP approved by all oracles
* Within staking time window
* Outcome ID valid

**Side Effects:**

* Updates pools and counters
* Notifies `PrivateNote` via callback

***

### `cancelStake`

```solidity
function cancelStake(
    uint32 outcomeId,
    uint128 stakeAmount,
    uint256 deposit_identifier_hash
) public;
```

Cancels a stake and refunds the user.

**Conditions:**

* Event cancelled **or**
* Staking period still open

***

## Resolution

### `resolve` (governance)

```solidity
function resolve(uint32 outcomeId) private;
```

Resolves the event outcome.

**Requirements:**

* PMP approved
* Within result window
* Executed via oracle proposal

***

### `cancelEvent` (governance)

Cancels the event and allows stake refunds.

***

## Claims

### `claim`

```solidity
function claim(
    uint128 stakeAmount,
    uint32 outcomeId,
    uint256 deposit_identifier_hash
) public;
```

Processes a user claim.

**Behavior:**

* If unresolved → zero payout
* If lost → zero payout
* If won → proportional payout

**Formula:**

```
payout = stakeAmount * totalPool / winningOutcomePool
```

**Finalization:**

* Contract self-destructs when all winning claims are processed

***

## Oracle Governance

### `createProposal`

```solidity
function createProposal(uint32 function_type, TvmCell data) public;
```

Creates a governance proposal.

**Proposal Types:**

* Set number of outcomes
* Set timing windows
* Resolve outcome
* Cancel event

***

### `vote`

```solidity
function vote(uint256 proposalId) public;
```

Votes on an existing proposal.

**Threshold:**

* Proposal executes when votes ≥ `THRESHOLD`

***

### `executeProposal`

```solidity
function executeProposal(uint256 proposalId) private;
```

Executes an approved proposal and applies changes.

***

## View Functions

### `getDetails`

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

Returns full PMP state and configuration.

***

### `getVersion`

```solidity
function getVersion() external pure returns (string);
```

**Returns:** `"PMP"`

***

## Access Control Summary

| Actor           | Permissions          |
| --------------- | -------------------- |
| PrivateNote     | Stake, cancel, claim |
| OracleEventList | Approve / reject PMP |
| Oracle pubkeys  | Governance proposals |
| Public          | View functions       |

***

## Notes & Caveats

* PMP is inactive until **all oracles approve**
* Outcomes and timings are immutable once approved
* Governance is **oracle-majority-based**
* Contract self-destructs after final claim resolution

