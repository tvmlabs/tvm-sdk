---
description: (Work in progress) OracleEventList Contract Interface Documentation
---

# OracleEventList

{% file src="../../.gitbook/assets/OracleEventList.abi.json" %}

## Overview

**OracleEventList** is a contract that manages a list of events an Oracle is willing to service.\
It allows the Oracle to publish events, confirm or cancel participation via **Prediction Market Pool (PMP)** contracts, and manage event lifecycle state.

Each `OracleEventList` is uniquely identified by:

* the Oracle address
* a static index

***

## Events

### EventAdded

Emitted when a new event is added to the OracleEventList.

```solidity
event EventAdded(
    uint256 event_id,
    string event_name,
    uint128 oracle_fee,
    uint64 deadline
);
```

* `event_id` — unique identifier (hash) of the event
* `event_name` — human-readable name of the event
* `oracle_fee` — fee required by the Oracle
* `deadline` — timestamp until which the Oracle is willing to service the event

***

### EventConfirmed

Emitted when an Oracle confirms participation in an event via a PMP contract.

```solidity
event EventConfirmed(uint256 event_id, address pmpAddress);
```

* `event_id` — identifier of the confirmed event
* `pmpAddress` — address of the PMP contract that initiated confirmation

***

## Public & External Interface

### **`addEvent`**

Adds a new event that the Oracle is willing to service.

```solidity
function addEvent(
    string event_name,
    uint128 oracle_fee,
    uint64 deadline,
    string describe,
    mapping(uint32 => string) outcomeNames,
    optional(uint256) trustAddr
)
    public
    onlyOwnerPubkey(_oracle_pubkey)
    accept;
```

**Access:** oracle owner only\
**Modifiers:** `onlyOwnerPubkey`, `accept`

**Parameters:**

* `event_name` — human-readable event name
* `oracle_fee` — Oracle fee for servicing the event
* `deadline` — timestamp until which the event is valid
* `describe` — detailed event description
* `outcomeNames` — mapping of outcome IDs to outcome names
* `trustAddr` — optional trusted address for the event

**Behavior:**

* Validates that the deadline is in the future
* Ensures sufficient native balance
* Requires at least 2 and fewer than 20 outcomes
* Computes a deterministic `event_id` from event parameters
* Stores event information in contract storage
* Emits `EventAdded` to an external address

***

### **`confirmEvent`**

Confirms Oracle participation in an event.

```solidity
function confirmEvent(
    uint256 event_id,
    uint256 oracle_list_hash,
    uint32 token_type
)
    public
    senderIs(
        DexLib.computePMPAddress(
            _PrivateNoteCode,
            _pmpCode,
            event_id,
            oracle_list_hash,
            token_type
        )
    )
    accept;
```

**Access:** PMP contract only\
**Modifiers:** `senderIs`, `accept`

**Parameters:**

* `event_id` — identifier of the event
* `oracle_list_hash` — hash of the oracle list
* `token_type` — token type used by the PMP

**Behavior:**

* Ensures sufficient native balance
* Transfers received fees to the Oracle owner
* Rejects the event if it does not exist
* Rejects the event if:
  * the deadline has passed
  * the paid fee is lower than the Oracle fee
* Approves the event via the PMP contract if all conditions are met
* Emits `EventConfirmed` upon successful confirmation

***

### **`cancelEvent`**

Cancels Oracle participation in an event.

```solidity
function cancelEvent(
    uint256 event_id,
    uint256 oracle_list_hash,
    uint32 token_type
)
    public
    senderIs(
        DexLib.computePMPAddress(
            _PrivateNoteCode,
            _pmpCode,
            event_id,
            oracle_list_hash,
            token_type
        )
    )
    accept;
```

**Access:** PMP contract only\
**Modifiers:** `senderIs`, `accept`

**Behavior:**

* Ensures sufficient native balance
* Decreases the confirmation counter for the event

***

### **`deleteEvent(uint256 event_id)`**

Deletes an event from the OracleEventList.

```solidity
function deleteEvent(uint256 event_id)
    public
    onlyOwnerPubkey(_oracle_pubkey)
    accept;
```

**Access:** oracle owner only\
**Modifiers:** `onlyOwnerPubkey`, `accept`

**Behavior:**

* Ensures sufficient native balance
* Deletes the event if:
  * no active confirmations exist, or
  * the event deadline has passed

***

### **`getVersion()`**

Returns the contract version information.

```solidity
function getVersion() external pure returns (string, string);
```

**Returns:**

* Semantic version string (e.g. `"1.0.0"`)
* Contract identifier string: `"OracleEventList"`

