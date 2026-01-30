---
description: Oracle Contract Interface Documentation
---

# Oracle

{% file src="../../.gitbook/assets/Oracle.abi.json" %}

## Overview

**Oracle** is a core contract responsible for managing Oracle events and deploying `OracleEventList` contracts.\
It provides authorization via an oracle public key, supports fee withdrawal, event list deployment, and helper utilities for proposal data encoding.

***

## Events

### OracleEventListDeployed

Emitted when a new `OracleEventList` contract is deployed.

```solidity
event OracleEventListDeployed(address eventListAddress, uint128 index);
```

* `eventListAddress` — address of the deployed OracleEventList
* `index` — index identifier of the event list

***

### EventPublished

Emitted when an event is published by the Oracle.

```solidity
event EventPublished(uint256 event_id, string event_name);
```

* `event_id` — unique identifier of the event
* `event_name` — human-readable event name

***

## Public & External Interface

### **`deployEventList`**

Deploys a new `OracleEventList` with a specified index.

```solidity
function deployEventList(uint128 index)
    public
    view
    onlyOwnerPubkey(_oraclePubkey)
    accept;
```

**Access:** oracle owner only\
**Modifiers:** `onlyOwnerPubkey`, `accept`

**Parameters:**

* `index` — index identifier of the new OracleEventList

**Behavior:**

* Ensures the contract has sufficient native balance
* Deploys a new `OracleEventList` for the given index
* Emits `OracleEventListDeployed` with the deployed address and index

***

### **`withdrawFees`**

Withdraws accumulated fees to a specified address.

```solidity
function withdrawFees(address to, uint128 amount)
    public
    view
    onlyOwnerPubkey(_oraclePubkey)
    accept;
```

**Access:** oracle owner only\
**Modifiers:** `onlyOwnerPubkey`, `accept`

**Parameters:**

* `to` — recipient address
* `amount` — amount of fees to withdraw

**Behavior:**

* Transfers the specified amount in shell currency to the recipient
* Uses a minimal attached value for the transfer

***

### **`getCellForProposalSetStakeDeadline`**

Encodes staking and result submission deadlines into a `TvmCell`.

```solidity
function getCellForProposalSetStakeDeadline(
    uint64 stakeStart,
    uint64 stakeEnd,
    uint64 resultStart,
    uint64 resultEnd
) public pure returns (TvmCell);
```

**Parameters:**

* `stakeStart` — staking period start timestamp
* `stakeEnd` — staking period end timestamp
* `resultStart` — result submission start timestamp
* `resultEnd` — result submission end timestamp

**Returns:**

* Encoded `TvmCell` containing all timestamps

***

### **`getCellForProposalSetResolve`**

Encodes event resolution data into a `TvmCell`.

```solidity
function getCellForProposalSetResolve(uint32 outcomeId)
    public
    pure
    returns (TvmCell);
```

**Parameters:**

* `outcomeId` — identifier of the winning outcome

**Returns:**

* Encoded `TvmCell` containing the outcome ID

***

### **`getEventListAddress`**

Returns the address of an `OracleEventList` for a given index.

```solidity
function getEventListAddress(uint128 index)
    external
    view
    returns (address);
```

**Parameters:**

* `index` — index of the OracleEventList (currently index `0` is supported)

**Returns:**

* Address of the corresponding OracleEventList contract

***

### **`getVersion`**

Returns the contract version information.

```solidity
function getVersion() external pure returns (string, string);
```

**Returns:**

* Semantic version string (e.g. `"1.0.0"`)
* Contract identifier string: `"Oracle"`
