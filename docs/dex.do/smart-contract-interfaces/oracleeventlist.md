---
description: OracleEventList Contract Interface Documentation
---

# OracleEventList

## Overview

**OracleEventList** is a registry contract owned by an **Oracle**.\
It defines which events the oracle is willing to service, along with the associated fees and deadlines, and acts as a validation layer during **PMP** **(Prediction Market Pool)** deployment and event confirmation.

Each `OracleEventList` is deployed by an `Oracle` contract and is uniquely bound to it.

***

## Contract Metadata

* **Contract name:** `OracleEventList`
* **Role:** Oracle event registry & validation contract
* **Owner:** Oracle contract (`_oracle`)
* **Authorization model:** Oracle public key (`_oracle_pubkey`)
* **Associated contracts:** `Oracle`, `PMP`, `PrivateNote`

***

## Storage Overview

#### Static & State Variables

* `_oracle` (`address`, static)\
  Address of the owning `Oracle` contract.
* `_oracle_pubkey` (`uint256`)\
  Public key used to authorize oracle actions.
* `_PrivateNoteCode` (`TvmCell`)\
  Code of the `PrivateNote` contract (used for PMP address derivation).
* `_pmpCode` (`TvmCell`)\
  Code of the `PMP` contract.
* `_events` (`mapping(uint256 => EventInfo)`)\
  Mapping of supported oracle events by `event_id`.

***

## Events

### `EventAdded`

```solidity
event EventAdded(
    uint256 event_id,
    string event_name,
    uint128 oracle_fee,
    uint64 deadline
);
```

Emitted when the oracle registers a new event.

**Parameters:**

* `event_id` — Hash identifier of the event
* `event_name` — Human-readable event name
* `oracle_fee` — Fee required for oracle service
* `deadline` — Timestamp until which the oracle is willing to service the event

***

### `EventConfirmed`

```solidity
event EventConfirmed(
    uint256 event_id,
    address pmpAddress
);
```

Emitted when the oracle confirms willingness to service an event requested by a PMP.

**Parameters:**

* `event_id` — Event identifier
* `pmpAddress` — Address of the requesting PMP contract

***

## Constructor

#### `constructor`

```solidity
constructor(
    uint256 pubkey,
    TvmCell PrivateNoteCode,
    TvmCell pmpCode
)
```

Initializes the event list and binds it to an `Oracle`.

**Parameters:**

* `pubkey` — Oracle public key
* `PrivateNoteCode` — `PrivateNote` contract code
* `pmpCode` — `PMP` contract code

**Behavior:**

* Sets the deploying address as the oracle owner
* Stores contract codes for PMP address verification

***

## Oracle Event Management

### `addEvent`

```solidity
function addEvent(
    string event_name,
    uint128 oracle_fee,
    uint64 deadline
) public onlyOwnerPubkey accept;
```

Registers a new event that the oracle is willing to service.

**Parameters:**

* `event_name` — Human-readable event name
* `oracle_fee` — Required oracle fee in shell tokens
* `deadline` — Timestamp after which the event is no longer valid

**Requirements:**

* `deadline` must be greater than the current block timestamp
* Callable only by the oracle owner (`_oracle_pubkey`)

**Side Effects:**

* Stores event metadata in `_events`
* Emits `EventAdded`

***

## PMP Interaction

### `confirmEvent`

```solidity
function confirmEvent(
    uint256 event_id,
    uint256 oracle_list_hash,
    uint32 token_type
) public view accept;
```

Confirms or rejects a PMP request to use the oracle for a specific event.

**Access Control:**

* Callable **only** by a valid `PMP` contract\
  (address is verified using `DexLib.computePMPAddress`)

**Parameters:**

* `event_id` — Event identifier
* `oracle_list_hash` — Hash of the oracle list used by PMP
* `token_type` — Token type associated with the PMP

**Behavior:**

1. Transfers received shell tokens to the owning `Oracle`
2. Rejects the request if:
   * Event does not exist
   * Deadline has passed
   * Provided fee is insufficient
3. Approves the request otherwise

**Outgoing Calls:**

* `PMP.rejectEvent()` — if validation fails
* `PMP.approveEvent(oracle_pubkey)` — if validation succeeds

**Emits:**

* `EventConfirmed` on successful approval

***

## View Functions

### `getVersion`

```solidity
function getVersion() external pure returns (string);
```

Returns the contract identifier.

**Returns:**

* `"OracleEventList"`

***

## Internal Mechanics (Informational)

### Balance Safety

The contract maintains a minimum native balance via an internal `ensureBalance()` helper.\
If the balance drops below `MIN_BALANCE`, shell tokens are minted automatically.

***

## Access Control Summary

| Function       | Access              |
| -------------- | ------------------- |
| `addEvent`     | Oracle owner only   |
| `confirmEvent` | Authorized PMP only |
| `getVersion`   | Public              |

***

## Notes & Caveats

* `event_id` is computed as `tvm.hash(event_name)`
* Fees are transferred immediately to the owning `Oracle`
* Event approval is **time-bound** via `deadline`
* Event existence and fee sufficiency are validated atomically
